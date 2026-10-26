//! Ticket #634 (version 0.1.0.6), at the designer's word: **war over Colonies made legible**. A
//! Colony's or station's threats, as its holder reads them: rival warships in any orbit of its Body,
//! a rival Army aboard a Carrier there, rival warships or loaded Carriers bound for it, and a
//! rival's Standing there at half its price or more. An Accord partner under non-aggression is no
//! threat, and a Ship the reader cannot see is never named.

use crate::ids::*;
use crate::state::*;

/// One thing threatening a Colony, ordered as the line reads them.
#[derive(Debug, Clone, PartialEq)]
pub enum Threat {
    /// Rival warships in one orbit of the Colony's Body.
    Warships { seat: Seat, orbit: Orbit, ships: Vec<ShipId> },
    /// Rival Armies aboard Carriers in one orbit of the Body.
    Landing { seat: Seat, orbit: Orbit, ships: Vec<ShipId> },
    /// Rival warships and loaded Carriers bound for the Body, arriving in this many turns.
    Inbound { seat: Seat, turns: u32, warships: Vec<ShipId>, carriers: Vec<ShipId> },
    /// A rival's Standing at the Colony, against the price it would pay.
    Standing { seat: Seat, standing: i64, price: i64 },
}

impl Threat {
    fn ships(&self) -> Vec<ShipId> {
        match self {
            Threat::Warships { ships, .. } | Threat::Landing { ships, .. } => ships.clone(),
            Threat::Inbound { warships, carriers, .. } => warships.iter().chain(carriers).copied().collect(),
            Threat::Standing { .. } => Vec::new(),
        }
    }
}

impl Game {
    /// The Faction's name as an adjective: "Prospector", "Arkwright".
    fn faction_adjective(&self, seat: Seat) -> String {
        let name = self.seat_name(seat);
        name.strip_suffix('s').unwrap_or(&name).to_string()
    }

    /// Ticket #634: the seat the threats are read for -- the Colony's holder, the one that formally
    /// controls it, so an Occupied Colony's threats are its holder's and not the occupier's (from the
    /// review); the occupier where there is no holder to speak for.
    pub fn threat_holder(&self, cid: ColonyId) -> Option<Seat> {
        let c = self.colony(cid)?.control;
        c.controller().or(c.director())
    }

    /// Ticket #634: what threatens this Colony, as `viewer` can see it, for its holder. Nothing for a
    /// Colony nobody holds.
    pub fn colony_threats(&self, viewer: Seat, cid: ColonyId) -> Vec<Threat> {
        let Some(col) = self.colony(cid) else { return Vec::new() };
        let Some(holder) = self.threat_holder(cid) else { return Vec::new() };
        let (body, own) = (col.body, self.colony_orbit(col));
        let rival = |s: Seat| s != holder && !self.accord_has(holder, s, Term::NonAggression);
        // From the review: a Ship that fled a Battle threatens nothing, as every warship rule reads it.
        let seen: Vec<&Ship> = self.ships.iter().filter(|s| rival(s.seat) && !s.escaped && self.sees_ship(viewer, s)).collect();
        let mut out = Vec::new();
        // Q4: the orbit that touches the place first, then the others in their order.
        let mut orbits = self.orbits_of(body);
        orbits.sort_by_key(|o| *o != own);
        for orbit in orbits {
            for seat in Seat::ALL {
                let here = |s: &&&Ship| s.seat == seat && s.at == ShipAt::Body(body) && self.ship_orbit(s) == orbit;
                let ships: Vec<ShipId> = seen.iter().filter(here).filter(|s| s.army.is_some()).map(|s| s.id).collect();
                if !ships.is_empty() {
                    out.push(Threat::Landing { seat, orbit, ships });
                }
                let ships: Vec<ShipId> = seen.iter().filter(here).filter(|s| s.kind.is_warship()).map(|s| s.id).collect();
                if !ships.is_empty() {
                    out.push(Threat::Warships { seat, orbit, ships });
                }
            }
        }
        // Bound for the Body, soonest first, warships and loaded Carriers counted apart (the review).
        let mut inbound: Vec<(u32, Seat, &Ship)> = seen
            .iter()
            .filter_map(|s| match s.at {
                ShipAt::Transit { to, turns_left, .. } if to == body && (s.kind.is_warship() || s.army.is_some()) => Some((turns_left, s.seat, *s)),
                _ => None,
            })
            .collect();
        inbound.sort_by_key(|(t, seat, s)| (*t, seat.index(), s.id.0));
        for (turns, seat, s) in inbound {
            let at = match out.iter().position(|t| matches!(t, Threat::Inbound { seat: x, turns: y, .. } if *x == seat && *y == turns)) {
                Some(i) => i,
                None => {
                    out.push(Threat::Inbound { seat, turns, warships: Vec::new(), carriers: Vec::new() });
                    out.len() - 1
                }
            };
            if let Threat::Inbound { warships, carriers, .. } = &mut out[at] {
                if s.army.is_some() { carriers.push(s.id) } else { warships.push(s.id) }
            }
        }
        let place = Place::Colony(cid);
        for seat in Seat::ALL.into_iter().filter(|s| rival(*s)) {
            let standing = self.seat(seat).influence.get(&place).copied().unwrap_or(0);
            let price = self.influence_needed_for(seat, place);
            if standing > 0 && standing * 2 >= price {
                out.push(Threat::Standing { seat, standing, price });
            }
        }
        out
    }

    /// Ticket #634, Q6: whether anything threatens this Colony, for the roster's mark.
    pub fn colony_threatened(&self, viewer: Seat, cid: ColonyId) -> bool {
        !self.colony_threats(viewer, cid).is_empty()
    }

    fn threat_words(&self, cid: ColonyId, t: &Threat) -> String {
        let body = self.colony(cid).map(|c| c.body).unwrap_or(BodyId::Earth);
        let where_ = |o: &Orbit| match o {
            Orbit::Low => "in low orbit".to_string(),
            Orbit::Slot(n) => format!("at {}", self.station_name(body, *n)),
        };
        let some = |n: usize, one: &str, many: &str, adj: &str| {
            let adj = if adj.is_empty() { String::new() } else { format!("{adj} ") };
            if n == 1 { format!("{} {adj}{one}", if adj.starts_with(['A', 'E', 'I', 'O', 'U']) || (adj.is_empty() && one.starts_with(['A', 'E', 'I', 'O', 'U'])) { "an" } else { "a" }) } else { format!("{n} {adj}{many}") }
        };
        match t {
            Threat::Warships { seat, orbit, ships } => format!("{} {}", some(ships.len(), "warship", "warships", &self.faction_adjective(*seat)), where_(orbit)),
            Threat::Landing { seat, orbit, ships } => {
                let aboard = if ships.len() == 1 { "aboard a Carrier" } else { "aboard Carriers" };
                format!("{} {aboard} {}", some(ships.len(), "Army", "Armies", &self.faction_adjective(*seat)), where_(orbit))
            }
            Threat::Inbound { seat, turns, warships, carriers } => {
                let adj = self.faction_adjective(*seat);
                let what = match (warships.len(), carriers.len()) {
                    (0, n) => some(n, "Army", "Armies", &adj),
                    (n, 0) => some(n, "warship", "warships", &adj),
                    (w, n) => format!("{} and {}", some(w, "warship", "warships", &adj), some(n, "Army", "Armies", "")),
                };
                format!("{what} arriving in {}", crate::counted(turns, "turn", "turns"))
            }
            Threat::Standing { seat, standing, price } => format!("{} at {standing} of {price} Standing", self.seat_name(*seat)),
        }
    }

    /// Ticket #634, Q3: the card's one line -- *Threats: 2 Prospector warships in low orbit; ...* --
    /// or nothing where nothing threatens (Q2).
    pub fn threat_line(&self, viewer: Seat, cid: ColonyId) -> Option<String> {
        Self::threat_line_of(self, cid, &self.colony_threats(viewer, cid))
    }

    /// The line from threats already read, so a card that wants the hover too reads them once.
    pub fn threat_line_of(&self, cid: ColonyId, threats: &[Threat]) -> Option<String> {
        if threats.is_empty() {
            return None;
        }
        let parts: Vec<String> = threats.iter().map(|t| self.threat_words(cid, t)).collect();
        Some(format!("Threats: {}.", parts.join("; ")))
    }

    /// Ticket #634, Q3: the line's hover -- each hull by name and strength, a Carrier's by the Army
    /// aboard it (from the review), within six lines.
    pub fn threat_hover(&self, viewer: Seat, cid: ColonyId) -> Vec<String> {
        self.threat_hover_of(&self.colony_threats(viewer, cid))
    }

    pub fn threat_hover_of(&self, threats: &[Threat]) -> Vec<String> {
        let mut lines: Vec<String> = threats
            .iter()
            .flat_map(|t| t.ships())
            .filter_map(|id| self.ship(id))
            .map(|s| match s.army.and_then(|a| self.army(a)) {
                Some(a) => format!("{} ({}), {} aboard, strength {}", self.ship_name(s), s.kind.name().to_lowercase(), self.army_name(a), self.army_strength(a)),
                None => format!("{} ({}), strength {}", self.ship_name(s), s.kind.name().to_lowercase(), self.ship_fighting_strength(s)),
            })
            .collect();
        if lines.len() > 6 {
            let more = lines.len() - 5;
            lines.truncate(5);
            lines.push(format!("and {more} more"));
        }
        lines
    }
}
