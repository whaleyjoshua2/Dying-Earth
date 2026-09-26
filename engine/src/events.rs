//! The Event Deck (spec 13, amended by ticket #25): no Calm Cards, a draw chance that rises with the
//! Temperature, and twenty-two Events over forty cards since ticket #76 (version 0.05.5).

use crate::data::{CardEffect, CardRule, CardThing, Tables};
use crate::ids::*;
use crate::state::*;
use rand::seq::SliceRandom;
use rand::Rng;
use rand_chacha::ChaCha8Rng;

/// The deck as the table deals it, shuffled with the game seed; never reshuffled.
pub fn new_deck(tables: &Tables, rng: &mut ChaCha8Rng) -> Deck {
    let mut cards: Vec<Card> = Vec::new();
    // Ticket #259 (version 0.08.4): the cards that can only land off Earth are not dealt at the
    // start; `join_off_earth_cards` shuffles them in on `off_earth_join_turn`. Measured before the
    // change: a fifth of the cards drawn in a game found nowhere to land and were spent for good.
    for e in tables.events.event.iter().filter(|e| !e.off_earth) {
        for _ in 0..e.copies {
            cards.push(Card::Event(e.id));
        }
    }
    cards.shuffle(rng);
    Deck { cards, drawn: Vec::new(), off_earth_joined: false }
}

impl Game {
    /// The chance a card is drawn this turn: the base at +1.2 C, plus a step per full 0.2 C above it.
    pub fn draw_chance(&self) -> f64 {
        let e = &self.tables.events;
        let base = self.tables.climate.base_temperature;
        let steps = ((self.climate.temperature - base) / e.draw_chance_step_degrees + 1e-9).floor().max(0.0);
        (e.draw_chance_base + e.draw_chance_per_step * steps).clamp(0.0, 1.0)
    }

    /// One roll against the draw chance.
    pub fn rolls_a_card(&mut self) -> bool {
        let p = self.draw_chance();
        self.rng.random::<f64>() < p
    }

    /// Ticket #259 (version 0.08.4): on the joining turn, the off-Earth cards go into the deck --
    /// shuffled into whatever remains of it, so the next draw may be one of them -- once, and the
    /// Report says so. A save from before this version has never dealt them and joins them on its
    /// next Event phase past the turn.
    fn join_off_earth_cards(&mut self) {
        if self.deck.off_earth_joined || self.turn < self.tables.events.off_earth_join_turn {
            return;
        }
        let mut n = 0usize;
        for e in self.tables.events.event.iter().filter(|e| e.off_earth) {
            for _ in 0..e.copies {
                self.deck.cards.push(Card::Event(e.id));
                n += 1;
            }
        }
        self.deck.cards.shuffle(&mut self.rng);
        self.deck.off_earth_joined = true;
        let line = format!("The {n} Events that can only land off Earth join the deck, shuffled in among the {} left.", self.deck.cards.len() - n);
        self.log(line.clone());
        let text = self.say("deck_joined", &[("n", n.to_string())]);
        self.report_line(LineKind::Event, None, text);
    }

    /// Ticket #337 (version 0.09.0): **the Question, at the head of the turn and before orders.**
    ///
    /// The DRAW moved here from the Event phase, because a card that says *hold every Ship in orbit
    /// this turn* can only mean something if it is answered before the orders it binds are given,
    /// and orders are committed in phase 4. The roll, the chance it is rolled against, the
    /// never-reshuffled deck and the off-Earth join are all exactly as they were; only the moment
    /// the top card leaves the deck has moved.
    ///
    /// An ordinary card is **held in silence** -- nothing is logged, nothing is reported, no target
    /// is chosen -- and the Event phase takes it up and announces and applies it precisely where it
    /// always did, so none of the 22 behaves differently. A choice card becomes the turn's pending
    /// question: every seat is asked it, except the seats neither of whose sides it can reach.
    pub fn question_phase(&mut self) {
        self.question = None;
        self.draw = CardDraw::NoCard;
        self.join_off_earth_cards();
        if !self.rolls_a_card() {
            return;
        }
        let Some(card) = self.deck.cards.pop() else {
            self.draw = CardDraw::DeckEmpty;
            return;
        };
        self.deck.drawn.push(card);
        let Card::Event(id) = card;
        if !self.tables.event(id).asks() {
            // One of the 22: held, unannounced, for the Event phase.
            self.draw = CardDraw::Ordinary(id);
            return;
        }
        self.draw = CardDraw::Choice(id);
        let mut q = Question { card: id, answers: [None; SEAT_COUNT] };
        for seat in Seat::ALL {
            // A seat the card cannot touch is NOT asked: its answer is recorded as nothing to
            // decide, it never holds the turn, and the Report says so for it.
            if !self.card_reaches(id, seat) {
                q.answers[seat.index()] = Some(CardAnswer::NothingToDecide);
                self.choice_not_asked[seat.index()] += 1;
            }
        }
        let (name, question) = {
            let c = self.tables.event(id);
            (c.name.clone(), c.choice.as_ref().map(|c| c.question.clone()).unwrap_or_default())
        };
        self.log(format!("Question: {name}: {question}"));
        let text = self.say("card_asked", &[("card", name), ("question", question)]);
        // Ticket #353 (version 0.09.1): `LineKind::Card`, which never headlines. The Card modal holds
        // the screen with this very sentence until the seat answers it, and the Report then opened by
        // repeating it back; the dispatch's headline is for news the player has not read yet.
        self.report_line(LineKind::Card, None, text);
        self.question = Some(q);
    }

    /// Ticket #337: the card asking this turn, and what each seat has answered.
    pub fn pending_question(&self) -> Option<&Question> {
        self.question.as_ref()
    }

    /// Ticket #337: record one seat's answer. Refused where no card is asking, where the seat has
    /// already answered, and where the card was never the seat's to answer.
    pub fn answer_card(&mut self, seat: Seat, taken: bool) -> Result<(), String> {
        let Some(q) = self.question.as_ref() else {
            return Err("No card is asking anything this turn.".to_string());
        };
        let name = self.tables.event(q.card).name.clone();
        match q.answers[seat.index()] {
            Some(CardAnswer::NothingToDecide) => {
                return Err(format!("{name} has nothing in it for the {}: they were not asked.", self.seat_name(seat)));
            }
            Some(_) => return Err(format!("The {} have already answered {name} this turn.", self.seat_name(seat))),
            None => {}
        }
        // Ticket #337: the offer is closed to a seat that cannot pay it, and taking is refused at
        // the door rather than silently turned into a refusal.
        if taken && !self.may_take_card(seat) {
            return Err(format!("The {} cannot pay what {name} asks; they may only refuse.", self.seat_name(seat)));
        }
        let answer = if taken { CardAnswer::Taken } else { CardAnswer::Refused };
        if let Some(q) = self.question.as_mut() {
            q.answers[seat.index()] = Some(answer);
        }
        if taken {
            self.choice_taken[seat.index()] += 1;
        } else {
            self.choice_refused[seat.index()] += 1;
        }
        self.log(format!("{name}: the {} {}.", self.seat_name(seat), answer.word()));
        Ok(())
    }

    /// Ticket #337 R4: a computer seat answers by the rule on its card, read off its own board,
    /// when its orders are computed -- so it never holds the turn and never sees a human's answer.
    pub fn ai_answer_card(&mut self, seat: Seat) {
        let Some(q) = self.question.as_ref() else { return };
        if q.answers[seat.index()].is_some() {
            return;
        }
        let Some(rule) = self.tables.event(q.card).choice.as_ref().map(|c| c.take_when.clone()) else { return };
        // Ticket #337: the card's rule says whether the seat WANTS the offer; it takes it only if
        // it can also pay for it.
        let take = self.card_rule_holds(seat, &rule) && self.may_take_card(seat);
        self.answer_card(seat, take).ok();
    }

    /// Ticket #337 R4: whether the card's rule holds on this seat's board. The predicates are the
    /// ones the ticket names -- Unrest, Ducats, Blame, and whether a landing is under way -- and
    /// every figure comes off the card in `events.toml`.
    pub fn card_rule_holds(&self, seat: Seat, rule: &CardRule) -> bool {
        match rule {
            CardRule::Always => true,
            CardRule::Never => false,
            CardRule::DucatsAtLeast { ducats } => self.seat(seat).stockpile.ducats >= *ducats,
            CardRule::UnrestAtLeast { unrest } => self.controlled_states(seat).iter().any(|s| self.state(*s).unrest >= *unrest),
            CardRule::UnrestBelow { unrest } => self.controlled_states(seat).iter().all(|s| self.state(*s).unrest < *unrest),
            CardRule::BlameAtLeast { blame } => self.blame(seat) >= *blame,
            CardRule::LandingUnderWay => self.ships.iter().any(|s| s.seat == seat && matches!(s.at, ShipAt::Transit { .. })),
            CardRule::NoLandingUnderWay => !self.ships.iter().any(|s| s.seat == seat && matches!(s.at, ShipAt::Transit { .. })),
        }
    }

    // ---------------------------------------------------------------- the effect vocabulary

    /// Ticket #337: the effects of the side this seat answered with, or nothing.
    pub fn card_effects(&self, seat: Seat) -> &[CardEffect] {
        let Some(q) = self.question.as_ref() else { return &[] };
        let taken = match q.answers[seat.index()] {
            Some(CardAnswer::Taken) => true,
            Some(CardAnswer::Refused) => false,
            _ => return &[],
        };
        self.tables.event(q.card).choice.as_ref().map(|c| c.side(taken)).unwrap_or(&[])
    }

    /// Ticket #337: **whether this card has a question for this seat at all.** Each side reaches a
    /// seat when every effect on it has something to land on; a card asks a seat only when both
    /// sides reach it.
    ///
    /// This is where "a seat that cannot be touched is not asked" falls out of the vocabulary
    /// rather than being written per card: a Faction holding no Region is never asked the Hard
    /// Winter, because the refusing side raises Unrest in every Region it holds and there are
    /// none; and a Faction with no Ship is never asked the Grounded Fleet.
    ///
    /// **Being unable to PAY is not the same as having nothing to decide**, at the designer's word
    /// after the first build skipped such a seat: it is asked all the same, the offer closed to it
    /// (`may_take_card`) and refusing its only move, so a struggling Faction still feels the card.
    /// Measured under the old rule: 34% of seat-card pairs over eighty games were never asked,
    /// most of them for want of the price.
    pub fn card_reaches(&self, id: EventId, seat: Seat) -> bool {
        let Some(c) = self.tables.event(id).choice.as_ref() else { return false };
        self.side_reaches(&c.take_does, seat) && self.side_reaches(&c.refuse_does, seat)
    }

    fn side_reaches(&self, side: &[CardEffect], seat: Seat) -> bool {
        !side.is_empty() && side.iter().all(|e| self.card_effect_has_target(e, seat))
    }

    /// Ticket #337: whether this seat could PAY for the card's offer, which is a different question
    /// from whether the offer has anything to land on. A seat that cannot pay is asked all the same
    /// (the designer's word when the first build skipped it): the offer is closed to it and refusing
    /// is its only move, so a struggling Faction still feels the card.
    pub fn may_take_card(&self, seat: Seat) -> bool {
        let Some(q) = self.question.as_ref() else { return false };
        let Some(c) = self.tables.event(q.card).choice.as_ref() else { return false };
        c.take_does.iter().all(|e| self.card_effect_affordable(e, seat))
    }

    /// Ticket #337: the price half of `card_effect_can_land`. Only an effect that costs something
    /// can answer no; everything else is free to choose whether or not it does anything.
    fn card_effect_affordable(&self, e: &CardEffect, seat: Seat) -> bool {
        let s = self.seat(seat);
        match e {
            CardEffect::Resources { materials, fuel, energy, ducats, research: _ } => {
                s.stockpile.materials + materials >= 0 && s.stockpile.fuel + fuel >= 0 && s.stockpile.energy + energy >= 0 && s.stockpile.ducats + ducats >= 0
            }
            CardEffect::PerUnitCost { per, resource, amount } => self.stock_of(seat, *resource) >= self.card_things(seat, *per) as i64 * amount,
            _ => true,
        }
    }

    /// Ticket #337: has this one effect anything on this seat's board to land on? A price is not
    /// asked about here: whether the seat can PAY is `card_effect_affordable`, and a seat that
    /// cannot pay is still asked the card.
    fn card_effect_has_target(&self, e: &CardEffect, seat: Seat) -> bool {
        match e {
            CardEffect::Resources { .. } => true,
            CardEffect::PerUnitCost { per, .. } => self.card_things(seat, *per) > 0,
            CardEffect::PopulationToMostPopulous { .. } | CardEffect::StandingAtMostPopulous { .. } | CardEffect::UnrestAtMostPopulous { .. } | CardEffect::PioneersFree { .. } => {
                self.card_most_populous(seat).is_some()
            }
            CardEffect::StandingAllHeld { .. } | CardEffect::UnrestAllHeld { .. } => !self.controlled_states(seat).is_empty(),
            CardEffect::UnrestAtBusiest { .. } | CardEffect::WidgetsNow { .. } => self.card_busiest(seat).is_some(),
            CardEffect::EmissionsNext { .. } | CardEffect::TradePrice { .. } | CardEffect::RelationsAllRivals { .. } | CardEffect::BlamePpm { .. } => true,
            CardEffect::HoldShips | CardEffect::HoldOneShip => self.ships.iter().any(|s| s.seat == seat),
            CardEffect::DamageShips { in_orbit, .. } => self.ships.iter().any(|s| s.seat == seat && (!in_orbit || matches!(s.at, ShipAt::Body(_)))),
            CardEffect::FacilityOutputMultiplier { facility, .. } => self
                .directed_states(seat)
                .iter()
                .any(|sid| self.state(*sid).facilities.iter().any(|f| f.kind.does_the_job_of(*facility))),
            CardEffect::DiscoveryAtColony { module, .. } => self.card_discovery_body(seat, *module).is_some(),
            CardEffect::FreeBuilding { module, army } => {
                if *army {
                    self.card_most_populous(seat).is_some()
                } else if let Some(k) = module {
                    self.card_smallest_colony(seat, *k).is_some()
                } else {
                    false
                }
            }
        }
    }

    /// Ticket #337: how many things of a kind a `per_unit_cost` counts for this seat.
    fn card_things(&self, seat: Seat, per: CardThing) -> usize {
        match per {
            CardThing::Facility(kind) => self
                .directed_states(seat)
                .iter()
                .map(|s| self.state(*s).facilities.iter().filter(|f| f.kind.does_the_job_of(kind)).count())
                .sum(),
            CardThing::ShipInOrbit => self.ships.iter().filter(|s| s.seat == seat && matches!(s.at, ShipAt::Body(_))).count(),
        }
    }

    fn stock_of(&self, seat: Seat, r: Resource) -> i64 {
        let s = &self.seat(seat).stockpile;
        match r {
            Resource::Materials => s.materials,
            Resource::Fuel => s.fuel,
            Resource::Energy => s.energy,
            Resource::Ducats => s.ducats,
            // Research is the table's pool and never a seat's stock, so nothing is ever held in it.
            Resource::Research | Resource::Widgets => 0,
        }
    }

    /// Ticket #337: the seat's most populous HELD Region -- what the cards mean by "your most
    /// populous state". Ties go to the lower id, so a seeded game is never moved by this.
    pub fn card_most_populous(&self, seat: Seat) -> Option<StateId> {
        self.controlled_states(seat)
            .into_iter()
            .filter(|s| self.state(*s).population > 0.0)
            .max_by(|a, b| {
                let (pa, pb) = (self.state(*a).population, self.state(*b).population);
                pa.partial_cmp(&pb).unwrap_or(std::cmp::Ordering::Equal).then(b.index().cmp(&a.index()))
            })
    }

    /// Ticket #337: the seat's BUSIEST Region -- the one it directs that makes the most Widgets,
    /// which is what the Overtime card means by "your busiest Region" and by "there". Ties go to
    /// the lower id.
    pub fn card_busiest(&self, seat: Seat) -> Option<StateId> {
        self.directed_states(seat)
            .into_iter()
            .max_by_key(|s| (self.widgets_at(Place::State(*s)), std::cmp::Reverse(s.index())))
    }

    /// Ticket #337: the seat's smallest Colony with room for another Module -- where a free one
    /// would stand. Fewest Colonists first, then the lower id.
    pub fn card_smallest_colony(&self, seat: Seat, _kind: ModuleKind) -> Option<ColonyId> {
        self.directed_colonies(seat)
            .into_iter()
            .filter(|c| self.colony(*c).map(|col| self.free_module_slots(col) > 0).unwrap_or(false))
            .min_by_key(|c| (self.colony(*c).map(|col| col.colonists).unwrap_or(0), c.0))
    }

    /// Ticket #337: the Body a `discovery_at_colony` would land over -- the Body of the seat's
    /// Colony holding the most Modules of the kind the Discovery lifts, ties to the lower id.
    ///
    /// **An implementation choice, not the designer's, named here for correction**: a Discovery in
    /// this game is a Body-wide effect on a Module kind (`Discovery { body, kind, .. }`, which is
    /// what a Rich Seam pushes), and the card says "a Discovery at one Colony". Rather than build a
    /// second, Colony-scoped kind of Discovery, the card picks one of the seat's Colonies and the
    /// Discovery stands over its Body, which also reaches a rival's Mines there.
    pub fn card_discovery_body(&self, seat: Seat, kind: ModuleKind) -> Option<BodyId> {
        self.directed_colonies(seat)
            .into_iter()
            .filter_map(|c| self.colony(c))
            .filter(|col| col.modules.iter().any(|m| m.kind == kind))
            .max_by_key(|col| (col.modules.iter().filter(|m| m.kind == kind).count(), std::cmp::Reverse(col.id.0)))
            .map(|col| col.body)
    }

    // ------------------------------------------------- the effects read BEFORE the Resolution

    /// Ticket #337: this seat grounded its fleet, so no transit of its own resolves this turn --
    /// the Solar Storm's shape (`resolve_transits`), for one seat.
    pub fn card_holds_ships(&self, seat: Seat) -> bool {
        self.card_effects(seat).iter().any(|e| matches!(e, CardEffect::HoldShips))
    }

    /// Ticket #337: the one Ship this seat turned aside to answer a call, and so holds this turn.
    ///
    /// **An implementation choice, not the designer's, named here for correction**: it is the
    /// seat's Ship with the most Fuel in its tank, ties to the lower id, so the pick is
    /// deterministic and a seeded game is not moved by it.
    pub fn card_holds_one_ship(&self, seat: Seat) -> Option<ShipId> {
        if !self.card_effects(seat).iter().any(|e| matches!(e, CardEffect::HoldOneShip)) {
            return None;
        }
        self.ships
            .iter()
            .filter(|s| s.seat == seat)
            .max_by_key(|s| (s.fuel, std::cmp::Reverse(s.id.0)))
            .map(|s| s.id)
    }

    /// Ticket #337: the Widgets the Overtime card adds this turn, by place -- read in
    /// `resolve_builds` beside what the place makes of its own, and counted to its director as made
    /// and applied like any other.
    pub fn card_widgets_now(&self) -> Vec<(Place, i64)> {
        let mut out = Vec::new();
        for seat in Seat::ALL {
            for e in self.card_effects(seat) {
                if let CardEffect::WidgetsNow { widgets } = e
                    && let Some(s) = self.card_busiest(seat)
                {
                    out.push((Place::State(s), *widgets));
                }
            }
        }
        out
    }

    /// Ticket #337: the share a Facility of this kind makes for this seat at THIS Income, set by a
    /// card answered last turn. One, where no card touched it.
    pub fn card_facility_multiplier(&self, seat: Seat, kind: FacilityKind) -> f64 {
        match self.seat(seat).card_facility {
            Some((k, m)) if kind.does_the_job_of(k) => m,
            _ => 1.0,
        }
    }

    // ------------------------------------------------------ the effects applied at Resolution

    /// Ticket #337: Resolution (h), beside `apply_event_now`: every seat's answer, effect by
    /// effect. The two that are read earlier in the Resolution -- the held fleet before the
    /// transits, the Widgets before the builds -- do nothing here, and say so.
    pub fn apply_card_answers(&mut self) {
        let Some(q) = self.question.clone() else { return };
        let Some(choice) = self.tables.event(q.card).choice.clone() else { return };
        let name = self.tables.event(q.card).name.clone();
        for seat in Seat::ALL {
            let taken = match q.answers[seat.index()] {
                Some(CardAnswer::Taken) => true,
                Some(CardAnswer::Refused) => false,
                _ => continue,
            };
            for e in choice.side(taken) {
                self.apply_card_effect(seat, e);
            }
            let word = if taken { CardAnswer::Taken } else { CardAnswer::Refused }.word();
            let text = self.say("card_answered", &[("card", name.clone()), ("faction", self.seat_name(seat)), ("answer", word.to_string())]);
            // Ticket #353 (version 0.09.1): a rival's answer is `LineKind::Card`, which has no
            // headline rank, where it was an Event line at rank 7 and headlined a quiet turn. The
            // interface filtered it out of `headline()` afterwards and got `None` back, so the
            // dispatch opened with NO headline at all. The kind carries the rule now and the
            // interface's guard is gone. The player's own answer stays under Your works, which never
            // headlined either; the four are drawn together in their own block below.
            self.report_line_of(seat, LineKind::YourWorks, LineKind::Card, None, text);
        }
        // A seat that was never asked is named too, so the Report does not simply pass it over.
        for seat in Seat::ALL {
            if q.answers[seat.index()] == Some(CardAnswer::NothingToDecide) {
                let text = self.say(
                    "card_answered",
                    &[("card", name.clone()), ("faction", self.seat_name(seat)), ("answer", CardAnswer::NothingToDecide.word().to_string())],
                );
                self.report_line_of(seat, LineKind::YourWorks, LineKind::Card, None, text);
            }
        }
    }

    fn apply_card_effect(&mut self, seat: Seat, e: &CardEffect) {
        match e {
            CardEffect::Resources { materials, fuel, energy, ducats, research } => {
                let s = &mut self.seat_mut(seat).stockpile;
                s.materials = (s.materials + materials).max(0);
                s.fuel = (s.fuel + fuel).max(0);
                s.energy = (s.energy + energy).max(0);
                s.ducats = (s.ducats + ducats).max(0);
                if *research != 0 {
                    self.add_research_unattributed(*research);
                }
            }
            CardEffect::PerUnitCost { per, resource, amount } => {
                let bill = self.card_things(seat, *per) as i64 * amount;
                let s = &mut self.seat_mut(seat).stockpile;
                match resource {
                    Resource::Materials => s.materials = (s.materials - bill).max(0),
                    Resource::Fuel => s.fuel = (s.fuel - bill).max(0),
                    Resource::Energy => s.energy = (s.energy - bill).max(0),
                    Resource::Ducats => s.ducats = (s.ducats - bill).max(0),
                    Resource::Research | Resource::Widgets => {}
                }
            }
            CardEffect::PopulationToMostPopulous { population } => {
                if let Some(sid) = self.card_most_populous(seat) {
                    self.state_mut(sid).population += population;
                }
            }
            // The Methane Burst's own field: ppm charged at the next Climate phase, worldwide.
            CardEffect::EmissionsNext { ppm } => {
                self.climate.card_emissions_next += ppm;
            }
            CardEffect::StandingAllHeld { standing } => {
                for sid in self.controlled_states(seat) {
                    let e = self.seat_mut(seat).influence.entry(Place::State(sid)).or_insert(0);
                    *e = (*e + standing).max(0);
                }
            }
            CardEffect::StandingAtMostPopulous { standing } => {
                if let Some(sid) = self.card_most_populous(seat) {
                    let e = self.seat_mut(seat).influence.entry(Place::State(sid)).or_insert(0);
                    *e = (*e + standing).max(0);
                }
            }
            CardEffect::UnrestAllHeld { unrest } => {
                for sid in self.controlled_states(seat) {
                    self.card_unrest(sid, *unrest);
                }
            }
            CardEffect::UnrestAtMostPopulous { unrest } => {
                if let Some(sid) = self.card_most_populous(seat) {
                    self.card_unrest(sid, *unrest);
                }
            }
            CardEffect::UnrestAtBusiest { unrest } => {
                if let Some(sid) = self.card_busiest(seat) {
                    self.card_unrest(sid, *unrest);
                }
            }
            // Read before the transits (`resolve_transits`) and before the builds
            // (`resolve_builds`); by the time the Resolution reaches here they have already bitten.
            CardEffect::HoldShips | CardEffect::HoldOneShip | CardEffect::WidgetsNow { .. } => {}
            CardEffect::DamageShips { damage, in_orbit } => {
                let mut hit = 0;
                for s in self.ships.iter_mut().filter(|s| s.seat == seat && (!in_orbit || matches!(s.at, ShipAt::Body(_)))) {
                    s.damage += damage;
                    hit += 1;
                }
                let t = self.tables.clone();
                let destroyed: Vec<ShipId> = self.ships.iter().filter(|s| s.seat == seat && s.damage >= t.unit(s.kind).hit_points).map(|s| s.id).collect();
                for sid in destroyed {
                    self.destroy_ship(sid, "a card");
                }
                if hit > 0 {
                    self.log(format!("{}: {hit} Ship(s) damaged by this turn's card.", self.seat_name(seat)));
                }
            }
            CardEffect::TradePrice { resource, to, by, turns } => {
                let Some(row) = Game::market_row(*resource) else { return };
                // A card that MOVES a price moves the banded one, so two cards in three turns do
                // not compound one override onto another.
                let price = match to {
                    Some(p) => *p,
                    None => self.banded_price_at(row) + by,
                };
                self.market.card_price[row] = price.max(1);
                // The effect lands at this turn's Resolution, after this turn's trading, so the
                // turns it names are the ones that follow it.
                self.market.card_price_until[row] = self.turn + turns;
            }
            CardEffect::RelationsAllRivals { relations } => {
                let c = self.tables.relations.clone();
                for rival in Seat::ALL {
                    if rival == seat {
                        continue;
                    }
                    let (v, o) = (rival.index(), seat.index());
                    self.relations.score[v][o] = (self.relations.score[v][o] + relations).clamp(c.worst, c.deeds_ceiling);
                }
            }
            // The Smear and the Greenwash ledgers: ppm laid on, or taken off, for good.
            CardEffect::BlamePpm { ppm } => {
                if *ppm >= 0.0 {
                    self.seat_mut(seat).blame_smeared += ppm;
                } else {
                    self.seat_mut(seat).blame_cleaned += -ppm;
                }
            }
            CardEffect::FacilityOutputMultiplier { facility, multiplier } => {
                self.seat_mut(seat).card_facility = Some((*facility, *multiplier));
            }
            CardEffect::DiscoveryAtColony { module, multiplier, turns } => {
                if let Some(body) = self.card_discovery_body(seat, *module) {
                    self.discoveries.push(Discovery { body, kind: *module, multiplier: *multiplier, turns_left: *turns });
                }
            }
            CardEffect::PioneersFree { pioneers } => {
                if let Some(sid) = self.card_most_populous(seat) {
                    // Ticket #334's rule is deliberately not charged here: the card says the state
                    // pays no people for them, which is the whole of what it offers.
                    self.muster_emigrants(sid, *pioneers);
                }
            }
            CardEffect::FreeBuilding { module, army } => {
                if *army {
                    if let Some(sid) = self.card_most_populous(seat) {
                        let place = Place::State(sid);
                        self.raise_army(place, false);
                        self.war.armies_built[seat.index()] += 1;
                        self.log(format!("{}: an Army was raised free in {} by this turn's card.", self.seat_name(seat), self.tables.state(sid).name));
                    }
                } else if let Some(k) = module
                    && let Some(cid) = self.card_smallest_colony(seat, *k)
                {
                    if let Some(col) = self.colony_mut(cid) {
                        col.modules.push(Module::new(*k));
                    }
                    self.log(format!("{}: a free {} stands at {}.", self.seat_name(seat), k.name(), self.place_name(Place::Colony(cid))));
                }
            }
        }
    }

    /// Ticket #337: a card's move on a Region's Unrest, up through the damping or down flat.
    fn card_unrest(&mut self, sid: StateId, amount: f64) {
        if amount >= 0.0 {
            self.raise_unrest(sid, amount, UnrestSource::Plain);
        } else {
            self.lower_unrest(sid, -amount);
        }
    }

    /// Phase 5: take up the card the Question phase drew and apply it, exactly as before. The only
    /// change ticket #337 makes here is where the card came from: the deck was touched at the head
    /// of the turn rather than now, and a choice card was asked there and is answered by then.
    pub fn event_phase(&mut self) {
        let chance = self.draw_chance();
        let id = match self.draw {
            CardDraw::NoCard => {
                self.last_event = None;
                let text = format!("No Event this turn (a card comes {:.0}% of turns at this Temperature).", chance * 100.0);
                self.log(format!("Event: {text}"));
                self.report.event = Some(text);
                return;
            }
            CardDraw::DeckEmpty => {
                self.last_event = None;
                self.log("Event: the deck is empty.");
                self.report.event = Some("The Event Deck is empty.".to_string());
                return;
            }
            CardDraw::Choice(id) => {
                // Ticket #337: the question was asked at the head of the turn and the answers are
                // applied at Resolution (h). `last_event` stays empty, because nothing landed on
                // the table: what happened, happened to each seat by its own answer.
                self.last_event = None;
                let card = self.tables.event(id);
                self.report.event = Some(format!("{}: {}", card.name, card.choice.as_ref().map(|c| c.question.clone()).unwrap_or_default()));
                return;
            }
            CardDraw::Ordinary(id) => id,
        };
        let drawn = self.target_event(id);
        if matches!(drawn.target, EventTarget::None) {
            self.events_no_target += 1;
        }
        self.log(format!("Event: {}", drawn.text));
        self.report.event = Some(drawn.text.clone());
        let text = self.say("event_drawn", &[("text", drawn.text.clone())]);
        // Ticket #353 (version 0.09.1): `LineKind::Card`, as the question above it is. The Event
        // modal, headed "Event drawn", shows this exact sentence and waits for Continue; the driver
        // then printed it twice over, once as the headline and once as EVENT. It still reads under
        // The climate, and `report.event` still carries it where the interface wants it whole.
        self.report_line(LineKind::Card, None, text);
        self.last_event = Some(drawn);
    }

    pub fn climate_scale(&self) -> f64 {
        1.0 + (self.climate.temperature - self.tables.climate.base_temperature).max(0.0) / 2.0
    }

    fn target_event(&mut self, id: EventId) -> DrawnEvent {
        let t = self.tables.clone();
        let card = t.event(id);
        let scale = if card.kind == EventKind::Climate { self.climate_scale() } else { 1.0 };
        let (target, text) = match id {
            EventId::SolarStorm | EventId::RadiationSurge | EventId::CommsBlackout | EventId::MeteorShower => {
                (EventTarget::Everyone, format!("{}: {}.", card.name, card.effect))
            }
            EventId::SolarMaximum => {
                let m = if self.has_tech(TechId::EfficientGrids) { t.events.solar_maximum_multiplier_with_tech } else { t.events.solar_maximum_multiplier };
                (EventTarget::Everyone, format!("{}: every Power Plant and Generator makes x{} at the next Income.", card.name, m))
            }
            EventId::MethaneBurst => {
                let e = if self.has_tech(TechId::GreenConsensus) { t.events.methane_emissions / 2.0 } else { t.events.methane_emissions } * scale;
                (EventTarget::Everyone, format!("{}: +{:.1} Emissions next turn (x{:.2} at this Temperature).", card.name, e, scale))
            }
            EventId::LaunchPadFire => {
                let states: Vec<StateId> = StateId::ALL
                    .into_iter()
                    .filter(|s| self.state(*s).facilities.iter().any(|f| f.kind.does_the_job_of(FacilityKind::LaunchSite)))
                    .collect();
                match self.pick_uniform(&states) {
                    Some(s) => {
                        // Ticket #46: no Ship is built at a Launch Site now; the fire closes the lifts.
                        let what = if self.has_tech(TechId::CleanPropellant) { "its Launch Site stays open (Clean Propellant)".to_string() } else { "its Launch Site is offline until the next Resolution, so nothing lifts to orbit from there".to_string() };
                        (EventTarget::State(s), format!("{} in {}: {}.", card.name, t.state(s).name, what))
                    }
                    None => (EventTarget::None, format!("{}: no Launch Site stands anywhere, so nothing happens.", card.name)),
                }
            }
            EventId::LabourDispute => match self.pick_state_by_population() {
                Some(s) => {
                    let what = if self.has_tech(TechId::PublicScience) { "one of its Facilities makes nothing at the next Income (Public Science)" } else { "its Facilities make nothing at the next Income" };
                    (EventTarget::State(s), format!("{} in {}: {what}.", card.name, t.state(s).name))
                }
                None => (EventTarget::None, format!("{}: nobody lives anywhere, so nothing happens.", card.name)),
            },
            EventId::GridFailure => {
                let cols: Vec<ColonyId> = self.colonies.iter().filter(|c| !c.modules.is_empty()).map(|c| c.id).collect();
                match self.pick_uniform(&cols) {
                    Some(c) => {
                        let what = if self.has_tech(TechId::ClosedLoopColonies) { "no effect (Closed-Loop Colonies)" } else { "its Modules are offline until the next Resolution" };
                        (EventTarget::Colony(c), format!("{} at {}: {what}.", card.name, self.place_name(Place::Colony(c))))
                    }
                    None => (EventTarget::None, format!("{}: no Colony exists, so nothing happens.", card.name)),
                }
            }
            EventId::ReactorLeak => {
                let cols: Vec<ColonyId> = self.colonies.iter().filter(|c| c.modules.iter().any(|m| m.kind == ModuleKind::Generator)).map(|c| c.id).collect();
                match self.pick_uniform(&cols) {
                    Some(c) => {
                        let what = if self.has_tech(TechId::ClosedLoopColonies) { "no effect (Closed-Loop Colonies)".to_string() } else { format!("its Generators are offline until the next Resolution and its holder loses {} Energy", t.events.reactor_leak_energy) };
                        (EventTarget::Colony(c), format!("{} at {}: {what}.", card.name, self.place_name(Place::Colony(c))))
                    }
                    None => (EventTarget::None, format!("{}: no Colony has a Generator, so nothing happens.", card.name)),
                }
            }
            // Ticket #76: a Moonquake is the Moon's Dust Storm.
            EventId::DustStorm | EventId::Moonquake => {
                let body = if id == EventId::DustStorm { BodyId::Mars } else { BodyId::Moon };
                if self.colonies.iter().any(|c| c.body == body && !c.modules.is_empty()) {
                    let what = if self.has_tech(TechId::ClosedLoopColonies) { "no effect (Closed-Loop Colonies)".to_string() } else { format!("every Module on {} is offline until the next Resolution", t.body(body).name) };
                    (EventTarget::Body(body), format!("{}: {what}.", card.name))
                } else {
                    (EventTarget::None, format!("{}: nobody lives on {}, so nothing happens.", card.name, t.body(body).name))
                }
            }
            // Ticket #76: a Helium-3 Vein is the Moon's Rich Seam, for its Generators.
            EventId::HeliumVein => {
                if self.colonies.iter().any(|c| c.body == BodyId::Moon && c.modules.iter().any(|m| m.kind == ModuleKind::Generator)) {
                    let m = if self.has_tech(TechId::EfficientGrids) { t.events.discovery_multiplier_with_tech } else { t.events.discovery_multiplier };
                    (EventTarget::Body(BodyId::Moon), format!("{}: the Moon's Generators produce x{} for {} turns.", card.name, m, t.events.discovery_turns))
                } else {
                    (EventTarget::None, format!("{}: no Generator stands on the Moon, so nothing happens.", card.name))
                }
            }
            // Ticket #76: the one card that cools.
            EventId::VolcanicEruption => (EventTarget::Everyone, format!("{}: {:.1} ppm leave the CO2 Stock at once (x{:.2} at this Temperature).", card.name, t.events.volcanic_co2 * scale, scale)),
            // Ticket #76: a Drought halves a state's Facilities at the next Income.
            EventId::Drought => match self.pick_state_by_population() {
                Some(s) => {
                    let what = if self.has_tech(TechId::GreenConsensus) { "no effect (Green Consensus)".to_string() } else { format!("its Facilities make half at the next Income, and its Unrest rises by {}", Game::unrest_figure(t.events.drought_unrest)) };
                    (EventTarget::State(s), format!("{} in {}: {what}.", card.name, t.state(s).name))
                }
                None => (EventTarget::None, format!("{}: nobody lives anywhere, so nothing happens.", card.name)),
            },
            EventId::RichSeam | EventId::IceDeposit => {
                let kind = if id == EventId::RichSeam { ModuleKind::Mine } else { ModuleKind::Refinery };
                let bodies: Vec<BodyId> = BodyId::ALL
                    .into_iter()
                    .filter(|b| *b != BodyId::Earth)
                    .filter(|b| self.colonies.iter().any(|c| c.body == *b && c.modules.iter().any(|m| m.kind == kind)))
                    .collect();
                match self.pick_uniform(&bodies) {
                    Some(b) => {
                        let tech = if id == EventId::RichSeam { TechId::DeepMining } else { TechId::AutomatedRefining };
                        let m = if self.has_tech(tech) { t.events.discovery_multiplier_with_tech } else { t.events.discovery_multiplier };
                        (EventTarget::Body(b), format!("{} on {}: its {}s produce x{} for {} turns.", card.name, t.body(b).name, kind.name(), m, t.events.discovery_turns))
                    }
                    None => (EventTarget::None, format!("{}: no {} exists off Earth, so nothing happens.", card.name, kind.name())),
                }
            }
            EventId::Breakthrough => match self.research.current {
                Some(tech) => {
                    let r = if self.has_tech(TechId::PublicScience) { t.events.breakthrough_research_public_science } else { t.events.breakthrough_research };
                    (EventTarget::Tech, format!("{}: +{} Research toward {}.", card.name, r, t.tech(tech).name))
                }
                None => (EventTarget::None, format!("{}: no Tech is under research, so nothing happens.", card.name)),
            },
            EventId::Heatwave | EventId::Wildfire | EventId::Unrest => {
                let state = self.pick_state_by_population();
                match state {
                    Some(s) => {
                        let name = t.state(s).name.clone();
                        let text = match id {
                            EventId::Heatwave => {
                                let loss = if self.has_tech(TechId::GreenConsensus) { t.events.heatwave_loss_green_consensus } else { t.events.heatwave_loss };
                                format!("{} in {}: population -{:.1}% now (x{:.2} at this Temperature).", card.name, name, loss * scale * 100.0, scale)
                            }
                            EventId::Wildfire => {
                                let em = if self.has_tech(TechId::CleanManufacturing) { 0.0 } else { t.events.wildfire_emissions * scale };
                                format!("{} in {}: one Facility offline until next Resolution; +{:.1} Emissions next turn.", card.name, name, em)
                            }
                            // Ticket #52: the Unrest card is a flat rise in the state's Unrest.
                            _ => format!("{} in {}: its Unrest rises by {}.", card.name, name, t.events.unrest_card_unrest),
                        };
                        (EventTarget::State(s), text)
                    }
                    None => (EventTarget::None, format!("{}: nobody lives anywhere, so nothing happens.", card.name)),
                }
            }
            EventId::StormSurge => {
                let states: Vec<StateId> = StateId::ALL
                    .into_iter()
                    .filter(|s| t.state(*s).coastal_exposure > 0 && self.state(*s).thresholds_fired.iter().any(|f| !f))
                    .collect();
                match self.pick_uniform(&states) {
                    // Ticket #257 (version 0.08.4): on a walled state the wall holds, and the
                    // coastal Facilities make less at the next Income instead.
                    Some(s) if self.sea_wall_working(s) => {
                        let cut = ((1.0 - t.events.storm_surge_coastal_multiplier) * 100.0).round();
                        (EventTarget::State(s), format!("{} in {}: the Sea Wall holds; its coastal Facilities make {cut:.0}% less at the next Income.", card.name, t.state(s).name))
                    }
                    Some(s) => (EventTarget::State(s), format!("{} in {}: its next sea-level threshold applies now.", card.name, t.state(s).name)),
                    None => (EventTarget::None, format!("{}: no exposed coast has a threshold ahead, so nothing happens.", card.name)),
                }
            }
            // Ticket #337 (version 0.09.0): a choice card never reaches here. The Question phase
            // takes it at the head of the turn and the Event phase returns before targeting. The
            // eighteen are written out rather than caught by a wildcard, so a card added to the
            // deck without an arm of its own is still a compile error, as it has always been.
            EventId::RefugeeConvoy
            | EventId::GroundedFleet
            | EventId::CheapOreOffer
            | EventId::OvertimeAtTheYards
            | EventId::TheAuditors
            | EventId::SalvageRights
            | EventId::FuelContract
            | EventId::TheHardWinter
            | EventId::DistressCall
            | EventId::StrikeAtTheRefineries
            | EventId::DeepSurvey
            | EventId::EmergencyShutdown
            | EventId::TheRecruiters
            | EventId::CarbonOffsetScheme
            | EventId::OrbitalDebris
            | EventId::TheWhistleblower
            | EventId::SurplusHabitats
            | EventId::ConscriptionNotice => (EventTarget::None, format!("{}: {}.", card.name, card.effect)),
        };
        DrawnEvent { card: Card::Event(id), target, scale, text }
    }

    fn pick_uniform<T: Copy>(&mut self, list: &[T]) -> Option<T> {
        if list.is_empty() {
            None
        } else {
            Some(list[self.rng.random_range(0..list.len())])
        }
    }

    fn pick_state_by_population(&mut self) -> Option<StateId> {
        let total: f64 = self.states.iter().map(|s| s.population).sum();
        if total <= 0.0 {
            return None;
        }
        let mut r = self.rng.random_range(0.0..total);
        for s in &self.states {
            if r < s.population {
                return Some(s.id);
            }
            r -= s.population;
        }
        self.states.iter().rev().find(|s| s.population > 0.0).map(|s| s.id)
    }

    /// Ticket #52: a Heatwave, a Wildfire or a Storm Surge landing on a state raises its Unrest
    /// as a climate source, so the green Techs and a Constabulary damp it.
    fn climate_card_unrest(&mut self, s: StateId) {
        let n = self.tables.unrest.climate_card;
        self.climate_unrest_by(s, n);
    }

    /// Ticket #76: the same rise by a card's own figure (a Drought's).
    fn climate_unrest_by(&mut self, s: StateId, n: f64) {
        let rose = self.raise_unrest(s, n, UnrestSource::Climate);
        if rose > 0.0 {
            let line = format!("{}: Unrest rose by {} to {}.", self.tables.state(s).name, Game::unrest_figure(rose), self.unrest_text(s));
            self.log(line);
            let text = self.say(
                "unrest_rose_state",
                &[("state", self.tables.state(s).name.clone()), ("rose", Game::unrest_figure(rose).to_string()), ("unrest", self.unrest_text(s))],
            );
            self.report_line(LineKind::Unrest, Some(ReportPlace::State(s)), text);
        }
    }

    /// Whether this turn's card is the named Event.
    pub fn event_is(&self, id: EventId) -> bool {
        matches!(&self.last_event, Some(DrawnEvent { card: Card::Event(e), target, .. }) if *e == id && *target != EventTarget::None)
    }

    /// Resolution (h): the card effects that act now, in card text order.
    pub fn apply_event_now(&mut self) {
        let Some(ev) = self.last_event.clone() else { return };
        let t = self.tables.clone();
        let Card::Event(id) = ev.card;
        match (id, ev.target) {
            (EventId::RadiationSurge, EventTarget::Everyone) | (EventId::MeteorShower, EventTarget::Everyone) => {
                if !self.has_tech(TechId::HardenedHulls) {
                    let in_transit = id == EventId::RadiationSurge;
                    let dmg = if in_transit { 1 } else { t.events.meteor_damage };
                    let mut hit = 0;
                    for s in self.ships.iter_mut().filter(|s| matches!(s.at, ShipAt::Transit { .. }) == in_transit) {
                        s.damage += dmg;
                        hit += 1;
                    }
                    let destroyed: Vec<ShipId> = self.ships.iter().filter(|s| s.damage >= t.unit(s.kind).hit_points).map(|s| s.id).collect();
                    for sid in destroyed {
                        self.destroy_ship(sid, t.event(id).name.as_str());
                    }
                    if hit > 0 {
                        let text = self.say("event_damaged_ships", &[("event", t.event(id).name.clone()), ("n", hit.to_string())]);
                        self.report_line(LineKind::Ship, None, text);
                    }
                }
            }
            (EventId::SolarMaximum, EventTarget::Everyone) => {
                self.solar_maximum_next = true;
            }
            (EventId::MethaneBurst, EventTarget::Everyone) => {
                let e = if self.has_tech(TechId::GreenConsensus) { t.events.methane_emissions / 2.0 } else { t.events.methane_emissions };
                self.climate.card_emissions_next += e * ev.scale;
            }
            (EventId::GridFailure, EventTarget::Colony(c)) => {
                let immune = self.has_tech(TechId::ClosedLoopColonies);
                if let Some(col) = self.colony_mut(c).filter(|_| !immune) {
                    col.grid_failed = true;
                    for m in &mut col.modules {
                        m.online = false;
                    }
                }
            }
            (EventId::ReactorLeak, EventTarget::Colony(c)) => {
                if !self.has_tech(TechId::ClosedLoopColonies) {
                    let holder = self.colony(c).and_then(|c| c.control.director());
                    if let Some(col) = self.colony_mut(c) {
                        for m in col.modules.iter_mut().filter(|m| m.kind == ModuleKind::Generator) {
                            m.online = false;
                            m.offline_until_resolution = true;
                        }
                    }
                    if let Some(h) = holder {
                        let s = &mut self.seat_mut(h).stockpile;
                        s.energy = (s.energy - t.events.reactor_leak_energy).max(0);
                    }
                }
            }
            (EventId::DustStorm, EventTarget::Body(b)) | (EventId::Moonquake, EventTarget::Body(b)) => {
                if !self.has_tech(TechId::ClosedLoopColonies) {
                    for col in self.colonies.iter_mut().filter(|c| c.body == b) {
                        col.grid_failed = true;
                        for m in &mut col.modules {
                            m.online = false;
                        }
                    }
                }
            }
            (EventId::RichSeam, EventTarget::Body(b)) | (EventId::IceDeposit, EventTarget::Body(b)) => {
                let kind = if id == EventId::RichSeam { ModuleKind::Mine } else { ModuleKind::Refinery };
                let tech = if id == EventId::RichSeam { TechId::DeepMining } else { TechId::AutomatedRefining };
                let m = if self.has_tech(tech) { t.events.discovery_multiplier_with_tech } else { t.events.discovery_multiplier };
                self.discoveries.push(Discovery { body: b, kind, multiplier: m, turns_left: t.events.discovery_turns });
            }
            // Ticket #76: the Moon's Generators, as Rich Seam does for a Body's Mines.
            (EventId::HeliumVein, EventTarget::Body(b)) => {
                let m = if self.has_tech(TechId::EfficientGrids) { t.events.discovery_multiplier_with_tech } else { t.events.discovery_multiplier };
                self.discoveries.push(Discovery { body: b, kind: ModuleKind::Generator, multiplier: m, turns_left: t.events.discovery_turns });
            }
            // Ticket #76: the one card that cools, scaled like every Climate card.
            (EventId::VolcanicEruption, EventTarget::Everyone) => {
                self.climate.co2 = (self.climate.co2 - t.events.volcanic_co2 * ev.scale).max(0.0);
            }
            // Ticket #76: a Drought, unless Green Consensus blunts it.
            (EventId::Drought, EventTarget::State(s)) => {
                if !self.has_tech(TechId::GreenConsensus) {
                    self.state_mut(s).drought = true;
                    self.climate_unrest_by(s, t.events.drought_unrest);
                }
            }
            (EventId::Breakthrough, EventTarget::Tech) => {
                let r = if self.has_tech(TechId::PublicScience) { t.events.breakthrough_research_public_science } else { t.events.breakthrough_research };
                self.add_research_unattributed(r);
            }
            (EventId::Heatwave, EventTarget::State(s)) => {
                let loss = if self.has_tech(TechId::GreenConsensus) { t.events.heatwave_loss_green_consensus } else { t.events.heatwave_loss };
                let st = self.state_mut(s);
                st.population = (st.population * (1.0 - loss * ev.scale)).max(0.0);
                // Ticket #52: a Heatwave is one of the three Climate cards that raise Unrest.
                self.climate_card_unrest(s);
            }
            (EventId::LaunchPadFire, EventTarget::State(s)) => {
                if self.has_tech(TechId::CleanPropellant) {
                    return;
                }
                let st = self.state_mut(s);
                for f in st.facilities.iter_mut().filter(|f| f.kind.does_the_job_of(FacilityKind::LaunchSite)) {
                    f.offline_until_resolution = true;
                    f.online = false;
                }
            }
            (EventId::LabourDispute, EventTarget::State(s)) => {
                let n = self.state(s).facilities.len();
                if n == 0 {
                    return;
                }
                if self.has_tech(TechId::PublicScience) {
                    let i = self.rng.random_range(0..n);
                    let st = self.state_mut(s);
                    st.facilities[i].offline_until_resolution = true;
                    st.facilities[i].online = false;
                } else {
                    for f in &mut self.state_mut(s).facilities {
                        f.offline_until_resolution = true;
                        f.online = false;
                    }
                }
            }
            (EventId::Wildfire, EventTarget::State(s)) => {
                let n = self.state(s).facilities.len();
                if n > 0 {
                    let i = self.rng.random_range(0..n);
                    let st = self.state_mut(s);
                    st.facilities[i].offline_until_resolution = true;
                    st.facilities[i].online = false;
                }
                if !self.has_tech(TechId::CleanManufacturing) {
                    self.state_mut(s).wildfire_emissions_next += t.events.wildfire_emissions * ev.scale;
                }
                // Ticket #52: a Wildfire is one of the three Climate cards that raise Unrest.
                self.climate_card_unrest(s);
            }
            // Ticket #52: the card is a flat rise in the state's Unrest, damped by nothing.
            (EventId::Unrest, EventTarget::State(s)) => {
                let n = t.events.unrest_card_unrest;
                let rose = self.raise_unrest(s, n, UnrestSource::Plain);
                if rose > 0.0 {
                    let line = format!("Unrest in {}: its Unrest rose by {} to {}.", t.state(s).name, Game::unrest_figure(rose), self.unrest_text(s));
                    self.log(line);
                    let text = self.say(
                        "unrest_card",
                        &[("state", t.state(s).name.clone()), ("rose", Game::unrest_figure(rose).to_string()), ("unrest", self.unrest_text(s))],
                    );
                    self.report_line(LineKind::Unrest, Some(ReportPlace::State(s)), text);
                }
            }
            (EventId::StormSurge, EventTarget::State(s)) => {
                // Ticket #257 (version 0.08.4): a standing, working Sea Wall holds the surge as it
                // holds a threshold, and the state's coastal Facilities make less at the next Income.
                if self.sea_wall_working(s) {
                    self.state_mut(s).storm_surge = true;
                    let cut = ((1.0 - self.tables.events.storm_surge_coastal_multiplier) * 100.0).round();
                    let name = self.tables.state(s).name.clone();
                    self.log(format!("Storm Surge in {name}: the Sea Wall held; its coastal Facilities make {cut:.0}% less at the next Income."));
                    let text = self.say("storm_surge_wall", &[("state", name), ("percent", format!("{cut:.0}"))]);
                    self.report_line(LineKind::Event, Some(ReportPlace::State(s)), text);
                } else if let Some(i) = self.state(s).thresholds_fired.iter().position(|f| !f) {
                    self.apply_sea_threshold(s, i);
                }
                // Ticket #52: a Storm Surge is one of the three Climate cards that raise Unrest,
                // on top of what the threshold it brings forward costs in build slots.
                self.climate_card_unrest(s);
            }
            _ => {}
        }
    }
}
