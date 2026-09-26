//! A headless driver for seat 0, so an agent (or a person at a terminal) can play a whole game
//! without a window: one process per command, the whole game kept in a save file between them.
//!
//!   cargo run -q -p dying-earth-engine --example play -- new --save G.ron --seed 7 \
//!       --faction custodians --start europe
//!   cargo run -q -p dying-earth-engine --example play -- show --save G.ron
//!   cargo run -q -p dying-earth-engine --example play -- turn --save G.ron --orders turn.txt
//!   cargo run -q -p dying-earth-engine --example play -- help
//!
//! `turn` reads a plain-text order list, refuses the whole list if any line is illegal (unless
//! `--force`), commits what is left, ends the turn with the three AI seats ordering for
//! themselves, and prints the new Report and the board.
//!
//! Ticket #337 (version 0.09.0): a Choice Card is drawn at the HEAD of the turn, before orders, and
//! the turn cannot end while this seat owes it an answer. `show` prints the question; an
//! `answer take` or `answer refuse` line in the order list answers it.

use dying_earth_engine::data::{default_data_dir, CardEffect, CardThing, Tables};
use dying_earth_engine::ids::*;
use dying_earth_engine::orders::{BuildingRef, LoadSource, Order, UnitRef, UnloadTarget};
use dying_earth_engine::save::{self, SaveKind};
use dying_earth_engine::state::*;
use std::path::{Path, PathBuf};
use std::sync::Arc;

// ------------------------------------------------------------------ parsing helpers

/// Lowercase with every separator taken out, so "sea_wall", "Sea Wall" and "sea-wall" are one word.
fn norm(s: &str) -> String {
    s.chars().filter(|c| c.is_ascii_alphanumeric()).collect::<String>().to_ascii_lowercase()
}

/// Match a word against a list of ids by their `Debug` spelling: exact first, then a unique prefix.
fn pick<T: Copy + std::fmt::Debug>(list: &[T], word: &str) -> Result<T, String> {
    let w = norm(word);
    if w.is_empty() {
        return Err("expected a name here".into());
    }
    let names: Vec<String> = list.iter().map(|v| norm(&format!("{v:?}"))).collect();
    if let Some(i) = names.iter().position(|n| *n == w) {
        return Ok(list[i]);
    }
    let hits: Vec<usize> = names.iter().enumerate().filter(|(_, n)| n.starts_with(&w)).map(|(i, _)| i).collect();
    match hits.len() {
        1 => Ok(list[hits[0]]),
        0 => Err(format!("{word:?} is not one of: {}", names.join(", "))),
        _ => Err(format!(
            "{word:?} is short for more than one of: {}",
            hits.iter().map(|i| names[*i].clone()).collect::<Vec<_>>().join(", ")
        )),
    }
}

const RESOURCES: [Resource; 5] = [Resource::Materials, Resource::Fuel, Resource::Energy, Resource::Research, Resource::Ducats];
// Ticket #278 (version 0.08.5) and ticket #297 (version 0.08.6): a Blockade is a STANCE a Ship
// stack is given, and Dig In is an Army's. The driver offered neither, so seat 0 could not give a
// Blockade at all -- the very move ticket #335 handed the player an orbit to give it from.
const STANCES: [Stance; 6] = [Stance::Attack, Stance::Hold, Stance::Intercept, Stance::Evade, Stance::Blockade, Stance::DigIn];
const CHANGES: [BuildingChange; 3] = [BuildingChange::Mothball, BuildingChange::Restart, BuildingChange::Decommission];
// Ticket #226 (version 0.08.2): the four Terms an Accord may hold. The driver could name none of
// them, so the whole of the diplomacy -- the one system a seat cannot reach alone -- was invisible
// from here and would have been reported as absent from the game.
const TERMS: [Term; 4] = [Term::NonAggression, Term::Passage, Term::Refuel, Term::ResearchAgreement];

fn number(word: &str) -> Result<i64, String> {
    word.parse::<i64>().map_err(|_| format!("{word:?} is not a whole number"))
}

fn count(word: &str) -> Result<u32, String> {
    word.parse::<u32>().map_err(|_| format!("{word:?} is not a whole number"))
}

/// A whole percent, written plainly or with its sign: `25` or `25%`. Ticket #235 (version 0.08.3):
/// the Research Directive has been a per-percent figure since, and the driver offered only nought
/// and a hundred -- the two positions the order had BEFORE that ticket.
fn percent(word: &str) -> Result<u8, String> {
    let n = number(word.trim_end_matches('%'))?;
    if !(0..=100).contains(&n) {
        return Err(format!("{word:?} is not a whole percent from 0 to 100"));
    }
    Ok(n as u8)
}

/// A Faction at the table: a seat number (`2`, `seat:2`), or the Faction's own name shortened as far
/// as it stays unique (`ark` is the Arkwrights). Which seat holds which Faction is a thing of THIS
/// game, so the word is matched against the game's own table rather than a fixed list.
fn seat_of(g: &Game, word: &str) -> Result<Seat, String> {
    let lower = word.to_ascii_lowercase();
    let w = lower.trim_start_matches("seat").trim_start_matches(':');
    if let Ok(n) = w.parse::<u8>() {
        if (n as usize) < SEAT_COUNT {
            return Ok(Seat(n));
        }
        return Err(format!("{word:?} is not a seat: there are {SEAT_COUNT} at the table"));
    }
    let kinds: Vec<FactionKind> = Seat::ALL.iter().map(|s| g.kind(*s)).collect();
    let kind = pick(&kinds, w)?;
    Ok(Seat::ALL[kinds.iter().position(|k| *k == kind).unwrap_or(0)])
}

/// `europe`, `state:europe`, `colony:3`, `c:3` or a bare colony number.
fn place(word: &str) -> Result<Place, String> {
    let w = word.to_ascii_lowercase();
    if let Some(rest) = w.strip_prefix("colony:").or_else(|| w.strip_prefix("c:")) {
        return Ok(Place::Colony(ColonyId(count(rest)?)));
    }
    if let Some(rest) = w.strip_prefix("state:").or_else(|| w.strip_prefix("s:")) {
        return Ok(Place::State(pick(&StateId::ALL, rest)?));
    }
    if let Ok(n) = w.parse::<u32>() {
        return Ok(Place::Colony(ColonyId(n)));
    }
    Ok(Place::State(pick(&StateId::ALL, &w)?))
}

/// `ship:4`, `army:2`, `ship4`, `army2`, or -- ticket #324 (version 0.08.8) -- `battery:<colony>:<n>`
/// for a Colony's Battery, which a Battle damages and a Repair may name, and which the driver could
/// not write at all.
fn unit_ref(word: &str) -> Result<UnitRef, String> {
    let w = word.to_ascii_lowercase();
    if let Some(rest) = w.strip_prefix("battery") {
        let mut parts = rest.trim_start_matches(':').split(':');
        let colony = colony_id(parts.next().unwrap_or(""))?;
        let index = count(parts.next().ok_or("a Battery is battery:<colony>:<index>")?)? as usize;
        return Ok(UnitRef::Battery { colony, index });
    }
    if let Some(rest) = w.strip_prefix("ship") {
        return Ok(UnitRef::Ship(ShipId(count(rest.trim_start_matches(':'))?)));
    }
    if let Some(rest) = w.strip_prefix("army") {
        return Ok(UnitRef::Army(ArmyId(count(rest.trim_start_matches(':'))?)));
    }
    Err(format!("{word:?} should be ship:<n>, army:<n> or battery:<colony>:<index>"))
}

fn ship_id(word: &str) -> Result<ShipId, String> {
    Ok(ShipId(count(word.to_ascii_lowercase().trim_start_matches("ship").trim_start_matches(':'))?))
}

fn army_id(word: &str) -> Result<ArmyId, String> {
    Ok(ArmyId(count(word.to_ascii_lowercase().trim_start_matches("army").trim_start_matches(':'))?))
}

fn colony_id(word: &str) -> Result<ColonyId, String> {
    let w = word.to_ascii_lowercase();
    let w = w.trim_start_matches("colony").trim_start_matches('c').trim_start_matches(':');
    Ok(ColonyId(count(w)?))
}

/// Ticket #335 (version 0.09.0): an ORBIT as an order names it -- `low` (or nothing at all) for low
/// orbit, a number for that Orbital Slot's ring. The shape `Transit` and `ChangeOrbit` both carry.
fn orbit_slot(word: &str) -> Result<Option<u32>, String> {
    let w = word.to_ascii_lowercase();
    if w == "low" || w == "-" {
        return Ok(None);
    }
    Ok(Some(count(&w)?))
}

/// `slot <body> <n>` or `colony <n>`, as an unload or an Antarctic landing wants it.
fn unload_target(words: &[&str]) -> Result<UnloadTarget, String> {
    match words.first().map(|w| w.to_ascii_lowercase()).as_deref() {
        Some("slot") => {
            let body = pick(&BodyId::ALL, words.get(1).ok_or("slot needs a Body")?)?;
            let n = count(words.get(2).ok_or("slot needs a number")?)?;
            Ok(UnloadTarget::Slot(body, n))
        }
        Some("colony") => Ok(UnloadTarget::Colony(colony_id(words.get(1).ok_or("colony needs a number")?)?)),
        _ => Err("expected `slot <body> <n>` or `colony <n>`".into()),
    }
}

const GRAMMAR: &str = r#"ORDER LINES (one per line; `#` starts a comment; blank lines are skipped)

  answer take                          answer THIS TURN'S CHOICE CARD: take the offer,
  answer refuse                        or refuse it.
                                       Ticket #337 (version 0.09.0): a card is drawn at the HEAD of
                                       the turn, before orders, and the turn CANNOT END while this
                                       seat owes it an answer. Ticket #367 (0.09.2): never on turn 1. `show` prints the card, its question
                                       and what each side does; a seat that cannot pay the offer may
                                       only refuse.

  tech <tech>                          pick the next shared Tech (when the board says one is awaited)

  build facility <state> <facility>    e.g. build facility europe factory
  build facility-ducats <state> <facility>
  build module <colony> <module>       e.g. build module 1 habitat
  build module-ducats <colony> <module>
  build ship <colony> <ship>           at a Colony or station of yours with a WORKING SHIPYARD
                                       (ticket #46): never in a Nation State, whatever its Launch
                                       Site. The new Ship starts in the orbit of the yard that built
                                       it -- a station's own orbit, or low orbit for a ground Colony.
  build army <place>                   a Nation State you control, or a Colony with a Barracks.
                                       Materials and Widgets AND PEOPLE (ticket #334, version
                                       0.09.0): a Region loses population for it, a Colony a
                                       Colonist, and a place without the people to spare is refused.
  build station <body> <slot>          e.g. build station moon 0; Materials alone, no Widgets
  build archive <colony>               the Archivists only, at a Colony OFF EARTH (neither Antarctica
                                       nor a station over Earth), where four Colonists already live.
                                       Both are checked at the order and never again. The Upload
                                       gates the Archivists' WIN, not this order (ticket #361,
                                       version 0.09.1, undoing ticket #199).
  industry <state>                     raise the Industry Level
  cancel-build <place> <index>         cancel a build ANOTHER seat began at a place you now direct,
                                       which is what a conquest leaves behind: its Materials come
                                       back at your own price and the Widgets done are lost. One a
                                       turn at a place, and never a build of your own -- take that
                                       back out of this turn's order list instead. The index is the
                                       number `show` prints beside the item in that place's
                                       `building:` list.

  A BUILD HAS NO TURN COUNT (ticket #332, version 0.09.0). Materials are paid in full at the order,
  and a build needs a figure in WIDGETS, which the place makes at its own rate every turn and spends
  on its queue in the order the builds were given. `show` prints each place's Widgets a turn, what
  its queue owes, and every build as `done/needed w`. The `-ducats` forms buy a building outright
  at twice its Materials and complete at the next Resolution whatever their Widget figure.

  transit <ship> <body> [slot|low]     fly, spending the Ship's own tank. The leg names the ORBIT it
                                       ends in: a slot number for that Orbital Slot's ring, or `low`
                                       (or nothing at all) for low orbit.
  change-orbit <ship> <slot|low>       move a Ship between the orbits of the Body it ALREADY stands
                                       at, for {fuel} Fuel out of its own tank. Resolved with the
                                       transits, before the Battles, so a Ship that changes orbit
                                       fights in its new orbit.
  refuel <ship>                        fill the tank at a station of yours (or of a Refuel partner's)
                                       AT THAT STATION'S OWN ORBIT: a Ship in low orbit, or at
                                       another station's ring, refuels at nothing until it moves.
  load <ship> <n> from <place> [army <army>]
  unload <ship> <n> [army] into slot <body> <slot>
  unload <ship> <n> [army] into colony <n>
  move-army <army> <state>
  ship-stance <body> <stance>          stance = attack|hold|intercept|evade|blockade
  army-stance <place> <stance>         stance = attack|evade|digin (hold does nothing to an Army)
  repair <unit> <points>               unit = ship:<n>, army:<n> or battery:<colony>:<index>
  repair-ducats <unit> <points>

  A BODY'S ORBITS (ticket #335, version 0.09.0) are LOW ORBIT and one per Orbital Slot, and every
  Ship at a Body sits in exactly one of them; there is no Body at large. LOW ORBIT is what touches
  the ground: landing an Army at a ground Colony, unloading Colonists into one, founding one, and
  Bombarding one. A STATION'S OWN ORBIT is what touches that station: unloading into it, refuelling
  at it, blockading it, attacking it. A LIFT FROM A LAUNCH SITE reaches ANY orbit of Earth (ticket
  #357, version 0.09.1). A refusal that a change of orbit would cure names the move first. Orbital Control is
  of LOW orbit and gates the ground. A BLOCKADE IS A STANCE, chosen (`ship-stance <body> blockade`)
  and shutting the orbit the stack sits in -- never a side effect of arriving anywhere. A
  rival's working Battery in that orbit opens a Battle on a blockader (ticket #363). `show` names
  the orbit every Ship sits in, and lists each Body's orbits under FREE SLOTS.

  bombard <ship> <colony>              a Battleship of yours breaks one Module of a RIVAL'S Colony,
                                       never over Earth. Ticket #335 (version 0.09.0): IT ACTS IN
                                       THE ORBIT IT SITS IN. A Colony on the ground is bombarded
                                       from LOW ORBIT and wants Orbital Control of that Body
                                       outright; a station from that station's own orbit, which must
                                       hold no rival warship and no rival working Battery. Move the
                                       Battleship there first with `change-orbit`, and in an EARLIER
                                       turn: a Ship takes one order a turn, and a Bombard is one.

  launch <ship> <place>                a Missile Carrier of yours fires its one Warhead at a RIVAL'S
                                       Region, ground Colony or Space Station. Ticket #343 (version
                                       0.09.1): the same orbit rule a Bombard reads -- low orbit for
                                       the ground (and Orbital Control of that Body outright), a
                                       station's own orbit for a station -- but EARTH IS NOT
                                       EXCEPTED, and a Region is a lawful target. Free.
  rearm <ship>                         load another Warhead, at a Colony or station of yours with a
                                       working Shipyard, in that place's own orbit. It is a build
                                       in that yard's queue: Materials now, Widgets over the turns.

  influence <place> <amount>
  buy-influence <amount>
  max <place>                          repeat next turn's WHOLE Allotment on one place you hold,
  max off                              turn after turn until it is switched off. Free, and it
                                       spends nothing by itself.
  buy <resource> <amount>              materials|fuel|energy
  sell <resource> <amount>             materials|fuel

CAMPAIGNS AND CARBON CREDITS -- the Blame ledger the Influence thresholds read (`show` prints it)

  smear <faction> <amount>             Influence spent on a RIVAL'S name, laying {smear} ppm on their
                                       ledger for good per point spent. One a turn per target; an
                                       offence, and the Report names who paid.
  greenwash <amount>                   Influence spent on your own name, with {gwducats} Ducat(s) beside
                                       every point, taking {greenwash} ppm off your own ledger for good
                                       per point. One a turn, and no offence.
  offer-credits <ppm>                  the Custodians only: the ppm of carbon credit they sell a
                                       turn, standing until it is set again; nought refuses
                                       everybody.
  buy-credits <ppm>                    buy carbon credit from the Custodians at {creditprice} Ducat(s) a ppm
                                       before their view of you multiplies it, at most {creditcap} ppm a
                                       turn, one purchase a turn, and only while they are offering.
                                       It comes off your ledger at the Resolution. `show` prints
                                       what stands offered and what a ppm would cost you.

DIPLOMACY (ticket #226; `show` prints Relations both ways and every Accord standing)

  accord offer <faction> <term>...     offer an Accord holding one or more Terms:
                                         non-aggression  neither spends Influence on a place the
                                                         other holds, nor opens a Battle on them
                                         passage         neither treats the other's Ships as a
                                                         target, a Blockade does not shut the other
                                                         out of a slot, and an Army may cross
                                         refuel          either may Refuel at the other's stations
                                         research        both parties' Research rises a tenth; it
                                                         wants Friendly on BOTH sides to strike
                                       One offer a turn to a Faction, and none where an Accord
                                       already stands. THE ANSWER IS NOBODY'S TO WRITE: every seat,
                                       this one included, answers an offer made to it at the
                                       Resolution by its own weights, so there is no order to accept
                                       or decline one. A refusal is not an offence. An Accord kept
                                       {kept} turns pays both sides.
  accord end <faction>                 declare a standing Accord over: free, and it lapses at the
                                       next turn's start -- which gives the board a turn's warning
                                       that something is coming.
  tribute <faction> ducats             a fixed gift, one per Faction per turn, paying Relations:
  tribute <faction> materials          {tributeducats} Ducats, or {tributematerials} Materials.

REGIONS AND PEOPLE

  relief <state>                       Ducats lower by one the Unrest of a Region you direct
  agitate <state>                      Relief's mirror: {agitateducats} Ducats and {agitateinf} Influence raise
                                       by one the Unrest of a Region a RIVAL holds, once a turn per
                                       Region. An offence.
  resettle <state>
  emigrants <state> <n>                recruit Pioneers
  send-antarctica <state> <n> slot earth <slot>
  send-antarctica <state> <n> colony <n>
  lift <state> <n> <colony>            lift waiting Pioneers from a Region of yours with a working
                                       Launch Site straight onto a station of yours over Earth, as
                                       far as its Habitat room goes. A launch: it emits, and what it
                                       emits goes on your Blame.
  change facility <state> <index> <mothball|restart|decommission>
  change module <colony> <index> <mothball|restart|decommission>

RESEARCH, THE ARCHIVE AND THE FACTION ORDERS

  directive <percent>                  the RESEARCH DIRECTIVE: the share of your Labs' Research that
                                       goes somewhere other than the shared Tech, at ANY whole
                                       percent. The Archivists may go to {dirarchivists}%, which fills the
                                       Archive fund; every other Faction to {dirmax}%, and what that
                                       share turns into is its own -- the Custodians' into the
                                       Natural Sink, the Prospectors' into Ducats, the Arkwrights'
                                       into Fuel. `show` names yours and its rate. Read at the next
                                       Income, and it holds until it is set again.
  fund-archive                         the same order at your Faction's cap,
  unfund-archive                       and the same order at nought.
  upload <colony> <n>                  the Archivists only, and only once the Archive is COMPLETE:
                                       Colonists at the Archive's own place are read into it and
                                       leave the living population. Free, and irreversible.
  venture-share <percent>              the Prospectors only, in steps of {venturestep}% from 0% to {venturemax}%
  draw-venture <amount>                the Prospectors only
  leapfrog <state>                     the Custodians only
  strip-permit <state>                 the Prospectors only
  exodus-call <state>                  the Arkwrights only, on a Region they control, once per Region
                                       EVER, for {exodus} Ducats: while it runs it doubles what they may
                                       recruit there and suspends Coach Class's double population
                                       charge.

A FACTION is named by its own name, shortened as far as it stays unique (`ark` is the Arkwrights),
or by its seat number. `show` prints both beside every Faction.

Names may be shortened as long as they stay unique: `eur` is Europe, `colonys` is a Colony Ship.
Colonies, ships and armies are named by the numbers the board prints beside them."#;

/// The grammar with this game's own figures written into it, so a table that moves a figure moves
/// the help with it. Where the data directory cannot be found at all, every figure reads `?` and
/// the grammar still prints: a player with no tables can still be told the shape of a line.
fn grammar(t: Option<&Tables>) -> String {
    let n = |get: fn(&Tables) -> String| -> String { t.map(get).unwrap_or_else(|| "?".to_string()) };
    let mut s = GRAMMAR.to_string();
    for (key, value) in [
        ("{fuel}", n(|t| t.orbit_change_fuel.to_string())),
        ("{smear}", n(|t| format!("{:.2}", t.influence.smear.ppm_per_influence))),
        ("{greenwash}", n(|t| format!("{:.2}", t.influence.greenwash.ppm_per_influence))),
        ("{gwducats}", n(|t| t.influence.greenwash.ducats_per_influence.to_string())),
        ("{creditprice}", n(|t| t.carbon_credits.price_per_ppm.to_string())),
        ("{creditcap}", n(|t| t.carbon_credits.cap_per_turn.to_string())),
        ("{kept}", n(|t| t.relations.accord_kept_turns.to_string())),
        ("{tributeducats}", n(|t| t.relations.tribute_ducats.to_string())),
        ("{tributematerials}", n(|t| t.relations.tribute_materials.to_string())),
        ("{agitateducats}", n(|t| t.unrest.agitate_ducats.to_string())),
        ("{agitateinf}", n(|t| t.unrest.agitate_influence.to_string())),
        ("{dirarchivists}", n(|t| t.research_directive.archivists_max.to_string())),
        ("{dirmax}", n(|t| t.research_directive.max.to_string())),
        ("{venturestep}", n(|t| format!("{:.0}", t.venture.share_step * 100.0))),
        ("{venturemax}", n(|t| format!("{:.0}", t.venture.max_share * 100.0))),
        ("{exodus}", n(|t| t.ducats.per_exodus_call.to_string())),
    ] {
        s = s.replace(key, &value);
    }
    s
}

/// One line is an order, or the `tech` line or an `answer` line, neither of which is one.
enum Line {
    Order(Box<Order>),
    Tech(TechId),
    /// Ticket #337: this seat's answer to the turn's Choice Card -- taken, or refused.
    Answer(bool),
}

/// The game is read here as well as written: a Faction is named by its own name, and which seat
/// holds which Faction is a thing of the board, not of the grammar. Nothing is changed through it.
fn parse_line(g: &Game, line: &str) -> Result<Line, String> {
    let w: Vec<&str> = line.split_whitespace().collect();
    let at = |i: usize| -> Result<&str, String> { w.get(i).copied().ok_or_else(|| format!("`{}` wants more words", w[0])) };
    let verb = w[0].to_ascii_lowercase();
    let o = match verb.as_str() {
        "tech" => return Ok(Line::Tech(pick(&TechId::ALL, at(1)?)?)),
        // Ticket #337 (version 0.09.0): the only door the driver has onto the turn's question. It
        // is not an Order -- it is answered the moment the line is read, as a `tech` pick is.
        "answer" => {
            let side = at(1)?.to_ascii_lowercase();
            return match side.as_str() {
                "take" => Ok(Line::Answer(true)),
                "refuse" => Ok(Line::Answer(false)),
                _ => Err(format!("`answer` wants `take` or `refuse`, not {side:?}")),
            };
        }
        "build" => {
            let what = at(1)?.to_ascii_lowercase();
            match what.as_str() {
                "facility" => Order::BuildFacility { state: pick(&StateId::ALL, at(2)?)?, kind: pick(&FacilityKind::ALL, at(3)?)? },
                "facility-ducats" => Order::BuildFacilityWithDucats { state: pick(&StateId::ALL, at(2)?)?, kind: pick(&FacilityKind::ALL, at(3)?)? },
                "module" => Order::BuildModule { colony: colony_id(at(2)?)?, kind: pick(&ModuleKind::BUILDABLE, at(3)?)? },
                "module-ducats" => Order::BuildModuleWithDucats { colony: colony_id(at(2)?)?, kind: pick(&ModuleKind::BUILDABLE, at(3)?)? },
                "ship" => Order::BuildShip { site: place(at(2)?)?, kind: pick(&UnitKind::SHIPS, at(3)?)? },
                "army" => Order::BuildArmy { place: place(at(2)?)? },
                "station" => Order::BuildStation { body: pick(&BodyId::ALL, at(2)?)?, slot: count(at(3)?)? },
                "archive" => Order::BuildArchive { colony: colony_id(at(2)?)? },
                _ => return Err(format!("`build {what}` is not one of facility, module, ship, army, station, archive")),
            }
        }
        "industry" => Order::RaiseIndustry { state: pick(&StateId::ALL, at(1)?)? },
        // Ticket #332 (version 0.09.0): a build another seat began at a place that has changed hands.
        "cancel-build" => Order::CancelBuild { place: place(at(1)?)?, index: count(at(2)?)? as usize },
        "transit" => Order::Transit {
            ship: ship_id(at(1)?)?,
            to: pick(&BodyId::ALL, at(2)?)?,
            // Ticket #99: the Orbital Slot to arrive into. Ticket #335 (version 0.09.0): the leg
            // names the ORBIT it ends in, and a leg naming none ends in low orbit, which is an
            // orbit like any other and not "the Body at large".
            slot: match w.get(3) {
                None => None,
                Some(v) => orbit_slot(v)?,
            },
        },
        // Ticket #335 (version 0.09.0): between the orbits of the Body the Ship already stands at.
        "change-orbit" => Order::ChangeOrbit { ship: ship_id(at(1)?)?, slot: orbit_slot(at(2)?)? },
        "refuel" => Order::Refuel { ship: ship_id(at(1)?)? },
        // Ticket #328 (version 0.08.8), and ticket #335 (version 0.09.0) for the orbit: the
        // Battleship acts in the orbit it sits in, so the line names no orbit of its own.
        "bombard" => Order::Bombard { ship: ship_id(at(1)?)?, colony: colony_id(at(2)?)? },
        // Ticket #343 (version 0.09.1): the Missile Carrier fires its one Warhead, and loads
        // another at a yard of its own. Both act in the orbit the hull sits in.
        "launch" => Order::Launch { ship: ship_id(at(1)?)?, target: place(at(2)?)? },
        "rearm" => Order::Rearm { ship: ship_id(at(1)?)? },
        "load" => {
            let ship = ship_id(at(1)?)?;
            let colonists = count(at(2)?)?;
            let rest: Vec<&str> = w[3..].to_vec();
            let i = rest.iter().position(|x| x.eq_ignore_ascii_case("from")).ok_or("load wants `from <place>`")?;
            let from = match place(rest.get(i + 1).ok_or("load wants a place after `from`")?)? {
                Place::State(s) => LoadSource::State(s),
                Place::Colony(c) => LoadSource::Colony(c),
            };
            let army = match rest.iter().position(|x| x.eq_ignore_ascii_case("army")) {
                Some(j) => Some(army_id(rest.get(j + 1).ok_or("`army` wants a number")?)?),
                None => None,
            };
            Order::Load { ship, colonists, from, army }
        }
        "unload" => {
            let ship = ship_id(at(1)?)?;
            let colonists = count(at(2)?)?;
            let rest: Vec<&str> = w[3..].to_vec();
            let army = rest.iter().any(|x| x.eq_ignore_ascii_case("army"));
            let i = rest.iter().position(|x| x.eq_ignore_ascii_case("into")).ok_or("unload wants `into ...`")?;
            Order::Unload { ship, colonists, army, into: unload_target(&rest[i + 1..])? }
        }
        "move-army" => Order::MoveArmy { army: army_id(at(1)?)?, to: pick(&StateId::ALL, at(2)?)? },
        "ship-stance" => Order::ShipStance { body: pick(&BodyId::ALL, at(1)?)?, stance: pick(&STANCES, at(2)?)? },
        "army-stance" => Order::ArmyStance { place: place(at(1)?)?, stance: pick(&STANCES, at(2)?)? },
        "repair" => Order::Repair { unit: unit_ref(at(1)?)?, points: count(at(2)?)? },
        "repair-ducats" => Order::RepairWithDucats { unit: unit_ref(at(1)?)?, points: count(at(2)?)? },
        "influence" => Order::Influence { target: place(at(1)?)?, amount: number(at(2)?)? },
        "buy-influence" => Order::BuyInfluence { amount: number(at(1)?)? },
        // Version 0.07.3 (ticket #134): Max, the standing order. `max off` switches it off.
        "max" => {
            let word = at(1)?.to_ascii_lowercase();
            let target = if word == "off" || word == "none" { None } else { Some(place(&word)?) };
            Order::SetMaxStanding { target }
        }
        // Ticket #267 (version 0.08.4) and ticket #275/#277 (version 0.08.5): the two campaigns
        // that spend Influence on a NAME rather than a place -- a rival's, and the seat's own.
        "smear" => Order::Smear { target: seat_of(g, at(1)?)?, amount: number(at(2)?)? },
        "greenwash" => Order::Greenwash { amount: number(at(1)?)? },
        // Ticket #268 (version 0.08.4): the Custodians' standing offer, and everybody else's buy.
        "offer-credits" => Order::OfferCredits { ppm: number(at(1)?)? },
        "buy-credits" => Order::BuyCredits { ppm: number(at(1)?)? },
        // Ticket #226 (version 0.08.2): the diplomacy. `accord accept` and `accord decline` are
        // named here on purpose: a player who looks for them should be told the rule rather than
        // that the word is not an order.
        "accord" => {
            let what = at(1)?.to_ascii_lowercase();
            match what.as_str() {
                "offer" | "propose" => {
                    let to = seat_of(g, at(2)?)?;
                    if w.len() < 4 {
                        return Err("an Accord wants at least one Term: non-aggression, passage, refuel or research".into());
                    }
                    let mut terms: Vec<Term> = Vec::new();
                    for word in &w[3..] {
                        let term = pick(&TERMS, word)?;
                        if !terms.contains(&term) {
                            terms.push(term);
                        }
                    }
                    Order::ProposeAccord { to, terms }
                }
                "end" => Order::EndAccord { with: seat_of(g, at(2)?)? },
                "accept" | "decline" | "refuse" => {
                    return Err(
                        "an offer made to you is answered at the Resolution by your own seat's weights: no order accepts or declines one. `accord end <faction>` ends an Accord that stands."
                            .into(),
                    );
                }
                _ => return Err(format!("`accord {what}` is not one of offer, end")),
            }
        }
        "tribute" => {
            let to = seat_of(g, at(1)?)?;
            let what = at(2)?.to_ascii_lowercase();
            let materials = match what.as_str() {
                "materials" => true,
                "ducats" => false,
                _ => return Err(format!("a tribute is paid in `ducats` or `materials`, not {what:?}")),
            };
            Order::Tribute { to, materials }
        }
        "buy" => Order::Buy { resource: pick(&RESOURCES, at(1)?)?, amount: number(at(2)?)? },
        "sell" => Order::Sell { resource: pick(&RESOURCES, at(1)?)?, amount: number(at(2)?)? },
        "relief" => Order::Relief { state: pick(&StateId::ALL, at(1)?)? },
        // Ticket #269 (version 0.08.4): Relief's mirror, on a Region a rival holds.
        "agitate" => Order::Agitate { state: pick(&StateId::ALL, at(1)?)? },
        "resettle" => Order::Resettle { state: pick(&StateId::ALL, at(1)?)? },
        "emigrants" => Order::BuildEmigrants { state: pick(&StateId::ALL, at(1)?)?, n: count(at(2)?)? },
        "send-antarctica" => Order::SendToAntarctica { state: pick(&StateId::ALL, at(1)?)?, n: count(at(2)?)?, into: unload_target(&w[3..])? },
        // Version 0.07.3 (ticket #141): Pioneers lifted from a Launch Site onto a station over Earth.
        "lift" => Order::LiftToStation { state: pick(&StateId::ALL, at(1)?)?, n: count(at(2)?)?, colony: colony_id(at(3)?)? },
        "change" => {
            let what = at(1)?.to_ascii_lowercase();
            let index = count(at(3)?)? as usize;
            let change = pick(&CHANGES, at(4)?)?;
            let building = match what.as_str() {
                "facility" => BuildingRef::Facility(pick(&StateId::ALL, at(2)?)?, index),
                "module" => BuildingRef::Module(colony_id(at(2)?)?, index),
                _ => return Err("`change` wants `facility` or `module`".into()),
            };
            Order::Change { building, what: change }
        }
        // Ticket #235 (version 0.08.3): the Research Directive is a per-percent figure. The two
        // words are what the order used to be, kept as shorthands and now reading the Faction's own
        // cap rather than a flat hundred, which no Faction but the Archivists may even set.
        "directive" => Order::SetResearchDirective { percent: percent(at(1)?)? },
        "fund-archive" => Order::SetResearchDirective { percent: g.research_directive_cap(Seat(0)) },
        "unfund-archive" => Order::SetResearchDirective { percent: 0 },
        // Ticket #192 (version 0.08.0): the Upload, which is what the Archive is for.
        "upload" => Order::Upload { colony: colony_id(at(1)?)?, n: count(at(2)?)? },
        "venture-share" => Order::SetVentureShare { share: count(at(1)?)? },
        "draw-venture" => Order::DrawVenture { amount: number(at(1)?)? },
        "leapfrog" => Order::Leapfrog { state: pick(&StateId::ALL, at(1)?)? },
        "strip-permit" => Order::StripPermit { state: pick(&StateId::ALL, at(1)?)? },
        // Ticket #237 (version 0.08.3): the Arkwrights' remaking of a country.
        "exodus-call" => Order::ExodusCall { state: pick(&StateId::ALL, at(1)?)? },
        _ => return Err(format!("`{verb}` is not an order; run `help` for the list")),
    };
    Ok(Line::Order(Box::new(o)))
}

// ------------------------------------------------------------------ the board, in words

fn control_text(g: &Game, c: Control) -> String {
    match c {
        Control::Neutral => "neutral".into(),
        Control::Controlled(s) => format!("{} (seat {})", g.seat_name(s), s.0),
        Control::Occupied { occupier, turns, .. } => format!("occupied by the {} ({turns}t)", g.seat_name(occupier)),
    }
}

/// Where a Ship is. Ticket #335 (version 0.09.0): a Ship sits in an ORBIT of a Body, and which
/// orbit decides what it can touch, refuel at, blockade or fight, so the orbit is what is printed:
/// a Ship at a station's ring reads differently from one in low orbit. A Ship in transit names the
/// orbit its leg ends in, which was chosen when it launched.
fn ship_at_text(g: &Game, s: &Ship) -> String {
    match s.at {
        ShipAt::Body(b) => g.orbit_name(b, g.ship_orbit(s)),
        ShipAt::Transit { from, to, turns_left } => format!("{} -> {} ({turns_left} left)", from.name(), g.orbit_name(to, Orbit::of(s.slot))),
    }
}

/// Ticket #337 (version 0.09.0): one effect of one side of a Choice Card, in words. The eighteen
/// cards are not eighteen pieces of written code: each side is a list of effects composed in
/// `events.toml` from one vocabulary, so a card given new figures in the table says the new figures
/// here without this being touched.
fn card_effect_text(e: &CardEffect) -> String {
    match e {
        CardEffect::Resources { materials, fuel, energy, ducats, research } => {
            let parts: Vec<String> = [(*materials, Resource::Materials), (*fuel, Resource::Fuel), (*energy, Resource::Energy), (*ducats, Resource::Ducats), (*research, Resource::Research)]
                .into_iter()
                .filter(|(n, _)| *n != 0)
                .map(|(n, r)| format!("{n:+} {}", r.name()))
                .collect();
            parts.join(", ")
        }
        CardEffect::PerUnitCost { per, resource, amount } => {
            let thing = match per {
                CardThing::Facility(k) => k.name().to_string(),
                CardThing::ShipInOrbit => "Ship of yours in orbit".to_string(),
            };
            format!("{amount} {} for every {thing}", resource.name())
        }
        CardEffect::PopulationToMostPopulous { population } => format!("{population:+.1} population in your most populous Region"),
        CardEffect::EmissionsNext { ppm } => format!("{ppm:+.1} ppm of Emissions next turn"),
        CardEffect::StandingAllHeld { standing } => format!("{standing:+} Standing in every Region you hold"),
        CardEffect::StandingAtMostPopulous { standing } => format!("{standing:+} Standing in your most populous Region"),
        CardEffect::UnrestAllHeld { unrest } => format!("{unrest:+.1} Unrest in every Region you hold"),
        CardEffect::UnrestAtMostPopulous { unrest } => format!("{unrest:+.1} Unrest in your most populous Region"),
        CardEffect::UnrestAtBusiest { unrest } => format!("{unrest:+.1} Unrest in your busiest Region"),
        CardEffect::HoldShips => "every Ship of yours holds this turn: no transit of yours resolves".to_string(),
        CardEffect::HoldOneShip => "one Ship of yours holds this turn".to_string(),
        CardEffect::DamageShips { damage, in_orbit } => format!("{damage} damage to each of your Ships{}", if *in_orbit { " in orbit" } else { "" }),
        CardEffect::TradePrice { resource, to, by, turns } => match to {
            Some(to) => format!("the {} price stands at {to} for {turns} turn(s)", resource.name()),
            None => format!("the {} price moves {by:+} for {turns} turn(s)", resource.name()),
        },
        CardEffect::RelationsAllRivals { relations } => format!("{relations:+} Relations with every rival"),
        CardEffect::BlamePpm { ppm } => {
            if *ppm < 0.0 {
                format!("{:.1} ppm off your Blame", -ppm)
            } else {
                format!("{ppm:.1} ppm onto your Blame")
            }
        }
        CardEffect::WidgetsNow { widgets } => format!("{widgets:+} Widgets this turn in your busiest Region"),
        CardEffect::FacilityOutputMultiplier { facility, multiplier } => {
            format!("every {} of yours makes {:.0}% of its output at the next Income", facility.name(), multiplier * 100.0)
        }
        CardEffect::DiscoveryAtColony { module, multiplier, turns } => {
            format!("a Discovery at one of your Colonies: its {} at x{multiplier:.1} for {turns} turn(s)", module.name())
        }
        CardEffect::PioneersFree { pioneers } => format!("{pioneers} Pioneers waiting, costing their Region no people"),
        CardEffect::FreeBuilding { module, army } => match (module, army) {
            (Some(m), _) => format!("a {} standing free at your smallest Colony", m.name()),
            (None, true) => "an Army raised free in your most populous Region, costing it no people".to_string(),
            (None, false) => String::new(),
        },
    }
}

fn card_side_text(side: &[CardEffect]) -> String {
    let parts: Vec<String> = side.iter().map(card_effect_text).filter(|s| !s.is_empty()).collect();
    parts.join("; ")
}

/// Ticket #337 (version 0.09.0): **the turn's question**, which a player who cannot see cannot
/// answer -- and the turn cannot end until they have. The card, what it asks, what each side does,
/// and whether the offer is one this seat can pay.
fn print_question(g: &Game) {
    let me = Seat(0);
    let Some(q) = g.pending_question() else { return };
    let card = g.tables.event(q.card);
    let Some(c) = card.choice.as_ref() else { return };
    println!("\n=== THE TURN'S QUESTION: {} ===", card.name);
    println!("{}", c.question);
    println!("  `answer take`    {}: {}", c.take, card_side_text(&c.take_does));
    println!("  `answer refuse`  {}: {}", c.refuse, card_side_text(&c.refuse_does));
    match q.answer_of(me) {
        Some(a) => println!("  Your answer is given: you {a_word}.", a_word = a.word()),
        None if !g.may_take_card(me) => {
            println!("  *** THE OFFER IS CLOSED TO YOU: you cannot pay what it asks, so `answer refuse` is your only move. ***")
        }
        None => println!("  *** UNANSWERED. Both sides are open to you. The turn cannot end until an `answer` line is given. ***"),
    }
}

/// Ticket #337: what this seat still owes the turn's card, in the driver's own words -- naming the
/// card and the line that answers it, since a line is the only door the driver has. `Game::end_turn`
/// refuses in its own words; a player reading only those would not know what to write.
fn owed_answer(g: &Game) -> Option<String> {
    let q = g.pending_question()?;
    if q.answer_of(Seat(0)).is_some() {
        return None;
    }
    let card = g.tables.event(q.card);
    let question = card.choice.as_ref().map(|c| c.question.as_str()).unwrap_or_default();
    Some(format!(
        "{} is asking you: {question}\n  Put `answer take` or `answer refuse` in the order list. {}",
        card.name,
        if g.may_take_card(Seat(0)) {
            "Either side is open to you."
        } else {
            "You cannot pay what it asks, so `answer refuse` is your only move."
        }
    ))
}

/// What it would take this seat to take `target` as it stands, which on a held place is the
/// greater of its own threshold and the holder's Standing plus the challenge margin. The bare
/// threshold is NOT the bar on a held place, and printing it understates what a push costs.
fn standing_note(g: &Game, seat: Seat, target: Target) -> String {
    let need = g.influence_needed_for(seat, target);
    let mine = g.seat(seat).influence.get(&target).copied().unwrap_or(0);
    match g.place_control(target).controller() {
        Some(c) if c == seat => {
            // A rival's own bar is its threshold or this Standing plus the margin, whichever is
            // greater; the threshold half is the rival's to know, so only the margin is named.
            format!("you hold it on {} Standing, over which a rival must climb by the challenge margin", mine)
        }
        Some(c) => format!("{} needed to take it from the {}", need, g.seat_name(c)),
        None => format!("{need} needed to take it"),
    }
}

/// Ticket #332 (version 0.09.0): a place's queue, each build as its INDEX (what `cancel-build`
/// names), its item, and the Widgets done of the Widgets it wants. A build begun by another seat --
/// what a conquest leaves behind -- is the only kind that may be cancelled, so it says whose it is.
fn queue_text(g: &Game, place: Place) -> Vec<String> {
    g.queue_at(place)
        .iter()
        .enumerate()
        .map(|(i, b)| {
            let whose = if b.seat == Seat(0) { String::new() } else { format!(" (the {}'s, cancellable)", g.seat_name(b.seat)) };
            format!("{i}:{} {}/{}w{whose}", b.item.name(), b.done, b.widgets)
        })
        .collect()
}

/// Ticket #332: what a place MAKES and what it OWES, which is the whole of planning a build now
/// that the flat turn count is gone: the queue is served in order at this rate.
fn widgets_text(g: &Game, place: Place, queue: &[String]) -> String {
    format!(
        "Widgets {} a turn, {} owed | building: {}",
        g.widgets_at(place),
        g.widgets_owed(place),
        if queue.is_empty() { "nothing".to_string() } else { queue.join(", ") }
    )
}

/// Ticket #226 (version 0.08.2): one Term of an Accord in the words the order line takes.
fn term_text(t: Term) -> &'static str {
    match t {
        Term::NonAggression => "non-aggression",
        Term::Passage => "passage",
        Term::Refuel => "refuel",
        Term::ResearchAgreement => "research",
    }
}

/// Ticket #235 (version 0.08.3): where a Faction's Research Directive sends its Research, which is
/// a different place for each of the four, at the rate its own table row sets.
fn directive_destination(g: &Game, seat: Seat) -> String {
    let t = &g.tables.research_directive;
    match g.kind(seat) {
        FactionKind::Archivists => "into the Archive fund".to_string(),
        FactionKind::Custodians => format!("into the Natural Sink, {:.3} ppm a point, for good", t.custodians_ppm_per_point),
        FactionKind::Prospectors => format!("into Ducats, {:.2} a point", t.prospectors_ducats_per_point),
        FactionKind::Arkwrights => format!("into Fuel, {:.2} a point", t.arkwrights_fuel_per_point),
    }
}

/// **What the campaigns, the credits and the diplomacy act on.** An Accord cannot be offered by a
/// player who cannot see Relations, a Smear laid by one who cannot see Blame, a credit bought by
/// one who cannot see what is offered, a Directive set by one who cannot see where it goes. One
/// line a Faction and one line a system: the board is long already.
fn print_standing(g: &Game) {
    let me = Seat(0);
    println!("\n--- RELATIONS, ACCORDS, BLAME AND CREDITS ---");
    for seat in Seat::ALL {
        let s = g.seat(seat);
        let blame = format!(
            "Blame {:.2} ppm ({:.0}% of the table, thresholds x{:.2}; {:.2} smeared on, {:.2} washed off)",
            g.blame(seat),
            g.blame_share(seat) * 100.0,
            g.blame_threshold_multiplier(seat),
            s.blame_smeared,
            s.blame_cleaned
        );
        if seat == me {
            println!("seat {} the {:<12} (YOU){:<41} | {blame}", seat.0, g.seat_name(seat), "");
            continue;
        }
        println!(
            "seat {} the {:<12} you see them {:<8} ({:+}), they see you {:<8} ({:+}) | {blame}",
            seat.0,
            g.seat_name(seat),
            g.relations_level(me, seat),
            g.relations_score(me, seat),
            g.relations_level(seat, me),
            g.relations_score(seat, me)
        );
    }
    // Every Accord at the table, not only this seat's: an Accord between two rivals is as much a
    // thing to plan around as one of your own.
    let accords: Vec<String> = g
        .accords
        .iter()
        .map(|a| {
            format!(
                "the {} & the {} [{}] struck turn {}{}",
                g.seat_name(a.a),
                g.seat_name(a.b),
                a.terms.iter().map(|t| term_text(*t)).collect::<Vec<_>>().join(", "),
                a.struck,
                if a.ending { ", ENDING at the next turn's start" } else { "" }
            )
        })
        .collect();
    println!("Accords standing: {}", if accords.is_empty() { "none anywhere at all".to_string() } else { accords.join(" | ") });
    match g.credit_seller() {
        None => println!("Carbon credits: nobody at this table sells them"),
        Some(seller) if seller == me => println!(
            "Carbon credits: you offer {} ppm a turn, at most {} to one buyer (`offer-credits <ppm>`); you have sold {:.1} ppm.",
            g.seat(me).credits_offered,
            g.tables.carbon_credits.cap_per_turn,
            g.seat(me).credits_sold
        ),
        Some(seller) => println!(
            "Carbon credits: the {} offer {} ppm a turn, at most {} to one buyer; {}. You have bought {:.1} ppm, they have sold {:.1}.",
            g.seat_name(seller),
            g.seat(seller).credits_offered,
            g.tables.carbon_credits.cap_per_turn,
            match g.credit_cost(me, 1) {
                Some(d) => format!("a ppm would cost you {d} Ducat(s)"),
                None => "they will not sell to you: they are Hostile".to_string(),
            },
            g.seat(me).credits_bought,
            g.seat(seller).credits_sold
        ),
    }
    let s = g.seat(me);
    println!(
        "Research Directive: {}% of your Labs' Research goes {} (any whole percent to a cap of {}%); {}% was in force at the last Income.",
        s.research_directive,
        directive_destination(g, me),
        g.research_directive_cap(me),
        s.directive_last_income
    );
    println!(
        "Max repeats: {}",
        match s.max_standing {
            Some(p) => format!("next turn's whole Allotment goes on {} until `max off`", g.place_name(p)),
            None => "off -- no place takes next turn's Allotment by itself".to_string(),
        }
    );
}

fn slot_name(g: &Game, body: BodyId, slot: u32) -> String {
    g.tables.body(body).slots.get(slot as usize).map(|s| s.name.clone()).unwrap_or_else(|| format!("slot {slot}"))
}

fn print_costs(g: &Game) {
    let me = Seat(0);
    // Ticket #332 (version 0.09.0): Materials and Widgets, at this seat's discounts.
    println!("\n--- WHAT THINGS COST YOU (Materials / Widgets) ---");
    let f: Vec<String> = FacilityKind::ALL
        .iter()
        .map(|k| format!("{} {}/{}w", k.name(), g.facility_materials(me, *k), g.build_widgets(me, BuildItem::Facility(*k))))
        .collect();
    println!("Facilities: {}", f.join(" | "));
    let m: Vec<String> = ModuleKind::ALL
        .iter()
        .map(|k| format!("{} {}/{}w", k.name(), g.module_materials(me, *k), g.build_widgets(me, BuildItem::Module(*k))))
        .collect();
    println!("Modules (before the working-Mine discount): {}", m.join(" | "));
    let u: Vec<String> = UnitKind::SHIPS
        .iter()
        .map(|k| {
            let c = g.tables.unit(*k);
            format!("{} {}M+{}F/{}w tank {} hp {}", k.name(), g.ship_materials(me, *k), c.tank, g.build_widgets(me, BuildItem::Unit(*k)), c.tank, c.hit_points)
        })
        .collect();
    println!("Ships: {}", u.join(" | "));
    println!(
        "Army {}M | Space Station {}M | Industry Level {}M",
        g.tables.unit(UnitKind::Army).materials,
        g.station_materials(me),
        g.industry_cost(me)
    );
    println!(
        "Market: Materials {}, Fuel {}, Energy {} Ducats each; Influence {} Ducats a point; Relief {} Ducats",
        g.market_price(me, g.trade_price(Resource::Materials).unwrap_or(0)),
        g.market_price(me, g.trade_price(Resource::Fuel).unwrap_or(0)),
        g.market_price(me, g.trade_price(Resource::Energy).unwrap_or(0)),
        g.tables.ducats.per_influence,
        g.tables.unrest.relief_ducats
    );
}

fn print_report(g: &Game) {
    println!("\n=== REPORT, turn {} ===", g.report.turn);
    if let Some(h) = g.report.headline() {
        println!("HEADLINE: {}", h.text);
    }
    if let Some(e) = &g.report.event {
        println!("EVENT: {e}");
    }
    let head = g.report.headline_index();
    for (i, l) in g.report.lines.iter().enumerate() {
        if Some(i) == head {
            continue;
        }
        println!("  [{:?}] {}", l.kind, l.text);
    }
    for b in &g.report.battles {
        println!("  [Battle] {}", b.text(&|s| g.seat_name(s), "a neutral force"));
    }
    for (seat, para) in g.faction_paragraphs() {
        if seat != Seat(0) {
            println!("  [Rival] {para}");
        }
    }
}

fn print_board(g: &Game) {
    let me = Seat(0);
    let t = &g.tables;
    // Ticket #337: at the head of the board, because it is asked at the head of the turn and holds
    // End Turn until it is answered.
    print_question(g);
    println!("\n=== TURN {} of {} ({}) ===", g.turn, t.victory.turns, g.date_text());
    println!("{}", g.outcome_text());
    println!(
        "Climate: {:+.2} C (collapse at {:+.1}), CO2 {:.1} ppm; emissions {:.2} - sink {:.2} = net {:+.2}",
        g.climate.temperature,
        t.climate.collapse_line,
        g.climate.co2,
        g.climate.last.total(),
        g.climate.last.total_sink(),
        g.climate.last.net()
    );
    println!(
        "  emissions by seat: {}",
        Seat::ALL.iter().map(|s| format!("{} {:.2}", g.seat_name(*s), g.climate.last.by_seat[s.index()])).collect::<Vec<_>>().join(", ")
    );
    match g.last_turn_to_act() {
        dying_earth_engine::LastTurn::Turn(n) => println!("  Last turn to act: {n}"),
        dying_earth_engine::LastTurn::TooLate => println!("  Last turn to act: gone by"),
        dying_earth_engine::LastTurn::NoCollapse => println!("  Last turn to act: the world is safe on this course"),
    }
    if let Some(c) = g.projection().collapse_turn {
        println!("  On this course the world collapses on turn {c}.");
    }
    let fired: Vec<&str> = t.climate.breaks.iter().zip(&g.climate.breaks_fired).filter(|(_, f)| **f).map(|(b, _)| b.name.as_str()).collect();
    println!("  Breaks fired: {}", if fired.is_empty() { "none".to_string() } else { fired.join(", ") });
    println!("  Antarctica: {}", if g.antarctica_open { "open" } else { "shut under the ice" });

    let s = g.seat(me);
    println!("\n--- YOU: seat 0, the {} ---", g.seat_name(me));
    println!(
        "Stockpile: {} Materials, {} Fuel, {} Energy, {} Ducats",
        s.stockpile.materials, s.stockpile.fuel, s.stockpile.energy, s.stockpile.ducats
    );
    println!(
        "Last income: {}M {}F {}E {}D",
        s.income_last_turn.materials, s.income_last_turn.fuel, s.income_last_turn.energy, s.income_last_turn.ducats
    );
    let sources: Vec<String> = s.income_sources.iter().map(|(what, r, n)| format!("{what} {n:+}{}", &r.name()[..1])).collect();
    if !sources.is_empty() {
        println!("  from: {}", sources.join(", "));
    }
    // Ticket #351 (version 0.09.1): the Shortfall alarm the top bar shows in red, in the same words.
    // The playtest met the Shortfall through this driver, so it is where a tester will meet it again.
    if let Some(f) = g.shortfall_forecast(me, &[]) {
        println!("  ENERGY ALARM: next Income is {} Energy short. These go dark, in this order:", f.short_by);
        for d in &f.dark {
            println!("    the {} {}", d.name.strip_prefix("The ").unwrap_or(&d.name), d.at);
        }
    }
    println!("Influence allotment this turn: {}", g.influence_allotment(me));
    println!(
        "Blame: {:.2} ppm emitted, {:.2} removed; share {:.0}%, Influence thresholds x{:.2}",
        s.blame_emitted,
        s.blame_removed,
        g.blame_share(me) * 100.0,
        g.blame_threshold_multiplier(me)
    );
    match g.research.current {
        Some(tech) => println!(
            "Research: {} at {}/{}; your Labs made {} last turn, {} over the game",
            t.tech(tech).name,
            g.research.progress,
            t.tech(tech).cost,
            s.research_last_turn,
            s.research_total
        ),
        None => println!("Research: no Tech is under research"),
    }
    println!(
        "  Techs done: {}",
        if g.research.done.is_empty() { "none".into() } else { g.research.done.iter().map(|x| t.tech(*x).name.clone()).collect::<Vec<_>>().join(", ") }
    );
    // Version 0.07.3: a complete tree owes nobody a pick; the marker was printed with nothing under it
    // once every Tech was done, and a driver that trusted it wrote a `tech` line that could not parse.
    // Ticket #173 (version 0.07.6): a pick made this turn is not locked in until the turn ends, so
    // the board says the choice is open rather than owed, and a second `tech` line in the same turn
    // is now accepted where it used to be refused. The driver and the game have to agree.
    let owed = (g.research.awaiting_pick == Some(me) || g.research.current.is_none()) && !g.available_techs().is_empty();
    let changeable = g.research.current.is_some() && !g.research.pick_committed;
    if owed || changeable {
        let drawn = !g.research.shortlist.is_empty();
        if changeable {
            let name = g.research.current.map(|x| t.tech(x).name.clone()).unwrap_or_default();
            println!("  *** {name} IS CHOSEN FOR THIS TURN, and not locked in until the turn ends. Another `tech <name>` line changes it. ***");
        } else {
            println!(
                "  *** YOU MUST PICK THE NEXT TECH (a `tech <name>` line). {} ***",
                if drawn { "The Research Lead's shortlist:" } else { "A free choice of everything available:" }
            );
        }
        for x in g.pickable_techs() {
            let c = t.tech(x);
            println!("      {} ({} Research, rung {}): {}", c.name, c.cost, c.rung, c.effect);
        }
    }
    println!("  {}", g.research_lead_text());
    if s.kind == FactionKind::Prospectors {
        println!(
            "Venture Capital Fund: {} Materials, banking {:.0}% of output ({} banked last turn)",
            s.venture_fund,
            s.venture_share * 100.0,
            s.venture_banked_last_turn
        );
    }
    if s.kind == FactionKind::Archivists {
        // Ticket #192 (version 0.08.0): what an `upload` line needs to know -- where the Archive
        // stands, whether it is complete (nothing may be uploaded until it is), and how many
        // Colonists are at that place to read in.
        let stands = match g.archive_colony(me) {
            Some(c) => format!(
                "{} at {}, where {} Colonists stand to be uploaded",
                if g.archive_complete(me) { "is complete" } else { "stands unfinished" },
                g.place_name(Place::Colony(c)),
                g.uploadable_at(c, 0)
            ),
            None => "is not built at any Colony of yours".to_string(),
        };
        println!("Archive fund: {} of a cap of {}; the Archive {stands}; {} uploaded so far", s.archive_fund, g.archive_fund_cap(me), s.uploaded);
    }
    print_standing(g);

    println!("\n--- VICTORY ---");
    for seat in Seat::ALL {
        let p = g.progress(seat);
        let gate = t
            .victory_gate(g.kind(seat))
            .map(|x| format!("{} {}", t.tech(x).name, if g.has_tech(x) { "DONE" } else { "not yet" }))
            .unwrap_or_default();
        let held = if p.met() { " MET".to_string() } else { p.first_held_back.clone().map(|r| format!(" (held: {r})")).unwrap_or_default() };
        println!(
            "seat {} the {:<12} {}: {:.0}/{:.0} | {} | score {:.2}{} | gate: {}",
            seat.0,
            g.seat_name(seat),
            p.first_name,
            p.first_value,
            p.first_bar,
            p.second_text,
            p.score(),
            held,
            gate
        );
    }

    println!("\n--- NATION STATES ---");
    for st in &g.states {
        let card = t.state(st.id);
        let facs: Vec<String> = st
            .facilities
            .iter()
            .enumerate()
            .map(|(i, f)| {
                let mark = if f.mothballed { "(mothballed)" } else if !f.online { "(offline)" } else { "" };
                format!("{i}:{}{mark}", f.kind.name())
            })
            .collect();
        let q = queue_text(g, Place::State(st.id));
        let inf = g.seat(me).influence.get(&Place::State(st.id)).copied().unwrap_or(0);
        println!(
            "{:<16} {:<30} pop {:.1} ind {} unrest {} | free slots {}/{} (coastal {}) | your Standing {}, {} | emigrants {}",
            format!("{:?}", st.id),
            control_text(g, st.control),
            st.population,
            st.industry_level,
            Game::unrest_figure(st.unrest),
            g.free_slots(st.id),
            g.build_slots(st.id),
            g.free_coastal(st.id),
            inf,
            standing_note(g, me, Place::State(st.id)),
            st.emigrants
        );
        println!(
            "    {} | lean {} | GDP {} | facilities: {}",
            card.name,
            card.resource_lean.name(),
            card.gdp,
            if facs.is_empty() { "none".into() } else { facs.join(", ") }
        );
        println!("    {}", widgets_text(g, Place::State(st.id), &q));
        let note = g.unrest_note(st.id);
        if !note.is_empty() {
            println!("    {note}");
        }
    }

    println!("\n--- COLONIES AND STATIONS ---");
    if g.colonies.is_empty() {
        println!("none anywhere");
    }
    for c in &g.colonies {
        let mods: Vec<String> = c
            .modules
            .iter()
            .enumerate()
            .map(|(i, m)| {
                let mark = if m.mothballed { "(mothballed)" } else if !m.online { "(offline)" } else { "" };
                format!("{i}:{}{mark}", m.kind.name())
            })
            .collect();
        let q = queue_text(g, Place::Colony(c.id));
        let name = if c.in_orbit { g.station_name(c.body, c.slot) } else { slot_name(g, c.body, c.slot) };
        println!(
            "colony {:<3} {:<22} {:<7} {:<30} colonists {} (room {}) | Modules {}/{} | yields {}",
            c.id.0,
            name,
            if c.in_orbit { "orbit" } else { "ground" },
            control_text(g, c.control),
            c.colonists,
            g.habitat_room(c),
            g.module_slots_used(c),
            g.module_slots(c),
            g.colony_yields(c).text()
        );
        if c.in_orbit {
            let by = g.slot_blockaders(c.body, c.slot);
            let rivals: Vec<String> = by.iter().filter(|s| **s != me).map(|s| g.seat_name(*s)).collect();
            if !rivals.is_empty() {
                println!("    *** BLOCKADED by the {} : no unloading here, and it refuels nothing ***", rivals.join(" and the "));
            }
        }
        println!(
            "    your Standing {}, {}",
            g.seat(me).influence.get(&Place::Colony(c.id)).copied().unwrap_or(0),
            standing_note(g, me, Place::Colony(c.id))
        );
        // Ticket #335: the ORBIT a Ship must sit in to touch this place -- its own for a station,
        // low orbit for a Colony on the ground, since low orbit is what touches the ground.
        let reached = if c.in_orbit {
            g.orbit_name(c.body, g.colony_orbit(c))
        } else {
            format!("on the ground at {}, reached from {}", c.body.name(), g.orbit_name(c.body, Orbit::Low))
        };
        println!("    at {reached} | modules: {}", if mods.is_empty() { "none".into() } else { mods.join(", ") });
        println!("    {}", widgets_text(g, Place::Colony(c.id), &q));
    }

    println!("\n--- FREE SLOTS ---");
    for b in BodyId::ALL {
        let ground: Vec<String> = g.free_slots_on(b).iter().map(|n| format!("{n} {} [{}]", slot_name(g, b, *n), g.slot_yields(b, *n).text())).collect();
        let orbit: Vec<String> = g.free_orbital_slots(b).iter().map(|n| format!("{n} {}", g.station_name(b, *n))).collect();
        // Ticket #335 (version 0.09.0): every orbit of the Body, free or not -- the numbers a
        // `transit` or a `change-orbit` line names.
        let orbits: Vec<String> = g
            .orbits_of(b)
            .iter()
            .map(|o| match o {
                Orbit::Low => "low".to_string(),
                Orbit::Slot(n) => format!("{n} {}", g.station_name(b, *n)),
            })
            .collect();
        println!(
            "{:<7} ground: {}\n        orbital: {}\n        orbits to fly to: {}",
            b.name(),
            if ground.is_empty() { "none".into() } else { ground.join("; ") },
            if orbit.is_empty() { "none".into() } else { orbit.join("; ") },
            orbits.join(" | ")
        );
    }

    println!("\n--- SHIPS AND ARMIES ---");
    if g.ships.is_empty() && g.armies.is_empty() {
        println!("none anywhere");
    }
    for sh in &g.ships {
        // Ticket #335 (version 0.09.0): the ORBIT, not the bare Body and a slot number, so a Ship at
        // a station's ring can be told from one in low orbit without doing the arithmetic.
        println!(
            "ship {:<3} {:<12} seat {} at {:<34} tank {}/{} | colonists {} | army {:?} | hp {} | {}{}",
            sh.id.0,
            sh.kind.name(),
            sh.seat.0,
            ship_at_text(g, sh),
            sh.fuel,
            t.unit(sh.kind).tank,
            sh.colonists,
            sh.army.map(|a| a.0),
            t.unit(sh.kind).hit_points as i64 - sh.damage as i64,
            sh.stance.name(),
            if g.stranded(sh.id) { "  *** STRANDED ***" } else { "" }
        );
    }
    for a in &g.armies {
        let at = match a.at {
            ArmyAt::Place(p) => g.place_name(p),
            ArmyAt::Aboard(s) => format!("aboard ship {}", s.0),
        };
        println!(
            "army {:<3} seat {:?} at {:<26} hp {} | {}{}",
            a.id.0,
            g.army_seat(a).map(|s| s.0),
            at,
            t.unit(UnitKind::Army).hit_points as i64 - a.damage as i64,
            a.stance.name(),
            if a.standing { " (standing)" } else { "" }
        );
    }

    println!("\n--- FLYING (turns/Fuel, at your Faction's rate, from this turn) ---");
    for from in BodyId::ALL {
        let legs: Vec<String> = BodyId::ALL
            .iter()
            .filter(|to| **to != from && Game::leg_allowed(from, **to))
            .map(|to| {
                let (turns, fuel) = g.transit_cost_for(me, from, *to);
                format!("{} {}t/{}F", to.name(), turns, fuel)
            })
            .collect();
        if !legs.is_empty() {
            println!("from {:<9}: {}", from.name(), legs.join(" | "));
        }
    }
    for b in [BodyId::Mars, BodyId::Venus] {
        println!("{}", g.window_text(b));
    }
    println!(
        "Colony Ship capacity: {} safe, {} crowded; lifting one Colonist costs {:.2} population; Pioneers a turn: {}",
        g.colony_ship_capacity(me),
        g.colony_ship_crowded_capacity(me),
        g.lift_population(me, 1),
        g.emigrants_per_turn(me)
    );
}

// ------------------------------------------------------------------ commands

fn flag(args: &[String], name: &str) -> Option<String> {
    args.iter().find_map(|a| a.strip_prefix(&format!("--{name}=")).map(|s| s.to_string())).or_else(|| {
        let i = args.iter().position(|a| a == &format!("--{name}"))?;
        args.get(i + 1).cloned()
    })
}

fn load(tables: Arc<Tables>, path: &Path) -> Game {
    match save::load_from(path, tables) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(2);
        }
    }
}

fn store(game: &Game, path: &Path) {
    let text = save::to_text(game, SaveKind::Manual).unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(2);
    });
    if let Some(dir) = path.parent().filter(|d| !d.as_os_str().is_empty()) {
        let _ = std::fs::create_dir_all(dir);
    }
    if let Err(e) = std::fs::write(path, text) {
        eprintln!("{} could not be written: {e}", path.display());
        std::process::exit(2);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let command = args.first().cloned().unwrap_or_else(|| "help".into());
    if command == "help" {
        // The figures in the help are the table's, not literals in this file, so a table that moves
        // one moves the help with it. Where the data cannot be found, the grammar still prints.
        println!("{}", grammar(Tables::load(&default_data_dir()).ok().as_ref()));
        return;
    }
    let tables = Arc::new(Tables::load(&default_data_dir()).expect("tables"));
    let path = PathBuf::from(flag(&args, "save").unwrap_or_else(|| {
        eprintln!("--save <file> is wanted");
        std::process::exit(2);
    }));

    match command.as_str() {
        "new" => {
            let seed: u64 = flag(&args, "seed").and_then(|s| s.parse().ok()).unwrap_or(1);
            let player = flag(&args, "faction").and_then(|f| FactionKind::from_id(&f)).unwrap_or_else(|| {
                eprintln!("--faction custodians|prospectors|arkwrights|archivists is wanted");
                std::process::exit(2);
            });
            let start = match flag(&args, "start") {
                None => StateId::EastAsia,
                Some(s) => pick(&StateId::ALL, &s).unwrap_or_else(|e| {
                    eprintln!("{e}");
                    std::process::exit(2);
                }),
            };
            let mut game = Game::new(tables, NewGame { seed, player, player_is_ai: false, player_start: start });
            game.start();
            print_report(&game);
            print_board(&game);
            print_costs(&game);
            store(&game, &path);
            println!("\nSaved to {}. Run the `help` command for the order grammar.", path.display());
        }
        "show" => {
            let game = load(tables, &path);
            print_report(&game);
            print_board(&game);
            if args.iter().any(|a| a == "--costs") {
                print_costs(&game);
            }
            if let Some(n) = flag(&args, "log").and_then(|s| s.parse::<usize>().ok()) {
                println!("\n--- LOG, last {n} lines ---");
                let tail: Vec<&String> = game.log.iter().rev().take(n).collect();
                for l in tail.into_iter().rev() {
                    println!("{l}");
                }
            }
        }
        "turn" | "check" => {
            let mut game = load(tables, &path);
            if game.is_over() {
                println!("{}", game.outcome_text());
                return;
            }
            let text = match flag(&args, "orders") {
                Some(f) => std::fs::read_to_string(&f).unwrap_or_else(|e| {
                    eprintln!("{f} could not be read: {e}");
                    std::process::exit(2);
                }),
                None => String::new(),
            };
            let mut kept: Vec<Order> = Vec::new();
            let mut bad = 0;
            for (n, raw) in text.lines().enumerate() {
                let line = raw.split('#').next().unwrap_or("").trim();
                if line.is_empty() {
                    continue;
                }
                match parse_line(&game, line) {
                    Err(e) => {
                        println!("line {}: REFUSED `{line}`: {e}", n + 1);
                        bad += 1;
                    }
                    // Ticket #337 (version 0.09.0): answered the moment the line is read, as a
                    // `tech` pick is, so it is given before the turn it holds can end.
                    Ok(Line::Answer(taken)) => match game.answer_card(Seat(0), taken) {
                        Ok(()) => println!("line {}: answered: you {}", n + 1, if taken { "take the offer" } else { "refuse it" }),
                        Err(e) => {
                            println!("line {}: REFUSED `{line}`: {e}", n + 1);
                            bad += 1;
                        }
                    },
                    Ok(Line::Tech(t)) => match game.pick_tech(Seat(0), t) {
                        Ok(()) => println!("line {}: picked {}", n + 1, game.tables.tech(t).name),
                        Err(e) => {
                            println!("line {}: REFUSED `{line}`: {e}", n + 1);
                            bad += 1;
                        }
                    },
                    Ok(Line::Order(o)) => match game.check_order(Seat(0), &kept, &o) {
                        Ok(cost) => {
                            println!("line {}: ok `{line}` costs {}", n + 1, cost.text());
                            kept.push(*o);
                        }
                        Err(e) => {
                            println!("line {}: REFUSED `{line}`: {}", n + 1, e.0);
                            bad += 1;
                        }
                    },
                }
            }
            let (left, influence) = game.remaining(Seat(0), &kept);
            println!(
                "\n{} order(s) stand, {bad} refused. After them: {}M {}F {}E {}D, {} Influence left of the allotment.",
                kept.len(),
                left.materials,
                left.fuel,
                left.energy,
                left.ducats,
                influence
            );
            // Ticket #337 (version 0.09.0): the turn's question holds End Turn. The engine refuses
            // in its own words; this says which card is asking and what line answers it, because a
            // line is the only door the driver has and a raw refusal names none.
            let owed = owed_answer(&game);
            if let Some(why) = &owed {
                println!("\nSTILL OWED: {why}");
            }
            if command == "check" {
                return;
            }
            if bad > 0 && !args.iter().any(|a| a == "--force") {
                eprintln!("\nThe turn was NOT ended: {bad} line(s) were refused. Fix them, or pass --force to end the turn with the rest.");
                std::process::exit(1);
            }
            if let Some(why) = owed {
                eprintln!("\nThe turn was NOT ended: {why}");
                std::process::exit(1);
            }
            let mut all: [Vec<Order>; SEAT_COUNT] = std::array::from_fn(|_| Vec::new());
            all[0] = kept;
            // Ticket #105 (version 0.07.0): the engine owns the rule, so the driver is bound by it
            // too. This is the whole point: what the driver measures is what the game does.
            if let Err(why) = game.end_turn(all) {
                eprintln!("
The turn did NOT end: {why}");
                std::process::exit(1);
            }
            store(&game, &path);
            print_report(&game);
            print_board(&game);
            if game.is_over() {
                println!("\n*** {} ***", game.outcome_text());
            }
        }
        other => {
            eprintln!("`{other}` is not a command: new, show, check, turn, help");
            std::process::exit(2);
        }
    }
}
