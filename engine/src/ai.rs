//! The AI faction (spec 16): enumerate every legal action, score it, spend greedily.

use crate::combat::first_round_odds;
use crate::data::{VictoryFirstKind, VictorySecondKind};
use crate::ids::*;
use crate::orders::*;
use crate::state::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cat {
    Producer,
    RaiseIndustry,
    /// Ticket #490 (version 0.09.9): a Colony or station to its next tier.
    RaiseTier,
    ResearchLab,
    /// Ticket #81 (version 0.06.0): the Observatory, weighted apart from the Lab.
    Observatory,
    Habitat,
    LaunchSiteOrShipyard,
    ColonyShip,
    Warship,
    /// Ticket #343 (version 0.09.1): the Missile Carrier, and the Launch it was bought for.
    MissileCarrier,
    Launch,
    ArmyOrBarracks,
    /// Ticket #36: an Embassy or a Relay.
    BuildInfluence,
    /// Ticket #227 (version 0.08.2): offering an Accord. Without this the computer seats never
    /// propose one and the whole system is invisible in a game they play among themselves -- which
    /// the first sweep after the Accords were built showed exactly: zero standing at the end of 20
    /// games. The Accords ticket recorded that as its own largest risk; this is the answer to it.
    Accord,
    /// Ticket #496 (version 0.09.9): a Trade, one thing for one thing.
    Trade,
    /// Ticket #52: a Constabulary, Relief and Resettle.
    Constabulary,
    /// Ticket #389 (version 0.09.3): a Stadium, after a Constabulary.
    Stadium,
    /// Ticket #411 (version 0.09.4): a Nature Reserve.
    NatureReserve,
    Relief,
    Resettle,
    /// Ticket #267 (version 0.08.4): a Smear campaign against a rival.
    Smear,
    /// Ticket #277 (version 0.08.5): a Greenwash of the seat's own Blame.
    Greenwash,
    /// Ticket #268 (version 0.08.4): carbon credits bought from the Custodians.
    BuyCredits,
    /// Ticket #269 (version 0.08.4): Agitate in a rival's Region.
    Agitate,
    /// Ticket #51: divert this turn's Research into the Archive fund.
    FundArchive,
    /// Ticket #51: build the Archive; one Module since ticket #68.
    BuildArchive,
    /// Ticket #192 (version 0.08.0): read Colonists at the Archive's place into it.
    Upload,
    Influence,
    Transit,
    LoadUnload,
    FoundColony,
    /// Ticket #54: the Scrubber, which took Restoration's place and its Stabilization gap.
    Scrubber,
    /// Ticket #56: the Sea Wall, raised before the sea takes the coastal slot it stands in.
    SeaWall,
    /// Ticket #54: Mothball, Restart and Decommission, on a Facility or a Module.
    Mothball,
    Restart,
    Decommission,
    /// Ticket #54: the Custodians' Leapfrog and the Prospectors' Strip Permit.
    Leapfrog,
    StripPermit,
    StanceAttack,
    StanceIntercept,
    StanceHold,
    StanceEvade,
    /// Ticket #278 (version 0.08.5): a warship stack blockading a rival station's slot.
    StanceBlockade,
    /// Ticket #297 (version 0.08.6): dig in.
    StanceDigIn,
}

/// Ticket #50 removed the denial multiplier: every AI pursues its own Victory Condition and never
/// spends a multiplier on holding a rival back.
#[derive(Debug, Clone)]
struct Candidate {
    orders: Vec<Order>,
    cat: Cat,
    base: f64,
    gap: f64,
    threat: f64,
    opportunity: f64,
    note: String,
    /// The stack a Stance candidate belongs to; one Stance per stack.
    stack: Option<String>,
    /// Ticket #332 (version 0.09.0): the pace of a build at its place -- 1.0 for everything that
    /// is not a build, and for a build at the seat's place whose Widgets would finish it soonest
    /// behind its queue; less, down to `build_pace_floor`, at a place that would take longer.
    pace: f64,
}

impl Candidate {
    fn score(&self) -> f64 {
        self.base * self.gap * self.threat * self.opportunity * self.pace
    }
}

/// Which part of its Victory Condition a seat is furthest behind on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Behind {
    First,
    Presence,
}

impl Game {
    /// Ticket #99 (version 0.07.0): the Orbital Slot a warship of this seat should arrive into at
    /// `body`: the slot of the richest rival station there, by Modules standing, and nothing for a
    /// Ship that is not a warship or a Body where no rival keeps a station. The slot is chosen with
    /// the leg, so this reads the board as it stands when the Ship departs.
    /// Ticket #284 (version 0.08.5): whether this seat has cause to open a fight at a place. A
    /// neutral place needs none; a place it lost to an Occupation still running may be retaken;
    /// a place a rival holds only if the seat has cause against that rival (`has_cause`: `war_cause`, Wary),
    /// the AI's first reading of Relations for war. A seat that keeps every rival Neutral is
    /// marched on by nobody.
    /// Ticket #446 (version 0.09.6): a rival's share of this turn's emissions, every seat's sources
    /// counted; nought before the first Climate phase has run.
    pub fn emissions_share(&self, rival: Seat) -> f64 {
        let by = &self.climate.last.by_seat;
        let total: f64 = by.iter().sum();
        if total <= 0.0 { 0.0 } else { by[rival.index()] / total }
    }

    /// Ticket #446 (version 0.09.6): **the Custodians' aggression follows a rival's CO2.** Against a
    /// rival above a fair quarter of this turn's emissions, the computer Custodians weigh every hostile
    /// act -- Smear, Agitate, Influence on its places, attack, march, Blockade -- by `1 + emitter_k ×
    /// (share − fair)`, at most `emitter_cap`. One for any other seat, or any other rival.
    pub fn emitter_lift(&self, seat: Seat, rival: Seat) -> f64 {
        let th = &self.tables.ai.thresholds;
        let fair = self.tables.influence.blame.fair_share;
        let share = self.emissions_share(rival);
        if self.kind(seat) != FactionKind::Custodians || rival == seat || share <= fair {
            return 1.0;
        }
        (1.0 + th.emitter_k * (share - fair)).min(th.emitter_cap)
    }

    /// Ticket #531 (version 0.1.0.1): the pressure a seat at `own` of its Victory puts on a leader
    /// at `lead`: the leader's ramp from `gang_from` to the end, times the seat's trail past
    /// `gang_ahead` over `gang_full`; nought short of either bar.
    pub fn gang_pressure_of(lead: f64, own: f64, th: &crate::data::AiThresholds) -> f64 {
        if lead < th.gang_from || lead - own < th.gang_ahead || th.gang_full <= 0.0 {
            return 0.0;
        }
        let ramp = ((lead - th.gang_from) / (1.0 - th.gang_from).max(1e-9)).clamp(0.0, 1.0);
        let trail = ((lead - own - th.gang_ahead) / th.gang_full).clamp(0.0, 1.0);
        ramp * trail
    }

    /// Ticket #531: this seat's pressure on `rival`, if `rival` is its leader -- the rival highest
    /// in Victory, by the Victory window's share -- and nought otherwise.
    pub fn ai_gang_pressure(&self, seat: Seat, rival: Seat) -> f64 {
        if seat == rival {
            return 0.0;
        }
        let share = |s: Seat| self.progress(s).score();
        let lead = share(rival);
        if Seat::ALL.into_iter().any(|r| r != seat && r != rival && share(r) > lead) {
            return 0.0;
        }
        Game::gang_pressure_of(lead, share(seat), &self.tables.ai.thresholds)
    }

    /// Ticket #531: the most pressure this seat puts on any leader, which draws it to the pack.
    pub fn ai_gang_felt(&self, seat: Seat) -> f64 {
        Seat::ALL.into_iter().map(|r| self.ai_gang_pressure(seat, r)).fold(0.0, f64::max)
    }

    /// Ticket #531: what every hostile act on `rival` is weighed by -- the Custodians' emitter lift,
    /// times one and the pressure on a leader.
    pub fn hostile_lift(&self, seat: Seat, rival: Seat) -> f64 {
        self.emitter_lift(seat, rival) * (1.0 + self.ai_gang_pressure(seat, rival))
    }

    /// Ticket #446 (version 0.09.6): whether this seat has cause against a rival -- Relations at
    /// `war_cause` or worse (−5, Wary), or for the computer Custodians against a rival above a fair
    /// quarter of the emissions, `emitter_cause` (−3). Every hostile act's gate reads this.
    pub fn has_cause(&self, seat: Seat, rival: Seat) -> bool {
        let th = &self.tables.ai.thresholds;
        let bar = if self.emitter_lift(seat, rival) > 1.0 { th.emitter_cause } else { th.war_cause };
        // Ticket #447 (version 0.09.6): a Blockade is cause, at once. Relations fall one a turn of
        // it, so a blockaded seat's warships sat on Hold through the turns that mattered.
        // Ticket #531 (version 0.1.0.1): and a runaway leader is cause in itself from half pressure.
        rival != seat && (self.relations_score(seat, rival) <= bar || self.blockaded_by(seat, rival) || self.ai_gang_pressure(seat, rival) >= th.gang_cause)
    }

    /// Ticket #447 (version 0.09.6): whether a place of this seat's is starved by that rival's
    /// Blockade.
    pub fn blockaded_by(&self, seat: Seat, rival: Seat) -> bool {
        self.colonies.iter().any(|c| c.control.director() == Some(seat) && self.starved_by(c.id) == Some(rival))
    }

    /// Ticket #447 (version 0.09.6): whether any place of this seat's is under Blockade.
    pub fn blockaded(&self, seat: Seat) -> bool {
        self.colonies.iter().any(|c| c.control.director() == Some(seat) && self.starved_by(c.id).is_some())
    }

    pub fn war_cause_at(&self, seat: Seat, place: Place) -> bool {
        match self.place_control(place) {
            Control::Neutral => true,
            Control::Controlled(r) => r != seat && self.has_cause(seat, r),
            Control::Occupied { occupier, previous, .. } => previous == Some(seat) || (occupier != seat && self.has_cause(seat, occupier)),
        }
    }

    /// Ticket #99 (version 0.07.0): the Orbital Slot a warship of this seat should arrive into at
    /// `body` -- the richest rival station there, by Modules standing, and nothing for a Ship that
    /// is not a warship or a Body where no rival keeps a station. The slot is chosen with the leg,
    /// so this reads the board as it stands when the Ship departs.
    ///
    /// Ticket #335 (version 0.09.0): and only a station this seat MEANS to shut. Until this ticket
    /// every warship leg anywhere named the richest rival station whatever the seat thought of its
    /// holder, so no warship ever held low orbit -- which is the lane to the ground and the orbit
    /// Orbital Control is now of. A station is meant now when the seat is Wary or worse toward its
    /// holder (`war_cause`, the bar a march and a Bombard read), and when that holder keeps no
    /// working Battery in the station's own ring, where a Blockade shuts nothing (ticket #324).
    /// The reading is the director's, as the Blockade stance's is, so the leg and the stance it
    /// serves name the same station.
    pub fn ai_blockade_slot(&self, seat: Seat, body: BodyId, kind: UnitKind) -> Option<u32> {
        if !kind.is_warship() {
            return None;
        }
        // Ticket #355 (version 0.09.1): a station its holder has put a Battery on is still a ring
        // worth going to when the seat's warships at this Body TOGETHER clear the Attack bar
        // against that Battery -- they gather there and the Attack stance takes it. Refusing
        // every defended ring outright meant a bought Battery ended the contest with no fight:
        // traced, one blockader held at odds 0.58 and then left.
        let fleet: i64 = self.ships.iter().filter(|s| s.seat == seat && s.at == ShipAt::Body(body) && s.kind.is_warship() && !s.escaped).map(|s| self.ship_strength(s)).sum();
        let bar = self.tables.ai.thresholds.attack_odds;
        self.colonies
            .iter()
            .filter(|c| c.in_orbit && c.body == body && self.rival_holds(seat, c))
            .filter(|c| {
                c.control.director().is_some_and(|h| {
                    let guard = self.battery_strength(h, body, Orbit::Slot(c.slot));
                    self.has_cause(seat, h) && (guard == 0 || first_round_odds(fleet, guard) >= bar)
                })
            })
            .max_by_key(|c| (c.modules.len(), c.colonists))
            .map(|c| c.slot)
    }

    /// Ticket #335 (version 0.09.0): whether this seat wants the GROUND of a Body -- a landing, a
    /// founding, a Bombard of a ground Colony, or simply denying a rival the surface it already
    /// keeps a Colony on. What a seat that wants the ground wants is **low orbit**, the one orbit
    /// that touches the surface: Orbital Control is held there, and since ticket #570 (version
    /// 0.1.0.3) a Blockade there is what shuts the surface, so its warships go there and the
    /// contest its stance reads is low orbit's.
    pub fn ai_wants_the_ground(&self, seat: Seat, body: BodyId) -> bool {
        // A Colony of its own on the surface to keep, or one a rival holds that it has cause
        // against: either way the surface is the thing, and the surface is shut from low orbit --
        // by a Blockade there since ticket #570 (version 0.1.0.3), by Orbital Control before.
        let on_the_ground = self.colonies.iter().any(|c| {
            !c.in_orbit
                && c.body == body
                && match c.control.director() {
                    Some(d) if d == seat => true,
                    Some(d) => self.has_cause(seat, d),
                    None => false,
                }
        });
        let bound_here = |s: &Ship| s.seat == seat && (s.at == ShipAt::Body(body) || matches!(s.at, ShipAt::Transit { to, .. } if to == body));
        // A hull of its own carrying people to a free slot, or an Army to land: a founding and a
        // landing both wait on low orbit.
        // Ticket #489 (version 0.09.9): with four aboard, the least that founds.
        let settling = !self.free_slots_on(body).is_empty() && self.ships.iter().any(|s| bound_here(s) && s.colonists >= self.tables.emigrants.found_with);
        let landing = self.ships.iter().any(|s| bound_here(s) && s.army.is_some());
        // Ticket #363 (version 0.09.1): an armed Missile Carrier of its own here with a target on the
        // ground -- a Region, over Earth, or a ground Colony -- fires from low orbit held outright, so
        // the garrison that holds it stays. Traced: in 12 of 14 turns an armed carrier sat ready in
        // Earth's low orbit, its seat's warships left for a ring in the same turn and the Launch
        // failed at Resolution with the orbit no longer held.
        let firing = self.ships.iter().any(|s| bound_here(s) && s.kind == UnitKind::MissileCarrier && s.warhead)
            && self.nuke_targets(seat).iter().any(|t| match t {
                Place::State(_) => body == BodyId::Earth,
                Place::Colony(c) => self.colony(*c).is_some_and(|c| c.body == body && !c.in_orbit),
            });
        on_the_ground || settling || landing || firing
    }

    /// Ticket #335 (version 0.09.0): this seat's warships holding low orbit at a Body, counting the
    /// ones on the way whose leg named no ring. `low_orbit_warships` of them and Orbital Control is
    /// covered; a hull beyond that is the one that flies up to a rival's ring to shut it.
    fn ai_low_orbit_warships(&self, seat: Seat, body: BodyId) -> u32 {
        self.ships
            .iter()
            .filter(|s| s.seat == seat && s.kind.is_warship() && !s.escaped)
            .filter(|s| self.ship_in_orbit(s, body, Orbit::Low) || matches!(s.at, ShipAt::Transit { to, .. } if to == body && s.slot.is_none()))
            .count() as u32
    }

    /// Ticket #335 (version 0.09.0): whether low orbit is what this seat's next warship at a Body
    /// is for -- the ground is wanted there and too few of its hulls are holding the lane to it.
    fn ai_low_orbit_wanted(&self, seat: Seat, body: BodyId) -> bool {
        self.ai_wants_the_ground(seat, body) && self.ai_low_orbit_warships(seat, body) < self.tables.ai.thresholds.low_orbit_warships
    }

    /// Ticket #335 (version 0.09.0): the hulls that hold low orbit at a Body for this seat -- its
    /// first `low_orbit_warships` warships standing there, by Ship id. A NAMED garrison and not a
    /// count, because a count oscillates: two hulls in low orbit each read "one other is holding
    /// it" and both fly up to the ring, then each reads "nobody is holding it" and both come back
    /// down, a Fuel a turn for ever. The named hull stays, and every hull past it is spare.
    fn ai_low_orbit_garrison(&self, seat: Seat, body: BodyId) -> Vec<ShipId> {
        let mut held: Vec<ShipId> = self
            .ships
            .iter()
            .filter(|s| s.seat == seat && s.kind.is_warship() && !s.escaped && self.ship_in_orbit(s, body, Orbit::Low))
            .map(|s| s.id)
            .collect();
        held.sort();
        held.truncate(self.tables.ai.thresholds.low_orbit_warships as usize);
        held
    }

    /// Ticket #346 (version 0.09.1): **what the Fuel a Battle would cost is worth against the
    /// prize.** A Battle now takes `[melee] battle_fuel` out of every tank in it, and that is the
    /// same figure a warship must hold to hold Orbital Control, to blockade and to intercept. So a
    /// seat whose whole armed line in the contested orbit would fall UNDER the bar by paying the
    /// charge is about to win a fight and lose the orbit in the same breath; and where it holds
    /// nothing at that Body, there was no ground the orbit was wanted for either. That is the
    /// "strands its fleet for nothing" case, and it is discounted by `battle_fuel_weight`.
    ///
    /// A WEIGHT and never a prohibition, at the designer's word: it multiplies the ODDS the seat
    /// reads against `attack_odds`, so a discounted Battle is one that has to look better to be
    /// worth a tank, and at 1.0 the weighing is off and nothing about the old reading moves. It is
    /// a figure in `ai.toml` and not an `if` in the code. Anything the
    /// seat holds at the Body -- a Colony on the ground or a station of its own in orbit, the same
    /// predicate the attack's own cause already reads -- makes the orbit worth the tank, and the
    /// appetite is whole. `orbit` is the contested orbit, or None where the stance covers the
    /// whole Body, as the stack's own reading of the contest has it.
    pub fn ai_battle_fuel_weight(&self, seat: Seat, body: BodyId, orbit: Option<Orbit>) -> f64 {
        let charge = self.tables.melee.battle_fuel;
        let here = |s: &Ship| match orbit {
            Some(o) => self.ship_in_orbit(s, body, o),
            None => s.at == ShipAt::Body(body),
        };
        let line: Vec<&Ship> = self.ships.iter().filter(|s| s.seat == seat && s.kind.is_warship() && !s.escaped && here(s)).collect();
        if line.is_empty() {
            return 1.0;
        }
        let all_stranded = line.iter().all(|s| (s.fuel - charge as f64).max(0.0) < charge as f64);
        let holds_something_here = self.colonies.iter().any(|c| c.body == body && c.control.controller() == Some(seat));
        if all_stranded && !holds_something_here { self.tables.ai.thresholds.battle_fuel_weight } else { 1.0 }
    }

    /// Ticket #335 (version 0.09.0): **the orbit a leg names before it leaves**. LOW ORBIT by
    /// default, which is the lane to the ground and the orbit Orbital Control is held in; a rival
    /// station's ring where the seat means to blockade or attack that station; its OWN station's
    /// ring where the Ship is going there to unload, to refuel or to sit. Until this ticket a
    /// warship's leg always named the richest rival station's slot and every other leg named
    /// nothing at all, so no computer seat ever chose an orbit for a reason.
    /// Ticket #442 (version 0.09.6): what the choice below names, or where it names nothing at a Body
    /// with no low orbit (Venus), the seat's own station's ring, else the first.
    pub fn ai_destination_orbit(&self, seat: Seat, ship: &Ship, to: BodyId) -> Option<u32> {
        self.ai_destination_orbit_chosen(seat, ship, to).or_else(|| self.arrival_slot(seat, to))
    }

    fn ai_destination_orbit_chosen(&self, seat: Seat, ship: &Ship, to: BodyId) -> Option<u32> {
        // The unload: people aboard go to a station of this seat's with room for them, and a
        // station is touched from its own ring alone.
        if ship.colonists > 0
            && let Some(c) = self
                .colonies
                .iter()
                .filter(|c| c.in_orbit && c.body == to && c.control.director() == Some(seat) && self.habitat_room(c) > c.colonists)
                .max_by_key(|c| self.habitat_room(c) - c.colonists)
        {
            return Some(c.slot);
        }
        // The blockade, and the attack: a warship names the ring of the richest rival station it
        // means to shut. Where the GROUND is what its seat wants at that Body it names no ring at
        // all, since low orbit is the one orbit Orbital Control is held in and Control is what the
        // surface waits on. Read before the tank below, because a hull sent to hold the lane holds
        // it on the Fuel it arrives with; the station it might have docked at is still there next
        // turn, an orbit change away.
        if ship.kind.is_warship() {
            if self.ai_low_orbit_wanted(seat, to) {
                return None;
            }
            if let Some(slot) = self.ai_blockade_slot(seat, to, ship.kind) {
                return Some(slot);
            }
        }
        // The refuel, and the dock: a tank that will not pay the way back off this leg wants the
        // ring of a station that fills it, since a station fuels only a Ship in its own orbit.
        // Ticket #396 (version 0.09.3): a Refinery Colony's depot is low orbit, which is where a
        // leg lands when no ring is named, so it needs no choice here.
        if let ShipAt::Body(from) = ship.at {
            let cost = self.transit_cost_for(seat, from, to).1;
            if ship.fuel - cost < cost && let Some(c) = self.colonies.iter().find(|c| c.in_orbit && c.body == to && self.fuels_for(c, seat)) {
                return Some(c.slot);
            }
        }
        None
    }

    /// Ticket #530 (version 0.1.0.1): **the race for the Research Lead.** The Lead picks the next
    /// Tech, and only the Lead's shortlist keeps a place for its gate, so a seat whose gate waits
    /// races for the Lead: from the turn its gate is open to research, or its first part stands at
    /// `gate_pick_fraction`, until the gate is researched. The race runs the harder the nearer its
    /// Victory, at the Victory window's share (since version 0.1.0.2 the weighted mean of the parts). Nought when not racing.
    pub fn ai_race_share(&self, seat: Seat) -> f64 {
        let Some(gate) = self.tables.victory_gate(self.kind(seat)) else { return 0.0 };
        if self.has_tech(gate) {
            return 0.0;
        }
        // The review: the cheap test first.
        let open = self.research.current == Some(gate) || self.available_techs().contains(&gate);
        let p = self.progress(seat);
        if !open && p.first_fraction_as_it_stands() < self.tables.ai.thresholds.gate_pick_fraction {
            return 0.0;
        }
        p.score().clamp(0.0, 1.0)
    }

    /// Ticket #530: what the race multiplies a Lab's and an Observatory's weight by.
    pub fn ai_race_boost(&self, seat: Seat) -> f64 {
        1.0 + self.tables.ai.thresholds.race_lab_weight * self.ai_race_share(seat)
    }

    fn base_weight(&self, seat: Seat, cat: Cat) -> f64 {
        let w = self.tables.ai_weights(self.kind(seat));
        match cat {
            Cat::Producer => w.build_producer,
            Cat::RaiseIndustry => w.raise_industry,
            Cat::RaiseTier => w.raise_tier,
            // Ticket #530 (version 0.1.0.1): the race for the Lead.
            Cat::ResearchLab => w.build_research_lab * self.ai_race_boost(seat),
            Cat::Observatory => w.build_observatory * self.ai_race_boost(seat),
            Cat::Habitat => w.build_habitat,
            Cat::LaunchSiteOrShipyard => w.build_launch_site_or_shipyard,
            Cat::ColonyShip => w.build_colony_ship,
            Cat::Warship => w.build_warship,
            Cat::MissileCarrier => w.build_missile_carrier,
            Cat::Launch => w.launch,
            Cat::ArmyOrBarracks => w.build_army_or_barracks,
            Cat::BuildInfluence => w.build_influence,
            Cat::Constabulary => w.build_constabulary,
            Cat::Stadium => w.build_stadium,
            Cat::NatureReserve => w.build_nature_reserve,
            Cat::Relief => w.relief,
            Cat::Resettle => w.resettle,
            Cat::Smear => w.smear,
            Cat::Greenwash => w.greenwash,
            Cat::BuyCredits => w.buy_credits,
            Cat::Agitate => w.agitate,
            Cat::Accord => w.accord,
            Cat::Trade => w.trade,
            Cat::FundArchive => w.fund_archive,
            Cat::BuildArchive => w.build_archive,
            Cat::Upload => w.upload,
            Cat::Influence => w.influence,
            Cat::Transit => w.transit,
            Cat::LoadUnload => w.load_unload,
            Cat::FoundColony => w.found_colony,
            Cat::Scrubber => w.build_scrubber,
            Cat::SeaWall => w.build_sea_wall,
            Cat::Mothball => w.mothball,
            Cat::Restart => w.restart,
            Cat::Decommission => w.decommission,
            Cat::Leapfrog => w.leapfrog,
            Cat::StripPermit => w.strip_permit,
            Cat::StanceAttack => w.stance_attack,
            Cat::StanceIntercept => w.stance_intercept,
            Cat::StanceHold => w.stance_hold,
            Cat::StanceEvade => w.stance_evade,
            Cat::StanceBlockade => w.stance_blockade,
            Cat::StanceDigIn => w.stance_dig_in,
        }
    }

    /// Pace (spec 16.3): the value the schedule expects now, by linear interpolation.
    fn expected(schedule: &[[i64; 2]], turn: u32) -> f64 {
        let t = turn as f64;
        let mut prev = (0.0, 0.0);
        for [tt, v] in schedule {
            let (tt, v) = (*tt as f64, *v as f64);
            if t <= tt {
                let span = tt - prev.0;
                if span <= 0.0 {
                    return v;
                }
                return prev.1 + (v - prev.1) * (t - prev.0) / span;
            }
            prev = (tt, v);
        }
        prev.1
    }

    /// The measure this seat's Victory Condition counts first (ticket #50).
    fn first_kind(&self, seat: Seat) -> VictoryFirstKind {
        self.tables.faction(self.kind(seat)).victory_first.kind
    }

    /// Ticket #410 (version 0.09.4): whether a computer seat offers a Constabulary or a Stadium in
    /// this Region at all: a Constabulary from `constabulary_from`; a Stadium from `stadium_from`,
    /// where a Constabulary stands, or where the Region has `stadium_alone_free_slots` slot left and
    /// no Constabulary on order to take it.
    pub fn ai_offers_calming(&self, sid: StateId, kind: FacilityKind) -> bool {
        let th = &self.tables.ai.thresholds;
        let unrest = self.state(sid).unrest;
        match kind {
            FacilityKind::Constabulary => unrest >= th.constabulary_from,
            FacilityKind::Stadium => {
                let queued = self.state(sid).queue.iter().any(|b| matches!(b.item, BuildItem::Facility(k) if k.does_the_job_of(FacilityKind::Constabulary)));
                unrest >= th.stadium_from && (self.constabulary_online(sid) || (!queued && self.free_slots(sid) <= th.stadium_alone_free_slots))
            }
            _ => false,
        }
    }

    /// Ticket #410 (version 0.09.4): a Mothball's weight in a restive Region, where the point of
    /// Unrest it adds, which nothing damps, costs most. In a Colony, or a calm Region, no change.
    pub fn ai_mothball_price(&self, b: BuildingRef) -> f64 {
        let th = &self.tables.ai.thresholds;
        match b {
            BuildingRef::Facility(sid, _) if self.state(sid).unrest >= th.mothball_restive_from => th.mothball_restive_factor,
            _ => 1.0,
        }
    }

    /// Ticket #421 (version 0.09.4): the market Buy that brings the seat's Fuel up to `need`, or
    /// `None` where it holds enough already or its Ducats cannot pay for the shortfall.
    pub fn ai_fuel_top_up(&self, seat: Seat, need: f64) -> Option<Order> {
        let short = (need - self.seat(seat).stockpile.fuel).ceil() as i64;
        if short <= 0 {
            return None;
        }
        let buy = Order::Buy { resource: Resource::Fuel, amount: short };
        (self.order_cost(seat, &buy).ducats <= self.seat(seat).stockpile.ducats).then_some(buy)
    }

    /// Ticket #419 (version 0.09.4): how much more a Faction eased by its foundings (the
    /// Arkwrights) weighs founding a ground Colony or building a station, by its most restive
    /// Region: one at `founding_pull_from`, rising in a line to double at `founding_pull_double_at`.
    /// One for a Faction whose foundings ease nothing.
    pub fn ai_founding_pull(&self, seat: Seat) -> f64 {
        let card = self.tables.faction(self.kind(seat));
        if card.found_colony_unrest_ease <= 0.0 && card.found_station_unrest_ease <= 0.0 {
            return 1.0;
        }
        let th = &self.tables.ai.thresholds;
        let worst = self.controlled_states(seat).iter().map(|s| self.state(*s).unrest).fold(0.0, f64::max);
        let span = th.founding_pull_double_at - th.founding_pull_from;
        if worst < th.founding_pull_from || span <= 0.0 {
            return 1.0;
        }
        (1.0 + (worst - th.founding_pull_from) / span).min(2.0)
    }

    /// Ticket #410 (version 0.09.4): Relief's weight at this Unrest: none under `relief_from`, one
    /// there, rising in a line to double at `relief_double_at` (the Facilities' 7 and the throw-off's
    /// 10 either side of it) and no higher.
    pub fn ai_relief_weight(&self, unrest: f64) -> Option<f64> {
        let th = &self.tables.ai.thresholds;
        if unrest < th.relief_from {
            return None;
        }
        let span = th.relief_double_at - th.relief_from;
        Some(if span <= 0.0 { 2.0 } else { (1.0 + (unrest - th.relief_from) / span).min(2.0) })
    }

    /// Victory gap multiplier and the part it applies to.
    fn victory_gap(&self, seat: Seat) -> (f64, Behind) {
        let pace = self.tables.ai_pace(self.kind(seat));
        let m = &self.tables.ai.multipliers;
        let turn = self.turn;
        let ratio_of = |actual: f64, expected: f64| if expected <= 0.0 { 1.0 } else { (actual / expected).min(1.0) };
        let mut presence_ratio = ratio_of(self.victory_colonists(seat) as f64, Self::expected(&pace.colonists, turn));
        // Ticket #449 (version 0.09.6): a second part that counts BODIES is paced in Bodies. Nothing
        // read them: the Arkwrights' computer judged itself by Colonists off Earth alone, filled one
        // station over Earth, and scored nought with no Body settled.
        let second = self.tables.faction(self.kind(seat)).victory_second;
        if second.kind == VictorySecondKind::ColoniesOnBodies && !pace.bodies.is_empty() {
            let bodies = ratio_of(self.bodies_settled(seat, second.colonists_each) as f64, Self::expected(&pace.bodies, turn));
            presence_ratio = presence_ratio.min(bodies);
        }
        // And one that counts UPLOADS is paced in Uploads, at the designer's word: the Archivists'
        // computer judged itself by Colonists living off Earth, which an Upload takes away.
        // Only once the Archive stands complete: before that there is nothing to Upload into, and a
        // seat read as behind on Uploads from turn 1 chased Colonists and never built the Archive
        // (measured: 3 Archives in 80 games, no Archivist win).
        if second.kind == VictorySecondKind::ColonistsUploaded && !pace.uploads.is_empty() && self.archive_complete(seat) {
            let uploads = ratio_of(self.seat(seat).uploaded as f64, Self::expected(&pace.uploads, turn));
            presence_ratio = presence_ratio.min(uploads);
        }
        let first_ratio = match self.first_kind(seat) {
            // A Stabilization run has no useful interpolation: the pace is in ppm off the Sink.
            // Ticket #578 (version 0.1.0.3): the net the run reads, cards and permafrost counted.
            VictoryFirstKind::StabilizationRun => {
                let net = self.climate.last.net();
                if turn >= pace.under_sink_by_turn {
                    if net < 0.0 { 1.0 } else { (1.0 - net / 10.0).clamp(0.0, 1.0) }
                } else if turn >= pace.within_by_turn {
                    if net <= pace.within_ppm { 1.0 } else { (pace.within_ppm / net).clamp(0.0, 1.0) }
                } else {
                    1.0
                }
            }
            _ => ratio_of(self.progress(seat).first_value, Self::expected(&pace.first, turn)),
        };
        let (ratio, behind) = if first_ratio <= presence_ratio { (first_ratio, Behind::First) } else { (presence_ratio, Behind::Presence) };
        let mult = if ratio >= 1.0 { 1.0 } else { 1.0 + (m.victory_gap_max - 1.0) * ((1.0 - ratio) / 0.5).min(1.0) };
        (mult, behind)
    }

    /// Ticket #56: a Sea Wall standing or on order in this Nation State (at most one may).
    fn sea_wall_committed(&self, sid: StateId) -> bool {
        let st = self.state(sid);
        st.facilities.iter().any(|f| f.kind == FacilityKind::SeaWall) || st.queue.iter().any(|b| b.item == BuildItem::Facility(FacilityKind::SeaWall))
    }

    /// Ticket #56: a Sea Level threshold of any kind -- one still scheduled for this state, or the
    /// Ice Sheets Break -- standing within 0.2 C of the Temperature, with a coast still to lose.
    pub(crate) fn sea_is_close(&self, sid: StateId) -> bool {
        if self.coastal_slots(sid) == 0 {
            return false;
        }
        let now = self.climate.temperature;
        let scheduled = self
            .tables
            .climate
            .sea_level_thresholds
            .iter()
            .enumerate()
            .filter(|(i, _)| !self.state(sid).thresholds_fired[*i])
            .map(|(_, t)| *t);
        let breaks = self
            .tables
            .climate
            .breaks
            .iter()
            .enumerate()
            .filter(|(i, b)| b.effect == crate::data::BreakEffect::SeaLevelThreshold && !self.climate.breaks_fired[*i])
            .map(|(_, b)| b.temperature);
        scheduled.chain(breaks).any(|t| t - now <= 0.2)
    }

    /// Ticket #430 (version 0.09.5): under the fog a rival Ship AT a Body is always counted there
    /// (whose and how many is open), but one in flight toward it only while its books are open.
    fn enemy_present_or_inbound(&self, seat: Seat, body: BodyId) -> bool {
        self.ships.iter().any(|s| s.seat != seat && (s.at == ShipAt::Body(body) || (matches!(s.at, ShipAt::Transit { to, .. } if to == body) && self.sees_ship(seat, s))))
    }

    /// Ticket #50: an Army of ANY other seat, not only one rival's.
    fn enemy_army_near(&self, seat: Seat, place: Place) -> bool {
        let theirs = |a: &Army| self.army_seat(a).map(|o| o != seat).unwrap_or(false);
        match place {
            Place::State(s) => {
                let mut near = vec![s];
                near.extend(self.tables.state(s).neighbours.iter().copied());
                // Ticket #321 (version 0.08.8): a Region's own Army marched out is a threat like any other.
                self.armies.iter().any(|a| !self.army_at_home(a) && theirs(a) && matches!(a.at, ArmyAt::Place(Place::State(x)) if near.contains(&x)))
            }
            Place::Colony(c) => {
                let body = self.colony(c).map(|c| c.body);
                self.armies.iter().any(|a| theirs(a) && a.at == ArmyAt::Place(place))
                    // Ticket #430 (version 0.09.5): an Army aboard is cargo, seen only on a Ship the seat sees.
                    || body.map(|b| self.ships.iter().any(|s| s.seat != seat && s.army.is_some() && self.sees_ship(seat, s) && (s.at == ShipAt::Body(b) || matches!(s.at, ShipAt::Transit { to, .. } if to == b)))).unwrap_or(false)
            }
        }
    }

    /// The total strength of every other seat's Ships at a Body (ticket #50): a Battle there is a
    /// melee, so the odds preview counts all of them.
    /// Ticket #324 (version 0.08.8): a rival's Batteries at the Body count, since they stand in
    /// the line of any Battle there.
    pub fn enemy_ship_strength(&self, seat: Seat, body: BodyId) -> i64 {
        seat.others().iter().map(|s| self.ship_stack_strength(*s, body) + self.battery_strength_at_body(*s, body)).sum()
    }

    /// Whether any other seat directs this Colony.
    fn rival_holds(&self, seat: Seat, c: &Colony) -> bool {
        c.control.director().map(|d| d != seat).unwrap_or(false)
    }

    /// Ticket #320 (version 0.08.8): whether Passage with `other` is worth this seat's offering:
    /// it is Cordial or better toward them (a score of 3 or more; Friendly at first, and no pair
    /// of computer seats was ever Friendly in eighty games, so the designer set the band below)
    /// and holds a Region next door to one they hold, so an Army of either could use it.
    /// Accepted at Neutral or better, as non-aggression is.
    /// Ticket #325 (version 0.08.8): Refuel is worth offering where the other seat holds a station
    /// at a Body this seat has Ships or a Colony at and no station of its own: the one place the
    /// term would change what its Ships can do.
    pub fn refuel_worth_offering(&self, seat: Seat, other: Seat) -> bool {
        BodyId::ALL.into_iter().any(|body| {
            // Ticket #396 (version 0.09.3): a Refinery Colony's depot counts as a station does.
            self.own_depot_at(other, body)
                && !self.own_depot_at(seat, body)
                && (self.ships.iter().any(|s| s.seat == seat && s.at == ShipAt::Body(body)) || self.colonies.iter().any(|c| c.body == body && c.control.director() == Some(seat)))
        })
    }

    pub fn passage_worth_offering(&self, seat: Seat, other: Seat) -> bool {
        matches!(self.relations_level(seat, other), "Cordial" | "Friendly")
            && StateId::ALL.into_iter().any(|s| {
                self.state(s).control == Control::Controlled(seat) && self.tables.state(s).neighbours.iter().any(|n| self.state(*n).control == Control::Controlled(other))
            })
    }

    /// Ticket #319 (version 0.08.8): whether a Carrier has somewhere to go: a rival's Colony off
    /// Earth whose holder this seat has cause against (`war_cause_at`, Wary or worse). The
    /// Prospectors' Carrier appetite never asked for cause and still does not; every other seat's
    /// begins here. Taking such a Colony moves its Colonists from the holder's off-world count to
    /// the taker's, which is the Arkwrights' Victory denied, the review's "Diaspora denial".
    pub fn carrier_target_exists(&self, seat: Seat) -> bool {
        self.colonies.iter().any(|c| c.body != BodyId::Earth && self.rival_holds(seat, c) && self.war_cause_at(seat, Place::Colony(c.id)))
    }

    /// Ticket #57: what one Colony Slot's own yields are worth to the part the AI is furthest
    /// behind on. Every slot has its own four figures now, so the AI reads the slot, not the Body.
    fn slot_worth(&self, seat: Seat, y: SlotYields, behind: Behind) -> f64 {
        // Ticket #140 (version 0.07.3): a Habitat holds the same everywhere now, so a slot is worth
        // the same to Presence wherever it is, and the Energy that runs the Habitats decides; the
        // fourth yield is Research, which the science-first Factions read.
        match behind {
            Behind::Presence => y.generator,
            Behind::First => match self.first_kind(seat) {
                VictoryFirstKind::VentureFund => y.mine + y.refinery,
                VictoryFirstKind::ColonistsOffEarth => y.generator,
                VictoryFirstKind::StabilizationRun | VictoryFirstKind::ResearchProduced | VictoryFirstKind::ArchiveResearch => y.generator + y.research,
            },
        }
    }

    /// Ticket #82, the 0.06.0 AI sweep (ticket #94): a Module off Earth whose kind an idle Facility
    /// of the seat's would double (Production Moved) is worth twice its base while the seat holds
    /// more Facilities of the paired kind than Modules already doubled. Until the sweep the
    /// Custodian AI never built a Module off Earth in eight batches of twenty seeds: Earth's
    /// Facilities outscored them at the same base and the Materials reserve starved the rest.
    fn production_moved_boost(&self, seat: Seat, col: &Colony, mk: ModuleKind) -> f64 {
        if !self.off_earth(col) {
            return 1.0;
        }
        let pairs = &self.tables.faction(self.kind(seat)).mothball_pairs;
        let Some((fk, _)) = pairs.iter().find(|(_, m)| **m == mk) else { return 1.0 };
        // Ticket #426 (version 0.09.5): by the job, so a held Reactor counts as a Power Plant.
        let facilities = self.directed_states(seat).iter().flat_map(|s| self.state(*s).facilities.iter()).filter(|f| f.kind.common().unwrap_or(f.kind) == *fk).count();
        let doubled = self.doubled_modules(seat).iter().filter(|(cid, i)| self.colony(*cid).and_then(|c| c.modules.get(*i)).map(|m| m.kind == mk).unwrap_or(false)).count();
        if facilities > doubled {
            // Ticket #563 (version 0.1.0.3): the factor the rule applies, 2.08.
            self.tables.faction(self.kind(seat)).production_moved_factor
        } else {
            1.0
        }
    }

    /// Spec 16.4, as ticket #57 leaves it: the free Colony Slot on a Body whose own yields best
    /// serve the part the AI is furthest behind on.
    pub fn best_slot_for(&self, seat: Seat, body: BodyId, behind: Behind) -> Option<u32> {
        self.free_slots_on(body).into_iter().max_by(|a, b| {
            let (wa, wb) = (self.slot_worth(seat, self.slot_yields(body, *a), behind), self.slot_worth(seat, self.slot_yields(body, *b), behind));
            wa.partial_cmp(&wb).unwrap_or(std::cmp::Ordering::Equal)
        })
    }

    /// The Body whose best free slot serves that part best (spec 16.4, ticket #57), weighed by the
    /// share of the game left that the flight from Earth would eat: a Mars seventeen turns away is
    /// worth little next to a Moon one turn away, and a Body the Ship cannot reach before the last
    /// turn is not offered at all.
    fn best_body_for(&self, seat: Seat, behind: Behind, ship: ShipId) -> BodyId {
        let t = &self.tables;
        let turns_left = t.victory.turns.saturating_sub(self.turn).max(1) as f64;
        let flight = |b: BodyId| self.transit_cost_for(seat, BodyId::Earth, b).0 as f64;
        // The 0.06.0 AI sweep (ticket #94): a leg no tank could pay (Mars off its window can ask
        // 47 Fuel of a 30 tank, a Colony Ship's 40 since ticket #420) is not a destination either; before this the AI named it as its
        // one choice, the Transit was refused at the check, and the Ship sat at Earth.
        // Ticket #413 (version 0.09.4): the seat's own tanks, Clean Propellant's Fuel included.
        let tank = UnitKind::SHIPS.iter().map(|k| self.tank_of(seat, *k)).fold(0.0, f64::max);
        let payable = |b: BodyId| self.transit_cost_for(seat, BodyId::Earth, b).1 <= tank;
        // Ticket #93: Venus, with no Colony Slots, is a destination when the seat holds a station
        // there with room, or when a slot is free in its orbit and the Stockpile could raise one.
        let venus_open = |b: BodyId| {
            b == BodyId::Venus
                && (self.colonies.iter().any(|c| c.body == b && c.control.director() == Some(seat) && self.habitat_room(c) > c.colonists)
                    || (!self.free_orbital_slots(b).is_empty() && self.seat(seat).stockpile.materials >= self.station_materials(seat)))
        };
        let mut bodies: Vec<BodyId> = BodyId::ALL
            .into_iter()
            .filter(|b| *b != BodyId::Earth && (!self.free_slots_on(*b).is_empty() || venus_open(*b)) && flight(*b) < turns_left && payable(*b))
            .collect();
        if bodies.is_empty() {
            return BodyId::Moon;
        }
        // Ticket #564 (version 0.1.0.3), at the designer's word ("the race is for the Moon"): while
        // the Moon is unclaimed with a free slot and the seat directs no ground Colony off Earth, its
        // first Ship flies there, whatever the windfalls say. Measured before: the race weight alone
        // built the Ship early and the windfalls sent it past the Moon, the Prospectors to Mars by
        // turn 11, and they took 53 of 80 games.
        if self.first_at(BodyId::Moon).is_none() && bodies.contains(&BodyId::Moon) && !self.colonies.iter().any(|c| c.control.director() == Some(seat) && !c.in_orbit && c.body != BodyId::Earth) {
            return BodyId::Moon;
        }
        // Ticket #596 (version 0.1.0.4), at the designer's word: the race to Mars, once the Moon is
        // claimed -- Mars, whatever the windfalls say, for the seat's runner alone, where the tank
        // pays the crossing now or at the window and it lands before the game ends. The crossing
        // keeps the computer's soft window preference (`window_preference`), at the designer's word,
        // over a hold to two turns of the window that was built and taken back; a crossing the tank
        // cannot pay this turn is refused at the check, and the Ship goes when it can.
        if self.mars_race_open() && self.mars_runner(seat, ship) {
            let depart = if self.transit_cost_for(seat, BodyId::Earth, BodyId::Mars).1 <= tank { self.turn } else { self.next_window_turn(self.turn) };
            let (turns, fuel) = self.transit_cost_for_at(seat, BodyId::Earth, BodyId::Mars, depart);
            if ((depart - self.turn + turns) as f64) < turns_left && fuel <= tank {
                return BodyId::Mars;
            }
        }
        // Ticket #51, the Arkwrights' spread rule: once one Body of theirs holds the 4 Colonists
        // Diaspora asks of each, the next Colony goes to a Body they are not on yet.
        let each = t.faction(self.kind(seat)).victory_second.colonists_each;
        let spreading = each > 0 && BodyId::ALL.into_iter().any(|b| b != BodyId::Earth && self.colonists_at_body(seat, b) >= each);
        let key = |b: &BodyId| -> f64 {
            // Ticket #93: Venus has no slot to weigh; a station there is worth a plain slot.
            // The 0.06.0 AI sweep (ticket #94): a station at Venus is worth what a slot with the
            // Body's own yields would be, so Venus competes with the Moon and Mars on the same scale.
            let yields = self
                .best_slot_for(seat, *b, behind)
                .map(|s| self.slot_worth(seat, self.slot_yields(*b, s), behind))
                .unwrap_or(if *b == BodyId::Venus { self.slot_worth(seat, SlotYields::of_body(self.tables.body(*b)), behind) } else { 0.0 });
            let fresh = if spreading && self.colonists_at_body(seat, *b) == 0 { 10.0 } else { 0.0 };
            // Ticket #345 (version 0.09.1): and what being FIRST to the Body would pay. A world
            // nobody has settled carries a windfall for whoever lands on it first, and reading it
            // here is what makes a distant Body worth the voyage: without this the destination
            // list is yields against flight time, the Moon wins it from turn one, and no computer
            // seat founded a Colony in the Mars system in eighty measured games. It falls to
            // nought the moment the Body is claimed, so the second Colony Ship looks further out
            // than the first did. Venus is excluded by its own board: no ground slot, no landing,
            // and its figure is nought besides.
            let first = if self.first_at(*b).is_none() && !self.free_slots_on(*b).is_empty() {
                t.body(*b).first_windfall as f64 * t.ai.thresholds.first_windfall_worth
            } else {
                0.0
            };
            (yields + fresh + first) * (1.0 - flight(*b) / turns_left)
        };
        bodies.sort_by(|a, b| key(b).partial_cmp(&key(a)).unwrap());
        bodies[0]
    }

    /// Ticket #57: how much the AI wants a crossing into the Mars system this turn rather than at
    /// the window. On the window turn it wants it fully; off the window it wants it less, in
    /// proportion to how far off it is -- but a seat behind on its pace goes anyway, so the
    /// discount is lifted once the victory gap has opened.
    fn window_preference(&self, seat: Seat, from: BodyId, to: BodyId) -> f64 {
        let Some(offset) = self.crossing_offset(from, to, self.turn) else { return 1.0 };
        let (gap, _) = self.victory_gap(seat);
        if gap > 1.0 {
            return 1.0;
        }
        // A quarter of the weight at the far side of the cycle, all of it at the window.
        1.0 - 0.75 * (offset.abs() / 180.0).clamp(0.0, 1.0)
    }

    /// Ticket #596 (version 0.1.0.4): the race to Mars is on -- the Moon claimed, Mars itself with no
    /// ground Colony and a free slot.
    fn mars_race_open(&self) -> bool {
        self.tables.ai.thresholds.mars_race_weight > 0.0 && self.first_at(BodyId::Moon).is_some() && self.first_at(BodyId::Mars).is_none() && !self.free_slots_on(BodyId::Mars).is_empty()
    }

    /// Ticket #596 (the review): this Colony Ship is the seat's runner for Mars -- none of its others
    /// is flying there, and none loaded to found waits at Earth ahead of it (the lowest id first).
    fn mars_runner(&self, seat: Seat, ship: ShipId) -> bool {
        let found = self.tables.emigrants.found_with;
        !self.ships.iter().any(|o| {
            o.seat == seat
                && o.kind == UnitKind::ColonyShip
                && o.id != ship
                && (matches!(o.at, ShipAt::Transit { to: BodyId::Mars, .. }) || (o.at == ShipAt::Body(BodyId::Earth) && o.colonists >= found && o.id.0 < ship.0))
        })
    }

    /// Ticket #57: whether the Mars window is close enough that Fuel is worth holding for it.
    fn window_within(&self, turns: u32) -> bool {
        self.next_window_turn(self.turn).saturating_sub(self.turn) <= turns
    }

    /// Every Energy upkeep the seat pays now: Facilities, Modules, Ships and Armies.
    fn total_upkeep(&self, seat: Seat) -> i64 {
        self.unit_upkeep(seat) as i64
            // Ticket #54: a mothballed building pays no upkeep, so it is no part of the drain.
            + self.directed_states(seat).iter().flat_map(|s| self.state(*s).facilities.iter()).filter(|f| !f.mothballed).map(|f| self.tables.facility(f.kind).energy_upkeep).sum::<i64>()
            // The review of #565: the yield's figure, so a dark Module counts nothing, the Archive
            // its 20 plus its uploaded, and an incomplete Archive nothing.
            + self
                .directed_colonies(seat)
                .iter()
                .map(|c| {
                    let col = self.colony(*c).unwrap();
                    let dark = self.unmanned_modules(col);
                    col.modules.iter().enumerate().filter(|(i, m)| !m.mothballed && !dark.contains(i)).map(|(i, _)| self.module_yield_at(seat, *c, i).upkeep).sum::<f64>()
                })
                .sum::<f64>()
                .round() as i64
    }

    /// Energy the seat's producers make in a turn, at current multipliers.
    pub fn energy_production(&self, seat: Seat) -> i64 {
        let fac = self.tables.faction(self.kind(seat)).output_multiplier;
        let grids = if self.has_tech(TechId::EfficientGrids) { self.tables.tech(TechId::EfficientGrids).value } else { 1.0 };
        let mut e = 0.0;
        for sid in self.directed_states(seat) {
            let lean = if self.tables.state(sid).resource_lean == Resource::Energy { 1.5 } else { 1.0 };
            for f in self.state(sid).facilities.iter().filter(|f| !f.mothballed) {
                // Ticket #559 (version 0.1.0.3): the card's figure, so a Wind Farm counts its 3 beside a
                // Power Plant's 6, where a flat 6 was counted for every Power Plant's job.
                if let Some(p) = &self.tables.facility(f.kind).produces
                    && p.resource == Resource::Energy
                {
                    e += (p.amount * lean * fac * grids).floor();
                }
            }
        }
        for cid in self.directed_colonies(seat) {
            let col = self.colony(cid).unwrap();
            // The review of #565: a dark Generator makes nothing.
            let dark = self.unmanned_modules(col);
            for (_, m) in col.modules.iter().enumerate().filter(|(i, m)| !m.mothballed && !dark.contains(i)) {
                if m.kind == ModuleKind::Generator {
                    // Ticket #57: the Colony's slot makes the Energy, not the Body's average.
                    e += (5.0 * self.colony_yields(col).generator * fac * grids).floor();
                }
                // Ticket #630 (version 0.1.0.6), from the review: a Reactor Module's card figure, no
                // site yield; uncounted, a seat of Reactors read itself short of power for ever.
                if m.kind == ModuleKind::FissionReactor
                    && let Some(p) = &self.tables.module(m.kind).produces
                {
                    e += (p.amount * fac * grids).floor();
                }
            }
        }
        e as i64
    }

    /// Spec 16.2: the resource the seat is shortest of, relative to its next three affordable actions.
    /// Energy is a draining balance, so its shortage is measured in turns of drain over those actions.
    pub fn scarcest(&self, seat: Seat) -> Resource {
        let s = self.seat(seat).stockpile;
        // The three cheapest builds it could afford now stand in for "its next three affordable actions".
        // Ticket #537 (version 0.1.0.1): the rows carry tenths.
        let mut costs: Vec<(f64, i64)> = Vec::new();
        for f in &self.tables.facilities {
            costs.push((f.materials, f.energy_upkeep));
        }
        for m in &self.tables.modules {
            costs.push((m.materials, m.energy_upkeep));
        }
        for u in &self.tables.units {
            costs.push((u.materials, u.energy_upkeep));
        }
        costs.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));
        // Ticket #537: the floor is the cheapest paid row, where it was the literal 20 that row once was.
        let cheapest = costs.iter().map(|(m, _)| *m).find(|m| *m > 0.0).unwrap_or(0.0);
        let next: Vec<(f64, i64)> = costs.into_iter().filter(|(m, _)| *m <= s.materials.max(cheapest)).take(3).collect();
        let need_materials: f64 = next.iter().map(|(m, _)| *m).sum::<f64>().max(1.0);
        let need_upkeep: i64 = next.iter().map(|(_, u)| *u).sum::<i64>();
        let drain = (self.total_upkeep(seat) + need_upkeep - self.energy_production(seat)).max(0);
        let energy_ratio = if drain == 0 { f64::INFINITY } else { s.energy / (3.0 * drain as f64) };
        let materials_ratio = s.materials / need_materials;
        let has_ships = self.ships.iter().any(|x| x.seat == seat);
        // Ticket #575 (version 0.1.0.3): and the Fuel its power burns, three turns of it, kept as a
        // reserve: what stands under the reserve counts for nothing, so a tank at the reserve reads
        // as empty and Fuel is the scarcest resource.
        let burn = self.fuel_burned_by_buildings(seat);
        let reserve = self.fuel_reserve(seat);
        let fuel_ratio = if has_ships || burn > 0.0 { (s.fuel - reserve).max(0.0) / (6.0 + 3.0 * burn) } else { f64::INFINITY };
        if energy_ratio <= materials_ratio && energy_ratio <= fuel_ratio {
            Resource::Energy
        } else if materials_ratio <= fuel_ratio {
            Resource::Materials
        } else {
            Resource::Fuel
        }
    }

    /// Ticket #43: a Carrier is worth building when the seat has a free Army on Earth, a rival Colony
    /// to land on, and no empty Carrier (or one on the way) already.
    /// Ticket #319 (version 0.08.8): any seat with CAUSE against the holder of a rival Colony off
    /// Earth wants one, by the same predicate the marches read (`war_cause_at`, Wary or worse),
    /// at the designer's word; the Prospectors want one as before, cause or none. Until this
    /// ticket the other three seats never carried an Army anywhere, and no Army was landed at a
    /// Colony in eighty games.
    fn wants_carrier(&self, seat: Seat) -> bool {
        let free_army = self.armies.iter().any(|a| !a.standing && self.army_seat(a) == Some(seat) && matches!(a.at, ArmyAt::Place(Place::State(_))));
        let enemy_colony = self.carrier_target_exists(seat);
        let empty_carrier = self.ships.iter().any(|s| s.seat == seat && s.kind == UnitKind::Carrier && s.army.is_none());
        let queued = self.states.iter().flat_map(|s| s.queue.iter()).any(|b| b.seat == seat && b.item == BuildItem::Unit(UnitKind::Carrier));
        free_army && enemy_colony && !empty_carrier && !queued
    }

    /// Ticket #343 (version 0.09.1): what a Missile Carrier of this seat's is for -- the place it
    /// most wants and cannot take. A place qualifies when a RIVAL directs it, the seat has cause
    /// against that rival (`war_cause_at`, the bar every other weapon reads), and the seat cannot
    /// take it either way it knows how: no Army of its own stands there, so no Occupation is under
    /// way or in reach, and its own Standing there is not within the challenge margin, so no short
    /// push wins it. Ranked by what is standing on it, which is what a nuke is for: the buildings
    /// first and the people after.
    ///
    /// THE READING OF "cannot take" IS THE BUILD'S CHOICE, not the designer's: the resolution says
    /// "the holding it most wants and cannot take" and does not say by what test.
    pub fn nuke_target(&self, seat: Seat) -> Option<Place> {
        self.nuke_targets(seat).into_iter().next()
    }

    /// Ticket #343 (version 0.09.1): all of them, richest first, so a hull that cannot reach the
    /// best one is offered the best one it CAN reach rather than nothing. Without this a carrier at
    /// Mars would sit idle whenever a Region on Earth outranked the Colony above it, which -- a
    /// Region being millions of people and a Colony a dozen -- is nearly always.
    pub fn nuke_targets(&self, seat: Seat) -> Vec<Place> {
        let mut places: Vec<Place> = StateId::ALL.into_iter().map(Place::State).collect();
        places.extend(self.colonies.iter().map(|c| Place::Colony(c.id)));
        let mut out: Vec<Place> = places
            .into_iter()
            .filter(|p| matches!(self.place_director(*p), Some(h) if h != seat))
            .filter(|p| self.war_cause_at(seat, *p))
            .filter(|p| !self.armies.iter().any(|a| self.army_seat(a) == Some(seat) && a.at == ArmyAt::Place(*p)))
            .filter(|p| {
                let holder = self.place_director(*p).unwrap_or(seat);
                let mine = self.seat(seat).influence.get(p).copied().unwrap_or(0);
                let theirs = self.seat(holder).influence.get(p).copied().unwrap_or(0);
                mine < theirs + self.challenge_margin_at(*p)
            })
            .collect();
        out.sort_by_key(|p| std::cmp::Reverse(self.nuke_worth(*p)));
        out
    }

    /// Ticket #343 (version 0.09.1): what is standing at a place, for the ranking above -- its
    /// buildings, weighted over its people, since buildings are what the roll takes.
    fn nuke_worth(&self, place: Place) -> i64 {
        match place {
            Place::State(s) => self.state(s).facilities.len() as i64 * 2 + self.state(s).population as i64,
            Place::Colony(c) => self.colony(c).map(|c| c.modules.len() as i64 * 2 + c.colonists as i64).unwrap_or(0),
        }
    }

    /// Ticket #343 (version 0.09.1): a seat wants a Missile Carrier when Missile Technology stands,
    /// there is a place it most wants and cannot take, and it has neither an armed carrier of its
    /// own already nor one on the ways. One at a time: the hull is dearer than a Battleship.
    fn wants_missile_carrier(&self, seat: Seat) -> bool {
        self.has_tech(TechId::MissileTechnology)
            && self.nuke_target(seat).is_some()
            && !self.ships.iter().any(|s| s.seat == seat && s.kind == UnitKind::MissileCarrier && s.warhead)
            && !self.queues_of(seat).any(|b| matches!(b.item, BuildItem::Unit(UnitKind::MissileCarrier) | BuildItem::Warhead(_)))
    }

    /// Ticket #343 (version 0.09.1): every build under way anywhere for this seat.
    fn queues_of(&self, seat: Seat) -> impl Iterator<Item = &Build> {
        self.states
            .iter()
            .flat_map(|s| s.queue.iter())
            .chain(self.colonies.iter().flat_map(|c| c.queue.iter()))
            .filter(move |b| b.seat == seat)
    }

    /// Ticket #41: the rival's standing on a place the seat holds is within the challenge margin of
    /// the seat's own, so the place could be lost to a short push.
    fn standing_pressed(&self, seat: Seat, place: Place) -> bool {
        let mine = self.seat(seat).influence.get(&place).copied().unwrap_or(0);
        let rival = self.rival_standing(seat, place);
        rival > 0 && rival + self.challenge_margin_at(place) >= mine
    }

    /// The highest Standing any other seat has on a place (ticket #50).
    pub fn rival_standing(&self, seat: Seat, place: Place) -> i64 {
        seat.others().iter().map(|s| self.seat(*s).influence.get(&place).copied().unwrap_or(0)).max().unwrap_or(0)
    }

    /// Spec 16.2: the Energy balance is within one turn's upkeep of zero.
    fn energy_tight(&self, seat: Seat) -> bool {
        self.energy_tight_losing(seat, 0.0)
    }

    /// Ticket #559 (version 0.1.0.3): the same test with `lost` Energy a turn gone from the production
    /// -- what a Power Plant replaced with a Wind Farm would stop making.
    fn energy_tight_losing(&self, seat: Seat, lost: f64) -> bool {
        let drain = self.total_upkeep(seat) as f64 - self.energy_production(seat) as f64 + lost;
        self.seat(seat).stockpile.energy - drain <= self.total_upkeep(seat) as f64
    }

    /// Ticket #332 (version 0.09.0): the place and item a build order would queue, for the pace
    /// every build candidate is weighed by; nothing for an order that is not a build.
    fn build_target(o: &Order) -> Option<(Place, BuildItem)> {
        match o {
            Order::BuildFacility { state, kind } => Some((Place::State(*state), BuildItem::Facility(*kind))),
            Order::RaiseIndustry { state } => Some((Place::State(*state), BuildItem::IndustryLevel)),
            Order::BuildModule { colony, kind } => Some((Place::Colony(*colony), BuildItem::Module(*kind))),
            Order::BuildShip { site, kind, .. } => Some((*site, BuildItem::Unit(*kind))),
            Order::BuildArmy { place } => Some((*place, BuildItem::Unit(UnitKind::Army))),
            _ => None,
        }
    }

    /// Ticket #394 (version 0.09.3): **the size of a place**, its Colonists plus what it makes a
    /// turn -- the Output row's figures summed, Energy only where it makes more than it eats.
    pub fn ai_place_size(&self, cid: ColonyId) -> f64 {
        self.ai_place_size_for(None, cid)
    }

    /// Ticket #430 (version 0.09.5): the same, as `viewer` sees it: what a place earns is hidden
    /// where it does not see, so there it is sized by its Colonists alone. `None` reads the board.
    fn ai_place_size_for(&self, viewer: Option<Seat>, cid: ColonyId) -> f64 {
        let Some(col) = self.colony(cid) else { return 0.0 };
        let earns = viewer.is_none_or(|v| col.control.director() == Some(v) || self.sees_place(v, Place::Colony(cid)));
        col.colonists as f64 + if earns { self.place_output(Place::Colony(cid)).map(|o| o.made()).unwrap_or(0.0) } else { 0.0 }
    }

    /// Ticket #394 (version 0.09.3): the largest size any directed Colony or station on the board
    /// has, computed once a plan (the review's fix-up: computing it inside every bounty walked every
    /// place's yields once per rival Colony, R x D x Y a plan).
    pub fn ai_board_largest_size(&self) -> f64 {
        self.ai_board_largest_size_for(None)
    }

    fn ai_board_largest_size_for(&self, viewer: Option<Seat>) -> f64 {
        self.colonies.iter().filter(|c| c.control.director().is_some()).map(|c| self.ai_place_size_for(viewer, c.id)).fold(0.0, f64::max)
    }

    /// Ticket #394 (version 0.09.3): **the bounty a rival's Colony or station is**, rescaled to the
    /// board at the designer's word: `bounty_top` x its size / the largest size any directed place
    /// has, never dividing by less than `bounty_floor` -- so the fattest Colony or station on the board is the
    /// top once anything has reached the floor, and every other place is measured against it. The
    /// designer's counter to a player who founds one place and simply loads people and builds on
    /// it: the computer's appetite to take it grows with it. Multiplies the price rank of spending
    /// Influence on it and the base weight of landing an Army at it; 1 for a place nobody directs.
    /// Linear in the size, with no "1 +": measured with 1 + 0.05 x size the price term (80 / a price
    /// that grows 20 a Colonist) fell faster than the bounty rose, and a Colony of eight people and
    /// three Modules still ranked below one of two people and nothing built.
    pub fn ai_bounty(&self, cid: ColonyId) -> f64 {
        self.ai_bounty_against(None, cid, self.ai_board_largest_size())
    }

    /// The bounty with the board's largest size already in hand, for the planner's loops.
    fn ai_bounty_against(&self, viewer: Option<Seat>, cid: ColonyId, largest: f64) -> f64 {
        let m = &self.tables.ai.multipliers;
        let Some(col) = self.colony(cid) else { return 1.0 };
        if col.control.director().is_none() {
            return 1.0;
        }
        let archive = viewer.map(|v| self.archive_threat_lift(v, cid)).unwrap_or(1.0);
        (m.bounty_top * self.ai_place_size_for(viewer, cid) / largest.max(m.bounty_floor)).max(m.bounty_least) * archive
    }

    /// Ticket #461 (version 0.09.7): the Archive fund is lost with the Archive's Colony, so the
    /// Colony is worth `archive_target_lift` more to a seat with cause against its holder once the
    /// fund is at least `archive_target_fund` of its cap. Under that it is no threat yet.
    fn archive_threat_lift(&self, seat: Seat, cid: ColonyId) -> f64 {
        let th = &self.tables.ai.thresholds;
        let Some(col) = self.colony(cid) else { return 1.0 };
        match col.control.controller() {
            Some(owner) if owner != seat && self.archive_at_risk(owner, cid) && self.has_cause(seat, owner) => th.archive_target_lift,
            _ => 1.0,
        }
    }

    /// Ticket #461: this Colony holds its holder's Archive and the fund is worth taking.
    fn archive_at_risk(&self, owner: Seat, cid: ColonyId) -> bool {
        let th = &self.tables.ai.thresholds;
        self.colony(cid).is_some_and(|c| c.modules.iter().any(|m| m.kind == ModuleKind::Archive))
            && self.seat(owner).archive_fund as f64 >= th.archive_target_fund * self.archive_fund_cap(owner) as f64
    }

    /// Ticket #461: the Archivists' side of it -- a rival with cause against them while the Archive
    /// at this Colony is worth taking. The designer: "they need to be able to respond to heightened
    /// threat".
    fn archive_threatened(&self, seat: Seat, cid: ColonyId) -> bool {
        self.archive_at_risk(seat, cid) && seat.others().iter().any(|r| self.has_cause(*r, seat))
    }

    /// Ticket #618 (version 0.1.0.5), at the designer's word: **a rival's climb on a neutral Region,
    /// as this seat sees it** -- the climber (the rival with the most Standing there, the last on a
    /// tie) and the Standing to pass, where that Standing is at or over `contest_from` of the
    /// threshold as the climber reads it; else None. A held Region is nobody's climb. Any rival: the
    /// human's climbs and a computer seat's snipes alike. Before this no computer seat read a rival's
    /// Standing on a Region at all, and the one contest the designer saw in four games was a
    /// coincidence of the Region's value.
    pub fn ai_climb_on(&self, seat: Seat, sid: StateId) -> Option<(Seat, i64)> {
        if self.state(sid).control != Control::Neutral {
            return None;
        }
        let target = Place::State(sid);
        let (climber, top) = self.ai_top_rival_on(seat, target)?;
        let threshold = self.influence_threshold_for(climber, target).max(1);
        (top as f64 / threshold as f64 >= self.tables.ai.thresholds.contest_from).then_some((climber, top))
    }

    /// Tickets #618 and #619: the rival with the most Standing at a place, and that Standing; the
    /// last seat on a tie. None only with no rival at all.
    fn ai_top_rival_on(&self, seat: Seat, target: Place) -> Option<(Seat, i64)> {
        Seat::ALL.into_iter().filter(|s| *s != seat).map(|s| (s, self.seat(s).influence.get(&target).copied().unwrap_or(0))).max_by_key(|(_, n)| *n)
    }

    /// Tickets #618 and #619: what a climb lifts a Region by -- 1 + 2 x the climb, the climb capped
    /// at 1, so x2 at half the figure and x3 at it.
    fn climb_lift(climb: f64) -> f64 {
        1.0 + 2.0 * climb.min(1.0)
    }

    /// Ticket #618: **how much a rival's climb lifts a neutral Region for this seat.** The climb is
    /// answered by the computer seat with the most Standing there among the seats not climbing
    /// (strictly the most, and some: a tie answers for nobody), and by any computer seat holding a
    /// neighbour: 1 + 2 x the climb (the Standing over the threshold, capped at 1), so x2 at half
    /// the threshold and x3 at it. No climb, or a seat with no claim: 1. A second rival over the
    /// figure is "not climbing" here, so two seats climbing one Region lift it for each other.
    pub fn ai_contest_lift(&self, seat: Seat, sid: StateId) -> f64 {
        let Some((climber, top)) = self.ai_climb_on(seat, sid) else { return 1.0 };
        let target = Place::State(sid);
        let standing = |s: Seat| self.seat(s).influence.get(&target).copied().unwrap_or(0);
        let mine = standing(seat);
        let most = mine > 0 && Seat::ALL.into_iter().filter(|s| *s != seat && *s != climber).all(|s| standing(s) < mine);
        let next_door = self.tables.state(sid).neighbours.iter().any(|n| self.state(*n).control.controller() == Some(seat));
        if most || next_door { Self::climb_lift(top as f64 / self.influence_threshold_for(climber, target).max(1) as f64) } else { 1.0 }
    }

    /// Ticket #618: whether this seat contests the Region -- the lift is more than 1.
    pub fn ai_contested(&self, seat: Seat, sid: StateId) -> bool {
        self.ai_contest_lift(seat, sid) > 1.0
    }

    /// Ticket #619 (version 0.1.0.5), at the designer's word: **a rival's climb on a Region this
    /// seat holds** -- the climber (the rival with the most Standing there) and their Standing, where
    /// it is at or over `defend_from` of the price the climber would pay (`defend_home_from` on the
    /// home Region); else None. Before this a held place was never spent on: its Standing was what
    /// the take banked, less a point a turn, and the designer took two computer capitals in one turn
    /// each.
    pub fn ai_defence(&self, seat: Seat, sid: StateId) -> Option<(Seat, i64)> {
        if self.state(sid).control.controller() != Some(seat) {
            return None;
        }
        let (climber, top) = self.ai_top_rival_on(seat, Place::State(sid))?;
        if top <= 0 {
            return None;
        }
        let th = &self.tables.ai.thresholds;
        let from = if self.ai_home_is(seat, sid) { th.defend_home_from } else { th.defend_from };
        (self.ai_climb_against(climber, sid, top) >= from).then_some((climber, top))
    }

    /// Ticket #619: the climb as a share of the price the climber would pay for the held Region.
    fn ai_climb_against(&self, climber: Seat, sid: StateId, top: i64) -> f64 {
        top as f64 / self.influence_needed_for(climber, Place::State(sid)).max(1) as f64
    }

    /// Ticket #619: whether the Region is this seat's home.
    fn ai_home_is(&self, seat: Seat, sid: StateId) -> bool {
        self.seat(seat).home_region == Some(sid)
    }

    /// Ticket #619: **how much a rival's climb lifts a held Region for its holder**: 1 + 2 x the
    /// climb (the Standing over the price, capped at 1), the home Region doubled (the designer's
    /// "weighed double", read as the weight on the list); no climb, 1.
    pub fn ai_defence_lift(&self, seat: Seat, sid: StateId) -> f64 {
        let Some((climber, top)) = self.ai_defence(seat, sid) else { return 1.0 };
        let home = if self.ai_home_is(seat, sid) { 2.0 } else { 1.0 };
        Self::climb_lift(self.ai_climb_against(climber, sid, top)) * home
    }

    /// Ticket #619: **the Standing the home Region still wants to reach its floor** (`home_floor`),
    /// so a capital's price starts above the bare threshold; 0 at or over it, or with no home held.
    pub fn ai_home_topup(&self, seat: Seat) -> i64 {
        let Some(home) = self.seat(seat).home_region.filter(|h| self.state(*h).control.controller() == Some(seat)) else { return 0 };
        (self.tables.ai.thresholds.home_floor - self.seat(seat).influence.get(&Place::State(home)).copied().unwrap_or(0)).max(0)
    }

    /// Ticket #620 (version 0.1.0.5), at the designer's word: **the turn this seat wins at the rate it
    /// has made its first Victory part so far** -- this turn plus the rest of the bar over that rate.
    /// None with nothing made yet, and None for a Stabilization run, which is no race against the
    /// Collapse: its win is the cure.
    /// Ticket #629 (version 0.1.0.6), at the designer's word: **whether a computer seat builds this
    /// Ship as a nuclear hull** -- once Nuclear Rockets stands, when its Stockpile holds the second
    /// tank's fill and `nuclear_hull_reserve_tanks` tanks more, two crossings' worth. A chemical hull
    /// otherwise. It is taught to chase Fissionables on its own ticket (#643).
    /// Ticket #630 (version 0.1.0.6), at the designer's word: **whether a computer seat can keep a
    /// Reactor fed** -- Clean Power standing, and its Stockpile holding `reactor_reserve_turns` turns
    /// of a Reactor's upkeep. While it can, a Reactor takes the Power Plant's place on its build list
    /// (and the Reactor Module the Generator's); while it cannot, no Reactor is offered.
    pub fn ai_feeds_reactors(&self, seat: Seat) -> bool {
        let upkeep = self.tables.facility(FacilityKind::FissionReactor).fissionables_upkeep;
        self.has_tech(TechId::CleanPower) && self.seat(seat).stockpile.fissionables + 1e-9 >= upkeep * self.tables.ai.thresholds.reactor_reserve_turns
    }

    /// Ticket #630, from the review: the seat's Reactors, Facilities and Modules, each with whether
    /// it is mothballed.
    fn ai_reactors(&self, seat: Seat) -> Vec<(BuildingRef, bool)> {
        let mut out = Vec::new();
        for sid in self.directed_states(seat) {
            for (i, f) in self.state(sid).facilities.iter().enumerate() {
                if f.kind == FacilityKind::FissionReactor {
                    out.push((BuildingRef::Facility(sid, i), f.mothballed));
                }
            }
        }
        for cid in self.directed_colonies(seat) {
            if let Some(c) = self.colony(cid) {
                for (i, m) in c.modules.iter().enumerate() {
                    if m.kind == ModuleKind::FissionReactor {
                        out.push((BuildingRef::Module(cid, i), m.mothballed));
                    }
                }
            }
        }
        out
    }

    /// Ticket #630, from the review: **how many running Reactors this seat cannot feed this Income**,
    /// the Stockpile over a Reactor's upkeep, whole -- those and no more are mothballed.
    pub fn ai_reactors_to_mothball(&self, seat: Seat) -> usize {
        let upkeep = self.tables.facility(FacilityKind::FissionReactor).fissionables_upkeep;
        let running = self.ai_reactors(seat).iter().filter(|(_, m)| !*m).count();
        if upkeep <= 0.0 {
            return 0;
        }
        let fed = ((self.seat(seat).stockpile.fissionables + 1e-9) / upkeep).floor() as usize;
        running.saturating_sub(fed)
    }

    /// Ticket #630, from the review: **whether an idle Reactor may be restarted** -- once the Stockpile
    /// holds the reserve's turns for every running Reactor and this one, so a restart is never undone
    /// by the next Income.
    pub fn ai_may_restart_reactor(&self, seat: Seat) -> bool {
        let upkeep = self.tables.facility(FacilityKind::FissionReactor).fissionables_upkeep;
        let running = self.ai_reactors(seat).iter().filter(|(_, m)| !*m).count() as f64;
        self.has_tech(TechId::CleanPower) && self.seat(seat).stockpile.fissionables + 1e-9 >= upkeep * self.tables.ai.thresholds.reactor_reserve_turns * (running + 1.0)
    }

    pub fn ai_builds_nuclear(&self, seat: Seat, kind: UnitKind) -> bool {
        if !self.has_tech(TechId::NuclearRockets) || !UnitKind::SHIPS.contains(&kind) {
            return false;
        }
        let fill = self.fission_tank_of(seat, kind);
        fill > 0.0 && self.seat(seat).stockpile.fissionables + 1e-9 >= fill * (1.0 + self.tables.ai.thresholds.nuclear_hull_reserve_tanks)
    }

    pub fn ai_win_turn(&self, seat: Seat) -> Option<u32> {
        if self.first_kind(seat) == VictoryFirstKind::StabilizationRun || self.turn == 0 {
            return None;
        }
        let p = self.progress(seat);
        if p.first_value <= 0.0 {
            return None;
        }
        let rate = p.first_value / self.turn as f64;
        // Clamped to the game's last turn: a tiny rate must not wrap the cast (the review).
        let last = self.tables.victory.turns;
        Some(self.turn.saturating_add(((p.first_bar - p.first_value).max(0.0) / rate).ceil().min(last as f64) as u32).min(last))
    }

    /// Ticket #620: **whether the Collapse on the climate's heading falls at or before this seat's
    /// win turn** -- the race it would lose by emitting on. No Collapse on the heading, or no win
    /// turn, is no race; a win within `win_in_sight` turns is run for the line.
    pub fn ai_loses_the_race(&self, seat: Seat) -> bool {
        let Some(collapse) = self.projection().collapse_turn else { return false };
        let Some(win) = self.ai_win_turn(seat) else { return false };
        if win.saturating_sub(self.turn) <= self.tables.ai.thresholds.win_in_sight {
            return false;
        }
        collapse <= win
    }

    /// Ticket #620: **the weight an order takes while the race is lost**: `collapse_emit_weight` on
    /// an emitting one (a Strip Permit, an Industry Level, a Facility whose card carries Emissions,
    /// built or restarted), `collapse_clean_weight` on a cleaner one (a Facility with a sink, the
    /// Wind Farm, a Leapfrog, an emitter mothballed), 1 on the rest.
    pub fn ai_collapse_weight(&self, o: &Order) -> f64 {
        let th = &self.tables.ai.thresholds;
        let emits = |kind: FacilityKind| self.tables.facility(kind).emissions > 0.0;
        match o {
            Order::StripPermit { .. } | Order::RaiseIndustry { .. } => th.collapse_emit_weight,
            Order::Leapfrog { .. } => th.collapse_clean_weight,
            Order::BuildFacility { kind, .. } => {
                let card = self.tables.facility(*kind);
                if emits(*kind) {
                    th.collapse_emit_weight
                } else if card.sink_per_turn > 0.0 || matches!(kind, FacilityKind::WindFarm | FacilityKind::FissionReactor) {
                    th.collapse_clean_weight
                } else {
                    1.0
                }
            }
            // The review: a Mothball of an emitter cleans and its Restart emits; a Module off Earth
            // is neither, the Collapse being Earth's air.
            Order::Change { building: BuildingRef::Facility(sid, i), what } => match (self.state(*sid).facilities.get(*i).map(|f| emits(f.kind)), what) {
                (Some(true), BuildingChange::Mothball) => th.collapse_clean_weight,
                (Some(true), BuildingChange::Restart) => th.collapse_emit_weight,
                _ => 1.0,
            },
            _ => 1.0,
        }
    }

    /// Ticket #619: the Influence a held Region wants this turn -- enough to put the wall (the
    /// greater of the threshold and the seat's Standing plus the climber's margin) back above the
    /// climber's Standing plus their Allotment, and at home the floor's top-up, whichever is more;
    /// 0 where nothing is wanted. Where the threshold alone already stands above the reach, nothing
    /// (the review: the first draft spent against the Standing wall while the threshold held).
    pub fn ai_defence_wanted(&self, seat: Seat, sid: StateId) -> i64 {
        let target = Place::State(sid);
        let have = self.seat(seat).influence.get(&target).copied().unwrap_or(0);
        let defend = self
            .ai_defence(seat, sid)
            .map(|(climber, top)| {
                let reach = top + self.seat(climber).allotment;
                if self.influence_threshold_for(climber, target) > reach { 0 } else { reach + 1 - (have + self.challenge_margin_for(Some(climber), target)) }
            })
            .unwrap_or(0);
        let floor = if self.ai_home_is(seat, sid) { self.ai_home_topup(seat) } else { 0 };
        defend.max(floor)
    }

    /// Ticket #394 (version 0.09.3): the places a computer seat weighs spending Influence on, best
    /// first -- extracted from the planner so the order can be witnessed. A Region by its Influence
    /// value and Industry Level, closest first, a held one at a fraction of a neutral one (#75); a
    /// rival's Colony by the price this seat would pay, cheapest first (#336), times its bounty
    /// (#394); a neutral Region a rival is climbing lifted for the seats that answer it (#618).
    pub fn ai_influence_targets(&self, seat: Seat) -> Vec<(Place, f64)> {
        let th = &self.tables.ai.thresholds;
        let mut targets: Vec<(Place, f64)> = Vec::new();
        let my_states = self.controlled_states(seat);
        for sid in StateId::ALL {
            let st = self.state(sid);
            if st.control.controller() == Some(seat) {
                // Ticket #619 (version 0.1.0.5): a held Region is a target while a rival climbs it,
                // or while it is the home under its floor; held and quiet, it is skipped as before.
                if self.ai_defence(seat, sid).is_none() && !(self.ai_home_is(seat, sid) && self.ai_home_topup(seat) > 0) {
                    continue;
                }
                let value = (self.state_influence_value(sid) + st.industry_level as i64) as f64;
                targets.push((Place::State(sid), value * self.ai_defence_lift(seat, sid)));
                continue;
            }
            let card = self.tables.state(sid);
            let near = card.neighbours.iter().any(|n| my_states.contains(n));
            // Ticket #34: the state's Influence value plus its Industry Level, closest first.
            let value = (self.state_influence_value(sid) + st.industry_level as i64) as f64 + if near { 2.0 } else { 0.0 };
            // Ticket #75: a held place counts a fraction of a neutral one (0.3), so it is attacked
            // only when no neutral one is worth having.
            let neutral_bonus = if st.control == Control::Neutral { 1.0 } else { th.held_state_weight };
            // Ticket #446 (version 0.09.6): a heavy emitter's Region weighs more to the Custodians.
            let lift = st.control.controller().map(|r| self.hostile_lift(seat, r)).unwrap_or(1.0);
            // Ticket #618 (version 0.1.0.5): a rival's climb on a neutral Region lifts it.
            targets.push((Place::State(sid), value * neutral_bonus * lift * self.ai_contest_lift(seat, sid)));
        }
        // Ticket #50: any rival's Colony. Ticket #336 (version 0.09.0): weighed by THE PRICE THIS
        // SEAT WOULD PAY, cheapest first, where the rule was `3.0 - min(colonists, 2)` -- fewest
        // Colonists first, and the threshold never read at all. With the thresholds off Earth
        // doubled that rule would have had the seats ranking a place they cannot afford above one
        // they can, which would read as a balance change and be a defect. The pivot is the price of
        // a starting two-Colonist place, so the band was the one the old rule ran in, its ceiling
        // the old rule's own 3.0.
        // Ticket #394 (version 0.09.3): times its bounty (the designer's Q5 A), so a fat Colony is
        // wanted despite its price, which still ranks and so still gates what the seat can afford.
        // The band moves: the product can reach 9 on paper, about 2.5 in play (a Colony of n
        // Colonists holds n Modules, so its price rises with its size), which is under a neutral
        // Region's 5 to 10 still and above a held Region's 1.5 to 3 now. Measured with the bounty
        // alone and no price: the seats spent on places they could not afford, and collapses went
        // from 60 to 69 of 80.
        // Ticket #430 (version 0.09.5): sized as this seat sees them.
        let largest = self.ai_board_largest_size_for(Some(seat));
        for c in &self.colonies {
            if c.control.controller().map(|o| o != seat).unwrap_or(false) {
                let price = self.influence_needed_for(seat, Place::Colony(c.id)).max(1) as f64;
                let lift = c.control.controller().map(|r| self.hostile_lift(seat, r)).unwrap_or(1.0);
                targets.push((Place::Colony(c.id), (th.colony_price_pivot / price).min(3.0) * self.ai_bounty_against(Some(seat), c.id, largest) * lift));
            }
        }
        targets.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        targets
    }

    /// Ticket #398 (version 0.09.3): the seat's Colonies with a working Shipyard, the list a Ship
    /// can be offered at, in one place so the planner and `ai_ship_yard` read the same yards.
    fn ai_working_yards(&self, seat: Seat) -> Vec<ColonyId> {
        self.directed_colonies(seat).into_iter().filter(|c| self.colony(*c).unwrap().modules.iter().any(|m| m.kind == ModuleKind::Shipyard && m.working())).collect()
    }

    /// Ticket #398 (version 0.09.3): **the yard a computer seat builds its Ships at**: the one
    /// where a Ship costs least (Build Where You Dig at a low-gravity yard with a working Mine), the
    /// most Widgets among equals, the first on the list among those. The discount is one factor for
    /// every kind, so the Frigate's price ranks the yards for all of them. Before this ticket the
    /// yard was the most Widgets alone (ticket #332), which never landed on the Moon.
    pub fn ai_ship_yard(&self, seat: Seat) -> Option<ColonyId> {
        let price = |c: ColonyId| self.ship_materials_at(seat, Place::Colony(c), UnitKind::Frigate);
        let widgets = |c: ColonyId| self.widgets_at(Place::Colony(c));
        // `min_by` keeps the FIRST of equals, so the first on the list wins a tie (the review's
        // fix-up: the first cut reversed the list and kept the last).
        self.ai_working_yards(seat).into_iter().min_by(|a, b| price(*a).total_cmp(&price(*b)).then(widgets(*b).cmp(&widgets(*a))))
    }

    /// Ticket #398 (version 0.09.3): what a Shipyard's weight is multiplied by at this Colony --
    /// `low_gravity_yard` on a ground Colony on the Moon, Phobos or Deimos with a working Mine,
    /// where Build Where You Dig reaches the Ships it would build; 1 everywhere else.
    pub fn ai_shipyard_bonus(&self, cid: ColonyId) -> f64 {
        match self.colony(cid) {
            Some(col) if !col.in_orbit && self.tables.body(col.body).low_gravity && self.working_mines(col) > 0 => self.tables.ai.multipliers.low_gravity_yard,
            _ => 1.0,
        }
    }

    /// Ticket #590 (version 0.1.0.4): the capital building a computer seat takes for `capital`: of
    /// the free ones, the best score -- 1 for the Faction's own, plus `capital_pick_need` where the
    /// run needs what it gives (a stockpile yield its last Income lost; a Shipyard of its on the
    /// Capital's Body for the Spaceport; a Region of its past the first Unrest threshold for the
    /// Arcology) -- ties to the table's order.
    pub fn ai_capital_pick(&self, seat: Seat, capital: ColonyId) -> Option<CapitalBuilding> {
        let own = self.tables.faction(self.kind(seat)).capital_building;
        let need = self.tables.ai.thresholds.capital_pick_need;
        let last = self.seat(seat).income_last_turn;
        let body = self.colony(capital)?.body;
        let yard = self.colonies.iter().any(|c| c.body == body && c.control.director() == Some(seat) && c.modules.iter().any(|m| m.kind == ModuleKind::Shipyard));
        let restive = self.controlled_states(seat).iter().any(|s| self.unrest(*s) >= self.tables.unrest.army_threshold);
        let needed = |b: CapitalBuilding| -> bool {
            match self.tables.capital_building(b).resource {
                Some(Resource::Ducats) => last.ducats < 0.0,
                Some(Resource::Materials) => last.materials < 0.0,
                Some(Resource::Fuel) => last.fuel < 0.0,
                Some(Resource::Energy) => last.energy < 0.0,
                _ => match b {
                    CapitalBuilding::Spaceport => yard,
                    CapitalBuilding::Arcology => restive,
                    _ => false,
                },
            }
        };
        let score = |b: CapitalBuilding| f64::from(u8::from(own == Some(b))) + if needed(b) { need } else { 0.0 };
        let mut best: Option<(CapitalBuilding, f64)> = None;
        for b in self.free_capital_buildings() {
            let s = score(b);
            if best.is_none_or(|(_, top)| s > top) {
                best = Some((b, s));
            }
        }
        best.map(|(b, _)| b)
    }

    /// Ticket #590 (version 0.1.0.4): the place a seat builds up off Earth -- its most peopled ground
    /// Colony beyond Earth, ties to the lower id.
    pub fn best_off_earth(&self, seat: Seat) -> Option<ColonyId> {
        self.colonies
            .iter()
            .filter(|c| c.control.director() == Some(seat) && !c.in_orbit && c.body != BodyId::Earth)
            .max_by_key(|c| (c.colonists, std::cmp::Reverse(c.id.0)))
            .map(|c| c.id)
    }

    pub fn ai_orders(&mut self, seat: Seat) -> Vec<Order> {
        // Ticket #337 (version 0.09.0): the seat answers this turn's choice card HERE, before it
        // reads its own board for anything else, so a held fleet or a paid bill is already true of
        // the board the orders are chosen against -- and so the computer never holds the turn.
        self.ai_answer_card(seat);
        let (gap, behind) = self.victory_gap(seat);
        let kind = self.kind(seat);
        let m = self.tables.ai.multipliers.clone();
        // Ticket #394 (version 0.09.3): the board's largest place, once a plan, for the bounties.
        let largest_on_board = self.ai_board_largest_size_for(Some(seat));
        let th = self.tables.ai.thresholds.clone();
        let first_kind = self.first_kind(seat);
        let scarce = self.scarcest(seat);
        let tight = self.energy_tight(seat);
        let allotment = self.seat(seat).allotment;
        let materials_income = self.seat(seat).income_last_turn.materials;
        // Ticket #332 (version 0.09.0): on Earth it is the Mine that makes Materials now.
        let no_materials_income = materials_income <= 0.0
            && !self.directed_states(seat).iter().any(|s| self.state(*s).facilities.iter().any(|f| f.kind == FacilityKind::Mine))
            && !self.directed_colonies(seat).iter().any(|c| self.colony(*c).unwrap().modules.iter().any(|m| m.kind == ModuleKind::Mine))
            && !self.states.iter().flat_map(|s| s.queue.iter()).any(|b| b.seat == seat && b.item == BuildItem::Facility(FacilityKind::Mine));
        // Ticket #46: until the seat has a Shipyard anywhere, one counts as advancing whatever it is behind on.
        let no_shipyard = !self.directed_colonies(seat).iter().any(|c| {
            let col = self.colony(*c).unwrap();
            col.modules.iter().any(|m| m.kind == ModuleKind::Shipyard) || col.queue.iter().any(|b| b.item == BuildItem::Module(ModuleKind::Shipyard))
        });
        // Ticket #564 (version 0.1.0.3), at the designer's word: the race for the first Moon landing.
        // While the seat holds no Colony Ship, built or on order, and the Moon is still unclaimed
        // with a free slot, its Shipyard and Colony Ship are weighed at `moon_race_weight` times
        // their base; the lift ends with the first Ship or the first landing. Measured before: the
        // computer's first Moon landing came at median turn 15 to 32 against the human's 6 to 10;
        // a race for any unclaimed Body sent the first Ships past the Moon (see `best_body_for`).
        let moon_race = {
            let has_colony_ship = self.ships.iter().any(|s| s.seat == seat && s.kind == UnitKind::ColonyShip) || self.queues_of(seat).any(|b| b.item == BuildItem::Unit(UnitKind::ColonyShip));
            let unclaimed = self.first_at(BodyId::Moon).is_none() && !self.free_slots_on(BodyId::Moon).is_empty();
            if !has_colony_ship && unclaimed { th.moon_race_weight } else { 1.0 }
        };
        // Ticket #596 (version 0.1.0.4), at the designer's word: the race to Mars, once the Moon is
        // claimed. A Colony Ship in flight to another Body is busy; one anywhere else, or bound for
        // Mars, or on order, is the seat's runner already.
        let mars_race = {
            // The review: one flying home to Earth is the runner too, a turn from free.
            let runner = self.ships.iter().any(|s| s.seat == seat && s.kind == UnitKind::ColonyShip && !matches!(s.at, ShipAt::Transit { to, .. } if to != BodyId::Mars && to != BodyId::Earth))
                || self.queues_of(seat).any(|b| b.item == BuildItem::Unit(UnitKind::ColonyShip));
            if self.mars_race_open() && !runner { th.mars_race_weight.max(1.0) } else { 1.0 }
        };
        let moon_race = if moon_race > 1.0 { moon_race } else { mars_race };
        // Ticket #559 (version 0.1.0.3): each build's own card figure, where a flat 6 was counted
        // for every Power Plant's job, so a Wind Farm on order counts its 3 and a Replace under way
        // trades the Power Plant's figure for the Wind Farm's.
        let energy_of = |k: FacilityKind| self.tables.facility(k).produces.as_ref().filter(|p| p.resource == Resource::Energy).map_or(0.0, |p| p.amount);
        let generator = self.tables.module(ModuleKind::Generator).produces.as_ref().map_or(0.0, |p| p.amount);
        let reactor_module = self.tables.module(ModuleKind::FissionReactor).produces.as_ref().map_or(0.0, |p| p.amount);
        let queued_power: i64 = (self
            .states
            .iter()
            .flat_map(|s| s.queue.iter())
            .filter(|b| b.seat == seat)
            .map(|b| match b.item {
                BuildItem::Facility(k) => energy_of(k),
                BuildItem::Replace(k) => energy_of(k) - energy_of(FacilityKind::PowerPlant),
                _ => 0.0,
            })
            .sum::<f64>()
            + self.colonies.iter().flat_map(|c| c.queue.iter()).filter(|b| b.seat == seat && b.item == BuildItem::Module(ModuleKind::Generator)).count() as f64 * generator
            // Ticket #630 (version 0.1.0.6), from the review: and a Reactor Module on order.
            + self.colonies.iter().flat_map(|c| c.queue.iter()).filter(|b| b.seat == seat && b.item == BuildItem::Module(ModuleKind::FissionReactor)).count() as f64 * reactor_module)
            .floor() as i64;
        let energy_short = self.energy_production(seat) + queued_power < self.total_upkeep(seat) + 2;
        // Bootstrap needs: a producer of one of these counts as advancing whatever the seat is behind on.
        // The merely scarcest resource only gets the within-category preference.
        let mut needs: Vec<Resource> = Vec::new();
        if no_materials_income {
            needs.push(Resource::Materials);
        }
        if energy_short {
            needs.push(Resource::Energy);
        }
        // Ticket #41 tried adding Ducats to this list (a Bank or Trade Post preferred while the seat
        // cannot afford a step of bought Influence). Measured over twenty seeds it cost the Custodians
        // every win: a Bank on turn one displaced the Research Lab, and 4 Ducats a turn buys 2
        // Influence, which never repays 25 Materials the way a Factory does. Banks and Trade Posts
        // stay plain producers at their base weight.
        // Ticket #54: the state a Leapfrog would go to (the most populous the seat controls) and the
        // one a Strip Permit would (the one whose Facilities make the most), settled once.
        let most_populous = self
            .controlled_states(seat)
            .into_iter()
            .filter(|s| self.leapfrog_would_bite(*s))
            .max_by(|a, b| self.state(*a).population.partial_cmp(&self.state(*b).population).unwrap_or(std::cmp::Ordering::Equal));
        let output_of = |sid: StateId| -> f64 {
            self.state(sid).facilities.iter().filter(|f| !f.mothballed).map(|f| self.facility_yield(seat, sid, f.kind).amount).sum()
        };
        let highest_output = self.controlled_states(seat).into_iter().filter(|s| !self.state(*s).strip_permit_used).max_by(|a, b| output_of(*a).total_cmp(&output_of(*b)));
        // Ticket #54: behind on the first part's pace itself, whichever part the seat is furthest
        // behind on overall: the Strip Permit is bought against that schedule (ticket #72: the
        // Venture Capital Fund's).
        let behind_on_extraction = first_kind == VictoryFirstKind::VentureFund && {
            let pace = self.tables.ai_pace(kind);
            let want = Self::expected(&pace.first, self.turn);
            want > 0.0 && (self.progress(seat).first_value) < want
        };
        let mut cands: Vec<Candidate> = Vec::new();

        // Ticket #209 (version 0.08.1): while the Archivists have nowhere the Archive may stand,
        // everything that carries them to such a place is worth more than its standing weight says.
        // Narrowing `may_hold_archive` stops them ordering in Earth orbit on its own -- their
        // Archive push already filters on that function -- but nothing in it makes them GO. Ticket
        // #192's warning about the Upload was that a computer not taught the new move wins nothing
        // at all, and that warning proved right; this is the same shape, so the appetite is added
        // deliberately rather than hoped for.
        let homeless_archive = self.archive_is_homeless(seat);
        let homeless_bonus = self.tables.ai.multipliers.archive_needs_a_place;
        // Ticket #361 (version 0.09.1): **the Archivists FERRY until their uploads are made.** A
        // Colony on another Body stood in 19 games of 80 -- the homeless lift above raised a landing
        // in Antarctica or at a station over Earth as much as a crossing, and neither holds the
        // Archive -- and where the Archive did complete, the uploads it needs stopped at four or
        // eight for want of people arriving. While their uploads are short of the second part's
        // bar, a loaded Colony Ship does not land on Earth's ground or over it, and it crosses to the
        // Archive's Body (or the best Body off Earth while there is none) at the homeless lift.
        let ferrying = kind == FactionKind::Archivists && (self.seat(seat).uploaded as f64) < self.tables.faction(kind).victory_second.bar;
        // Ticket #514 (version 0.1.0.0): a seat whose win waits on a Colonial Capital, holding none,
        // has a CAPITAL CANDIDATE (its most peopled ground Colony off Earth on a Body with no
        // Capital), and once its Bodies part is within one of its bar it GATHERS people there: the
        // candidate's upgrades, every Module built there (so its slots fill for the next tier), a
        // loaded Ship's crossing to its Body while it has room, and a disembark into it all weigh
        // `capital_chase` times; a fresh Outpost on its Body weighs a chase less.
        // Measured before the gathering: the chase on the upgrade and the landing alone left no
        // Capital off Earth in 80 games (seven stood, all in Antarctica).
        // Ticket #590 (version 0.1.0.4), at the designer's word: every seat builds up its best place off
        // Earth, so every seat has a candidate and gathers at it; the Arkwrights once their Bodies are near.
        let capital_candidate = self.capital_candidate(seat);
        let capital_body = capital_candidate.and_then(|c| self.colony(c)).map(|c| c.body);
        let capital_chase = self.tables.ai.thresholds.capital_chase;
        // The people the Capital tier wants at one place, the room the candidate builds toward.
        // Ticket #590 (version 0.1.0.4): the Capital asks none, so the most any tier asks.
        let capital_room = self.tables.tiers.iter().map(|t| t.colonists).max().unwrap_or(0);
        // Room at the candidate for a landing: a crossing is lifted only where the people can land.
        let capital_has_room = capital_candidate.and_then(|c| self.colony(c)).is_some_and(|c| self.habitat_room(c) > c.colonists);
        let gathering = capital_candidate.is_some() && {
            let sec = self.tables.faction(kind).victory_second;
            sec.kind != VictorySecondKind::ColoniesOnBodies || self.bodies_settled(seat, sec.colonists_each) + 1 >= sec.bodies
        };
        // The ferry's SPENDING lift pauses while a site is ready and the Archive is not yet ordered:
        // measured, its Colony Ships spent every Material, and the 50 the Archive wants was never in
        // hand. The route does not pause -- nobody is landed where the Archive cannot use them.
        // Ticket #625 (version 0.1.0.5): a site is a place that may hold the Archive AND has climbed
        // to the tier the Archive asks for (the Colony tier). An Outpost of theirs off Earth is a
        // home, not a site: the lift stays on, so people keep arriving and the place can climb.
        // Measured without this: the lift paused at the first Outpost, and no computer Archive stood
        // in 80 games (43 before the tier).
        let site_stands = self.colonies.iter().any(|c| c.control.director() == Some(seat) && self.may_site_archive(c));
        let site_ready = kind == FactionKind::Archivists && !homeless_archive && site_stands && !self.archive_ordered(seat) && !self.archive_built(seat);
        let ferry_lift = ferrying && !site_ready;
        let archive_body: Option<BodyId> = if kind == FactionKind::Archivists {
            self.archive_colony(seat)
                .and_then(|c| self.colony(c))
                .or_else(|| self.colonies.iter().filter(|c| c.control.director() == Some(seat) && self.may_hold_archive(c)).max_by_key(|c| c.colonists))
                .map(|c| c.body)
        } else {
            None
        };

        let opening_unmet = self.seat(seat).opening_met_turn.is_none();
        let opening_kind = self.tables.faction(kind).opening.kind;
        // Ticket #504 (version 0.1.0.0): and its Second Objective, once the first is met.
        let second_open = self.seat(seat).opening_met_turn.is_some() && self.seat(seat).second_met_turn.is_none();
        let second_kind = self.tables.faction(kind).second.as_ref().map(|c| c.kind);
        fn advances(k: crate::data::OpeningKind, cat: Cat, note: &str) -> bool {
            use crate::data::OpeningKind;
            match k {
                OpeningKind::ScrubberWorking => cat == Cat::Scrubber,
                OpeningKind::InvestmentBanks => note.starts_with("build Investment Bank in"),
                OpeningKind::MoonColony => matches!(cat, Cat::FoundColony | Cat::Transit) && note.contains("the Moon"),
                OpeningKind::ResearchPair => matches!(cat, Cat::Observatory | Cat::ResearchLab),
                OpeningKind::Leapfrogs | OpeningKind::IndustryLevel | OpeningKind::FarColony | OpeningKind::Observatories => false,
            }
        }
        // The Second Objectives, read from the orders themselves (the review): a Leapfrog of a
        // Region not yet counted; a raise; Colonists carried or put down beyond Earth and the Moon,
        // whatever the note calls the place; an Observatory.
        let far = |b: BodyId| !matches!(b, BodyId::Earth | BodyId::Moon);
        let second_advances = |cat: Cat, orders: &[Order]| -> bool {
            use crate::data::OpeningKind;
            orders.iter().any(|o| match (second_kind, o) {
                (Some(OpeningKind::Leapfrogs), Order::Leapfrog { state }) => !self.seat(seat).second_leapfrogs.contains(state),
                (Some(OpeningKind::IndustryLevel), _) => cat == Cat::RaiseIndustry,
                (Some(OpeningKind::FarColony), Order::Transit { ship, to, .. }) => far(*to) && self.ship(*ship).is_some_and(|s| s.colonists > 0),
                (Some(OpeningKind::FarColony), Order::Unload { colonists, into, .. }) => {
                    *colonists > 0
                        && match into {
                            crate::orders::UnloadTarget::Slot(b, _) | crate::orders::UnloadTarget::Ring(b, _) => far(*b),
                            crate::orders::UnloadTarget::Colony(c) => self.colony(*c).is_some_and(|c| far(c.body)),
                        }
                }
                (Some(OpeningKind::Observatories), _) => cat == Cat::Observatory,
                _ => false,
            })
        };
        let opening_lift = self.tables.ai.multipliers.opportunity;
        let mut push = |orders: Vec<Order>, cat: Cat, base: f64, gap: f64, threat: f64, opportunity: f64, note: String, stack: Option<String>| {
            let base = if (homeless_archive || ferry_lift) && matches!(cat, Cat::ColonyShip | Cat::Transit | Cat::FoundColony | Cat::LoadUnload | Cat::LaunchSiteOrShipyard) {
                base * homeless_bonus
            } else {
                base
            };
            // Ticket #471 (version 0.09.7): the seat's Opening Objective, until it is met. What
            // advances it takes the opportunity multiplier -- there is no deadline, so it is a
            // lean, not a rush.
            // Ticket #504 (version 0.1.0.0): and its Second Objective once the first is met.
            let opens = (opening_unmet && advances(opening_kind, cat, &note)) || (second_open && second_advances(cat, &orders));
            let opportunity = if opens { opportunity.max(opening_lift) } else { opportunity };
            cands.push(Candidate { orders, cat, base, gap, threat, opportunity, note, stack, pace: 1.0 });
        };

        // What advances the Faction's own first Victory part (ticket #50).
        let advances_first = |cat: Cat, item: Option<&str>| -> bool {
            match first_kind {
                VictoryFirstKind::VentureFund => {
                    // Ticket #332 (version 0.09.0): nor a Factory, whose Widgets fill no Fund.
                    cat == Cat::Producer && item.map(|i| i != "Power Plant" && i != "Generator" && i != "Factory").unwrap_or(false)
                        || cat == Cat::RaiseIndustry
                        // Ticket #54: a Strip Permit is three turns of double Extraction.
                        || cat == Cat::StripPermit
                        // Version 0.07.0: ticket #84 put every Victory Condition behind a Tech, so
                        // Research advances this part too. Without this the Prospector AI built 0
                        // Research Labs in 20 seeds and never reached the Extraction Charter.
                        || cat == Cat::ResearchLab
                        || cat == Cat::Observatory
                }
                // Ticket #54: a Scrubber is what a Custodian buys Stabilization with now.
                VictoryFirstKind::StabilizationRun => cat == Cat::Scrubber || cat == Cat::Leapfrog || cat == Cat::ResearchLab || cat == Cat::Observatory || cat == Cat::NatureReserve,
                // Version 0.07.0: the Research Lab and Observatory join the list for the same
                // reason as the Venture Fund's: Generation Ships gates this win.
                VictoryFirstKind::ColonistsOffEarth => {
                    matches!(cat, Cat::Habitat | Cat::ColonyShip | Cat::FoundColony | Cat::LoadUnload | Cat::Transit | Cat::ResearchLab | Cat::Observatory)
                }
                VictoryFirstKind::ResearchProduced => cat == Cat::ResearchLab || cat == Cat::Observatory,
                // Ticket #51: the Archive wants Research, a fund and a Colony off Earth to stand at,
                // which the Colony Ship, the transit and the founding provide. Ticket #68: and the
                // Launch Site and Shipyard before them, which #51 left out, so an Archivist AI with
                // neither (its station starts bare) spent every turn on Influence and never left
                // Earth in twenty seeds of thirty-six turns.
                VictoryFirstKind::ArchiveResearch => {
                    matches!(cat, Cat::BuildArchive | Cat::Upload | Cat::FundArchive | Cat::ResearchLab | Cat::Observatory | Cat::ColonyShip | Cat::FoundColony | Cat::Transit | Cat::LoadUnload | Cat::LaunchSiteOrShipyard | Cat::Habitat)
                }
            }
        };
        // Ticket #449 (version 0.09.6): a Bodies part is advanced by going, not by housing, so a
        // Habitat takes no "behind" boost from it.
        let counts_bodies = self.tables.faction(kind).victory_second.kind == VictorySecondKind::ColoniesOnBodies;
        let advances_presence = |cat: Cat| matches!(cat, Cat::ColonyShip | Cat::FoundColony | Cat::LoadUnload | Cat::Transit) || (cat == Cat::Habitat && !counts_bodies);
        let gap_for = |cat: Cat, item: Option<&str>| -> f64 {
            // Nothing advances without Energy: while it is the scarcest resource, an Energy producer
            // counts as advancing whichever part the Faction is behind on.
            // Ticket #426 (version 0.09.5): and the Archivists' Reactor, which is a Power Plant
            // everywhere -- a Faction building carries its base building's reasons to be built.
            // Ticket #630 (version 0.1.0.6): the Accumulator by its new name, and the Reactor.
            let energy_producer = cat == Cat::Producer && matches!(item, Some("Power Plant") | Some("Accumulator") | Some("Reactor") | Some("Generator"));
            if energy_producer && needs.contains(&Resource::Energy) {
                return gap;
            }
            // Likewise nothing is built without Materials: until the seat has any Materials income,
            // a Materials producer counts as advancing the part it is behind on.
            // Ticket #332 (version 0.09.0): the Mine alone. A Factory makes Widgets now, and a seat
            // short of Materials that read it as a Materials producer built 541 Factories to 283
            // Mines over twenty games and earned 265 Materials in a whole one.
            let materials_producer = cat == Cat::Producer && item == Some("Mine");
            if materials_producer && needs.contains(&Resource::Materials) {
                return gap;
            }
            if cat == Cat::LaunchSiteOrShipyard && item == Some("Shipyard") && no_shipyard {
                return gap;
            }
            match behind {
                Behind::First if advances_first(cat, item) => gap,
                Behind::Presence if advances_presence(cat) => gap,
                _ => 1.0,
            }
        };
        // Ticket #332 (version 0.09.0): every seat wants a Mine early in its most Materials-lean
        // Region at the Factory's weight -- the designer's words. Through `early_mine_turn`, while
        // the seat directs fewer Mines on Earth (standing or on order) than `early_mines`, the
        // Region with a slot free where a Mine would make the most (`facility_yield`: the lean, Deep
        // Mining, a Strip Permit, Unrest 7 all read) is the one; the first on the list among equals.
        // The Mine there takes the gap and the opportunity below, as the opening Habitat of ticket
        // #290 does, so it comes before the Labs and Scrubbers it used to lose to at a plain
        // Producer's weight. The probe that opened this lane found the Arkwrights, who then started
        // with no Mine, holding fifteen Materials and earning none from turn 2 to turn 7; since
        // ticket #377 (version 0.09.2) every home Region opens with one, and the lane is for the
        // second.
        let earth_mines: u32 = self
            .directed_states(seat)
            .iter()
            .map(|s| self.state(*s).facilities.iter().filter(|f| f.kind == FacilityKind::Mine).count() + self.state(*s).queue.iter().filter(|b| b.seat == seat && b.item == BuildItem::Facility(FacilityKind::Mine)).count())
            .sum::<usize>() as u32;
        let early_mine_region: Option<StateId> = if self.turn <= th.early_mine_turn && earth_mines < th.early_mines {
            // `max_by_key` keeps the LAST of equals, so the list is walked backwards to keep the first.
            self.directed_states(seat).into_iter().filter(|s| self.free_slots(*s) > 0).rev().max_by(|a, b| self.facility_yield(seat, *a, FacilityKind::Mine).amount.total_cmp(&self.facility_yield(seat, *b, FacilityKind::Mine).amount))
        } else {
            None
        };
        // --- Earth builds
        for sid in self.directed_states(seat) {
            let free = self.free_slots(sid);
            // Ticket #534 (version 0.1.0.1): one to a Region, and one under construction counts.
            let has_launch = self.state(sid).facilities.iter().any(|f| f.kind.does_the_job_of(FacilityKind::LaunchSite))
                || self.state(sid).queue.iter().any(|b| matches!(b.item, BuildItem::Facility(k) if k.does_the_job_of(FacilityKind::LaunchSite)));
            if free > 0 {
                for fk in FacilityKind::ALL {
                    // Ticket #181 (version 0.08.0): a Faction builds its own Unique Facility in
                    // place of the common one and never the reverse, so the common kind is passed
                    // over where this seat has a replacement for that job, and every other Faction's
                    // Unique Facility is passed over outright. What is left is weighed by the JOB it
                    // does, so a Reactor is weighed as the Power Plant it is and no arm below has to
                    // learn four new names.
                    if fk.built_by(self.kind(seat)) != fk || fk.unique_to().is_some_and(|f| f != self.kind(seat)) {
                        continue;
                    }
                    let job = fk.common().unwrap_or(fk);
                    // Ticket #332 (version 0.09.0): a Factory is wanted where the Region's queue is
                    // `factory_module_queue_depth` deep, as the Factory Module is at a Colony, and
                    // nowhere else: Widgets a Region does not spend are lost, so a Factory beside an
                    // empty queue is a slot and twenty Materials for nothing.
                    if job == FacilityKind::Factory && self.state(sid).queue.len() < th.factory_module_queue_depth {
                        continue;
                    }
                    // Ticket #559 (version 0.1.0.3): once Clean Power stands the Wind Farm takes the
                    // Power Plant's place on the list, unless Energy is tight, when the Power Plant's 6
                    // is wanted and the Wind Farm's 3 is not. The Archivists' first Reactor is still a
                    // Reactor -- its clause is the upkeep cut, which no Wind Farm gives -- and a second
                    // is an ordinary power station, so it gives way as the Power Plant does.
                    // Ticket #630 (version 0.1.0.6): the Reactor, while the seat can keep it fed, in
                    // the Power Plant's and the Wind Farm's place; never otherwise. The Archivists'
                    // first Accumulator keeps its own reason, the upkeep cut.
                    let feeds = self.ai_feeds_reactors(seat);
                    if job == FacilityKind::FissionReactor && !feeds {
                        continue;
                    }
                    if feeds && job == FacilityKind::WindFarm {
                        continue;
                    }
                    if feeds && job == FacilityKind::PowerPlant {
                        let first_accumulator = fk == FacilityKind::Accumulator
                            && !self.directed_states(seat).iter().any(|s| self.state(*s).facilities.iter().any(|f| f.kind == FacilityKind::Accumulator))
                            && !self.states.iter().flat_map(|s| s.queue.iter()).any(|b| b.seat == seat && b.item == BuildItem::Facility(FacilityKind::Accumulator));
                        if !first_accumulator {
                            continue;
                        }
                    }
                    if job == FacilityKind::PowerPlant && self.has_tech(TechId::CleanPower) && !tight {
                        let first_reactor = fk == FacilityKind::Accumulator
                            && !self.directed_states(seat).iter().any(|s| self.state(*s).facilities.iter().any(|f| f.kind == FacilityKind::Accumulator))
                            && !self.states.iter().flat_map(|s| s.queue.iter()).any(|b| b.seat == seat && b.item == BuildItem::Facility(FacilityKind::Accumulator));
                        if !first_reactor {
                            continue;
                        }
                    }
                    if job == FacilityKind::WindFarm && tight {
                        continue;
                    }
                    let (cat, mut base) = match job {
                        // Ticket #332 (version 0.09.0): the Mine is a producer as the Factory was;
                        // the Factory, making Widgets now, keeps its arm behind the queue test above.
                        // Ticket #559 (version 0.1.0.3): and the Wind Farm, an Energy producer.
                        FacilityKind::Factory | FacilityKind::Mine | FacilityKind::PowerPlant | FacilityKind::WindFarm | FacilityKind::FissionReactor | FacilityKind::Refinery | FacilityKind::Bank => (Cat::Producer, self.base_weight(seat, Cat::Producer)),
                        FacilityKind::ResearchLab => (Cat::ResearchLab, self.base_weight(seat, Cat::ResearchLab)),
                        // Ticket #185 (version 0.08.0): the School is a Research building in all but
                        // name, so it is weighed as one. Ticket #416 (version 0.09.4): every Region
                        // makes Research from its Education now, so it no longer waits for a Lab.
                        FacilityKind::School => (Cat::ResearchLab, self.base_weight(seat, Cat::ResearchLab)),
                        FacilityKind::Embassy => (Cat::BuildInfluence, self.base_weight(seat, Cat::BuildInfluence)),
                        // Ticket #52: a Constabulary is worth raising only where Unrest has taken hold.
                        // Ticket #410 (version 0.09.4): from where the Standing Army stops, not past it.
                        FacilityKind::Constabulary => {
                            if !self.ai_offers_calming(sid, FacilityKind::Constabulary) {
                                continue;
                            }
                            (Cat::Constabulary, self.base_weight(seat, Cat::Constabulary))
                        }
                        // Ticket #389 (version 0.09.3): the Stadium after the Constabulary, at the
                        // designer's word -- only where one already stands and Unrest is still 5 or
                        // more, the second answer to a Region that stays restive.
                        // Ticket #410 (version 0.09.4): or alone, where the Region has one slot left
                        // and the Constabulary cannot have it too.
                        FacilityKind::Stadium => {
                            if !self.ai_offers_calming(sid, FacilityKind::Stadium) {
                                continue;
                            }
                            (Cat::Stadium, self.base_weight(seat, Cat::Stadium))
                        }
                        // Ticket #411 (version 0.09.4): the Nature Reserve, a little for everyone and
                        // on the Sink gap for the Custodians; part of every seat's Unrest management
                        // at the designer's word, at the Stadium's weight from Unrest 4.
                        FacilityKind::NatureReserve => {
                            let base = self.base_weight(seat, Cat::NatureReserve);
                            let restive = self.state(sid).unrest >= self.tables.ai.thresholds.constabulary_from;
                            (Cat::NatureReserve, if restive { base.max(self.base_weight(seat, Cat::Stadium)) } else { base })
                        }
                        FacilityKind::LaunchSite => {
                            if has_launch {
                                continue;
                            }
                            (Cat::LaunchSiteOrShipyard, self.base_weight(seat, Cat::LaunchSiteOrShipyard))
                        }
                        // Ticket #54: a Scrubber has its own weight, its own cap and no build slot,
                        // so it is enumerated below rather than here.
                        FacilityKind::Scrubber => continue,
                        // Ticket #77: a Sea Wall takes no build slot, so it is enumerated below with
                        // the Scrubber rather than here among the slot-takers.
                        FacilityKind::SeaWall => continue,
                        // Unreachable: every Unique Facility was mapped to its common job above.
                        FacilityKind::InvestmentBank | FacilityKind::Spaceport | FacilityKind::Accumulator | FacilityKind::Academy => continue,
                    };
                    // Ticket #181: a slight bias toward a seat's own Unique Facility over the common
                    // counterpart -- the designer's words, and deliberately small, because ticket #41
                    // measured a Ducat-hungry AI costing the Custodians every win in twenty seeds.
                    // Ticket #182: the Prospectors are the exception. An Investment Bank's worth IS
                    // 1% of the Venture Capital Fund, so their appetite tracks the balance rather
                    // than a flat bias: worth nothing beside a Factory while the Fund is empty, and
                    // outbidding one on its own merits once the Fund is in the hundreds.
                    if fk.unique_to().is_some() {
                        base *= if fk == FacilityKind::InvestmentBank {
                            1.0 + self.seat(seat).venture_fund * m.investment_bank_per_fund
                        } else {
                            m.unique_bias
                        };
                    }
                    let produces = self.tables.facility(fk).produces.as_ref().map(|p| p.resource);
                    if cat == Cat::Producer {
                        if produces.map(|p| needs.contains(&p) || p == scarce).unwrap_or(false) {
                            base *= 1.5;
                        }
                        if produces == Some(Resource::Energy) && tight {
                            base += m.energy_shortage_bonus;
                        }
                    }
                    let name = fk.name();
                    // Ticket #41: the first Embassy in a state is a threat answer while the rival's
                    // standing presses on the seat's own there. (An opportunity multiplier on the seat's
                    // most valuable state was tried too: four Embassies a game, and no wins.)
                    let first_embassy = fk == FacilityKind::Embassy
                        && !self.state(sid).facilities.iter().any(|f| f.kind == FacilityKind::Embassy)
                        && !self.state(sid).queue.iter().any(|b| b.item == BuildItem::Facility(FacilityKind::Embassy));
                    // Ticket #52: a Constabulary in a state the seat has just Occupied is worth more:
                    // Occupation is what put the Unrest there, and Pacification halves above 4.
                    // Ticket #389 (version 0.09.3): the Stadium takes the Constabulary's multipliers
                    // here and below, at the designer's word: its second answer to the same Region,
                    // which in a Region just Occupied is the same restive Region the Occupation made.
                    let calms = matches!(fk, FacilityKind::Constabulary | FacilityKind::Stadium)
                        || (fk == FacilityKind::NatureReserve && self.state(sid).unrest >= self.tables.ai.thresholds.constabulary_from);
                    let just_occupied = calms && self.state(sid).control.is_occupied();
                    let sway = if (first_embassy && self.standing_pressed(seat, Place::State(sid))) || just_occupied { m.threat } else { 1.0 };
                    // Ticket #56: a Facility that waits on a Tech is not offered until it is in.
                    if self.tables.facility(fk).needs_tech.map(|t| !self.has_tech(t)).unwrap_or(false) {
                        continue;
                    }
                    // Ticket #56: the Sea Wall doubles in worth while a threshold of any kind stands
                    // within 0.2 C of the Temperature and the state still has a coast to lose.
                    // Ticket #60: and a Constabulary doubles at Unrest 9, where one more turn would
                    // throw the seat off the state, exactly as Relief doubles at the same figure.
                    let sea_close = fk == FacilityKind::SeaWall && self.sea_is_close(sid);
                    // Ticket #332 (version 0.09.0): and the early Mine, in the one Region chosen above.
                    let early_mine = job == FacilityKind::Mine && early_mine_region == Some(sid);
                    let seizes_the_moment = sea_close || early_mine || (calms && self.state(sid).unrest >= 9.0);
                    let opportunity = if seizes_the_moment { m.opportunity } else { 1.0 };
                    // Ticket #70 (version 0.05.5): the rising sea is a threat to the state, so a Sea
                    // Wall with the sea close takes the threat multiplier as well.
                    let sway = if sea_close { m.threat } else { sway };
                    // Ticket #60: #52 and #53 measured no Constabulary in any AI game -- a building
                    // that fixes nothing economic never beat a producer under the victory-gap
                    // multiplier, so it never won a build slot while the gap was wide. From the Unrest
                    // the candidate is offered at (`ai_offers_calming`, 4 since ticket #410) it takes the
                    // multiplier too, because a state at 7 halves every Facility's output and every
                    // Facility's Emissions: calming it advances whatever the seat is behind on.
                    // Ticket #70: and the victory-gap multiplier, as the Constabulary does at Unrest
                    // 5, so it competes with the Scrubber on even terms. The Research ticket of
                    // 0.05.5 found Coastal Engineering done by turn 16 to 18 in every seed and no
                    // Sea Wall ever built: the Custodian AI held its Materials for a Scrubber every time.
                    let pull = if calms || sea_close || early_mine { gap } else { gap_for(cat, Some(name)) };
                    let note = if early_mine { format!("build {} in {} (the early Mine)", name, self.tables.state(sid).name) } else { format!("build {} in {}", name, self.tables.state(sid).name) };
                    push(vec![Order::BuildFacility { state: sid, kind: fk }], cat, base, pull, sway, opportunity, note, None);
                }
            }
            // Ticket #54: a Scrubber takes no build slot, so it is offered whether or not one is
            // free, up to the state's cap, and only while the seat's Energy is not already tight:
            // 4 Energy upkeep with nothing to run it is a Facility shut down at the next Income.
            // Ticket #561 (version 0.1.0.3): and only while last turn's Ducat income covers one
            // more Scrubber's Ducat upkeep, so a seat short of Ducats builds fewer.
            if kind == FactionKind::Custodians
                && self.state(sid).control == Control::Controlled(seat)
                && self.scrubbers_committed(sid) < self.scrubber_cap(sid)
                && !tight
                && self.seat(seat).income_last_turn.ducats >= self.tables.facility(FacilityKind::Scrubber).ducats_upkeep
            {
                let cat = Cat::Scrubber;
                let opp = if self.scrubbers_online(sid) == 0 { m.opportunity } else { 1.0 };
                push(
                    vec![Order::BuildFacility { state: sid, kind: FacilityKind::Scrubber }],
                    cat,
                    self.base_weight(seat, cat),
                    gap_for(cat, Some("Scrubber")),
                    1.0,
                    opp,
                    format!("build a Scrubber in {} ({} of {})", self.tables.state(sid).name, self.scrubbers_committed(sid) + 1, self.scrubber_cap(sid)),
                    None,
                );
            }
            // Ticket #77 (version 0.05.5): a Sea Wall takes no build slot, as the Scrubber does, so it
            // is offered whether or not a slot is free: with Coastal Engineering in, no wall standing
            // or on order, and a coast still to protect. With the sea within 0.2 C it takes the
            // victory-gap, threat and opportunity multipliers (ticket #70), so it competes with the
            // Scrubber on even terms.
            if self.has_tech(TechId::CoastalEngineering) && !self.sea_wall_committed(sid) && self.coastal_slots(sid) > 0 {
                let close = self.sea_is_close(sid);
                let (pull, sway, opp) = if close { (gap, m.threat, m.opportunity) } else { (1.0, 1.0, 1.0) };
                push(vec![Order::BuildFacility { state: sid, kind: FacilityKind::SeaWall }], Cat::SeaWall, self.base_weight(seat, Cat::SeaWall), pull, sway, opp, format!("build Sea Wall in {}", self.tables.state(sid).name), None);
            }
            // Ticket #54: Leapfrog, the Custodians' other clause, on the most populous state they
            // hold once they have Ducats to spare.
            if kind == FactionKind::Custodians
                && self.seat(seat).stockpile.ducats + 3.0 * self.seat(seat).income_last_turn.ducats >= self.tables.ducats.per_leapfrog as f64
                && self.state(sid).control == Control::Controlled(seat)
                && self.leapfrog_would_bite(sid)
                && most_populous == Some(sid)
            {
                push(
                    vec![Order::Leapfrog { state: sid }],
                    Cat::Leapfrog,
                    self.base_weight(seat, Cat::Leapfrog),
                    gap_for(Cat::Leapfrog, None),
                    1.0,
                    1.0,
                    format!("Leapfrog {} ({:.2} per hundred million now)", self.tables.state(sid).name, self.population_coefficient(sid) * self.tables.units_per_hundred_million()),
                    None,
                );
            }
            // Ticket #54: the Strip Permit, on the Prospectors' highest-output state while they are
            // behind on the Extraction pace. Its Ducats are paid at the order (ticket #537, version 0.1.0.1),
            // its Baseline and Unrest three turns later.
            if kind == FactionKind::Prospectors
                && behind_on_extraction
                && !self.state(sid).strip_permit_used
                && self.state(sid).control == Control::Controlled(seat)
                && highest_output == Some(sid)
            {
                push(
                    vec![Order::StripPermit { state: sid }],
                    Cat::StripPermit,
                    self.base_weight(seat, Cat::StripPermit),
                    gap_for(Cat::StripPermit, Some("Strip Permit")),
                    1.0,
                    1.0,
                    format!("issue a Strip Permit in {}", self.tables.state(sid).name),
                    None,
                );
            }
            let base = self.base_weight(seat, Cat::RaiseIndustry);
            push(vec![Order::RaiseIndustry { state: sid }], Cat::RaiseIndustry, base, gap_for(Cat::RaiseIndustry, Some("Industry Level")), 1.0, 1.0, format!("raise Industry Level in {}", self.tables.state(sid).name), None);
            // Ticket #46: no Ship is built at a Launch Site; Shipyards on stations and Colonies build them.
            // Ticket #302 (version 0.08.6): an Army is worth its home's Industry + 1, fixed, so it is
            // raised where that is highest among the Regions the seat controls.
            let best_industry = self.controlled_states(seat).into_iter().map(|s| self.state(s).industry_level).max().unwrap_or(0);
            if self.state(sid).control == Control::Controlled(seat) && self.state(sid).industry_level == best_industry {
                let threat = if self.enemy_army_near(seat, Place::State(sid)) { m.threat } else { 1.0 };
                let armies = self.armies.iter().filter(|a| !a.standing && self.army_seat(a) == Some(seat)).count();
                if armies < 2 {
                    push(vec![Order::BuildArmy { place: Place::State(sid) }], Cat::ArmyOrBarracks, self.base_weight(seat, Cat::ArmyOrBarracks), 1.0, threat, 1.0, format!("build Army in {}", self.tables.state(sid).name), None);
                }
            }
        }

        // --- Colony builds
        for cid in self.directed_colonies(seat) {
            let col = self.colony(cid).unwrap().clone();
            // Ticket #278 (version 0.08.5): a starved Colony is the threat made good; the seat
            // learns to want a warship where it is blockaded.
            // Ticket #461 (version 0.09.7): and at the Archive's Colony while a rival has cause and the
            // fund is worth taking, so the Archivists raise a Barracks and an Army there.
            let threat = if self.enemy_present_or_inbound(seat, col.body) || self.enemy_army_near(seat, Place::Colony(cid)) || self.starved_by(cid).is_some() || self.archive_threatened(seat, cid) { m.threat } else { 1.0 };
            // Ticket #97 (version 0.07.0): no room, nothing to enumerate. Without this the AI scores
            // Modules it cannot build, spends its list on them and has them dropped at commit.
            if self.free_module_slots(&col) == 0 {
                continue;
            }
            // Ticket #447 (version 0.09.6): a place under Blockade makes no Widgets, so nothing built
            // for Materials there ever finishes. It weighs none; the Battery bought with Ducats, below,
            // is the one build that lands, and it comes first.
            let starved_here = self.starved_by(cid).is_some();
            for mk in ModuleKind::BUILDABLE {
                if starved_here {
                    break;
                }
                // Ticket #186 (version 0.08.0): as on Earth -- the Custodians build their Academy in
                // place of the Institute, nobody else builds an Academy, and what is left is weighed
                // by the job it does.
                if mk.built_by(self.kind(seat)) != mk || mk.unique_to().is_some_and(|f| f != self.kind(seat)) {
                    continue;
                }
                // Ticket #46: a station holds only a Shipyard and Habitats; ticket #80: and an
                // Observatory. Ticket #81: a Habitat over Earth now houses people who count as off
                // Earth, so the AI builds them there too.
                //
                // Ticket #239 (version 0.08.3): by JOB, for the reason the same list in `ui.rs`
                // carries -- a Unique Module is not its common kind, so a list of kinds throws
                // every Unique away the moment `built_by` swaps one in.
                // The Institute is excluded here and NOT in the interface's copy of this rule.
                // That difference predates ticket #239 -- ticket #185 added the Institute to the
                // player's station list and not to this one -- so the computer has never raised an
                // Institute on a station while a player may. It is left standing rather than
                // quietly changed, because changing it changes what the computer builds.
                if col.in_orbit && (!mk.stands_on_a_station() || mk.does_the_job_of(ModuleKind::Institute)) {
                    continue;
                }
                // Ticket #90: one Trade Post per Body; worth more once a second Body is held, since
                // the network is what it pays for.
                // Ticket #366 (version 0.09.2): by the job, so the Prospectors' Exchange is under it.
                if mk.does_the_job_of(ModuleKind::TradePost) && self.trade_post_at_body(seat, col.body) {
                    continue;
                }
                // Ticket #89: a station-only Module stands on no ground Colony.
                if !col.in_orbit && self.tables.module(mk).station_only {
                    continue;
                }
                let module_job = mk.common().unwrap_or(mk);
                // Ticket #630 (version 0.1.0.6): the Reactor Module in the Generator's and the Wind
                // Farm's place while the seat can keep it fed; never otherwise.
                let feeds = self.ai_feeds_reactors(seat);
                if (module_job == ModuleKind::FissionReactor && !feeds) || (feeds && matches!(module_job, ModuleKind::Generator | ModuleKind::WindFarm)) {
                    continue;
                }
                // Ticket #332 (version 0.09.0): a Factory Module speeds the Colony's whole queue.
                let mut speeds_the_queue = false;
                let (cat, mut base) = match module_job {
                    // Ticket #332 (version 0.09.0): a Factory Module is wanted at any Colony whose
                    // queue is `factory_module_queue_depth` deep or where a Ship is wanted (a
                    // Shipyard standing or on order) -- the designer's words -- and nowhere else,
                    // since four Widgets a turn at a Colony with nothing under way are simply lost.
                    // Where it is wanted it takes the gap and the opportunity below: the queue is the
                    // moment, and the Module speeds whatever the seat is behind on there. At a plain
                    // Producer's weight it lost to the Mine beside it whenever Materials were scarce
                    // (6 against 9, measured), which is every turn of every game.
                    ModuleKind::Factory => {
                        let ship_wanted = col.modules.iter().any(|m| m.kind == ModuleKind::Shipyard) || col.queue.iter().any(|b| b.item == BuildItem::Module(ModuleKind::Shipyard));
                        // Ticket #441 (version 0.09.6): a yard wants ONE Factory. Nothing counted the
                        // Factories already there, so a yard Colony filled every free slot with them
                        // at the full victory gap, and the Arkwrights' one station over Earth spent its
                        // Materials on Factories and never launched. One standing or on order ends the
                        // yard's claim; a queue deep enough still wants one on its own account, below.
                        let has_factory = col.modules.iter().any(|m| m.kind == ModuleKind::Factory) || col.queue.iter().any(|b| b.item == BuildItem::Module(ModuleKind::Factory));
                        let ship_wanted = ship_wanted && !has_factory;
                        if col.queue.len() < th.factory_module_queue_depth && !ship_wanted {
                            continue;
                        }
                        speeds_the_queue = true;
                        (Cat::Producer, self.base_weight(seat, Cat::Producer) * self.production_moved_boost(seat, &col, mk))
                    }
                    // Ticket #89: a Solar Array is an Energy producer; the Energy-shortage bonus below
                    // is what makes the AI raise one when the Stockpile is within a turn of nothing.
                    ModuleKind::SolarArray => (Cat::Producer, self.base_weight(seat, Cat::Producer)),
                    // Ticket #559 (version 0.1.0.3): the Wind Farm Module, an Energy producer on Mars.
                    ModuleKind::WindFarm => (Cat::Producer, self.base_weight(seat, Cat::Producer)),
                    // Ticket #80: an Observatory once the Colony holds enough Colonists to make it worth
                    // its keep (`observatory_colonists`), at the Research Lab's weight.
                    // Ticket #185 (version 0.08.0): an Institute multiplies an Observatory exactly as
                    // a School multiplies a Lab, so it waits for one and then takes the same weight.
                    ModuleKind::Institute => {
                        if !col.modules.iter().any(|m| m.kind == ModuleKind::Observatory)
                            || col.modules.iter().any(|m| m.kind.does_the_job_of(ModuleKind::Institute))
                            || col.queue.iter().any(|b| matches!(b.item, BuildItem::Module(k) if k.does_the_job_of(ModuleKind::Institute)))
                        {
                            continue;
                        }
                        (Cat::ResearchLab, self.base_weight(seat, Cat::ResearchLab))
                    }
                    ModuleKind::Observatory => {
                        if col.colonists < self.tables.ai_weights(self.kind(seat)).observatory_colonists
                            || col.modules.iter().any(|m| m.kind == ModuleKind::Observatory)
                            || col.queue.iter().any(|b| b.item == BuildItem::Module(ModuleKind::Observatory))
                        {
                            continue;
                        }
                        // Ticket #81: at its own weight; ticket #82: twice it when an idle Research
                        // Lab of the seat's would double it. Ticket #140 (version 0.07.3): times the
                        // Research yield here, so the computer builds its Observatories where the
                        // science is, the way it digs where the ore is.
                        (Cat::Observatory, self.base_weight(seat, Cat::Observatory) * self.production_moved_boost(seat, &col, mk) * self.research_yield_at(&col))
                    }
                    // Ticket #90: a Trade Post pays for the network. Ticket #239 (version 0.08.3):
                    // and the weight now READS that network instead of taking a step at two Bodies.
                    //
                    // It had the disease tickets #232 named on the Mine and the Relay, in its worst
                    // form: a Trade Post is the only Producer whose resource is Ducats, and the
                    // Producer bonus fires only for a resource the seat is SHORT of -- `scarcest`
                    // returns only Energy, Materials or Fuel and `needs` holds only Materials or
                    // Energy, deliberately, since ticket #41 measured Ducats on that list as
                    // costing the Custodians every win. So a Trade Post could never earn the x1.5
                    // its rivals routinely earn, and the sweep found ZERO standing in 120 games --
                    // on a building worth about 20 Ducats a turn to the Prospectors at their
                    // busiest Body, which is nearly two Regions' income.
                    //
                    // The ratio is its real yield here against its card's bare figure, CLAMPED to
                    // the same band the Mine's tech factor runs in (1.0 to 2.06). Unclamped it
                    // reaches 5 at a four-Colonist Colony and 12 at a rich one, which is how
                    // ticket #232's first attempt at the Mine put 329 Mines on the board.
                    ModuleKind::TradePost => {
                        // Ticket #426 (version 0.09.5): read for the kind this seat builds, so the
                        // Prospectors' Exchange is weighed with its extra Ducat.
                        let bare = self.tables.module(ModuleKind::TradePost).produces.as_ref().map(|p| p.amount).unwrap_or(1.0).max(1.0);
                        let with = self.module_yield(seat, cid, mk).amount;
                        // Ticket #491 (version 0.09.9): an Exchange pays the Investment Bank's interest,
                        // a share of the Fund, so its appetite tracks the balance as the Bank's does.
                        // One a Body (ticket #90), so never a second share at one place.
                        let shares = mk.built_by(self.kind(seat)) == ModuleKind::Exchange
                            && self.tables.faction(self.kind(seat)).victory_first.kind == crate::data::VictoryFirstKind::VentureFund;
                        let interest = if shares { 1.0 + self.seat(seat).venture_fund * m.investment_bank_per_fund } else { 1.0 };
                        (Cat::Producer, self.base_weight(seat, Cat::Producer) * (with / bare).clamp(0.25, 2.0) * interest)
                    }
                    // Ticket #92: a Mass Driver at a low-gravity ground Colony with a Mine, once the
                    // Tech stands, one per Colony; and a Mine beside one weighs what the driver adds.
                    ModuleKind::MassDriver => {
                        let card = self.tables.module(mk);
                        let allowed = !col.in_orbit
                            && self.tables.body(col.body).low_gravity
                            && card.needs_tech.map(|t| self.has_tech(t)).unwrap_or(true)
                            && col.modules.iter().any(|m| m.kind == ModuleKind::Mine)
                            && !col.modules.iter().any(|m| m.kind == ModuleKind::MassDriver)
                            && !col.queue.iter().any(|b| b.item == BuildItem::Module(ModuleKind::MassDriver));
                        if !allowed {
                            continue;
                        }
                        (Cat::Producer, self.base_weight(seat, Cat::Producer) * 1.5)
                    }
                    ModuleKind::Mine => {
                        let mut w = self.base_weight(seat, Cat::Producer) * self.production_moved_boost(seat, &col, mk);
                        // Ticket #232 (version 0.08.3): a Mine's weight reads WHAT ITS TECHS ARE
                        // WORTH, so every multiplier on it moves the seat's appetite. Before this
                        // the weight was flat, and the consequence was not small: Deep Mining's
                        // x1.5 and the Extraction Charter's x1.25 had never once made a computer
                        // seat want a Mine more, which is the likeliest reason the sweep found 26
                        // Mines standing across forty games. The designer, asked whether to fix
                        // Beneficiation alone or the whole gap: "let's fix that one outright".
                        //
                        // It reads the TECH factor and never the finished yield. The finished
                        // yield carries the SLOT's own yield, which is at least 1 everywhere, so a
                        // weight scaled by it lifts every Mine on the board with no Tech at all.
                        // Built that way first and measured: Mines standing over twenty games went
                        // 16 to 329, and the seating's win column swung from [14, 2, 0, 0] to
                        // [0, 18, 0, 0]. With the tech factor alone it is 1.0 until Deep Mining
                        // lands and 2.06 with all three.
                        w *= self.tech_output_multiplier_module(seat, ModuleKind::Mine);
                        if col.modules.iter().any(|m| m.kind == ModuleKind::MassDriver && m.working()) {
                            // The yield here already carries the bonus; weigh it against the bare figure.
                            let with = self.module_yield(seat, cid, ModuleKind::Mine).amount;
                            let plain = (with - self.tables.mass_driver.mine_bonus as f64).max(1.0);
                            w *= with / plain;
                        }
                        (Cat::Producer, w)
                    }
                    ModuleKind::Generator | ModuleKind::Refinery => (Cat::Producer, self.base_weight(seat, Cat::Producer) * self.production_moved_boost(seat, &col, mk)),
                    // Ticket #630 (version 0.1.0.6).
                    ModuleKind::FissionReactor => (Cat::Producer, self.base_weight(seat, Cat::Producer)),
                    // Ticket #232 (version 0.08.3): the Relay has the same disease the Mine had and
                    // worse -- a flat weight reading nothing about what a Relay produces, and ONE
                    // Relay built in forty games. Relay Networks would have changed the computer's
                    // behaviour by exactly zero without this. The designer: "this should resolve
                    // with Q5 full fix".
                    ModuleKind::Relay => {
                        // A Relay's Allotment has no slot component -- it is the card figure plus
                        // Relay Networks -- so this ratio IS the tech factor: 1.0 until the Tech
                        // lands, 2.0 after.
                        //
                        // Ticket #239 (version 0.08.3): weighed as `mk`, the kind this seat would
                        // ACTUALLY build, not as the common Relay. Written with `ModuleKind::Relay`
                        // hard-coded it read the common card twice and the Arkwrights' Chorus --
                        // whose whole clause is an Allotment that grows with the Colony -- would
                        // have been weighed as though the clause did not exist, which is the
                        // mistake ticket #232 found on the Mine wearing different clothes.
                        let bare = self.tables.module(mk).influence_allotment.max(1) as f64;
                        let with = self.module_yield(seat, cid, mk).allotment as f64;
                        (Cat::BuildInfluence, self.base_weight(seat, Cat::BuildInfluence) * (with / bare).max(0.25))
                    }
                    // Ticket #51: the Archive is never an ordinary Module build; it has its own order.
                    // Ticket #164 (version 0.07.5): nor is the Core Module, which a founding gives.
                    ModuleKind::Archive | ModuleKind::Core => continue,
                    // Ticket #361 (version 0.09.1): the ferrying Archivists make ROOM at the Archive's
                    // Colony, where the ferry lands and the uploads draw. Measured: no eligible Colony
                    // ever held more than the Core's four, so a ferry had nowhere to land them.
                    ModuleKind::Habitat if ferry_lift && archive_body == Some(col.body) && self.may_hold_archive(&col) => (Cat::Habitat, self.base_weight(seat, Cat::Habitat) * homeless_bonus),
                    ModuleKind::Habitat => (Cat::Habitat, self.base_weight(seat, Cat::Habitat)),
                    ModuleKind::Shipyard => {
                        if col.modules.iter().any(|m| m.kind == ModuleKind::Shipyard) {
                            continue;
                        }
                        // Ticket #398 (version 0.09.3): sought where Build Where You Dig reaches its Ships.
                        (Cat::LaunchSiteOrShipyard, self.base_weight(seat, Cat::LaunchSiteOrShipyard) * self.ai_shipyard_bonus(cid) * moon_race)
                    }
                    ModuleKind::Barracks => {
                        if col.modules.iter().any(|m| m.kind == ModuleKind::Barracks) {
                            continue;
                        }
                        (Cat::ArmyOrBarracks, self.base_weight(seat, Cat::ArmyOrBarracks))
                    }
                    // Ticket #324 (version 0.08.8): a Battery where a rival's warship stands at the
                    // Body or a rival's Carrier is inbound to it, one per Colony, at the Barracks'
                    // weight and the threat term; nowhere else, since it makes nothing and draws
                    // three Energy a turn.
                    ModuleKind::Battery => {
                        let pressed = self.ships.iter().any(|s| {
                            s.seat != seat
                                && ((s.at == ShipAt::Body(col.body) && s.kind.is_warship() && !s.escaped)
                                    || (s.kind == UnitKind::Carrier && matches!(s.at, ShipAt::Transit { to, .. } if to == col.body) && self.sees_ship(seat, s)))
                        });
                        if !pressed || col.modules.iter().any(|m| m.kind == ModuleKind::Battery) || col.queue.iter().any(|b| b.item == BuildItem::Module(ModuleKind::Battery)) {
                            continue;
                        }
                        (Cat::ArmyOrBarracks, self.base_weight(seat, Cat::ArmyOrBarracks))
                    }
                    // Unreachable: every Unique Module was mapped to its common job above.
                    ModuleKind::Academy | ModuleKind::Heliostat | ModuleKind::Exchange | ModuleKind::Chorus => continue,
                };
                // Ticket #181: the slight bias toward a seat's own Unique Module, as on Earth.
                if mk.unique_to().is_some() {
                    base *= m.unique_bias;
                }
                let produces = self.tables.module(mk).produces.as_ref().map(|p| p.resource);
                if cat == Cat::Producer {
                    if produces.map(|p| needs.contains(&p) || p == scarce).unwrap_or(false) {
                        base *= 1.5;
                    }
                    if produces == Some(Resource::Energy) && tight {
                        base += m.energy_shortage_bonus;
                    }
                }
                // A Habitat is only worth building when Colonists are coming.
                //
                // Ticket #290 (version 0.08.6): the opening, at the designer's word ("the computer
                // should know this"). While a STARTING station -- one over Earth that stood when the
                // game opened -- has a slot free and under four berths empty, its Habitat is pushed
                // at the opportunity weight so it comes before the Power Plants and Trade Posts it
                // used to lose to; the muster then follows the room aboard, as it always has. A
                // station with two aboard passes the gate above that a bare one, with four berths
                // empty, never did, which is why the opening had no need to exist before.
                //
                // Measured before the second clause: the Prospectors ranked their first Shipyard
                // and a Research Lab, both counted as advancing their Victory pace, above a Habitat
                // at twice its base weight, and opened with no Habitat. So the opening Habitat also
                // counts as advancing whatever the seat is behind on, as the first Shipyard does
                // (`gap_for`), and stands first in every seat. Not while Energy is tight: a
                // Habitat draws Energy, and the Solar Array a starved station wants would otherwise
                // lose its slot to the opening (measured on the ticket #89 test with Energy at
                // nought).
                //
                // And ONCE: only until the station's first Habitat stands or is on order. Written
                // without that clause it fired again every time the muster filled the station, so
                // the Prospectors put Habitat after Habitat on Tiangong at the head of every list
                // and never the Trade Post that earns their Ducats -- measured over twenty seeds
                // with the Custodians first, 330 Ducats a game against 2287 with the rule off, and
                // ten wins against nineteen. An opening is played once.
                let mut opening = false;
                // Ticket #514 (version 0.1.0.0): a ground Colony off Earth keeps slots for the Habitats
                // its next tier's people need (`habitats_to_reserve`): no other Module takes one of
                // them. Measured without it: every Capital candidate's slots filled with Mines,
                // Choruses and Trade Posts while it held four, and could never grow. The reserve is
                // the computer's own habit, judged against the board as committed: a Ducat buy, a
                // card's free Module and the player are not bound by it.
                if mk != ModuleKind::Habitat && self.free_module_slots(&col) <= self.habitats_to_reserve(&col) {
                    continue;
                }
                // Ticket #590 (version 0.1.0.4): and the seat's Capital site on a Body keeps one slot
                // open, the one its Capital's building will stand in, Habitats and all. Measured
                // without it: a slot open went to an Exchange ahead of the upgrade every turn.
                if self.capital_site(seat, col.body) == Some(cid) && self.free_module_slots(&col) <= 1 {
                    continue;
                }
                if mk == ModuleKind::Habitat {
                    let room = self.habitat_room(&col).saturating_sub(col.colonists);
                    // Ticket #514 (version 0.1.0.0): at the Capital candidate, while gathering, a
                    // Habitat is built AHEAD of need until the place can hold the Capital's people.
                    // Measured without this: the candidate's slots filled with Mines, Choruses and
                    // Trade Posts while it held room for four or five, and with every slot built no
                    // Habitat could follow, so nobody more could land, no slot could come and no
                    // tier could rise -- a deadlock at every candidate in 80 games.
                    let ahead = gathering && capital_candidate == Some(cid) && self.habitat_room(&col) < capital_room;
                    if room >= 4 && !ahead {
                        continue;
                    }
                    let first_habitat = !col.modules.iter().any(|m| m.kind == ModuleKind::Habitat) && !col.queue.iter().any(|b| b.item == BuildItem::Module(ModuleKind::Habitat));
                    opening = col.in_orbit && col.body == BodyId::Earth && col.founded_turn == 1 && first_habitat && !tight;
                }
                // Ticket #41: the first Relay at a Colony is a threat answer while the rival's standing
                // presses on the seat's own there, once the Colony has a producer Module (a Relay before
                // the first Mine starved the Colony). A second Relay is worth its base weight.
                // Ticket #426 (version 0.09.5): by the job, so an Exchange is a producer and a Chorus
                // is the first Relay, as the base buildings are.
                let has_producer = col.modules.iter().any(|m| [ModuleKind::Mine, ModuleKind::Generator, ModuleKind::Refinery, ModuleKind::TradePost].iter().any(|k| m.kind.does_the_job_of(*k)));
                let first_relay = mk.does_the_job_of(ModuleKind::Relay)
                    && has_producer
                    && !col.modules.iter().any(|m| m.kind.does_the_job_of(ModuleKind::Relay))
                    && !col.queue.iter().any(|b| matches!(b.item, BuildItem::Module(k) if k.does_the_job_of(ModuleKind::Relay)));
                let t = if matches!(cat, Cat::ArmyOrBarracks) {
                    threat
                } else if first_relay && self.standing_pressed(seat, Place::Colony(cid)) {
                    m.threat
                } else {
                    1.0
                };
                let (pull, opp) = if opening || speeds_the_queue { (gap, m.opportunity) } else { (gap_for(cat, Some(mk.name())), 1.0) };
                // Ticket #514 (version 0.1.0.0): every Module at the Capital candidate, while gathering,
                // so its slots fill for the next tier (a tier rises only with every slot built).
                // Measured without this: Settlements of 16 to 28 people stood at 9 or 10 Modules of 12.
                let base = if gathering && capital_candidate == Some(cid) { base * capital_chase } else { base };
                push(vec![Order::BuildModule { colony: cid, kind: mk }], cat, base, pull, t, opp, format!("build {} at {}", mk.name(), self.place_name(Place::Colony(cid))), None);
            }
            // Ticket #355 (version 0.09.1): a Colony STARVED by a rival's Blockade makes no Widgets,
            // so the Battery above can be queued and never finished, and the seat's only yard is
            // often this very station. It buys the Battery outright with Ducats instead, at the
            // threat's lift: a Battery covers its own orbit, so the blockader now has an enemy
            // present and a fight to take or leave. Measured before: the seat blockaded for 35
            // turns built no warship and answered nothing.
            if self.starved_by(cid).is_some() && !col.modules.iter().any(|m| m.kind == ModuleKind::Battery) {
                push(vec![Order::BuildModuleWithDucats { colony: cid, kind: ModuleKind::Battery }], Cat::ArmyOrBarracks, self.base_weight(seat, Cat::ArmyOrBarracks), 1.0, m.threat, m.opportunity, format!("buy a Battery for {} against the Blockade", self.place_name(Place::Colony(cid))), None);
            }
            // Ticket #359 (version 0.09.1): a WORKING Barracks, which is what the raise's door reads.
            if col.modules.iter().any(|m| m.kind == ModuleKind::Barracks && m.working()) && !self.armies.iter().any(|a| a.home == ArmyHome::Colony(cid)) {
                push(vec![Order::BuildArmy { place: Place::Colony(cid) }], Cat::ArmyOrBarracks, self.base_weight(seat, Cat::ArmyOrBarracks), 1.0, threat, 1.0, format!("build Army at {}", self.place_name(Place::Colony(cid))), None);
            }
        }

        // --- Ships. Ticket #332 (version 0.09.0): built at the yard with the most Widgets -- the
        // designer's words -- so a seat with two Shipyards offers each Ship at one of them, the
        // one whose Widgets finish it soonest, the first on the list among equals. A Carrier is
        // still built over Earth, where Armies board (ticket #43), so it goes to the Earth yard
        // with the most Widgets. Until this ticket the Ships were enumerated inside the Colony loop
        // above, after ticket #97's `continue` on a Colony with no Module slot free, so a full yard
        // offered no Ships at all; a yard's Ships take no Module slot, and they are enumerated here
        // whatever the yard's slots.
        // Ticket #398 (version 0.09.3): the yard where a Ship costs LEAST, the most Widgets among
        // equals (`ai_ship_yard`), so a Moon yard with a Mine is the fleet yard once it stands --
        // the designer's answer (Q5, A) to a sweep the rule never reached. A Carrier still goes to
        // the Earth yard with the most Widgets.
        let yards = self.ai_working_yards(seat);
        let most_widgets = |list: &[ColonyId]| list.iter().copied().rev().max_by_key(|c| self.widgets_at(Place::Colony(*c)));
        let best_yard = self.ai_ship_yard(seat);
        let best_earth_yard = most_widgets(&yards.iter().copied().filter(|c| self.colony(*c).unwrap().body == BodyId::Earth).collect::<Vec<_>>());
        for cid in yards {
            let col = self.colony(cid).unwrap();
            let threat = if self.enemy_present_or_inbound(seat, col.body) || self.enemy_army_near(seat, Place::Colony(cid)) || self.starved_by(cid).is_some() { m.threat } else { 1.0 };
            // Ticket #355 (version 0.09.1): CAUSE builds a fleet as a threat does. A seat at
            // `war_cause` or worse with a rival who holds a station or a Colony -- something a
            // warship can shut or break -- builds warships at the threat's lift. Measured before:
            // seats had cause in 710 of 2,456 seat-turns and a warship in 70.
            let cause_target = seat.others().iter().any(|o| self.has_cause(seat, *o) && self.colonies.iter().any(|c| c.control.director() == Some(*o)));
            let war_threat = if cause_target { m.threat.max(threat) } else { threat };
            // Ticket #447 (version 0.09.6): a STANDING fleet. A seat with a yard keeps
            // `warships_wanted` warships, `warships_wanted_blockaded` while a place of its own is
            // blockaded, counting those on order; below it a warship takes the threat's lift, and
            // the opportunity multiplier while blockaded. Measured before: 77 warships built in 80
            // games, and an orbital Battle in 7.
            let fleet = self.ships.iter().filter(|s| s.seat == seat && s.kind.is_warship()).count()
                + self.colonies.iter().filter(|c| c.control.director() == Some(seat)).flat_map(|c| c.queue.iter()).filter(|b| matches!(b.item, BuildItem::Unit(k) if k.is_warship())).count();
            let under_blockade = self.blockaded(seat);
            let fleet_short = (fleet as u32) < if under_blockade { th.warships_wanted_blockaded } else { th.warships_wanted };
            let war_threat = if fleet_short { m.threat.max(war_threat) } else { war_threat };
            let fleet_opp = if fleet_short && under_blockade { m.opportunity } else { 1.0 };
            for uk in UnitKind::SHIPS {
                let cat = match uk {
                    UnitKind::ColonyShip => Cat::ColonyShip,
                    // Ticket #343 (version 0.09.1): weighted apart from a warship, which it is not.
                    UnitKind::MissileCarrier => Cat::MissileCarrier,
                    _ => Cat::Warship,
                };
                // Ticket #43: a Carrier is built over Earth, where Armies board, and only when one wants carrying.
                if uk == UnitKind::Carrier {
                    if best_earth_yard != Some(cid) || !self.wants_carrier(seat) {
                        continue;
                    }
                // Ticket #343 (version 0.09.1): a Missile Carrier only with the Tech standing and a
                // place the seat wants and cannot take; one at a time, and at the busiest yard.
                } else if uk == UnitKind::MissileCarrier {
                    if best_yard != Some(cid) || !self.wants_missile_carrier(seat) {
                        continue;
                    }
                } else if best_yard != Some(cid) {
                    continue;
                }
                // Ticket #363 (version 0.09.1): a wanted Missile Carrier takes the threat's lift, as a
                // warship with cause does. Measured before: after the Tech, a seat had a target and a
                // yard in 561 seat-turns and never once queued a carrier.
                let lift = match cat {
                    Cat::Warship => war_threat,
                    Cat::MissileCarrier => m.threat,
                    _ => 1.0,
                };
                // Ticket #421 (version 0.09.4): a Colony Ship short of the Fuel its tank takes buys
                // the rest at the market in the same breath, at the Ship's weight.
                let build = Order::BuildShip { site: Place::Colony(cid), kind: uk, nuclear: self.ai_builds_nuclear(seat, uk) };
                // Ticket #455 (version 0.09.6): every Ship, a warship included, where the Colony Ship alone
                // was planned for and a warship short of its tank was refused for want of Fuel.
                let orders = match self.ai_fuel_top_up(seat, self.tank_of(seat, uk) + self.fuel_reserve(seat)) {
                    Some(buy) => vec![buy, build],
                    None => vec![build],
                };
                // Ticket #564 (version 0.1.0.3): the Colony Ship at the race weight while it stands.
                let race = if cat == Cat::ColonyShip { moon_race } else { 1.0 };
                push(orders, cat, self.base_weight(seat, cat) * race, gap_for(cat, None), lift, if cat == Cat::Warship { fleet_opp } else { 1.0 }, format!("build {} at {}", uk.name(), self.place_name(Place::Colony(cid))), None);
            }
        }

        // --- Influence, in units of 5 (spec 16.1), on the target rule of 16.4.
        let step = th.influence_step;
        // Ticket #394 (version 0.09.3): the targets in `ai_influence_targets`, a rival's Colony
        // weighed by its bounty.
        let targets = self.ai_influence_targets(seat);
        for (rank, (target, _)) in targets.iter().enumerate() {
            let have = self.seat(seat).influence.get(target).copied().unwrap_or(0);
            // Ticket #33: a controlled place needs a standing above the controller's as well, by the
            // challenge margin (ticket #41). Ticket #60: one computation, shared with the Resolution
            // and the state card.
            let needed = self.influence_needed_for(seat, *target);
            let base = self.base_weight(seat, Cat::Influence) * (1.0 - 0.15 * rank as f64).max(0.3);
            // Ticket #618 (version 0.1.0.5): on a contested Region the steps that would pass the
            // climber's Standing this turn take the opportunity multiplier -- enough to outbid where
            // the Allotment allows, else all of it -- so the planner stacks them ahead of the rest.
            let to_outbid = match target {
                Place::State(sid) if self.ai_contested(seat, *sid) => self.ai_climb_on(seat, *sid).map(|(_, top)| (top - have) / step + 1).unwrap_or(0).max(1),
                // Ticket #619 (version 0.1.0.5): on a held Region, the steps that answer the climb
                // or reach the home floor.
                Place::State(sid) if self.state(*sid).control.controller() == Some(seat) => (self.ai_defence_wanted(seat, *sid) + step - 1) / step,
                _ => 0,
            };
            let bought = if self.tables.ducats.per_influence > 0.0 { (self.seat(seat).stockpile.ducats / self.tables.ducats.per_influence).floor() as i64 } else { 0 };
            let copies = ((allotment + bought) / step).max(0);
            for i in 0..copies {
                let opp = if needed - have <= step || i < to_outbid { m.opportunity } else { 1.0 };
                push(vec![Order::Influence { target: *target, amount: step, break_accord: false }], Cat::Influence, base, 1.0, 1.0, opp, format!("spend {} Influence on {}", step, self.place_name(*target)), None);
            }
        }
        // Buy Influence with Ducats (ticket #35), in units of the step, weighted like Influence itself.
        let ducats = self.seat(seat).stockpile.ducats;
        let per = self.tables.ducats.per_influence;
        let buys = if per > 0.0 { (ducats / (per * step as f64)).floor() as i64 } else { 0 };
        for _ in 0..buys {
            push(vec![Order::BuyInfluence { amount: step }], Cat::Influence, self.base_weight(seat, Cat::Influence) * 0.9, 1.0, 1.0, 1.0, format!("buy {} Influence for {} Ducats", step, per * step as f64), None);
        }
        // Ticket #42: the trading window. While Materials are the scarcest resource (or the bootstrap
        // need), Ducats buy them in lots of 10 at a producer's weight; it sells, since ticket
        // #448 (version 0.09.6), at the end of its turn.
        // Ticket #448 (version 0.09.6): **Energy**, where the seat would be short at Income -- the
        // shortfall that shuts its buildings -- bought ahead of the spending, at a producer's weight
        // and the opportunity multiplier, since a building switched off pays for nothing.
        // Ticket #575 (version 0.1.0.3), at the designer's word: **the power reserve**, bought ahead
        // of everything else when the tank stands under it, at the Energy buy's weight and the
        // opportunity multiplier, since power at a quarter shuts the rest at the next Income.
        {
            let s = self.seat(seat);
            let power_reserve = self.fuel_reserve(seat);
            // The review of #575: a Ship's top-up buys the reserve beside its want, so while one is
            // on offer this buy stands down, or the two would buy the same Fuel twice.
            let top_up_on_offer = self.ships.iter().any(|sh| sh.seat == seat && matches!(sh.at, ShipAt::Body(_)) && sh.fuel < self.tank_of(seat, sh.kind) && self.ai_fuel_top_up(seat, self.tank_of(seat, sh.kind) - sh.fuel + power_reserve).is_some());
            if s.stockpile.fuel < power_reserve && !top_up_on_offer {
                let n = (power_reserve - s.stockpile.fuel).ceil() as i64;
                push(vec![Order::Buy { resource: Resource::Fuel, amount: n }], Cat::Producer, self.base_weight(seat, Cat::Producer) * 1.5, 1.0, 1.0, m.opportunity, format!("buy {n} Fuel for the power reserve ({power_reserve:.1})"), None);
            }
        }
        {
            let s = self.seat(seat);
            let short = s.stockpile.energy + s.income_last_turn.energy;
            if short < 0.0 {
                let n = (-short).ceil() as i64;
                push(vec![Order::Buy { resource: Resource::Energy, amount: n }], Cat::Producer, self.base_weight(seat, Cat::Producer), 1.0, 1.0, m.opportunity, format!("buy {n} Energy, short at Income otherwise"), None);
            }
        }
        let per_materials = self.tables.ducats.per_materials;
        if (scarce == Resource::Materials || needs.contains(&Resource::Materials)) && per_materials > 0 {
            let lots = (self.seat(seat).stockpile.ducats / (per_materials * 10) as f64).floor() as i64;
            for _ in 0..lots.min(4) {
                push(vec![Order::Buy { resource: Resource::Materials, amount: 10 }], Cat::Producer, self.base_weight(seat, Cat::Producer) * 1.5, 1.0, 1.0, 1.0, format!("buy 10 Materials for {} Ducats", self.lot_ducats(seat, Resource::Materials, 10, 0)), None);
            }
        }
        // Hold own places where a rival's standing approaches yours (ticket #33: spending raises your standing).
        // Ticket #75 (version 0.05.5): as many steps as it takes to stand two steps clear of the
        // rival's Standing plus the challenge margin, as many as the Allotment and the Ducats allow.
        // One hold a turn against a rival pouring its whole Allotment in lost seat 0's start state
        // on turn 7 in every seed of the Prospectors' batch.
        // Ticket #114 (version 0.07.1) had the computer defend by the same rule the player's Defence
        // button split by. Ticket #134 (version 0.07.3) retired Defence, button and rule together --
        // the designer's call: *"computer players lose it too"* -- and the AI is back on its own
        // arithmetic from ticket #75: once a rival's Standing comes within two steps of its own it
        // pushes as many holds as it takes to stand two steps clear of the rival plus the challenge
        // margin. Cruder than the rule it had (it does not ask whether the rival is above their own
        // threshold, nor count the decay), and measured by the sweep on the ticket.
        let mut owned: Vec<Place> = self.controlled_states(seat).into_iter().map(Place::State).collect();
        owned.extend(self.colonies.iter().filter(|c| c.control.controller() == Some(seat)).map(|c| Place::Colony(c.id)));
        let bought_steps = if per > 0.0 { (ducats / per).floor() as i64 } else { 0 };
        for place in owned {
            let rival = self.rival_standing(seat, place);
            let mine = self.seat(seat).influence.get(&place).copied().unwrap_or(0);
            // Ticket #461 (version 0.09.7): the Archive's Colony under threat is held from twice as far off.
            let reach = if matches!(place, Place::Colony(c) if self.archive_threatened(seat, c)) { 4 } else { 2 };
            if rival > 0 && rival + reach * step >= mine {
                let margin = self.challenge_margin_at(place);
                let need = (rival + margin + 2 * step - mine).max(step);
                let can = ((allotment + bought_steps) / step).max(1);
                let copies = ((need + step - 1) / step).clamp(1, can);
                let opp = if rival + step >= mine { m.opportunity } else { 1.0 };
                for _ in 0..copies {
                    push(vec![Order::Influence { target: place, amount: step, break_accord: false }], Cat::Influence, self.base_weight(seat, Cat::Influence), 1.0, m.threat, opp, format!("hold {} with {} Influence", self.place_name(place), step), None);
                }
            }
        }

        // --- Stations (ticket #46): one over each Body where the seat has a producing Colony and none yet.
        // Ticket #51: over Earth the foothold is a Nation State with a working Launch Site, so a
        // Faction that starts with no station (the Arkwrights) can build its first; while the seat
        // has no Shipyard anywhere, that first station is the thing that unlocks every Ship, so it
        // takes the opportunity multiplier.
        let has_shipyard = self.colonies.iter().any(|c| c.control.director() == Some(seat) && c.modules.iter().any(|m| m.kind == ModuleKind::Shipyard));
        for body in BodyId::ALL {
            let has_station = self.colonies.iter().any(|c| c.in_orbit && c.body == body && c.control.director() == Some(seat));
            let foothold = match body {
                BodyId::Earth => self.directed_states(seat).iter().any(|s| self.state(*s).facilities.iter().any(|f| f.kind.does_the_job_of(FacilityKind::LaunchSite) && f.working())),
                // Ticket #93: at Venus, a Body of orbits only, a Ship of the seat's there is the
                // foothold, and the station is what its Colonists land in.
                // Ticket #442 (version 0.09.6): no longer; a Colony Ship founds it, below.
                b if self.tables.body(b).colony_slots() == 0 => false,
                _ => self.colonies.iter().any(|c| !c.in_orbit && c.body == body && c.control.director() == Some(seat) && c.modules.iter().any(|m| matches!(m.kind, ModuleKind::Mine | ModuleKind::Generator | ModuleKind::Refinery))),
            };
            if has_station || !foothold {
                continue;
            }
            // Ticket #480 (version 0.09.8): a far orbit is founded by a Ship, not built.
            // Ticket #489 (version 0.09.9): and only with four to send up.
            if let Some(slot) = self.buildable_orbital_slots(body).first()
                && let Some(from) = self.station_founders(seat, body, &[])
            {
                let opp = if has_shipyard { 1.0 } else { m.opportunity };
                push(vec![Order::BuildStation { body, slot: *slot, from }], Cat::LaunchSiteOrShipyard, self.base_weight(seat, Cat::LaunchSiteOrShipyard) * self.ai_founding_pull(seat), 1.0, 1.0, opp, format!("build {} over {}", self.station_name(body, *slot), self.tables.body(body).name), None);
            }
        }
        // Ticket #490 (version 0.09.9): a place of the seat's whose slots are all taken at its tier's
        // cap, with the people the next tier wants, is raised to it.
        // Ticket #503 (version 0.1.0.0): the Colonial Capital too, where the Body has none (`open_tier`);
        // a station never sees it.
        // Ticket #514 (version 0.1.0.0): every upgrade at the Capital candidate weighs `capital_chase`
        // times more (the candidate and the figure are read at the head of this function).
        for c in self.colonies.iter().filter(|c| c.control.director() == Some(seat)) {
            // Ticket #590 (version 0.1.0.4): the Capital wants one slot open, not every slot built; a
            // lower tier, the people to fill its cap, not every slot built either: measured, 28 people
            // stood at an Outpost with 4 of its 6 slots built for want of the last two.
            if let Some(next) = self.open_tier(c)
                && c.colonists >= next.colonists
                && (if next.takes_slot { self.capital_site(seat, c.body) == Some(c.id) && self.free_module_slots(c) >= 1 } else { self.module_slots(c) >= self.tier_of(c).cap })
                && self.tier_under_way(c).is_none()
            {
                let chase = if capital_candidate == Some(c.id) { capital_chase } else { 1.0 };
                // Ticket #590 (version 0.1.0.4), at the designer's word: the seat builds up its best
                // place off Earth. Measured without it: its upgrade at 5 or 6 was held every turn for
                // the Materials of builds at 20 to 45, and no place off Earth passed Outpost.
                let chase = if self.best_off_earth(seat) == Some(c.id) { chase * self.tables.ai.thresholds.build_up } else { chase };
                push(vec![Order::RaiseTier { colony: c.id }], Cat::RaiseTier, self.base_weight(seat, Cat::RaiseTier) * chase, gap_for(Cat::RaiseTier, None), 1.0, 1.0, format!("upgrade {} to a {}", self.place_name(Place::Colony(c.id)), next.name), None);
            }
        }
        // Ticket #447 (version 0.09.6): **a backup yard.** A seat with exactly one Shipyard, and no
        // station of its own still waiting for one, wants a second station: over Earth first, where
        // two of its own may stand, then wherever the loop above offers one (the Moon first of them).
        // While a place of its own is blockaded the want takes the threat's lift. A seat whose one
        // yard is shut builds no Ship at all, which is how a Blockade held: measured, a seat
        // blockaded for 35 turns built no warship.
        {
            let has_yard = |c: &Colony| c.modules.iter().any(|m| m.kind == ModuleKind::Shipyard) || c.queue.iter().any(|b| b.item == BuildItem::Module(ModuleKind::Shipyard));
            let mine: Vec<&Colony> = self.colonies.iter().filter(|c| c.control.director() == Some(seat)).collect();
            let yards = mine.iter().filter(|c| has_yard(c)).count();
            let spare_station = mine.iter().any(|c| c.in_orbit && !has_yard(c));
            let over_earth = mine.iter().filter(|c| c.in_orbit && c.body == BodyId::Earth).count();
            if yards == 1 && !spare_station && over_earth == 1
                && let Some(slot) = self.buildable_orbital_slots(BodyId::Earth).first().copied()
                && let Some(from) = self.station_founders(seat, BodyId::Earth, &[])
            {
                let lift = if self.blockaded(seat) { m.threat } else { 1.0 };
                push(vec![Order::BuildStation { body: BodyId::Earth, slot, from }], Cat::LaunchSiteOrShipyard, self.base_weight(seat, Cat::LaunchSiteOrShipyard), 1.0, lift, 1.0, format!("build {} over Earth as a backup yard", self.station_name(BodyId::Earth, slot)), None);
            }
        }
        // Ticket #442 (version 0.09.6): the mirror -- a ground Colony BUILT from a working station of
        // the seat's at a Body where it has none on the ground, into the slot that best serves it,
        // at the station's weight; and the people it holds SENT DOWN to a ground Colony of the seat's
        // with room on the same Body, as many as fit.
        for body in BodyId::ALL.into_iter().filter(|b| *b != BodyId::Earth) {
            let station = self.colonies.iter().find(|c| c.in_orbit && c.body == body && c.control.director() == Some(seat) && self.starved_by(c.id).is_none()).map(|c| c.id);
            let Some(station) = station else { continue };
            let ground = self.colonies.iter().find(|c| !c.in_orbit && c.body == body && c.control.director() == Some(seat)).map(|c| c.id);
            match ground {
                None => {
                    // Ticket #489 (version 0.09.9): only from a station with four to spare.
                    if let Some(slot) = self.best_slot_for(seat, body, behind)
                        && let Some(from) = self.colony_founders(seat, body, &[])
                    {
                        push(vec![Order::BuildColony { body, slot, from }], Cat::LaunchSiteOrShipyard, self.base_weight(seat, Cat::LaunchSiteOrShipyard) * self.ai_founding_pull(seat), 1.0, 1.0, 1.0, format!("build a Colony at {} on {}", self.tables.body(body).slots[slot as usize].name, self.tables.body(body).name), None);
                    }
                }
                Some(down) => {
                    let room = self.colony(down).map(|c| self.habitat_room(c).saturating_sub(c.colonists)).unwrap_or(0);
                    let n = room.min(self.colony(station).map(|c| c.colonists).unwrap_or(0));
                    if n > 0 {
                        push(vec![Order::SendDown { from: station, to: down, colonists: n }], Cat::LoadUnload, self.base_weight(seat, Cat::LoadUnload), gap_for(Cat::LoadUnload, None), 1.0, 1.0, format!("send {} down to {}", n, self.place_name(Place::Colony(down))), None);
                    }
                }
            }
        }

        // --- The Archive (ticket #51). The Archivist AI funds it whenever the fund has room under
        // its cap, from turn one if it likes, and otherwise contributes to the shared Tech. Ticket
        // #68: it builds the one Module at the first Colony off Earth it took, and the fund's cap
        // is a quarter until that Module stands, so the Module is what opens the rest.
        // Ticket #235 (version 0.08.3): every Faction directs Research, not only the Archivists.
        // Ticket #236: and it weighs the Tech under research before deciding how much, at the
        // designer's word -- "the ai need to weigh the benefit of the new tech to which they
        // contributing". Without that a seat went to its cap on turn 2 and never moved, which was
        // measured at 24 to 28 turns of 36 below any contribution threshold, and left the
        // shared-pot rule of this ticket with no line worth drawing.
        //
        // The valuation is the pick list the AI already has: its own Victory gate and its `order`
        // are Techs it wants, its `last` and `never` are ones it does not, and anything else is
        // indifference. The three figures are in `ai.toml`, so the sweep can fit them.
        // The ARCHIVISTS are not in this: their directive is not a judgement about the Tech under
        // research at all, it is how they pay for the Archive, and their own branch below governs
        // it against the fund's cap. Weighing Techs for them would switch their Victory funding
        // off whenever the table researched something they liked.
        if kind != FactionKind::Archivists && self.seat(seat).research_last_turn > 0 {
            let th = &self.tables.ai.thresholds;
            let cap = self.research_directive_cap(seat);
            let want = match self.research.current {
                None => th.directive_when_indifferent,
                Some(t) => {
                    let picks = self.tables.ai_tech_picks(kind);
                    let mine = self.tables.victory_gate(kind) == Some(t);
                    if mine || picks.order.contains(&t) {
                        th.directive_when_wanted
                    // Ticket #462 (version 0.09.7): `last` is the rivals' gates, so a seat keeps
                    // back its whole cap while a gate that is not its own is under research.
                    } else if picks.never == Some(t) || picks.last.contains(&t) {
                        cap
                    } else {
                        // Ticket #530 (version 0.1.0.1): a seat racing for the Lead sends less of its
                        // Research to the Sink, the nearer its Victory.
                        ((th.directive_when_indifferent as f64) * (1.0 - self.ai_race_share(seat))).round() as _
                    }
                }
            };
            // Ticket #459 (version 0.09.7): the Custodians' Victory road done -- their gate stands,
            // and it needs every Tech before it -- Research only helps their rivals, so the whole
            // cap goes to the Sink. Past the free share it costs a point of Relations with every
            // rival; where that point would carry one to cause, they take the free share and no more.
            let want = if kind == FactionKind::Custodians && self.tables.victory_gate(kind).is_some_and(|g| self.has_tech(g)) {
                let rel = &self.tables.relations;
                let term_now = self.directive_relations_term(seat);
                // Giving everything earns a point, the free share earns none, more than that costs
                // one: the most diverted that puts no rival at cause who would not be there anyway.
                let at_cause = |term: i64| Seat::ALL.into_iter().filter(|r| *r != seat && self.relations_score(*r, seat) - term_now + term <= th.war_cause).count();
                let anyway = at_cause(rel.directive_step);
                if at_cause(-rel.directive_step) == anyway {
                    cap
                } else if at_cause(0) == anyway {
                    100 - rel.directive_min_contribution
                } else {
                    0
                }
            } else {
                want
            };
            let want = want.min(cap);
            if want != self.seat(seat).research_directive {
                let what = match kind {
                    FactionKind::Custodians => "the Natural Sink",
                    FactionKind::Prospectors => "their coffers",
                    FactionKind::Arkwrights => "propellant",
                    FactionKind::Archivists => "the Archive fund",
                };
                push(vec![Order::SetResearchDirective { percent: want }], Cat::FundArchive, self.base_weight(seat, Cat::FundArchive), gap_for(Cat::FundArchive, None), 1.0, 1.0, format!("direct {want} per cent of their Research into {what} from the next Income"), None);
            }
        }
        if kind == FactionKind::Archivists {
            let fund = self.seat(seat).archive_fund;
            let cap = self.archive_fund_cap(seat);
            if fund < cap && self.seat(seat).research_last_turn > 0 && self.seat(seat).research_directive == 0 {
                let opp = if fund + self.seat(seat).research_last_turn >= cap { m.opportunity } else { 1.0 };
                push(vec![Order::SetResearchDirective { percent: self.research_directive_cap(seat) }], Cat::FundArchive, self.base_weight(seat, Cat::FundArchive), gap_for(Cat::FundArchive, None), 1.0, opp, format!("pay the Labs into the Archive fund from the next Income, {} Research a turn", self.seat(seat).research_last_turn), None);
            }
            // Ticket #462 (version 0.09.7): a full fund takes nothing, and a directive left standing
            // still costs them Provisional Findings and a point with every rival, so it comes off;
            // the branch above puts it back the turn the fund has room again (#461: it can be lost).
            if fund >= cap && self.seat(seat).research_directive > 0 {
                push(vec![Order::SetResearchDirective { percent: 0 }], Cat::FundArchive, self.base_weight(seat, Cat::FundArchive), gap_for(Cat::FundArchive, None), 1.0, m.opportunity, "the fund is full: give all their Research to the shared Tech again".to_string(), None);
            }
            // Ticket #199 (version 0.08.0): the Archive also waits on the gate Tech, and the computer
            // is deliberately NOT taught that here. Every candidate goes through `check_order` before
            // it is chosen and through `check_order_legality` before it can reserve Materials, so a
            // refused Archive is skipped at no cost and never freezes the seat's build programme.
            // A guard here was written first and then removed: its false state could not be
            // constructed, which is the signal that the code was claiming to prevent something the
            // validator already prevents. Ticket #192's Colonist gate is different and stays -- it
            // chooses WHICH Colony to name, which no validator can do.
            if !self.archive_built(seat) && !self.archive_ordered(seat) {
                // Ticket #192 (version 0.08.0): the gate. The computer ordered the Archive on turn 1
                // of every one of 80 measured games, at a station with nobody on it; without this it
                // would simply spend that turn on a refused order, every turn, until somebody moved
                // in. The first place with enough people, oldest first as before.
                // Ticket #625 (version 0.1.0.5): the first place that has climbed to the tier the
                // Archive asks for (the Colony tier), where it was the first with four people.
                let home = self
                    .colonies
                    .iter()
                    .filter(|c| c.control.director() == Some(seat) && self.may_site_archive(c))
                    .min_by_key(|c| (c.founded_turn, c.id.0))
                    .map(|c| c.id);
                if let Some(cid) = home {
                    push(vec![Order::BuildArchive { colony: cid }], Cat::BuildArchive, self.base_weight(seat, Cat::BuildArchive), gap_for(Cat::BuildArchive, None), 1.0, m.opportunity, format!("build the Archive at {}", self.place_name(Place::Colony(cid))), None);
                }
            }
            // Ticket #192: and the Upload, which is the second half of their Victory Condition. The
            // rule is the simple one: whenever the Archive is complete and anybody is living at its
            // place, read them in. There is no reason to hold people back -- an uploaded Colonist
            // cannot be lost to a raid, a crowding death or a handover, and nothing else at that
            // place needs them. Without this the Archivists win nothing at all.
            if self.archive_complete(seat)
                && let Some(cid) = self.archive_colony(seat)
            {
                let here = self.colony(cid).map(|c| c.colonists).unwrap_or(0);
                let bar = self.tables.faction(kind).victory_second.bar as u32;
                let still_wanted = bar.saturating_sub(self.seat(seat).uploaded);
                // Ticket #565 (version 0.1.0.3): four a turn at most; what goes dark at the site is
                // accepted, at the designer's word -- an uploaded Colonist cannot be lost.
                let n = here.min(still_wanted).min(self.tables.archive.upload_per_turn);
                if n > 0 {
                    let opp = if self.seat(seat).uploaded + n >= bar { m.opportunity } else { 1.0 };
                    push(
                        vec![Order::Upload { colony: cid, n }],
                        Cat::Upload,
                        self.base_weight(seat, Cat::Upload),
                        gap_for(Cat::Upload, None),
                        1.0,
                        opp,
                        format!("upload {} Colonists into the Archive at {}", n, self.place_name(Place::Colony(cid))),
                        None,
                    );
                }
            }
        }

        // --- Ticket #52: Relief where Unrest has taken hold, and Resettle into a calm state of
        // the seat's own. Relief is one point per 10 Ducats the seat can spare, its weight read by
        // `ai_relief_weight` (ticket #410: from 5, double by 9).
        let u = self.tables.unrest.clone();
        let ducats = self.seat(seat).stockpile.ducats;
        for sid in self.directed_states(seat) {
            let n = self.state(sid).unrest;
            let Some(opp) = self.ai_relief_weight(n) else { continue };
            let points = if u.relief_ducats > 0 { ((ducats / u.relief_ducats as f64).floor() as i64).min(n.ceil() as i64) } else { 0 };
            for _ in 0..points {
                push(
                    vec![Order::Relief { state: sid }],
                    Cat::Relief,
                    self.base_weight(seat, Cat::Relief),
                    1.0,
                    1.0,
                    opp,
                    format!("pay Relief in {} (Unrest {})", self.tables.state(sid).name, Game::unrest_figure(n)),
                    None,
                );
            }
        }
        // Ticket #269 (version 0.08.4): Agitate in a Region held by a rival the seat is Wary or worse
        // toward -- most eagerly where Unrest already stands past the first threshold, a
        // cheap finish -- competing with Relief, Influence and the rest for the same Ducats and
        // Allotment, which is the whole price of it.
        {
            let ag = self.tables.unrest.clone();
            if ducats >= ag.agitate_ducats as f64 && allotment >= ag.agitate_influence {
                for sid in StateId::ALL {
                    let Some(holder) = self.state(sid).control.controller() else { continue };
                    if holder == seat || !self.has_cause(seat, holder) {
                        continue;
                    }
                    let n = self.state(sid).unrest;
                    let opp = if n >= ag.facility_threshold { m.opportunity } else { 1.0 };
                    push(
                        vec![Order::Agitate { state: sid }],
                        Cat::Agitate,
                        self.base_weight(seat, Cat::Agitate) * (1.0 + n / ag.max) * self.hostile_lift(seat, holder),
                        1.0,
                        1.0,
                        opp,
                        format!("agitate in {} ({}, Unrest {})", self.tables.state(sid).name, self.relations_level(seat, holder), Game::unrest_figure(n)),
                        None,
                    );
                }
            }
        }
        // Ticket #267 (version 0.08.4): a Smear campaign against a rival the seat is Wary or worse
        // toward whose Blame share stands above the fair quarter -- one step of Influence, weighed
        // by how far above the quarter it stands, at the opportunity multiplier when Hostile. It
        // competes with a place for the same Allotment, which is the whole price of it.
        {
            let fair = self.tables.influence.blame.fair_share;
            let step = th.influence_step;
            if allotment >= step {
                for rival in Seat::ALL.into_iter().filter(|r| *r != seat) {
                    let score = self.relations_score(seat, rival);
                    let over = self.blame_share(rival) - fair;
                    if !self.has_cause(seat, rival) || over <= 0.0 {
                        continue;
                    }
                    let opp = if score <= -8 { m.opportunity } else { 1.0 };
                    push(
                        vec![Order::Smear { target: rival, amount: step }],
                        Cat::Smear,
                        self.base_weight(seat, Cat::Smear) * (1.0 + over / fair) * self.hostile_lift(seat, rival),
                        1.0,
                        1.0,
                        opp,
                        format!("smear the {} (share {:.2}, {})", self.seat_name(rival), self.blame_share(rival), self.relations_level(seat, rival)),
                        None,
                    );
                }
            }
        }
        // Ticket #268 (version 0.08.4): carbon credits, while the seat stands above a fair share, the
        // Custodians are offering and will sell to it, and it can pay -- as much as the cap or the
        // offer allows, weighed by how far above the quarter it stands.
        // Ticket #277 (version 0.08.5): and where credits are to be had the seat buys them; a
        // Greenwash is for the seat that cannot -- the seller Hostile or not offering, or itself the seller.
        let mut credits_to_be_had = false;
        if let Some(seller) = self.credit_seller().filter(|s| *s != seat && !self.withholds_credits(*s, seat)) {
            let c = self.tables.carbon_credits.clone();
            let fair = self.tables.influence.blame.fair_share;
            let over = self.blame_share(seat) - fair;
            let offer = self.seat(seller).credits_offered;
            let ppm = c.cap_per_turn.min(offer);
            if over > 0.0
                && ppm > 0
                && let Some(cost) = self.credit_cost(seat, ppm)
                && self.seat(seat).stockpile.ducats >= cost
            {
                credits_to_be_had = true;
                push(
                    vec![Order::BuyCredits { ppm }],
                    Cat::BuyCredits,
                    self.base_weight(seat, Cat::BuyCredits) * (1.0 + over / fair),
                    1.0,
                    1.0,
                    1.0,
                    format!("buy {ppm} ppm of carbon credit for {cost} Ducats (share {:.2})", self.blame_share(seat)),
                    None,
                );
            }
        }
        // Ticket #277 (version 0.08.5): a Greenwash of the seat's own Blame while its share stands
        // above the fair quarter and credits are not to be had -- one step of Influence with its
        // Ducats beside it, weighed by how far above the quarter it stands, and only while the seat
        // keeps a reserve of Ducats past the price. It competes with a place for the Allotment and
        // with every building for the Ducats, which is the whole price of it.
        {
            let g = self.tables.influence.greenwash.clone();
            let fair = self.tables.influence.blame.fair_share;
            let over = self.blame_share(seat) - fair;
            let step = th.influence_step;
            let price = step * g.ducats_per_influence;
            if over > 0.0 && !credits_to_be_had && allotment >= step && self.seat(seat).stockpile.ducats >= (price + g.ai_ducats_reserve) as f64 {
                // Ticket #535 (version 0.1.0.1), at the designer's word: the campaign costs Relations
                // with every other Faction, and the seat weighs what it would lose with those it
                // values -- an Accord partner, or one that stands Cordial or better toward it.
                let valued_loss: f64 = seat
                    .others()
                    .into_iter()
                    .filter(|o| self.accords.iter().any(|acc| acc.holds(seat, *o)) || matches!(self.relations_level(*o, seat), "Cordial" | "Friendly"))
                    .map(|o| self.greenwash_relations_fall(o, seat, step))
                    .sum();
                let relations = 1.0 / (1.0 + th.greenwash_relations_cost * valued_loss);
                push(
                    vec![Order::Greenwash { amount: step }],
                    Cat::Greenwash,
                    self.base_weight(seat, Cat::Greenwash) * (1.0 + over / fair) * relations,
                    1.0,
                    1.0,
                    1.0,
                    format!("greenwash {step} Influence and {price} Ducats (share {:.2})", self.blame_share(seat)),
                    None,
                );
            }
        }
        // Resettle: while the world's population is falling there are flows to steer, and the
        // calmest state the seat directs is the one that can take them.
        if self.population_growth_rate() < 0.0
            && let Some(sid) = self
                .directed_states(seat)
                .into_iter()
                .filter(|s| self.state(*s).unrest < 3.0)
                .min_by(|a, b| self.state(*a).unrest.partial_cmp(&self.state(*b).unrest).unwrap_or(std::cmp::Ordering::Equal))
        {
            push(
                vec![Order::Resettle { state: sid }],
                Cat::Resettle,
                self.base_weight(seat, Cat::Resettle),
                1.0,
                1.0,
                1.0,
                format!("resettle this turn's refugees in {}", self.tables.state(sid).name),
                None,
            );
        }

        // Ticket #227 (version 0.08.2): offer an Accord to a Faction this seat neither loathes nor
        // is about to lose to. It offers what it would itself accept -- non-aggression alone, the
        // one term that means anything on today's board -- and only where none stands already. An
        // offer costs nothing and a refusal is not an offence, so the guard is on frequency rather
        // than on risk: a table where every seat proposed every turn would bury the player.
        for other in Seat::ALL {
            if other == seat {
                continue;
            }
            // Ticket #495 (version 0.09.9): not while its last offer waits on an answer, nor to a
            // Faction that refused it within the last few turns.
            if self.offer_waits(seat, other) || self.refused_recently(seat, other) {
                continue;
            }
            let mut terms = vec![Term::NonAggression];
            // Ticket #320 (version 0.08.8): and Passage in the same offer, to a Faction this seat
            // is Cordial or better with when it holds a Region next door to one that Faction holds, so the
            // term has somewhere to matter. In the same offer, because one Accord stands per pair
            // and a non-aggression Accord struck first would shut Passage out for good: offered on
            // its own it was struck in no seating of eighty games.
            // Ticket #531 (version 0.1.0.1): and to the pack while a leader runs, so its Armies may cross.
            if self.passage_worth_offering(seat, other) || self.ai_gang_felt(seat) > 0.0 {
                terms.push(Term::Passage);
            }
            // Ticket #325 (version 0.08.8): and Refuel in the same offer, where the other holds a
            // station at a Body this seat has Ships or a Colony at and no station of its own, so
            // the term has somewhere to matter.
            if self.refuel_worth_offering(seat, other) {
                terms.push(Term::Refuel);
            }
            // Ticket #509 (version 0.1.0.0): a research agreement between Friends; and to a Faction it
            // holds an Accord with, the Terms that Accord lacks, or nothing.
            if self.relations_score(seat, other) >= 7 && self.relations_score(other, seat) >= 7 {
                terms.push(Term::ResearchAgreement);
            }
            if let Some(acc) = self.accords.iter().find(|a| a.holds(seat, other)) {
                // The review: nothing to one in its last turn, which lapses before the answer.
                if self.accord_turns_left(acc) <= 1 {
                    continue;
                }
                terms.retain(|t| !acc.terms.contains(t));
            }
            if terms.is_empty() || !self.accord_acceptable(seat, other, &terms) {
                continue;
            }
            push(
                vec![Order::ProposeAccord { to: other, terms }],
                Cat::Accord,
                // Ticket #531: the pack draws together while a leader runs.
                self.base_weight(seat, Cat::Accord) * (1.0 + self.ai_gang_felt(seat)),
                1.0,
                1.0,
                1.0,
                format!("offer the {} an Accord", self.seat_name(other)),
                None,
            );
        }
        // Ticket #496 (version 0.09.9): **a Trade**, at most one a turn: where a Stockpile good is
        // short and another long, ask the short one of the Faction this seat stands best with, giving
        // as much of the long one as it is worth at the Trading window's prices. Never a place.
        // Ticket #572 (version 0.1.0.3): and never the same two goods again to a Faction whose last
        // refusal was of them, the cooldown over or not, until a Trade of other goods is refused.
        {
            let th = &self.tables.ai.thresholds;
            let s = self.seat(seat).stockpile;
            let goods = [(Resource::Materials, s.materials), (Resource::Fuel, s.fuel), (Resource::Energy, s.energy), (Resource::Ducats, s.ducats)];
            let price = |r: Resource| if r == Resource::Ducats { 1.0 } else { self.trade_price(r).unwrap_or(1) as f64 };
            let short = goods.iter().filter(|(r, n)| *r != Resource::Ducats && *n < th.trade_short).min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal)).map(|(r, _)| *r);
            let long = goods.iter().filter(|(r, n)| Some(*r) != short && n * price(*r) > th.trade_long).max_by(|a, b| (a.1 * price(a.0)).partial_cmp(&(b.1 * price(b.0))).unwrap_or(std::cmp::Ordering::Equal)).map(|(r, _)| *r);
            let partner = Seat::ALL
                .into_iter()
                .filter(|o| *o != seat && !self.hostile(seat, *o) && !self.offer_waits(seat, *o) && !self.refused_recently(seat, *o) && self.ai_gang_pressure(seat, *o) == 0.0)
                .filter(|o| !matches!((short, long), (Some(s), Some(l)) if self.trade_refused_before(seat, *o, l, s)))
                .max_by_key(|o| self.relations_score(seat, *o));
            if let (Some(short), Some(long), Some(other)) = (short, long, partner) {
                let ask = th.trade_ask;
                let give = ((ask as f64 * price(short)) / price(long)).ceil() as u32;
                let (give, get) = (crate::orders::TradeGood::Goods(long, give.max(1)), crate::orders::TradeGood::Goods(short, ask));
                if self.holds_good(seat, give).is_ok() {
                    push(
                        vec![Order::ProposeTrade { to: other, give, get }],
                        Cat::Trade,
                        self.base_weight(seat, Cat::Trade),
                        1.0,
                        1.0,
                        1.0,
                        format!("offer the {} a Trade", self.seat_name(other)),
                        None,
                    );
                }
            }
        }
        // --- Ticket #54: Mothball, Restart and Decommission.
        // A mothball answers an Energy shortfall a turn ahead: the highest-upkeep building that
        // produces nothing is the one to shut. A Custodian with Scrubbers standing, a run of nothing
        // and the net within a building's reach of the Sink mothballs its dirtiest Facility instead,
        // since then its own industry is what is keeping the net above the Sink (ticket #576).
        {
            let upkeep_of = |b: &BuildingRef| -> f64 {
                match b {
                    BuildingRef::Facility(sid, i) => self.state(*sid).facilities.get(*i).map(|f| self.tables.facility(f.kind).energy_upkeep as f64).unwrap_or(0.0),
                    BuildingRef::Module(cid, i) => self
                        .colony(*cid)
                        .and_then(|c| c.modules.get(*i))
                        .map(|md| self.module_yield(seat, *cid, md.kind).upkeep)
                        .unwrap_or(0.0),
                }
            };
            let mut standing: Vec<(BuildingRef, &'static str, bool, bool, f64)> = Vec::new();
            for sid in self.directed_states(seat) {
                for (i, f) in self.state(sid).facilities.iter().enumerate() {
                    // Ticket #56: never the Launch Site; it is the only way to lift anything.
                    if f.kind.does_the_job_of(FacilityKind::LaunchSite) {
                        continue;
                    }
                    let produces = self.tables.facility(f.kind).produces.is_some();
                    standing.push((BuildingRef::Facility(sid, i), f.kind.name(), f.mothballed, produces, self.facility_yield(seat, sid, f.kind).emissions));
                }
            }
            for cid in self.directed_colonies(seat) {
                let col = self.colony(cid).unwrap();
                for (i, md) in col.modules.iter().enumerate() {
                    // Ticket #56: never the Shipyard either; a mothballed one starved the
                    // Custodian AI of every Colony Ship while its Scrubbers ate the Energy.
                    // Ticket #359 (version 0.09.1): nor a Habitat, whose people it houses.
                    if md.kind == ModuleKind::Archive || md.kind == ModuleKind::Shipyard || md.kind == ModuleKind::Habitat {
                        continue;
                    }
                    let produces = self.tables.module(md.kind).produces.is_some();
                    standing.push((BuildingRef::Module(cid, i), md.kind.name(), md.mothballed, produces, 0.0));
                }
            }
            // The one to mothball for Energy: standing, working, making nothing, dearest to run.
            let idle_cost: Option<&(BuildingRef, &str, bool, bool, f64)> =
                standing.iter().filter(|(b, _, moth, produces, _)| !*moth && !*produces && upkeep_of(b) > 0.0).max_by(|(x, ..), (y, ..)| upkeep_of(x).total_cmp(&upkeep_of(y)));
            if tight && let Some((b, name, _, _, _)) = idle_cost {
                push(
                    vec![Order::Change { building: *b, what: BuildingChange::Mothball }],
                    Cat::Mothball,
                    self.base_weight(seat, Cat::Mothball) * self.ai_mothball_price(*b),
                    1.0,
                    1.0,
                    m.opportunity,
                    format!("mothball the {} at {} (Energy is a turn from short)", name, self.place_name(b.place())),
                    None,
                );
            }
            // The Custodians' other reason to mothball: a Stabilization run that will not start.
            // Ticket #576 (version 0.1.0.3), at the designer's word: only where shutting the building
            // could start it -- the world's net over the Sink no more than the building's own
            // Emissions and the pace's `within_ppm`. Measured from East Asia, a Scrubber standing
            // from turn 1 had this reason shut the Refinery on turn 2, the Power Plant on turn 3 and
            // the Mine on turn 4, for 1.1 and 0.6 ppm against a world 32 over the Sink: Fuel income
            // nought from turn 4, Energy at nought by turn 12, and the Colony Ship never paid.
            let scrubbers_stand = self.controlled_states(seat).iter().any(|s| self.scrubbers_online(*s) > 0);
            // The review: the last Climate phase's net, as `victory_gap` reads it; `over`, since
            // `gap` is the Victory multiplier in this scope.
            let over = self.climate.last.net();
            let within = self.tables.ai_pace(kind).within_ppm;
            if kind == FactionKind::Custodians && self.seat(seat).stabilization_run == 0 && scrubbers_stand {
                let dirtiest = standing
                    .iter()
                    .filter(|(_, _, moth, _, em)| !*moth && *em > 0.0 && over <= *em + within)
                    .max_by(|a, b| a.4.partial_cmp(&b.4).unwrap_or(std::cmp::Ordering::Equal));
                if let Some((b, name, _, _, em)) = dirtiest {
                    push(
                        vec![Order::Change { building: *b, what: BuildingChange::Mothball }],
                        Cat::Mothball,
                        self.base_weight(seat, Cat::Mothball) * self.ai_mothball_price(*b),
                        gap_for(Cat::Scrubber, None),
                        1.0,
                        1.0,
                        format!("mothball the {} at {} ({:.1} Emissions, and the run is still nothing)", name, self.place_name(b.place()), em),
                        None,
                    );
                }
            }
            // Ticket #82 (version 0.06.0): Production Moved. The Custodian AI idles a Facility once
            // an undoubled Module of its pair off Earth outproduces it, so the trade never loses,
            // and keeps a Facility idle while its doubling stands.
            let pairs = &self.tables.faction(kind).mothball_pairs;
            let doubled = self.doubled_modules(seat);
            // Ticket #426 (version 0.09.5): a Facility is read by the job it does, so a held Reactor
            // is idled, kept idle and counted as the Power Plant the rule treats it as.
            let job = |k: FacilityKind| k.common().unwrap_or(k);
            let mut in_use: Vec<FacilityKind> = Vec::new();
            // Ticket #563 (version 0.1.0.3), at the designer's word: the factor the rule applies, and
            // what a point of Emissions leaving Earth is worth to the run -- the table's figure times
            // the Stabilization gap multiplier, 1 on pace and up to 3 behind.
            let factor = self.tables.faction(kind).production_moved_factor;
            let worth = self.tables.ai.thresholds.production_moved_emission_worth * self.victory_gap(seat).0;
            for (fk, mk) in pairs {
                let idle = self.directed_states(seat).iter().flat_map(|s| self.state(*s).facilities.iter()).filter(|f| job(f.kind) == *fk && f.mothballed).count();
                let paired = doubled.iter().filter(|(cid, i)| self.colony(*cid).and_then(|c| c.modules.get(*i)).map(|m| m.kind == *mk).unwrap_or(false)).count();
                if idle > 0 && paired >= idle {
                    in_use.push(*fk);
                }
                let mut best_undoubled: Option<f64> = None;
                for cid in self.directed_colonies(seat) {
                    let col = self.colony(cid).unwrap();
                    if !self.off_earth(col) {
                        continue;
                    }
                    for (i, md) in col.modules.iter().enumerate() {
                        if md.kind == *mk && !md.mothballed && !doubled.contains(&(cid, i)) {
                            let y = self.module_yield_at(seat, cid, i);
                            let out = y.amount.max(y.research as f64);
                            best_undoubled = Some(best_undoubled.map_or(out, |b| b.max(out)));
                        }
                    }
                }
                let Some(best) = best_undoubled else { continue };
                for sid in self.directed_states(seat) {
                    for (i, f) in self.state(sid).facilities.iter().enumerate() {
                        if job(f.kind) != *fk || f.mothballed {
                            continue;
                        }
                        let y = self.facility_yield(seat, sid, f.kind);
                        let out = y.amount.max(y.research as f64);
                        // The 0.06.0 AI sweep (ticket #94): an even trade is a win for the Custodians,
                        // since the idled Facility's Emissions leave Earth with the output.
                        // Ticket #563 (version 0.1.0.3): the trade weighed whole -- the Module's lift
                        // (its output times the factor less one) plus the Emissions' worth, against
                        // the Facility's output; the dirtiest per output weighed first.
                        let gain = best * (factor - 1.0) + y.emissions * worth;
                        if gain + 1e-9 >= out {
                            push(
                                vec![Order::Change { building: BuildingRef::Facility(sid, i), what: BuildingChange::Mothball }],
                                Cat::Mothball,
                                self.base_weight(seat, Cat::Mothball) * self.ai_mothball_price(BuildingRef::Facility(sid, i)) * (1.0 + y.emissions / out.max(0.1)),
                                1.0,
                                1.0,
                                m.opportunity,
                                format!("mothball the {} in {} (a {} off Earth making {} would make {}; {:.2} Emissions leave Earth, worth {})", f.kind.name(), self.tables.state(sid).name, mk.name(), figure(best), figure(tenth(best * factor)), y.emissions, figure(y.emissions * worth)),
                                None,
                            );
                        }
                    }
                }
            }
            // Ticket #630 (version 0.1.0.6), at the designer's word: a Reactor short of Fissionables --
            // the Stockpile below what every online Reactor burns this Income -- is mothballed, so it
            // stops costing its Ducats for a quarter's Energy; restarted below once the seat can feed it.
            let is_reactor = |b: &BuildingRef| match b {
                BuildingRef::Facility(sid, i) => self.state(*sid).facilities.get(*i).is_some_and(|f| f.kind == FacilityKind::FissionReactor),
                BuildingRef::Module(cid, i) => self.colony(*cid).and_then(|c| c.modules.get(*i)).is_some_and(|m| m.kind == ModuleKind::FissionReactor),
            };
            // From the review: the ones it cannot feed and no more.
            let unfed = self.ai_reactors_to_mothball(seat);
            if unfed > 0 {
                for (b, name, _, _, _) in standing.iter().filter(|(b, _, moth, _, _)| !*moth && is_reactor(b)).take(unfed) {
                    push(
                        vec![Order::Change { building: *b, what: BuildingChange::Mothball }],
                        Cat::Mothball,
                        self.base_weight(seat, Cat::Mothball),
                        1.0,
                        1.0,
                        1.0,
                        format!("mothball the {} at {} (short of Fissionables)", name, self.place_name(b.place())),
                        None,
                    );
                }
            }
            // Restart once Energy is back above two turns of upkeep; otherwise scrap it for half.
            let restart_ok = self.seat(seat).stockpile.energy > 2.0 * self.total_upkeep(seat) as f64;
            let feeds = self.ai_may_restart_reactor(seat);
            for (b, name, mothballed, _, _) in standing.iter().filter(|(b, _, moth, _, _)| *moth && (feeds || !is_reactor(b))) {
                // Ticket #82: not a Facility whose idleness is doubling a Module off Earth.
                if let BuildingRef::Facility(sid, i) = b
                    && self.state(*sid).facilities.get(*i).map(|f| in_use.contains(&job(f.kind))).unwrap_or(false)
                {
                    continue;
                }
                if restart_ok {
                    push(
                        vec![Order::Change { building: *b, what: BuildingChange::Restart }],
                        Cat::Restart,
                        self.base_weight(seat, Cat::Restart),
                        1.0,
                        1.0,
                        1.0,
                        format!("restart the {} at {}", name, self.place_name(b.place())),
                        None,
                    );
                } else if let BuildingRef::Facility(sid, i) = b
                    && *mothballed
                    && self.state(*sid).facilities.get(*i).map(|f| self.takes_slot(f.kind)).unwrap_or(false)
                    && self.free_slots(*sid) == 0
                {
                    // Scrapping is for the slot: a mothballed Facility that cannot be restarted yet
                    // and is holding the state's last slot. A Scrubber takes no slot, so it is never
                    // scrapped, and a Colony has no slot limit, so a Module never is either.
                    push(
                        vec![Order::Change { building: *b, what: BuildingChange::Decommission }],
                        Cat::Decommission,
                        self.base_weight(seat, Cat::Decommission),
                        1.0,
                        1.0,
                        1.0,
                        format!("decommission the mothballed {} in {} for its slot", name, self.tables.state(*sid).name),
                        None,
                    );
                }
            }
        }

        // --- Ticket #559 (version 0.1.0.3): Replace a Power Plant with a Wind Farm, where the seat's
        // Blame share stands above the fair quarter and its Energy is not short -- not tight now, and
        // not tight once the Power Plant's figure is traded for the Wind Farm's. Weighed as a Greenwash
        // is, by how far above the quarter the share stands, on the Producer's base; one Region a
        // turn at most, the first standing Power Plant in each. A Reactor is never offered:
        // `check_change` refuses it, and the Reactor's clause is worth more than its smoke.
        // The review of #559: one Replace in flight at a time, anywhere, since each is guarded
        // against its own 3 Energy alone and three ordered at once would land 9 short.
        let replace_in_flight = self.states.iter().any(|s| s.queue.iter().any(|b| b.seat == seat && matches!(b.item, BuildItem::Replace(_))));
        if self.has_tech(TechId::CleanPower) && !tight && !replace_in_flight {
            let fair = self.tables.influence.blame.fair_share;
            let over = self.blame_share(seat) - fair;
            let lost = energy_of(FacilityKind::PowerPlant) - energy_of(FacilityKind::WindFarm);
            if over > 0.0 && !self.energy_tight_losing(seat, lost) {
                // The first standing Power Plant of the first Region that has one: one candidate.
                if let Some((sid, i)) = self.directed_states(seat).into_iter().find_map(|sid| self.state(sid).facilities.iter().position(|f| f.kind == FacilityKind::PowerPlant && !f.mothballed && !f.changing()).map(|i| (sid, i))) {
                    {
                        push(
                            vec![Order::Change { building: BuildingRef::Facility(sid, i), what: BuildingChange::Replace }],
                            Cat::Producer,
                            self.base_weight(seat, Cat::Producer) * (1.0 + over / fair),
                            1.0,
                            1.0,
                            1.0,
                            format!("replace the Power Plant in {} with a Wind Farm (share {:.2})", self.tables.state(sid).name, self.blame_share(seat)),
                            None,
                        );
                    }
                }
            }
        }

        // --- Ticket #73: Emigrants. Colonists are built now, so before a Colony Ship can be loaded
        // or Antarctica settled a batch must muster in a state the seat directs: those with a
        // working Launch Site, or, with the ice open, any (several since ticket #427, below). It musters while fewer
        // wait than two Ship loads (and one more while the ice is open), and never for nothing.
        let presence_needed = self.tables.victory.off_world_presence.saturating_sub(self.victory_colonists(seat));
        // Ticket #237 (version 0.08.3): the Exodus Call. Sounded where the Arkwrights hold their
        // MOST populous Region, since a Call is once per Region ever and a doubled muster is worth
        // most where there are most people to take -- and since the measured problem is that their
        // home state runs from 20 units to 1 over a game, spending the Call on a small Region
        // wastes it. Not sounded at all while they already have more waiting than they can lift.
        if kind == FactionKind::Arkwrights && self.seat(seat).stockpile.ducats >= self.tables.ducats.per_exodus_call as f64 {
            let waiting_now: u32 = self.directed_states(seat).iter().map(|s| self.state(*s).emigrants).sum();
            let best = self
                .directed_states(seat)
                .into_iter()
                .filter(|s| !self.state(*s).exodus_call_used && self.state(*s).control.controller() == Some(seat))
                .max_by(|a, b| self.state(*a).population.partial_cmp(&self.state(*b).population).unwrap_or(std::cmp::Ordering::Equal));
            // Gated on almost nothing on purpose. A first attempt also required fewer waiting than
            // a Ship holds and a population above twice a doubled muster, and over a whole
            // headless game the Call fired ZERO times: the Arkwrights spend their Ducats on
            // Influence and their one Region is already draining, so every extra condition closed
            // the door. `check_order` refuses what they cannot afford and the weighting decides
            // whether it is worth doing, which is where those judgements belong.
            let _ = waiting_now;
            if let Some(sid) = best {
                push(vec![Order::ExodusCall { state: sid }], Cat::LoadUnload, self.base_weight(seat, Cat::LoadUnload), gap_for(Cat::LoadUnload, None), 1.0, 1.0, format!("sound an Exodus Call in {}", self.tables.state(sid).name), None);
            }
        }
        {
            let per = self.emigrants_per_turn(seat);
            let capacity = self.colony_ship_capacity(seat);
            let waiting: u32 = self.directed_states(seat).iter().map(|s| self.state(*s).emigrants).sum();
            let has_ship_or_yard = self.ships.iter().any(|s| s.seat == seat && s.kind == UnitKind::ColonyShip)
                || self.colonies.iter().any(|c| c.control.director() == Some(seat) && c.modules.iter().any(|m| m.kind == ModuleKind::Shipyard));
            // Ticket #196 (version 0.08.0): room to put people opens the gate too. It used to want a
            // Colony Ship or a Shipyard, or the ice open -- and measured over 320 seat-games, NO seat
            // ever held a ship or a yard while the ice was still shut, because a bare station has zero
            // Module slots (`base = 0`, one per Colonist) and so cannot raise the Shipyard that would
            // let it muster the Colonists that earn the slots. Ticket #164's Core Module ended that
            // deadlock in the rules; the gate was never updated, so Antarctica opening was the only
            // door into the Colonist economy for everybody, and a Custodian holding the Temperature
            // down locked itself out of its own Victory Condition.
            let room_off_earth: u32 = self
                .colonies
                .iter()
                .filter(|c| c.control.director() == Some(seat))
                .map(|c| self.habitat_room(c).saturating_sub(c.colonists))
                .sum();
            // Ticket #489 (version 0.09.9): a station over Earth is built with four Pioneers now, so a
            // seat with none to stand on recruits the four. Without this a seat with no station had
            // no room anywhere, recruited nobody, and so could never build the station that is the
            // room: measured, the Arkwrights starting with two waiting won 1 game in 80.
            let station_over_earth = self.colonies.iter().any(|c| c.in_orbit && c.body == BodyId::Earth && c.control.director() == Some(seat));
            let to_build = if !station_over_earth && !self.buildable_orbital_slots(BodyId::Earth).is_empty() && !self.station_sources(seat, BodyId::Earth).is_empty() { self.tables.emigrants.found_with } else { 0 };
            let want = if has_ship_or_yard { capacity * 2 } else { 0 } + if self.antarctica_open { capacity } else { 0 } + room_off_earth + to_build;
            if per > 0 && waiting < want {
                // Ticket #427 (version 0.09.5): the cap is per state now, so the seat recruits from as
                // many states as it takes to fill the plan and NO further -- the designer's "only to
                // the extent that have plans to use them". The states with a working Launch Site
                // first (their Pioneers can lift), then, with the ice open, the rest; the most
                // populous first within each.
                let by_population = |a: &StateId, b: &StateId| self.state(*b).population.partial_cmp(&self.state(*a).population).unwrap_or(std::cmp::Ordering::Equal);
                let has_site = |s: &StateId| self.state(*s).facilities.iter().any(|f| f.kind.does_the_job_of(FacilityKind::LaunchSite) && f.working());
                let mut targets: Vec<StateId> = self.directed_states(seat).into_iter().filter(|s| has_site(s)).collect();
                targets.sort_by(by_population);
                if self.antarctica_open {
                    let mut rest: Vec<StateId> = self.directed_states(seat).into_iter().filter(|s| !has_site(s)).collect();
                    rest.sort_by(by_population);
                    targets.extend(rest);
                }
                let mut short = want - waiting;
                for st in targets {
                    if short == 0 {
                        break;
                    }
                    // Ticket #196: as many as the state can pay for, not all or nothing. A Coach
                    // Class batch costs the Arkwrights twice the people.
                    let n = self.emigrants_affordable(seat, st).min(short);
                    if n == 0 {
                        continue;
                    }
                    short -= n;
                    let opp = if presence_needed > 0 && waiting == 0 { m.opportunity } else { 1.0 };
                    push(vec![Order::BuildEmigrants { state: st, n }], Cat::LoadUnload, self.base_weight(seat, Cat::LoadUnload), gap_for(Cat::LoadUnload, None), 1.0, opp, format!("recruit {n} Pioneers in {}", self.tables.state(st).name), None);
                }
            }
            // Ticket #141 (version 0.07.3): waiting Emigrants lift straight to the seat's own station
            // over Earth while it has room, from a state with a working Launch Site. Presence, not a
            // foothold. Ticket #449 (version 0.09.6): at half weight, and no boost, while the Faction's
            // Bodies part is short -- a station over Earth is off Earth but settles no Body.
            for sid in self.directed_states(seat) {
                let n = self.state(sid).emigrants;
                if n == 0 || !self.state(sid).facilities.iter().any(|f| f.kind.does_the_job_of(FacilityKind::LaunchSite) && f.working()) {
                    continue;
                }
                let station = self
                    .colonies
                    .iter()
                    .filter(|c| c.body == BodyId::Earth && c.in_orbit && c.control.director() == Some(seat) && self.habitat_room(c) > c.colonists && !self.slot_blockaded_against(seat, BodyId::Earth, c.slot))
                    .max_by_key(|c| self.habitat_room(c) - c.colonists);
                if let Some(c) = station {
                    let k = n.min(self.habitat_room(c) - c.colonists);
                    let opp = if presence_needed > 0 { m.opportunity } else { 1.0 };
                    // Ticket #449 (version 0.09.6): while a Bodies part is short, a lift over Earth is
                    // a foothold, as a Ship's disembark there is (ticket #94): half weight, no gap.
                    let sec = self.tables.faction(kind).victory_second;
                    let bodies_short = sec.kind == VictorySecondKind::ColoniesOnBodies && self.bodies_settled(seat, sec.colonists_each) < sec.bodies;
                    let (w, g, opp) = if bodies_short { (self.base_weight(seat, Cat::LoadUnload) * 0.5, 1.0, 1.0) } else { (self.base_weight(seat, Cat::LoadUnload), gap_for(Cat::LoadUnload, None), opp) };
                    push(vec![Order::LiftToStation { state: sid, n: k, colony: c.id }], Cat::LoadUnload, w, g, 1.0, opp, format!("lift {} Pioneers from {} to {}", k, self.tables.state(sid).name, self.place_name(Place::Colony(c.id))), None);
                }
            }
            // With the ice open, waiting Emigrants go to Antarctica by sea: a free slot first, else
            // a Colony of the seat's with room. A foothold, not Presence: half weight and no gap,
            // as a Ship's unload there.
            // Ticket #361: the ferrying Archivists send nobody to the ice, which holds no Archive.
            if self.antarctica_open && !ferrying {
                for sid in self.directed_states(seat) {
                    let n = self.state(sid).emigrants;
                    if n == 0 {
                        continue;
                    }
                    // Ticket #489 (version 0.09.9): a free slot wants four.
                    if let Some(slot) = self.best_slot_for(seat, BodyId::Earth, behind).filter(|_| n >= self.tables.emigrants.found_with) {
                        push(vec![Order::SendToAntarctica { state: sid, n, into: UnloadTarget::Slot(BodyId::Earth, slot) }], Cat::FoundColony, self.base_weight(seat, Cat::FoundColony) * 0.5, 1.0, 1.0, 1.0, format!("send {} Pioneers from {} to {} by sea", n, self.tables.state(sid).name, self.tables.body(BodyId::Earth).slots[slot as usize].name), None);
                    } else if let Some(c) = self.colonies.iter().find(|c| c.body == BodyId::Earth && !c.in_orbit && c.control.director() == Some(seat) && self.habitat_room(c) > c.colonists) {
                        let k = n.min(self.habitat_room(c) - c.colonists);
                        push(vec![Order::SendToAntarctica { state: sid, n: k, into: UnloadTarget::Colony(c.id) }], Cat::LoadUnload, self.base_weight(seat, Cat::LoadUnload) * 0.5, 1.0, 1.0, 1.0, format!("send {} Pioneers from {} to {} by sea", k, self.tables.state(sid).name, self.place_name(Place::Colony(c.id))), None);
                    }
                }
            }
        }

        // --- Ships: load, unload, found, transit
        let ships: Vec<Ship> = self.ships.iter().filter(|s| s.seat == seat && matches!(s.at, ShipAt::Body(_)) && !s.arrived_this_turn).cloned().collect();
        for s in &ships {
            let ShipAt::Body(body) = s.at else { continue };
            let card = self.tables.unit(s.kind);
            // Ticket #586 (version 0.1.0.4), from the review: its name in the scored list, which the
            // tool's `--log` prints, never its id.
            let ship_name = self.ship_name(s);
            // Ticket #87: refuel at a station of its own whenever the tank is short and the
            // Stockpile has Fuel; a leg the tank cannot pay is refused at the check, so the AI
            // never flies on an empty tank.
            // Ticket #325 (version 0.08.8): or at a partner's station under a Refuel Accord.
            // Ticket #335 (version 0.09.0): a station fuels only a Ship in its own orbit, so the
            // Refuel is offered where the Ship already sits at one; the orbit change that reaches
            // it is the candidate below.
            let orbit = self.ship_orbit(s);
            // Ticket #363 (version 0.09.1): an armed Missile Carrier with a target in reach of the
            // orbit it sits in fires rather than tops up: a Ship takes one order a turn, and a
            // carrier that went to refuel on the turn it fired was the whole reason no computer
            // Launch ever landed.
            let ready_to_fire = s.kind == UnitKind::MissileCarrier
                && s.warhead
                && self.nuke_targets(seat).iter().any(|t| match t {
                    Place::State(_) => body == BodyId::Earth && orbit.is_low(),
                    Place::Colony(c) => self.colony(*c).is_some_and(|c| c.body == body && self.colony_orbit(c) == orbit),
                });
            // Ticket #421 (version 0.09.4): short of the Fuel to fill it, the seat buys the rest, in
            // the same candidate. The plain Refuel from what is held stays on offer beside it, so a
            // turn whose Ducats are held for something else still fills what it can.
            // Ticket #575 (version 0.1.0.3): the tank is filled from what stands above the power
            // reserve, and the top-up buys the reserve back too.
            let fuel_reserve = self.fuel_reserve(seat);
            let top_up = self.ai_fuel_top_up(seat, self.tank_of(seat, s.kind) - s.fuel + fuel_reserve);
            // Ticket #533 (version 0.1.0.1): a refuel never uses the Ship's turn, so a ready Missile
            // Carrier fills its tank beside its Launch; only the orbit change below still waits on it.
            // Ticket #629 (version 0.1.0.6): a nuclear hull short in its second tank refuels too.
            let wants_fission = s.nuclear && s.fissionables < self.fission_tank_of(seat, s.kind) && self.seat(seat).stockpile.fissionables > 0.0;
            if (s.fuel < self.tank_of(seat, s.kind) || wants_fission) && self.refuelling_station(seat, body, orbit) {
                let note = format!("refuel {} at {} ({} of {} in the tank)", ship_name, self.orbit_name(body, orbit), figure(s.fuel), figure(self.tank_of(seat, s.kind)));
                if let Some(buy) = top_up {
                    push(vec![buy, Order::Refuel { ship: s.id }], Cat::Transit, self.base_weight(seat, Cat::Transit), gap_for(Cat::Transit, None), 1.0, 1.0, format!("{note}, buying the rest"), None);
                }
                // The review of #575: a Refuel takes what the tank wants down to nought, so the plain
                // one is offered only when the whole want stands above the reserve.
                if self.seat(seat).stockpile.fuel - fuel_reserve >= self.tank_of(seat, s.kind) - s.fuel {
                    push(vec![Order::Refuel { ship: s.id }], Cat::Transit, self.base_weight(seat, Cat::Transit), gap_for(Cat::Transit, None), 1.0, 1.0, note, None);
                }
            }
            // Ticket #335 (version 0.09.0): **it changes orbit rather than flying away when what it
            // wants is at the same Body** -- a station of its own to fill the tank at, a Colony with
            // room for the people aboard, or low orbit, which is what touches the ground: founding
            // a Colony, taking a lift from a Launch Site and landing an Army all want it. Offered
            // at the transit's weight, since it is the same sort of move and the cheaper one.
            // Each want carries the weight of the order it is a step toward, so an orbit change
            // never outranks the thing it enables: the 0.06.0 sweep found that a loaded Colony Ship
            // offered its own station over Earth at full weight parked every load there and left
            // Mars unfounded, and a move toward that station must not reopen it.
            if s.fuel >= self.tables.orbit_change_fuel as f64 {
                let mut wants: Vec<(Orbit, String, Cat, f64)> = Vec::new();
                // Ticket #363 (version 0.09.1): a warship HOLDING THE LANE -- the low-orbit garrison of a
                // Body whose ground the seat wants -- does not leave it to top up a tank that can
                // still pay for a Battle. Traced: every warship holding Earth's low orbit for a
                // ready Missile Carrier changed orbit to its own station's ring to refuel on the
                // very turn the carrier fired, and the Launch failed with the orbit given up.
                let on_the_lane = s.kind.is_warship() && orbit.is_low() && self.ai_wants_the_ground(seat, body) && self.ai_low_orbit_garrison(seat, body).contains(&s.id);
                let can_fight = s.fuel >= self.tables.melee.battle_fuel as f64;
                // Ticket #629 (version 0.1.0.6), from the review: a nuclear hull short in its second
                // tank goes to a depot too.
                if ((s.fuel < self.tank_of(seat, s.kind) && self.seat(seat).stockpile.fuel > 0.0) || wants_fission) && !self.refuelling_station(seat, body, orbit) && !(on_the_lane && can_fight) && !ready_to_fire {
                    // Ticket #396 (version 0.09.3): a Refinery Colony's depot is its low orbit.
                    for c in self.colonies.iter().filter(|c| c.body == body && self.fuels_for(c, seat)) {
                        wants.push((self.colony_orbit(c), format!("to refuel at {}", self.place_name(Place::Colony(c.id))), Cat::Transit, self.base_weight(seat, Cat::Transit)));
                    }
                }
                if s.colonists > 0 {
                    for c in self.colonies.iter().filter(|c| c.body == body && c.control.director() == Some(seat) && self.habitat_room(c) > c.colonists) {
                        let weight = if body == BodyId::Earth { self.base_weight(seat, Cat::LoadUnload) * 0.5 } else { self.base_weight(seat, Cat::LoadUnload) };
                        wants.push((self.colony_orbit(c), format!("to disembark into {}", self.place_name(Place::Colony(c.id))), Cat::LoadUnload, weight));
                    }
                }
                // Ticket #480 (version 0.09.8): with every buildable station slot here taken, a
                // loaded Colony Ship goes out to a free far orbit to found there, at the founding's
                // own half weight. It pays the far figure, so the tank must hold it.
                if s.kind == UnitKind::ColonyShip && s.colonists >= self.tables.emigrants.found_with && !self.far_orbit(body, orbit) && self.buildable_orbital_slots(body).is_empty()
                    && let Some(n) = self.free_orbital_slots(body).into_iter().find(|n| self.far_slot(body, *n))
                    && s.fuel >= self.orbit_change_cost(seat, body, orbit, Orbit::Slot(n))
                {
                    wants.push((Orbit::Slot(n), format!("to found {}", self.station_name(body, n)), Cat::FoundColony, self.base_weight(seat, Cat::FoundColony) * 0.5));
                }
                // Ticket #357 (version 0.09.1): an empty Colony Ship at Earth no longer comes down
                // to low orbit to be loaded, since a Launch Site now lifts into any orbit of Earth.
                let wants_the_ground = (s.colonists >= self.tables.emigrants.found_with && !self.free_slots_on(body).is_empty()) || s.army.is_some();
                if wants_the_ground {
                    wants.push((Orbit::Low, "to reach the ground".to_string(), Cat::LoadUnload, self.base_weight(seat, Cat::LoadUnload)));
                }
                // Ticket #335 (version 0.09.0): the blockade appetite moves a warship ALREADY at
                // the Body into the ring it means to shut, since a Blockade shuts the orbit it is
                // given in and a stack sitting in low orbit shuts no station at all. Where the
                // ground is what the seat wants and its garrison of low orbit is short, the same
                // appetite runs the other way and brings the hull DOWN, since low orbit is the one
                // orbit Orbital Control is held in. At the Transit weight, as the refuel want
                // above is: it is a transit that never leaves the Body, and the stance it is a
                // step toward is the STACK'S, carried on its own key, so it can displace nothing.
                // A warship's leg away from the Body is offered at six-tenths of that weight
                // (nine for the Prospectors), so the hull that has business here stays for it.
                if s.kind.is_warship() && !s.escaped {
                    let garrison = self.ai_low_orbit_garrison(seat, body);
                    let holds_the_lane = self.ai_wants_the_ground(seat, body) && garrison.contains(&s.id);
                    // Ticket #355 (version 0.09.1): a station of its own shut by a rival's Blockade
                    // is answered -- the warship goes to that ring, where the Attack stance reads
                    // the blockader as the enemy present. Without it a Blockade was never
                    // contested: 75 Blockades over eighty games and not one orbital Battle.
                    let blockaded = self.colonies.iter().find(|c| c.in_orbit && c.body == body && c.control.director() == Some(seat) && self.slot_blockaded_against(seat, body, c.slot) && orbit != Orbit::Slot(c.slot));
                    if let Some(c) = blockaded {
                        wants.push((Orbit::Slot(c.slot), format!("to break the Blockade of {}", self.place_name(Place::Colony(c.id))), Cat::Transit, self.base_weight(seat, Cat::Transit) * m.threat));
                    } else if holds_the_lane {
                        // It is the garrison: it is where it should be, and wants nothing.
                    } else if self.ai_wants_the_ground(seat, body) && (garrison.len() as u32) < self.tables.ai.thresholds.low_orbit_warships {
                        wants.push((Orbit::Low, "to hold Orbital Control of low orbit".to_string(), Cat::Transit, self.base_weight(seat, Cat::Transit)));
                    } else if let Some(slot) = self.ai_blockade_slot(seat, body, s.kind) {
                        let target = self.station_at(body, slot).map(|c| self.place_name(Place::Colony(c.id))).unwrap_or_else(|| self.station_name(body, slot));
                        wants.push((Orbit::Slot(slot), format!("to blockade {target}"), Cat::Transit, self.base_weight(seat, Cat::Transit)));
                    }
                }
                let mut offered: Vec<Orbit> = Vec::new();
                for (want, why, cat, weight) in wants {
                    if want == orbit || offered.contains(&want) || !self.orbit_exists(body, want) {
                        continue;
                    }
                    offered.push(want);
                    let what = format!("move {} to {} {}", ship_name, self.orbit_name(body, want), why);
                    push(vec![Order::ChangeOrbit { ship: s.id, slot: want.slot() }], cat, weight, 1.0, 1.0, 1.0, what, None);
                }
            }
            if s.kind == UnitKind::ColonyShip {
                // Ticket #86: behind on Off-world Presence, the AI lifts the crowded load at Earth;
                // otherwise the safe one.
                let capacity = if body == BodyId::Earth && presence_needed > 0 { self.colony_ship_crowded_capacity(seat) } else { self.colony_ship_capacity(seat) };
                // Ticket #357 (version 0.09.1): a lift from a Launch Site reaches any orbit of
                // Earth, where ticket #335 held it to low orbit.
                if body == BodyId::Earth && s.colonists < capacity {
                    // Load from the directed state with the most Emigrants waiting (ticket #73).
                    // Ticket #46: only a state with a working Launch Site lifts them. Ticket #443
                    // (version 0.09.6): to a station's ring; in LOW orbit any Region of the seat's
                    // will do. And from EACH such Region in turn, most waiting first, until the Ship
                    // is full -- one Load a Region, all in the one bundle.
                    let in_low = s.slot.is_none();
                    let mut from: Vec<StateId> = self
                        .directed_states(seat)
                        .into_iter()
                        .filter(|s| self.state(*s).emigrants > 0 && (in_low || self.state(*s).facilities.iter().any(|f| f.kind.does_the_job_of(FacilityKind::LaunchSite) && f.working())))
                        .collect();
                    from.sort_by_key(|s| std::cmp::Reverse(self.state(*s).emigrants));
                    let mut room = capacity - s.colonists;
                    let mut loads = Vec::new();
                    for st in from {
                        if room == 0 {
                            break;
                        }
                        let n = room.min(self.state(st).emigrants);
                        loads.push(Order::Load { ship: s.id, colonists: n, from: LoadSource::State(st), army: None });
                        room -= n;
                    }
                    let n = capacity - s.colonists - room;
                    // Ticket #533 (version 0.1.0.1), at the designer's word: a lift never uses the
                    // Ship's turn and boards before it moves, so a lift that makes the four a founding
                    // wants is offered with the crossing too -- lift and leave in one turn -- at the
                    // crossing's weight, beside the lift alone.
                    if n > 0 && s.colonists + n >= self.tables.emigrants.found_with {
                        let d = if ferrying { archive_body.unwrap_or_else(|| self.best_body_for(seat, behind, s.id)) } else { self.best_body_for(seat, behind, s.id) };
                        if d != body {
                            let window = self.window_preference(seat, body, d);
                            let gather = if gathering && capital_has_room && Some(d) == capital_body { capital_chase } else { 1.0 };
                            let mut go = loads.clone();
                            go.push(Order::Transit { ship: s.id, to: d, slot: self.ai_destination_orbit(seat, s, d) });
                            push(go, Cat::Transit, self.base_weight(seat, Cat::Transit) * window * gather, gap_for(Cat::Transit, None), 1.0, 1.0, format!("load {} Colonists onto {} and send it to {}", n, ship_name, self.tables.body(d).name), None);
                        }
                    }
                    if n > 0 {
                        let opp = if presence_needed <= n { m.opportunity } else { 1.0 };
                        push(loads, Cat::LoadUnload, self.base_weight(seat, Cat::LoadUnload), gap_for(Cat::LoadUnload, None), 1.0, opp, format!("load {} Colonists onto {}", n, ship_name), None);
                    }
                }
                // Ticket #335 (version 0.09.0): a Colony is founded from low orbit, which is what
                // touches the ground; from a station's ring the order is refused.
                // Ticket #489 (version 0.09.9): every founding below wants four aboard.
                if s.colonists >= self.tables.emigrants.found_with && body != BodyId::Earth && orbit.is_low() {
                    let free = self.free_slots_on(body);
                    // Ticket #57: every slot has its own four yields, so the AI picks the free slot
                    // whose figures best serve the part it is furthest behind on, not the first one.
                    if let Some(slot) = self.best_slot_for(seat, body, behind) {
                        let opp = if free.len() == 1 || presence_needed <= s.colonists { m.opportunity } else { 1.0 };
                        // Ticket #345 (version 0.09.1): the landing that TAKES a world is worth
                        // more than the landing that joins one, because the first Faction down is
                        // paid a windfall and keeps a standing +1 while it holds the place. The
                        // appetite is lifted while the Body's first is unclaimed and falls back to
                        // the plain weight the moment somebody has it.
                        let unclaimed = self.first_at(body).is_none() && self.tables.body(body).first_windfall > 0;
                        // Ticket #409 (version 0.09.4): it asks for what lands, the most the slot
                        // takes, as every other Unload does; the rest stay aboard as before.
                        let lift = if unclaimed { self.tables.ai.thresholds.first_found_weight } else { 1.0 };
                        // Ticket #514 (version 0.1.0.0): on the Capital candidate's Body, while gathering,
                        // a fresh Outpost is worth a chase less than landing in the candidate. Measured
                        // without this: the people arrived and founded four-person Outposts beside it.
                        let lift = if gathering && Some(body) == capital_body { lift / capital_chase } else { lift };
                        push(vec![Order::Unload { ship: s.id, colonists: self.unload_most(s.id, UnloadTarget::Slot(body, slot)), army: false, into: UnloadTarget::Slot(body, slot) }], Cat::FoundColony, self.base_weight(seat, Cat::FoundColony) * lift * self.ai_founding_pull(seat), gap_for(Cat::FoundColony, None), 1.0, opp, format!("found a Colony at {} on {}", self.tables.body(body).slots[slot as usize].name, self.tables.body(body).name), None);
                    }
                }
                // Ticket #442 (version 0.09.6): a station FOUNDED from the ring a loaded Colony Ship
                // sits in, off Earth, where the seat has no station at that Body -- the one way to a
                // station at Venus, which has no ground and no low orbit. As founding a Colony.
                if s.colonists >= self.tables.emigrants.found_with && body != BodyId::Earth
                    && let Some(n) = orbit.slot()
                    && self.free_orbital_slots(body).contains(&n)
                    && !self.colonies.iter().any(|c| c.in_orbit && c.body == body && c.control.director() == Some(seat))
                {
                    let into = UnloadTarget::Ring(body, n);
                    push(vec![Order::Unload { ship: s.id, colonists: self.unload_most(s.id, into), army: false, into }], Cat::FoundColony, self.base_weight(seat, Cat::FoundColony) * self.ai_founding_pull(seat), gap_for(Cat::FoundColony, None), 1.0, 1.0, format!("found {} over {}", self.station_name(body, n), self.tables.body(body).name), None);
                }
                // Ticket #480 (version 0.09.8): **a far orbit**, Earth's L4 or L5, founded from the
                // loaded Colony Ship sitting in it, once every station slot that can be built in is
                // taken. A foothold over Earth, as Antarctica is: half weight and no gap.
                if s.colonists >= self.tables.emigrants.found_with && s.kind == UnitKind::ColonyShip
                    && let Some(n) = orbit.slot()
                    && self.far_slot(body, n)
                    && self.free_orbital_slots(body).contains(&n)
                    && self.buildable_orbital_slots(body).is_empty()
                {
                    let into = UnloadTarget::Ring(body, n);
                    push(vec![Order::Unload { ship: s.id, colonists: self.unload_most(s.id, into), army: false, into }], Cat::FoundColony, self.base_weight(seat, Cat::FoundColony) * 0.5, 1.0, 1.0, 1.0, format!("found {}", self.station_name(body, n)), None);
                }
                // Ticket #44: Antarctica, Earth's slots. A foothold, not Presence: half weight and no gap,
                // so it is taken when the Ship cannot go anywhere better.
                if s.colonists >= self.tables.emigrants.found_with && body == BodyId::Earth && orbit.is_low() && !ferrying {
                    // Ticket #56: Antarctica is shut until the ice opens; a loaded Ship goes elsewhere.
                    if let Some(slot) = self.best_slot_for(seat, BodyId::Earth, behind).filter(|_| self.antarctica_open) {
                        push(vec![Order::Unload { ship: s.id, colonists: self.unload_most(s.id, UnloadTarget::Slot(body, slot)), army: false, into: UnloadTarget::Slot(body, slot) }], Cat::FoundColony, self.base_weight(seat, Cat::FoundColony) * 0.5, 1.0, 1.0, 1.0, format!("found a Colony at {}", self.tables.body(BodyId::Earth).slots[slot as usize].name), None);
                    }
                }
                // The 0.06.0 AI sweep (ticket #94): a loaded Colony Ship disembarks into a Colony or
                // station of the seat's own with room at the Body it stands at, wherever that is.
                // Until this sweep the branch sat inside the at-Earth case and then asked for a Body
                // that was not Earth, so it could never run: a second load never joined a Colony and
                // nothing ever lived on a station at Venus. Antarctica's ground is still no place to
                // park them (a foothold, founded above). A station over Earth is off Earth since
                // ticket #81, so its Habitats count for Presence; offered at full weight the AI
                // parked every load there and Mars went unfounded (3 of 20 seeds, from 20), so over
                // Earth the landing is a foothold like Antarctica's: half weight, no gap, taken when
                // the Ship cannot go anywhere better.
                if s.colonists > 0 {
                    // Ticket #335 (version 0.09.0): and only into a place this Ship's orbit touches.
                    // Ticket #361: nor into a station over Earth while the Archivists ferry.
                    for c in self.colonies.iter().filter(|c| c.body == body && c.control.director() == Some(seat) && (c.in_orbit || c.body != BodyId::Earth) && self.ship_may_touch(s, c) && !(ferrying && c.body == BodyId::Earth)) {
                        let room = self.habitat_room(c).saturating_sub(c.colonists);
                        if room > 0 {
                            let n = room.min(s.colonists);
                            let over_earth = body == BodyId::Earth;
                            let opp = if presence_needed <= n && !over_earth { m.opportunity } else { 1.0 };
                            let (weight, gap) = if over_earth { (self.base_weight(seat, Cat::LoadUnload) * 0.5, 1.0) } else { (self.base_weight(seat, Cat::LoadUnload), gap_for(Cat::LoadUnload, None)) };
                            // Ticket #514 (version 0.1.0.0): a landing at the Capital candidate, chased or not.
                            let weight = if capital_candidate == Some(c.id) { weight * capital_chase } else { weight };
                            push(vec![Order::Unload { ship: s.id, colonists: n, army: false, into: UnloadTarget::Colony(c.id) }], Cat::LoadUnload, weight, gap, 1.0, opp, format!("disembark {} Colonists into {}", n, self.place_name(Place::Colony(c.id))), None);
                        }
                    }
                }
                if s.colonists > 0 && body == BodyId::Earth {
                    // Ticket #361: a ferrying Archivist crosses to its Archive's Body first.
                    let dest = if ferrying { archive_body.unwrap_or_else(|| self.best_body_for(seat, behind, s.id)) } else { self.best_body_for(seat, behind, s.id) };
                    let own_room = self.colonies.iter().any(|c| c.control.director() == Some(seat) && self.habitat_room(c) > c.colonists);
                    // Ticket #489 (version 0.09.9): with fewer than four aboard it founds nowhere, so
                    // it sails only to a place of its own with room; with none it stays to load.
                    let mut dests = if s.colonists >= self.tables.emigrants.found_with { vec![dest] } else { Vec::new() };
                    if own_room {
                        // The 0.06.0 AI sweep (ticket #94): not the Body the Ship is at.
                        for c in &self.colonies {
                            if c.control.director() == Some(seat) && c.body != body && !dests.contains(&c.body) {
                                dests.push(c.body);
                            }
                        }
                    }
                    for d in dests {
                        // Ticket #57: a crossing into the Mars system prefers the launch window. Off
                        // it the flight is longer and dearer, so the candidate is worth less --
                        // unless the seat is behind on its pace, where the gap multiplier says go.
                        let window = self.window_preference(seat, body, d);
                        // Ticket #514 (version 0.1.0.0): the crossing to the Capital candidate's Body, while gathering.
                        let gather = if gathering && capital_has_room && Some(d) == capital_body { capital_chase } else { 1.0 };
                        // Ticket #596 (version 0.1.0.4), at the designer's word: the seat's runner sent
                        // to Mars at the race weight, as the race's builds are. Measured before: at
                        // 4.9 and falling off the window the send lost every turn to the Ship's own
                        // 6.0 moves between low orbit and the station, and it never left.
                        let race = if d == BodyId::Mars && self.mars_race_open() && self.mars_runner(seat, s.id) { th.mars_race_weight.max(1.0) } else { 1.0 };
                        push(vec![Order::Transit { ship: s.id, to: d, slot: self.ai_destination_orbit(seat, s, d) }], Cat::Transit, self.base_weight(seat, Cat::Transit) * window * gather * race, gap_for(Cat::Transit, None), 1.0, 1.0, format!("send {} to {}", ship_name, self.tables.body(d).name), None);
                    }
                }
                if s.colonists == 0 && body != BodyId::Earth {
                    push(vec![Order::Transit { ship: s.id, to: BodyId::Earth, slot: self.ai_destination_orbit(seat, s, BodyId::Earth) }], Cat::Transit, self.base_weight(seat, Cat::Transit) * 0.8, gap_for(Cat::Transit, None), 1.0, 1.0, format!("send {} back to Earth", ship_name), None);
                }
            }
            // Ticket #43: an empty Carrier away from Earth goes home for an Army.
            if s.kind == UnitKind::Carrier && s.army.is_none() && body != BodyId::Earth {
                push(vec![Order::Transit { ship: s.id, to: BodyId::Earth, slot: self.ai_destination_orbit(seat, s, BodyId::Earth) }], Cat::Transit, self.base_weight(seat, Cat::Transit) * 0.8, 1.0, 1.0, 1.0, format!("send {} back to Earth", ship_name), None);
            }
            // Load an Army aboard a Carrier at Earth for an attack on a rival Colony (ticket #43).
            // Ticket #319 (version 0.08.8): any seat that wants a Carrier loads one. Ticket #532
            // (version 0.1.0.1), at the designer's word: out of the warship block below, which an
            // empty Carrier -- no warship, no Army aboard -- never entered, so no computer seat had
            // loaded one since at least #319.
            if card.carries_army && s.army.is_none() && body == BodyId::Earth && (kind == FactionKind::Prospectors || self.carrier_target_exists(seat)) {
                // Ticket #46: the Army lifts from its own state. Ticket #532: by the rule's own
                // check, so a state with no Launch Site lifts into low orbit.
                let army = self.armies.iter().find_map(|a| match a.at {
                    ArmyAt::Place(Place::State(st))
                        if !a.standing
                            && self.army_seat(a) == Some(seat)
                            && self.state(st).control.director() == Some(seat)
                            && self.check_order(seat, &[], &Order::Load { ship: s.id, colonists: 0, from: LoadSource::State(st), army: Some(a.id) }).is_ok() =>
                    {
                        Some((a.id, st))
                    }
                    _ => None,
                });
                if let Some((aid, st)) = army {
                    push(vec![Order::Load { ship: s.id, colonists: 0, from: LoadSource::State(st), army: Some(aid) }], Cat::LoadUnload, self.base_weight(seat, Cat::LoadUnload) * 0.8, 1.0, 1.0, 1.0, format!("load an Army onto {}", ship_name), None);
                }
            }
            if s.kind.is_warship() || s.army.is_some() {
                // Warships go where the Faction has or wants Colonies, or where a rival is: any
                // rival, nearest by transit turns first (ticket #50).
                let mut dests: Vec<BodyId> = Vec::new();
                for c in &self.colonies {
                    if c.body != body && (c.control.director() == Some(seat) || self.rival_holds(seat, c)) && !dests.contains(&c.body) {
                        dests.push(c.body);
                    }
                }
                dests.sort_by_key(|d| self.transit_cost_for(seat, body, *d).0);
                for d in dests {
                    let threat = if self.enemy_present_or_inbound(seat, d) { m.threat } else { 1.0 };
                    let base = self.base_weight(seat, Cat::Transit) * if kind == FactionKind::Prospectors { 0.9 } else { 0.6 };
                    push(vec![Order::Transit { ship: s.id, to: d, slot: self.ai_destination_orbit(seat, s, d) }], Cat::Transit, base, 1.0, threat, 1.0, format!("send {} to {}", ship_name, self.tables.body(d).name), None);
                }
                if let Some(aid) = s.army.filter(|_| body != BodyId::Earth) {
                    {
                        // Ticket #335 (version 0.09.0): a landing is given from the orbit that
                        // touches the place -- low orbit for the ground, the station's own ring.
                        for c in self.colonies.iter().filter(|c| c.body == body && self.ship_may_touch(s, c)) {
                            let enemy = self.rival_holds(seat, c);
                            let mine = c.control.director() == Some(seat);
                            if enemy || mine {
                                let defence: i64 =
                                    self.defenders_at(Place::Colony(c.id), seat).iter().filter_map(|id| self.army(*id)).map(|a| self.army_defended_strength(a)).sum();
                                let odds = first_round_odds(self.army_strength(self.army(aid).unwrap()), defence);
                                if enemy && odds < th.attack_odds {
                                    continue;
                                }
                                // Ticket #394 (version 0.09.3): a rival's Colony at its bounty, so a
                                // seat with cause and an Army goes for the fat one it can beat.
                                let base = self.base_weight(seat, Cat::LoadUnload) * if enemy { self.ai_bounty_against(Some(seat), c.id, largest_on_board) } else { 1.0 };
                                push(vec![Order::Unload { ship: s.id, colonists: 0, army: true, into: UnloadTarget::Colony(c.id) }], Cat::LoadUnload, base, 1.0, if mine { m.threat } else { 1.0 }, 1.0, format!("land an Army at {}", self.place_name(Place::Colony(c.id))), None);
                            }
                        }
                    }
                }
            }
        }

        // Ticket #284 (version 0.08.5): one new war a turn per seat -- at most one attack on a place a
        // rival holds is proposed each turn, so a freed table does not converge on one Region.
        let mut wars_opened = 0u32;
        // --- Stances for every Ship stack
        for body in BodyId::ALL {
            let stack: Vec<&Ship> = self.ships.iter().filter(|s| s.seat == seat && s.at == ShipAt::Body(body)).collect();
            if stack.is_empty() {
                continue;
            }
            let key = format!("ships@{body:?}");
            // Ticket #335 (version 0.09.0): where the seat wants the GROUND, what it wants is
            // Orbital Control of low orbit, so the contest its stance reads is **low orbit's** and
            // not the whole Body's: a rival sitting at a station's ring three orbits up neither
            // holds the surface against it nor is in the fight it would open. Everywhere else the
            // Body is still what a stance covers, since one stance order covers every orbit of it.
            let contest = if self.ai_wants_the_ground(seat, body) { Some(Orbit::Low) } else { None };
            // Ticket #355 (version 0.09.1): the Body-wide strengths and "enemy here" that stood here
            // are replaced by the per-orbit `fights` below.
            // Ticket #335 (version 0.09.0): an Intercept catches only arrivals into its OWN orbit,
            // so the candidate is offered only where a warship of this seat's picket is standing in
            // the ring the unarmed hull named when it left. Low orbit is the picket's place by
            // default, since that is the lane every arrival bound for the ground takes; a hull
            // flying to a station's ring is caught by a hull already at that ring.
            let inbound_target = self.ships.iter().any(|s| {
                s.seat != seat
                    && matches!(s.kind, UnitKind::ColonyShip | UnitKind::Carrier)
                    && matches!(s.at, ShipAt::Transit { to, .. } if to == body)
                    && self.sees_ship(seat, s)
                    && stack.iter().any(|p| p.kind.is_warship() && !p.escaped && self.ship_orbit(p) == self.ship_orbit(s))
            });
            let threat = if self.enemy_present_or_inbound(seat, body) { m.threat } else { 1.0 };
            let total_hp: u32 = stack.iter().map(|s| self.tables.unit(s.kind).hit_points).sum();
            let total_dmg: u32 = stack.iter().map(|s| s.damage).sum();
            let warships = stack.iter().any(|s| s.kind.is_warship());
            // Ticket #284 (version 0.08.5): every seat attacks in orbit on the Prospectors' terms --
            // the odds clear the bar -- given a cause: Wary or worse toward the seat holding Orbital
            // Control against it, or toward any enemy present; or, as before, a blockade of a Body
            // where it has a Colony. And the attack, when allowed, IS the stance: Hold is the candidate
            // only when it is not, a condition where a score contest let Hold win every tie.
            let mut attack: Option<f64> = None;
            // Ticket #355 (version 0.09.1): **the fights an Attack would open, orbit by orbit.** An
            // Attack is fought as one melee in each orbit where this seat's Ships meet another's
            // Ships or Batteries (#335), so that is what is read: the odds of each such fight, and
            // whether the seat has cause against anyone in it. The reading it replaces took one
            // contest for the whole stack -- low orbit wherever the seat wanted the ground, which
            // over Earth was nearly always -- so a fleet at a rival's ring, facing that station's
            // Battery, saw no enemy at all (127 of 141 readings, traced), and elsewhere it summed
            // every rival round the planet against the one fight in front of it.
            let fights: Vec<(f64, bool)> = self
                .orbits_of(body)
                .into_iter()
                .filter_map(|o| {
                    let in_o = |s: &&Ship| self.ship_in_orbit(s, body, o) && !s.escaped;
                    if !self.ships.iter().filter(in_o).any(|s| s.seat == seat) {
                        return None;
                    }
                    let foes: Vec<Seat> = seat.others().into_iter().filter(|x| self.ships.iter().filter(in_o).any(|s| s.seat == *x) || !self.batteries_at(*x, body, o).is_empty()).collect();
                    if foes.is_empty() {
                        return None;
                    }
                    let mine: i64 = self.ships.iter().filter(in_o).filter(|s| s.seat == seat).map(|s| self.ship_strength(s)).sum();
                    let theirs: i64 = self.ships.iter().filter(in_o).filter(|s| s.seat != seat).map(|s| self.ship_strength(s)).sum::<i64>() + foes.iter().map(|x| self.battery_strength(*x, body, o)).sum::<i64>();
                    Some((first_round_odds(mine, theirs), foes.iter().any(|x| self.has_cause(seat, *x))))
                })
                .collect();
            if warships && !fights.is_empty() {
                // Ticket #50: the odds are against every other seat's strength present -- now the
                // worst of the fights the Attack would open.
                let odds = fights.iter().map(|f| f.0).fold(f64::INFINITY, f64::min);
                let my_colony_here = self.colonies.iter().any(|c| c.body == body && c.control.controller() == Some(seat));
                let held_against_me = self.orbital_control(body).map(|o| o != seat).unwrap_or(false);
                // Ticket #335 (version 0.09.0): Wary or worse toward the seat holding Orbital
                // Control against it; ticket #355: or toward anyone in a fight the Attack opens.
                let cause = self.orbital_control(body).filter(|o| *o != seat).is_some_and(|o| self.has_cause(seat, o)) || fights.iter().any(|f| f.1);
                // Ticket #346 (version 0.09.1): **the Fuel a Battle would cost, weighed against the
                // prize.** The odds are discounted by `ai_battle_fuel_weight` before they are read
                // against the bar, so a Battle that would strand the fleet for nothing has to look
                // that much better to be worth a tank. The weighing lives in the ODDS rather than
                // in the candidate's score because only one stance candidate is ever pushed per
                // stack (see #284 below): a multiplier on a candidate with nothing to lose to
                // cannot change an order, and a rule that never reaches the computer seats is not
                // built. At `battle_fuel_weight = 1.0` the weighing is off and this reads exactly
                // as it did before the ticket.
                if odds * self.ai_battle_fuel_weight(seat, body, contest) >= th.attack_odds && (cause || (my_colony_here && held_against_me)) && wars_opened < 1 {
                    attack = Some(odds);
                    wars_opened += 1;
                }
            }
            // Ticket #355 (version 0.09.1): a Blockade, where one applies, IS the stance, on
            // #284's reading for the Attack. Offered beside Hold it lost every time: over twenty
            // games 33 warship-turns sat at a rival's ring with cause and every one held.
            // Ticket #570 (version 0.1.0.3), at the designer's word: and in LOW ORBIT, against a
            // rival's ground Colony below, with cause and no Battery of its holder's there -- a
            // Blockade of low orbit is what shuts the ground now, where Control on any stance did,
            // and the computer never ordered one there before.
            let blockade = if attack.is_none() {
                stack.iter().filter(|s| s.kind.is_warship()).find_map(|s| match s.slot {
                    Some(slot) => self.station_at(body, slot).filter(|c| self.rival_holds(seat, c) && c.control.director().is_some_and(|h| self.has_cause(seat, h) && self.batteries_at(h, body, Orbit::Slot(c.slot)).is_empty())),
                    None => self.colonies.iter().find(|c| c.body == body && !c.in_orbit && self.rival_holds(seat, c) && c.control.director().is_some_and(|h| self.has_cause(seat, h) && self.batteries_at(h, body, Orbit::Low).is_empty())),
                })
            } else {
                None
            };
            match (attack, blockade) {
                (Some(odds), _) => push(vec![Order::ShipStance { body, stance: Stance::Attack }], Cat::StanceAttack, self.base_weight(seat, Cat::StanceAttack) * Seat::ALL.into_iter().filter(|r| *r != seat && self.ships.iter().any(|x| x.seat == *r && x.at == ShipAt::Body(body))).map(|r| self.hostile_lift(seat, r)).fold(1.0, f64::max), 1.0, 1.0, 1.0, format!("Attack at {} (odds {:.0}%)", self.tables.body(body).name, odds * 100.0), Some(key.clone())),
                (None, Some(target)) => push(
                    vec![Order::ShipStance { body, stance: Stance::Blockade }],
                    Cat::StanceBlockade,
                    self.base_weight(seat, Cat::StanceBlockade) * target.control.director().map(|h| self.hostile_lift(seat, h)).unwrap_or(1.0),
                    1.0,
                    1.0,
                    1.0,
                    format!("Blockade {} at {}", self.place_name(Place::Colony(target.id)), self.tables.body(body).name),
                    Some(key.clone()),
                ),
                (None, None) => push(vec![Order::ShipStance { body, stance: Stance::Hold }], Cat::StanceHold, self.base_weight(seat, Cat::StanceHold), 1.0, threat, 1.0, format!("Hold at {}", self.tables.body(body).name), Some(key.clone())),
            }
            // Ticket #328 (version 0.08.8): a Battleship holding the orbit outright, off Earth, with
            // cause against a Colony's holder there, bombards it, at the orbital Attack's weight.
            // Its own key, since the stance is the stack's and a Bombard is the Ship's; a second
            // Bombard for the same Ship is dropped at the check.
            // Ticket #335 (version 0.09.0): the orbit the Battleship is in is the orbit it must
            // hold, so the candidate is offered per Colony: a ground Colony from low orbit under an
            // outright Orbital Control, a station from that station's own ring with no rival
            // warship and no rival working Battery in it. Offering it any other way would be
            // proposing an order the gate refuses.
            if body != BodyId::Earth {
                for s in stack.iter().filter(|s| s.kind == UnitKind::Battleship && !s.escaped) {
                    for c in self.colonies.iter().filter(|c| c.body == body && self.ship_may_touch(s, c)) {
                        let Some(h) = c.control.director().filter(|h| *h != seat) else { continue };
                        if !self.has_cause(seat, h) {
                            continue;
                        }
                        let holds = match self.colony_orbit(c) {
                            Orbit::Low => self.orbital_control(body) == Some(seat),
                            o => self.orbit_uncontested(seat, body, o),
                        };
                        if !holds {
                            continue;
                        }
                        push(vec![Order::Bombard { ship: s.id, colony: c.id }], Cat::StanceAttack, self.base_weight(seat, Cat::StanceAttack) * c.control.director().map(|h| self.hostile_lift(seat, h)).unwrap_or(1.0), 1.0, 1.0, 1.0, format!("Bombard {} from {}", self.place_name(Place::Colony(c.id)), self.ship_name(s)), None);
                    }
                }
            }
            // Ticket #343 (version 0.09.1): a Missile Carrier carrying its Warhead, in an orbit it
            // holds outright, fires at the place the seat most wants and cannot take -- if that
            // place is one this orbit touches. EARTH IS NOT EXCEPTED, unlike the Bombard above: a
            // Region is a lawful target and a nuke on Earth is the point of the weapon. A carrier
            // that has fired is offered a Rearm instead, at a yard of its own in its own orbit,
            // which puts it back in the war after the round trip home.
            let wanted = self.nuke_targets(seat);
            for s in stack.iter().filter(|s| s.kind == UnitKind::MissileCarrier && s.warhead && !s.escaped) {
                let orbit = self.ship_orbit(s);
                let holds = if orbit.is_low() { self.orbital_control(body) == Some(seat) } else { self.orbit_uncontested(seat, body, orbit) };
                if !holds {
                    continue;
                }
                let touched = wanted.iter().copied().find(|t| {
                    let at = match t {
                        Place::State(_) => (BodyId::Earth, Orbit::Low),
                        Place::Colony(c) => match self.colony(*c) {
                            Some(col) => (col.body, self.colony_orbit(col)),
                            None => return false,
                        },
                    };
                    at == (body, orbit)
                });
                if let Some(target) = touched {
                    push(vec![Order::Launch { ship: s.id, target }], Cat::Launch, self.base_weight(seat, Cat::Launch) * self.place_director(target).map(|h| self.hostile_lift(seat, h)).unwrap_or(1.0), 1.0, 1.0, 1.0, format!("Launch at {} from {}", self.place_name(target), self.ship_name(s)), None);
                }
            }
            // Ticket #343 (version 0.09.1): a carrier at the right Body in the wrong orbit moves to
            // the orbit its target is touched from. Without this a hull built at a station's yard
            // sits in that station's ring for the rest of the game, since the ring touches nothing
            // but the station: measured, five hulls were built over eighty games and none fired.
            for s in stack.iter().filter(|s| s.kind == UnitKind::MissileCarrier && s.warhead && !s.escaped) {
                let orbit = self.ship_orbit(s);
                let want = wanted.iter().copied().find_map(|t| {
                    let at = match t {
                        Place::State(_) => (BodyId::Earth, Orbit::Low),
                        Place::Colony(c) => (self.colony(c)?.body, self.colony_orbit(self.colony(c)?)),
                    };
                    (at.0 == body && at.1 != orbit).then_some(at.1)
                });
                if let Some(to) = want {
                    push(vec![Order::ChangeOrbit { ship: s.id, slot: to.slot() }], Cat::MissileCarrier, self.base_weight(seat, Cat::MissileCarrier), 1.0, 1.0, 1.0, format!("Move {} to {}", self.ship_name(s), self.orbit_name(body, to)), None);
                }
            }
            for s in stack.iter().filter(|s| s.kind == UnitKind::MissileCarrier && !s.warhead && !s.escaped) {
                if matches!(self.rearm_site(seat, s.id), RearmSite::Yard(_)) {
                    push(vec![Order::Rearm { ship: s.id }], Cat::MissileCarrier, self.base_weight(seat, Cat::MissileCarrier), 1.0, 1.0, 1.0, format!("Rearm {}", self.ship_name(s)), None);
                }
            }
            // Ticket #319 (version 0.08.8): Intercept FIRES. Until this ticket it was offered only
            // to the seat holding Orbital Control outright, and at the Hold weight, so Attack's
            // weight won the Body's one key every time; over eighty games no interception was
            // fought. Now any seat with warships at the Body may intercept an unarmed enemy hull
            // inbound, and the candidate scores above Hold, at the designer's word. An armed
            // inbound stack is not intercepted: that would open a Battle against whoever arrives.
            if warships && inbound_target {
                push(vec![Order::ShipStance { body, stance: Stance::Intercept }], Cat::StanceIntercept, self.base_weight(seat, Cat::StanceIntercept).max(self.base_weight(seat, Cat::StanceHold) + 1.0), 1.0, threat, 1.0, format!("Intercept at {}", self.tables.body(body).name), Some(key.clone()));
            }
            if total_hp > 0 && (total_dmg as f64) / (total_hp as f64) >= th.evade_damage_fraction {
                push(vec![Order::ShipStance { body, stance: Stance::Evade }], Cat::StanceEvade, self.base_weight(seat, Cat::StanceEvade) * 10.0, 1.0, 1.0, 1.0, format!("Evade at {}", self.tables.body(body).name), Some(key.clone()));
            }
            // Ticket #278 (version 0.08.5): a Blockade must be chosen, so the stack that landed in a
            // rival station's slot (ai_blockade_slot) is offered the stance that makes it one; not
            // against a station whose holder has a Battery in its own orbit (#324, #335). Ticket
            // #355 (version 0.09.1): that candidate is now the stance itself, above, with cause.
        }

        // --- Armies on Earth: Stances and attacks on neighbours
        let mut places: Vec<Place> = StateId::ALL.into_iter().map(Place::State).collect();
        places.extend(self.colonies.iter().map(|c| Place::Colony(c.id)));
        for place in places {
            let mine = self.armies_of_seat_at(seat, place);
            if mine.is_empty() {
                continue;
            }
            let key = format!("armies@{place:?}");
            let threat = if self.enemy_army_near(seat, place) { m.threat } else { 1.0 };
            let my_str = self.army_stack_strength(seat, place);
            // Ticket #284 (version 0.08.5): an occupier STAYS. A stack at a place this seat is
            // occupying holds at three times the weight and marches nowhere -- the measured
            // hit-and-run (Saudi Arabia abandoned for Nigeria) broke its own Occupation for free.
            let occupying = matches!(self.place_control(place), Control::Occupied { occupier, .. } if occupier == seat);
            // Ticket #296 (version 0.08.6): the live figure, since a Region's own Army has as many
            // hit points as its strength now; the card's 5 read here would have been a lie.
            let total_hp: u32 = mine.iter().filter_map(|id| self.army(*id)).map(|a| self.army_hit_points(a)).sum();
            let total_dmg: u32 = mine.iter().filter_map(|id| self.army(*id)).map(|a| a.damage).sum();
            if total_hp > 0 && (total_dmg as f64) / (total_hp as f64) >= th.evade_damage_fraction {
                push(vec![Order::ArmyStance { place, stance: Stance::Evade }], Cat::StanceEvade, self.base_weight(seat, Cat::StanceEvade) * 10.0, 1.0, 1.0, 1.0, format!("Evade at {}", self.place_name(place)), Some(key.clone()));
            }
            // Attack where this seat does not direct the place and defenders stand. Ticket #284
            // (version 0.08.5): every seat on the Prospectors' terms, given a cause -- a neutral
            // place, a place lost to a running Occupation, or a holder the seat is Wary or worse
            // toward -- and the attack, when allowed, IS the stance; Hold is the candidate otherwise.
            let mut attack: Option<f64> = None;
            if self.place_director(place) != Some(seat) {
                // Ticket #302 (version 0.08.6): against what the defenders FIGHT at, not their bare strength.
                let def: i64 = self.defenders_at(place, seat).iter().filter_map(|id| self.army(*id)).map(|a| self.army_defended_strength(a)).sum();
                let odds = first_round_odds(my_str, def);
                let held_by_rival = matches!(self.place_control(place), Control::Controlled(r) if r != seat);
                if (odds >= th.attack_odds || def == 0) && self.war_cause_at(seat, place) && (!held_by_rival || wars_opened < 1) {
                    attack = Some(odds);
                    if held_by_rival {
                        wars_opened += 1;
                    }
                }
            }
            match attack {
                Some(odds) => push(vec![Order::ArmyStance { place, stance: Stance::Attack }], Cat::StanceAttack, self.base_weight(seat, Cat::StanceAttack) * 1.5 * self.place_director(place).map(|h| self.hostile_lift(seat, h)).unwrap_or(1.0), 1.0, 1.0, 1.0, format!("Attack at {} (odds {:.0}%)", self.place_name(place), odds * 100.0), Some(key.clone())),
                // Ticket #297 (version 0.08.6): dig in rather than hold where a rival's Army stands
                // next door and there is no cause to attack, and wherever this seat occupies -- an
                // occupier stays (#284), and dug in it cannot leave. Hold is what is left.
                None if occupying || self.enemy_army_near(seat, place) => push(vec![Order::ArmyStance { place, stance: Stance::DigIn }], Cat::StanceDigIn, self.base_weight(seat, Cat::StanceDigIn) * if occupying { 3.0 } else { 1.0 }, 1.0, threat, 1.0, format!("Dig in at {}", self.place_name(place)), Some(key.clone())),
                None => push(vec![Order::ArmyStance { place, stance: Stance::Hold }], Cat::StanceHold, self.base_weight(seat, Cat::StanceHold), 1.0, threat, 1.0, format!("Hold at {}", self.place_name(place)), Some(key.clone())),
            }
            // Moves into neighbouring states with a non-standing Army. Ticket #284 (version 0.08.5):
            // any seat, on the odds, given a cause; an occupier marches nowhere.
            if let Place::State(sid) = place
                && !occupying
            {
                // Ticket #321 (version 0.08.8): a Region's own Army marches too, but last: the
                // raised Armies are offered first, since one new war a turn is the cap and the
                // first candidate pushed takes it, and its march is weighed at half, so the
                // computer empties a Region of its own defence only when nothing else will serve.
                let mut ordered: Vec<ArmyId> = mine.clone();
                ordered.sort_by_key(|aid| self.army(*aid).map(|a| a.standing).unwrap_or(true));
                // Ticket #322 (version 0.08.8): THE STACK MARCHES. Where two or more Armies of the
                // seat may march, one candidate moves them all, its odds read from their summed
                // strength, so two together clear the bar where one alone does not -- the
                // playtest note's "it never masses" made false. Offered before the single marches,
                // since one new war a turn is the cap and the first candidate pushed takes it;
                // weighed as a raised Army's march when any of the stack is raised.
                let marchable: Vec<ArmyId> = ordered.iter().copied().filter(|aid| self.army(*aid).is_some_and(|a| a.damage <= 2 && a.stance != Stance::DigIn)).collect();
                if marchable.len() > 1 {
                    let strength: i64 = marchable.iter().filter_map(|aid| self.army(*aid)).map(|a| self.army_strength(a)).sum();
                    let any_raised = marchable.iter().filter_map(|aid| self.army(*aid)).any(|a| !a.standing);
                    let own_army = if any_raised { 1.0 } else { 0.5 };
                    for n in &self.tables.state(sid).neighbours {
                        let ctrl = self.state(*n).control;
                        if ctrl == Control::Controlled(seat) || matches!(ctrl, Control::Controlled(h) if h != seat && self.accord_has(seat, h, Term::Passage)) {
                            continue;
                        }
                        let def: i64 = self.defenders_at(Place::State(*n), seat).iter().filter_map(|id| self.army(*id)).map(|a| self.army_defended_strength(a)).sum();
                        let odds = first_round_odds(strength, def);
                        let held_by_rival = matches!(ctrl, Control::Controlled(r) if r != seat);
                        let allowed = odds >= th.attack_odds && self.war_cause_at(seat, Place::State(*n)) && (!held_by_rival || wars_opened < 1);
                        if allowed {
                            if held_by_rival {
                                wars_opened += 1;
                            }
                            let value = (self.tables.state(*n).industry_level + self.tables.state(*n).size) as f64 / 7.0;
                            let orders: Vec<Order> = marchable.iter().map(|aid| Order::MoveArmy { army: *aid, to: *n }).collect();
                            push(orders, Cat::StanceAttack, self.base_weight(seat, Cat::StanceAttack) * (1.0 + value) * own_army * ctrl.controller().map(|h| self.hostile_lift(seat, h)).unwrap_or(1.0), 1.0, 1.0, 1.0, format!("march the stack of {} on {} (odds {:.0}%)", marchable.len(), self.tables.state(*n).name, odds * 100.0), None);
                        }
                    }
                }
                for aid in &ordered {
                    let a = self.army(*aid).unwrap();
                    // Ticket #297 (version 0.08.6): a dug-in Army is refused a march until its stance
                    // has changed and a turn has passed, so no march is offered for it.
                    if a.damage > 2 || a.stance == Stance::DigIn {
                        continue;
                    }
                    let own_army = if a.standing { 0.5 } else { 1.0 };
                    for n in &self.tables.state(sid).neighbours {
                        let ctrl = self.state(*n).control;
                        if ctrl == Control::Controlled(seat) {
                            continue;
                        }
                        let def: i64 = self.defenders_at(Place::State(*n), seat).iter().filter_map(|id| self.army(*id)).map(|a| self.army_defended_strength(a)).sum();
                        let odds = first_round_odds(self.army_strength(a), def);
                        let held_by_rival = matches!(ctrl, Control::Controlled(r) if r != seat);
                        // Ticket #320 (version 0.08.8): a partner's Region under Passage is never a
                        // target; it is a place to move through, which the computer does not plan.
                        if matches!(ctrl, Control::Controlled(h) if h != seat && self.accord_has(seat, h, Term::Passage)) {
                            continue;
                        }
                        let allowed = odds >= th.attack_odds && self.war_cause_at(seat, Place::State(*n)) && (!held_by_rival || wars_opened < 1);
                        if allowed {
                            if held_by_rival {
                                wars_opened += 1;
                            }
                            let value = (self.tables.state(*n).industry_level + self.tables.state(*n).size) as f64 / 7.0;
                            push(vec![Order::MoveArmy { army: *aid, to: *n }], Cat::StanceAttack, self.base_weight(seat, Cat::StanceAttack) * (1.0 + value) * own_army * ctrl.controller().map(|h| self.hostile_lift(seat, h)).unwrap_or(1.0), 1.0, 1.0, 1.0, format!("march on {} (odds {:.0}%)", self.tables.state(*n).name, odds * 100.0), None);
                        }
                    }
                }
            }
        }

        // --- Ticket #332 (version 0.09.0): a build is ordered where the place's Widgets finish it
        // soonest, the queue's depth as the gap -- the designer's words. Every build candidate is
        // weighed against the SAME build at the seat's other places by the Resolutions until this
        // place's Widgets would complete it behind everything already in its queue
        // (`turns_to_build`: the queue's Widgets still owed, plus the build's own, at the place's
        // rate): the soonest place at full weight, every other at soonest / turns, never below
        // `build_pace_floor`. So a Research Lab goes to the Region with the Factory and an empty
        // queue rather than the one two builds deep, and a build with one place to stand is at
        // full weight there. It never weighs one build against another: a first cut that
        // discounted every build by its turns outright halved the opening Habitat and quartered
        // the Solar Array on a starting station, whose Core makes one Widget a turn (ticket #290's
        // and #89's tests red, measured). A discount and never a lift, on ticket #232's lesson.
        // The two Widget makers are exempt: a slow place is exactly where a Factory or a Factory
        // Module is wanted, and its pace would say the opposite. So is the early Mine, whose Region
        // the designer chose by its Materials lean and not by its Widgets: paced, a Materials-lean
        // Region making one Widget a turn tied with an Energy-lean one making two (9 and 9,
        // measured), and the Mine went to the wrong one.
        let mut paced: Vec<(usize, BuildItem, u32)> = Vec::new();
        for (i, c) in cands.iter().enumerate() {
            let [o] = c.orders.as_slice() else { continue };
            let Some((place, item)) = Self::build_target(o) else { continue };
            if matches!(item, BuildItem::Facility(FacilityKind::Factory) | BuildItem::Module(ModuleKind::Factory)) {
                continue;
            }
            if item == BuildItem::Facility(FacilityKind::Mine) && matches!(place, Place::State(s) if early_mine_region == Some(s)) {
                continue;
            }
            paced.push((i, item, self.turns_to_build(seat, place, item)));
        }
        for (i, item, turns) in &paced {
            let soonest = paced.iter().filter(|(_, it, _)| it == item).map(|(_, _, t)| *t).min().unwrap_or(*turns);
            let c = &mut cands[*i];
            if *turns == u32::MAX {
                c.pace = m.build_pace_floor;
                c.note.push_str(" (no Widgets here)");
            } else {
                c.pace = (soonest as f64 / *turns as f64).clamp(m.build_pace_floor, 1.0);
                c.note.push_str(&format!(" ({turns} turn{} here)", if *turns == 1 { "" } else { "s" }));
            }
        }

        // Ticket #620 (version 0.1.0.5): while the Collapse falls before the seat's own win, what
        // emits weighs less and what cleans weighs more, across the whole list.
        if self.ai_loses_the_race(seat) {
            for c in cands.iter_mut() {
                if let [o] = c.orders.as_slice() {
                    c.base *= self.ai_collapse_weight(o);
                }
            }
            self.log(format!("AI {}: the Collapse falls before the win; emitting x{}, cleaner x{}", self.seat_name(seat), th.collapse_emit_weight, th.collapse_clean_weight));
            self.war.collapse_weighed[seat.index()] += 1;
        }

        // --- Sort and spend greedily; Stances last, one per stack.
        cands.sort_by(|a, b| b.score().partial_cmp(&a.score()).unwrap_or(std::cmp::Ordering::Equal));
        let mut chosen: Vec<Order> = Vec::new();
        let mut stacks_done: Vec<String> = Vec::new();
        let mut lines: Vec<String> = Vec::new();
        // Saving: once a legal, higher-scored action is out of reach now but within four more turns
        // of Materials income, Materials are held for it rather than spent on lower-scored actions
        // (ticket #56: one turn let a Factory bought every turn starve the Colony Ship for good).
        let mut reserve: Option<String> = None;
        // Ticket #54: the same for Ducats, held for a higher-scored Ducat action (a Leapfrog) that
        // three turns of Ducat income would bring within reach.
        let ducat_income = self.seat(seat).income_last_turn.ducats;
        let mut ducat_reserve: Option<String> = None;
        // Ticket #57: with the Mars window two turns away or less, Fuel is banked for the crossing
        // the seat most wants, exactly as Materials are banked for a build: nothing else burns Fuel
        // meanwhile. Away from the window it spends Fuel as it always did.
        let fuel_held_for: Option<String> = if self.window_within(2) {
            cands
                .iter()
                .find(|c| {
                    c.orders.iter().any(|o| match o {
                        Order::Transit { ship, to, .. } => {
                            let from = self.ships.iter().find(|s| s.id == *ship).and_then(|s| match s.at {
                                ShipAt::Body(b) => Some(b),
                                _ => None,
                            });
                            from.map(|f| self.crossing_offset(f, *to, self.turn).is_some()).unwrap_or(false)
                        }
                        _ => false,
                    })
                })
                .map(|c| c.note.clone())
        } else {
            None
        };
        for c in cands.iter().filter(|c| c.stack.is_none()) {
            let mut ok = true;
            let mut trial = chosen.clone();
            let ducats_cost: f64 = c.orders.iter().map(|o| self.order_cost(seat, o).ducats).sum();
            if ducats_cost > 0.0 {
                if let Some(note) = &ducat_reserve {
                    lines.push(format!("  save  {:6.1}  {} (holding Ducats for {})", c.score(), c.note, note));
                    continue;
                }
                let (left, _) = self.remaining(seat, &chosen);
                if ducats_cost > left.ducats
                    && ducats_cost <= left.ducats + 3.0 * ducat_income
                    && c.orders.iter().all(|o| self.check_order_legality(seat, &chosen, o).is_ok())
                {
                    ducat_reserve = Some(c.note.clone());
                    lines.push(format!("  wait  {:6.1}  {} (affordable within three turns)", c.score(), c.note));
                    continue;
                }
            }
            // Ticket #57: the Fuel bank. Only the crossing it is held for may spend Fuel. Ticket
            // #87: and a Refuel, which is how the crossing's Fuel reaches the tank now.
            if let Some(note) = &fuel_held_for
                && *note != c.note
                && c.orders.iter().map(|o| self.order_cost(seat, o).fuel).sum::<f64>() > 0.0
                && !c.orders.iter().any(|o| matches!(o, Order::Refuel { .. }))
            {
                lines.push(format!("  save  {:6.1}  {} (banking Fuel for {})", c.score(), c.note, note));
                continue;
            }
            let materials_cost: f64 = c.orders.iter().map(|o| self.order_cost(seat, o).materials).sum();
            if materials_cost > 0.0 {
                if let Some(note) = &reserve {
                    lines.push(format!("  save  {:6.1}  {} (holding Materials for {})", c.score(), c.note, note));
                    continue;
                }
                let (left, _) = self.remaining(seat, &chosen);
                // Ticket #68: an Archivist on four Materials a turn never has a 50-Materials Module
                // or a 35-Materials Shipyard within four turns of income, so for the steps of the
                // Archive's own path the horizon is twelve turns.
                let horizon = if first_kind == VictoryFirstKind::ArchiveResearch && advances_first(c.cat, None) { 12.0 } else { 4.0 };
                if materials_cost > left.materials
                    && materials_cost <= left.materials + horizon * materials_income
                    && c.orders.iter().all(|o| self.check_order_legality(seat, &chosen, o).is_ok())
                {
                    reserve = Some(c.note.clone());
                    lines.push(format!("  wait  {:6.1}  {} (affordable within four turns)", c.score(), c.note));
                    continue;
                }
            }
            for o in &c.orders {
                match self.check_order(seat, &trial, o) {
                    Ok(_) => trial.push(o.clone()),
                    Err(e) => {
                        lines.push(format!("  skip  {:6.1}  {} ({})", c.score(), c.note, e));
                        // Ticket #334 (version 0.09.0): a raise refused for want of people is
                        // counted for the sweep. The gate is `check_order`'s, as for every other
                        // candidate; the computer weighs nothing new against it.
                        if let Order::BuildArmy { place } = o
                            && self.army_people_refusal(&trial, *place).as_deref() == Some(e.0.as_str())
                        {
                            self.war.army_raises_refused_people[seat.index()] += 1;
                        }
                        ok = false;
                        break;
                    }
                }
            }
            if ok {
                lines.push(format!("  take  {:6.1}  {} [{:?}]", c.score(), c.note, c.cat));
                chosen = trial;
            }
        }
        for c in cands.iter().filter(|c| c.stack.is_some()) {
            let key = c.stack.clone().unwrap();
            if stacks_done.contains(&key) {
                continue;
            }
            let mut trial = chosen.clone();
            let mut ok = true;
            for o in &c.orders {
                match self.check_order(seat, &trial, o) {
                    Ok(_) => trial.push(o.clone()),
                    Err(_) => {
                        ok = false;
                        break;
                    }
                }
            }
            if ok {
                lines.push(format!("  take  {:6.1}  {}", c.score(), c.note));
                chosen = trial;
                stacks_done.push(key);
            }
        }
        // Ticket #268 (version 0.08.4): the computer Custodians' carbon-credit offer, a standing
        // figure re-set whenever it should move. They offer their whole credit while their own share
        // of the table's Blame is under the fair quarter, and withdraw the offer when it is not --
        // "I want them to refuse sometimes", the designer said: a seat that needs its credit keeps
        // it. Short of Ducats and clean, they oversell by the cap and take the Blame for the money.
        if kind == FactionKind::Custodians {
            let c = self.tables.carbon_credits.clone();
            let fair = self.tables.influence.blame.fair_share;
            let share = self.blame_share(seat);
            let credit = self.blame_credit(seat).floor() as i64;
            let mut offer = if share < fair { credit } else { 0 };
            if share < fair / 2.0 && self.seat(seat).stockpile.ducats < c.ai_oversell_when_ducats_below as f64 {
                offer += c.cap_per_turn;
            }
            if offer != self.seat(seat).credits_offered {
                lines.push(format!("  take          offer {offer} ppm of carbon credit a turn (was {}; share {share:.2}, credit {credit})", self.seat(seat).credits_offered));
                chosen.push(Order::OfferCredits { ppm: offer });
            }
        }
        // Ticket #72 (version 0.05.5): the Venture Capital Fund's share, played as the designer put
        // it: "an AI/player may set it at 50% for five turns then down to 0% if they're trying to
        // save; end game might try to max at 80% to reach the victory condition before others."
        // So: 0% until the pace's first waypoint (it builds first), then the smallest step that
        // reaches the bar by the pace's last turn at the current output, and the most it may when
        // nothing less will. (A first cut zeroed the share whenever Materials were being held for
        // a build, which is nearly every turn, so nothing was ever banked.)
        // Ticket #448 (version 0.09.6): **the market**, after everything else is chosen, so what is
        // sold is what the turn's own orders leave. Energy is bought where the seat would be short at
        // Income -- the shortfall that shuts buildings. Materials beyond three turns of this turn's
        // own spending plus a reserve are sold, and Fuel beyond what its Ships' tanks take plus a
        // reserve, each only at or above the midpoint price. (A queued build was paid at its order,
        // so "what the queue needs" is the spending the seat keeps up, not the queue itself.)
        {
            let th = self.tables.ai.thresholds.clone();
            let before = self.seat(seat).stockpile;
            let left = self.remaining(seat, &chosen).0;
            let spent = (before.materials - left.materials).max(0.0);
            // Three turns of the larger of its income and its own spending, at the designer's word: a
            // first cut kept three turns of the spending alone, and sold the stock a seat builds up
            // between big builds.
            let income = self.seat(seat).income_last_turn.materials.max(0.0);
            let keep = th.market_materials_reserve + th.market_materials_turns * spent.max(income);
            let surplus = (left.materials - keep).floor() as i64;
            // Never while Materials are held for a dearer build, nor in a turn it bought them: a sale
            // at half the price of a purchase is money burned, and the held Materials are the build.
            let bought_m = chosen.iter().any(|o| matches!(o, Order::Buy { resource: Resource::Materials, .. }));
            let bought_f = chosen.iter().any(|o| matches!(o, Order::Buy { resource: Resource::Fuel, .. }));
            if surplus > 0 && self.market_price_at(0) >= self.market_base(0) && reserve.is_none() && !bought_m {
                let sell = Order::Sell { resource: Resource::Materials, amount: surplus };
                if self.check_order(seat, &chosen, &sell).is_ok() {
                    lines.push(format!("  take          sell {surplus} Materials, beyond {keep:.0} kept"));
                    chosen.push(sell);
                }
            }
            let tanks: f64 = self.ships.iter().filter(|s| s.seat == seat).map(|s| (self.tank_of(seat, s.kind) - s.fuel).max(0.0)).sum();
            // Ticket #575 (version 0.1.0.3): and the power reserve, kept and bought back when short.
            let power_reserve = self.fuel_reserve(seat);
            let keep = tanks + th.market_fuel_reserve + power_reserve;
            let left = self.remaining(seat, &chosen).0;
            let mut bought_f = bought_f;
            if left.fuel < power_reserve && !bought_f {
                let short = (power_reserve - left.fuel).ceil() as i64;
                let buy = Order::Buy { resource: Resource::Fuel, amount: short };
                if self.trade_cost(seat, &chosen, &buy).ducats <= left.ducats && self.check_order(seat, &chosen, &buy).is_ok() {
                    lines.push(format!("  take          buy {short} Fuel for the power reserve ({power_reserve:.1})"));
                    chosen.push(buy);
                    bought_f = true;
                }
            }
            let surplus = (left.fuel - keep).floor() as i64;
            // Not while Fuel is held for the Mars window (ticket #57): a sale is a spend.
            if surplus > 0 && self.market_price_at(1) >= self.market_base(1) && !self.window_within(2) && !bought_f {
                let sell = Order::Sell { resource: Resource::Fuel, amount: surplus };
                if self.check_order(seat, &chosen, &sell).is_ok() {
                    lines.push(format!("  take          sell {surplus} Fuel, beyond {keep:.0} kept"));
                    chosen.push(sell);
                }
            }
        }
        if first_kind == VictoryFirstKind::VentureFund {
            let v = self.tables.venture.clone();
            let bar = self.tables.faction(kind).victory_first.bar;
            let pace = self.tables.ai_pace(kind);
            let first_waypoint = pace.first.first().map(|p| p[0]).unwrap_or(0) as u32;
            let last = pace.first.last().map(|p| p[0]).unwrap_or(self.tables.victory.turns as i64) as u32;
            let turns_to = last.saturating_sub(self.turn).max(1) as f64;
            let s0 = self.seat(seat);
            // Ticket #240 (version 0.08.3): against DUCAT income, which is what the Fund banks
            // now. The rule itself is unchanged and is the weighing the designer asked for -- the
            // SMALLEST share that still reaches the bar by the pace's last turn -- so a seat banks
            // the least it can and leaves the rest of its Ducats to spend on Influence, Relief and
            // repairs. It matters more than it did: measured over 120 games every seat ends every
            // game holding about five Ducats, so an over-large share starves the whole economy
            // where an over-large Materials share only slowed a build.
            let gross = (s0.income_last_turn.ducats + s0.venture_banked_last_turn).max(0.0);
            let need = (bar - s0.venture_fund).max(0.0);
            let share = if self.turn < first_waypoint || need <= 0.0 {
                0.0
            } else {
                let mut sh = 0.0;
                while sh < v.max_share - 1e-9 && sh * gross * turns_to < need {
                    sh += v.share_step;
                }
                sh.min(v.max_share)
            };
            let pct = (share * 100.0).round() as u32;
            let now = (s0.venture_share * 100.0).round() as u32;
            if pct != now {
                lines.push(format!("  take          set the Venture Capital Fund to {pct}% (was {now}%; {need:.0} still wanted over {turns_to:.0} turns at {gross:.0} a turn)"));
                chosen.push(Order::SetVentureShare { share: pct });
            }
        }
        self.log(format!("AI {} scored {} actions (gap x{:.2} on {:?}):", self.seat_name(seat), cands.len(), gap, behind));
        for l in &lines {
            self.log(l.clone());
        }
        // The 0.1.0.0 playtest (ticket #517): the computer keeps the non-aggression it gives. An order
        // that would cross a partner -- Influence on, Agitate in, a Smear of, an Army into, a Bombard
        // or a Launch at a place that partner directs -- is dropped. Measured before: computer seats
        // struck Accords and broke them the same turn by spending Influence on the partner's Regions,
        // which players read as an Accord accepted and never struck.
        // Ticket #569 (version 0.1.0.3): an Influence spend that crosses is refused at `check_order`
        // above, before it takes the Influence budget, so the next candidate gets the Influence it
        // used to waste; measured 19 / 49 / 4 / 8 from 9 / 59 / 5 / 7. The other five are dropped here.
        chosen.retain(|o| self.order_crosses(seat, o).is_none());
        // Ticket #618 (version 0.1.0.5): the contests, once a Region a turn, for the sweep. Counted on
        // the orders chosen, before `end_turn` commits them, so a step it refuses is counted still.
        let mut contested: Vec<StateId> = Vec::new();
        for sid in chosen.iter().filter_map(|o| if let Order::Influence { target: Place::State(sid), .. } = o { Some(*sid) } else { None }) {
            if !contested.contains(&sid) && self.ai_contested(seat, sid) {
                contested.push(sid);
            }
        }
        self.war.contests[seat.index()] += contested.len() as u32;
        // Ticket #58: the scored list is the AI's own head, so it stays in the log. What the Report
        // shows is built from the orders this seat actually commits, in `end_turn`.
        chosen
    }

    /// Ticket #517 (version 0.1.0.0): the Faction this seat holds non-aggression with that this order
    /// would act against -- the acts that break it. Ticket #520: the player's cost line says so too.
    pub fn order_crosses(&self, seat: Seat, order: &Order) -> Option<Seat> {
        let partner = |who: Option<Seat>| who.filter(|w| *w != seat && self.accord_has(seat, *w, Term::NonAggression));
        match order {
            Order::Influence { target, .. } | Order::Launch { target, .. } => partner(self.place_control(*target).director()),
            Order::Agitate { state } | Order::MoveArmy { to: state, .. } => partner(self.place_control(Place::State(*state)).director()),
            Order::Smear { target, .. } => partner(Some(*target)),
            Order::Bombard { colony, .. } => partner(self.place_control(Place::Colony(*colony)).director()),
            _ => None,
        }
    }
}
