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

use dying_earth_engine::data::{default_data_dir, Tables};
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
const STANCES: [Stance; 4] = [Stance::Attack, Stance::Hold, Stance::Intercept, Stance::Evade];
const CHANGES: [BuildingChange; 3] = [BuildingChange::Mothball, BuildingChange::Restart, BuildingChange::Decommission];

fn number(word: &str) -> Result<i64, String> {
    word.parse::<i64>().map_err(|_| format!("{word:?} is not a whole number"))
}

fn count(word: &str) -> Result<u32, String> {
    word.parse::<u32>().map_err(|_| format!("{word:?} is not a whole number"))
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

/// `ship:4`, `army:2`, `ship4`, `army2`.
fn unit_ref(word: &str) -> Result<UnitRef, String> {
    let w = word.to_ascii_lowercase();
    if let Some(rest) = w.strip_prefix("ship") {
        return Ok(UnitRef::Ship(ShipId(count(rest.trim_start_matches(':'))?)));
    }
    if let Some(rest) = w.strip_prefix("army") {
        return Ok(UnitRef::Army(ArmyId(count(rest.trim_start_matches(':'))?)));
    }
    Err(format!("{word:?} should be ship:<n> or army:<n>"))
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

  tech <tech>                          pick the next shared Tech (when the board says one is awaited)

  build facility <state> <facility>    e.g. build facility europe factory
  build facility-ducats <state> <facility>
  build module <colony> <module>       e.g. build module 1 habitat
  build module-ducats <colony> <module>
  build ship <place> <ship>            place is a state (needs a Launch Site) or colony:<n>
  build army <place>
  build station <body> <slot>          e.g. build station moon 0
  build archive <colony>               the Archivists only
  industry <state>                     raise the Industry Level

  transit <ship> <body>                fly; spends the ship's own tank
  refuel <ship>                        fill the tank where you hold a station
  load <ship> <n> from <place> [army <army>]
  unload <ship> <n> [army] into slot <body> <slot>
  unload <ship> <n> [army] into colony <n>
  move-army <army> <state>
  ship-stance <body> <stance>          stance = attack|hold|intercept|evade
  army-stance <place> <stance>
  repair <ship:n|army:n> <points>
  repair-ducats <ship:n|army:n> <points>

  influence <place> <amount>
  buy-influence <amount>
  buy <resource> <amount>              materials|fuel|energy
  sell <resource> <amount>             materials|fuel
  relief <state>
  resettle <state>
  emigrants <state> <n>                muster Emigrants
  send-antarctica <state> <n> slot earth <slot>
  send-antarctica <state> <n> colony <n>
  change facility <state> <index> <mothball|restart|decommission>
  change module <colony> <index> <mothball|restart|decommission>
  fund-archive                         the Archivists only: pay the Labs into the Archive fund
  unfund-archive                       the Archivists only: pay them back into the shared Tech
  venture-share <percent>              the Prospectors only, a step of 10, 0 to 80
  draw-venture <amount>                the Prospectors only
  leapfrog <state>                     the Custodians only
  strip-permit <state>                 the Prospectors only

Names may be shortened as long as they stay unique: `eur` is Europe, `colonys` is a Colony Ship.
Colonies, ships and armies are named by the numbers the board prints beside them."#;

/// One line is an order, or the `tech` line, which is not one.
enum Line {
    Order(Box<Order>),
    Tech(TechId),
}

fn parse_line(line: &str) -> Result<Line, String> {
    let w: Vec<&str> = line.split_whitespace().collect();
    let at = |i: usize| -> Result<&str, String> { w.get(i).copied().ok_or_else(|| format!("`{}` wants more words", w[0])) };
    let verb = w[0].to_ascii_lowercase();
    let o = match verb.as_str() {
        "tech" => return Ok(Line::Tech(pick(&TechId::ALL, at(1)?)?)),
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
        "transit" => Order::Transit { ship: ship_id(at(1)?)?, to: pick(&BodyId::ALL, at(2)?)? },
        "refuel" => Order::Refuel { ship: ship_id(at(1)?)? },
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
        "buy" => Order::Buy { resource: pick(&RESOURCES, at(1)?)?, amount: number(at(2)?)? },
        "sell" => Order::Sell { resource: pick(&RESOURCES, at(1)?)?, amount: number(at(2)?)? },
        "relief" => Order::Relief { state: pick(&StateId::ALL, at(1)?)? },
        "resettle" => Order::Resettle { state: pick(&StateId::ALL, at(1)?)? },
        "emigrants" => Order::BuildEmigrants { state: pick(&StateId::ALL, at(1)?)?, n: count(at(2)?)? },
        "send-antarctica" => Order::SendToAntarctica { state: pick(&StateId::ALL, at(1)?)?, n: count(at(2)?)?, into: unload_target(&w[3..])? },
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
        "fund-archive" => Order::SetArchiveFunding { on: true },
        "unfund-archive" => Order::SetArchiveFunding { on: false },
        "venture-share" => Order::SetVentureShare { share: count(at(1)?)? },
        "draw-venture" => Order::DrawVenture { amount: number(at(1)?)? },
        "leapfrog" => Order::Leapfrog { state: pick(&StateId::ALL, at(1)?)? },
        "strip-permit" => Order::StripPermit { state: pick(&StateId::ALL, at(1)?)? },
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

fn ship_at_text(at: ShipAt) -> String {
    match at {
        ShipAt::Body(b) => b.name().to_string(),
        ShipAt::Transit { from, to, turns_left } => format!("{} -> {} ({turns_left} left)", from.name(), to.name()),
    }
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

fn slot_name(g: &Game, body: BodyId, slot: u32) -> String {
    g.tables.body(body).slots.get(slot as usize).map(|s| s.name.clone()).unwrap_or_else(|| format!("slot {slot}"))
}

fn print_costs(g: &Game) {
    let me = Seat(0);
    println!("\n--- WHAT THINGS COST YOU (Materials / build turns) ---");
    let f: Vec<String> = FacilityKind::ALL
        .iter()
        .map(|k| format!("{} {}/{}t", k.name(), g.facility_materials(me, *k), g.tables.facility(*k).build_turns))
        .collect();
    println!("Facilities: {}", f.join(" | "));
    let m: Vec<String> = ModuleKind::ALL
        .iter()
        .map(|k| format!("{} {}/{}t", k.name(), g.module_materials(me, *k), g.tables.module(*k).build_turns))
        .collect();
    println!("Modules (before the working-Mine discount): {}", m.join(" | "));
    let u: Vec<String> = UnitKind::SHIPS
        .iter()
        .map(|k| {
            let c = g.tables.unit(*k);
            format!("{} {}M+{}F/{}t tank {} hp {}", k.name(), g.ship_materials(me, *k), c.tank, c.build_turns, c.tank, c.hit_points)
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
    if g.research.awaiting_pick == Some(me) || g.research.current.is_none() {
        println!("  *** YOU MUST PICK THE NEXT TECH (a `tech <name>` line). Available: ***");
        for x in g.available_techs() {
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
        println!(
            "Archive fund: {} of a cap of {}; the Archive {}",
            s.archive_fund,
            g.archive_fund_cap(me),
            if g.archive_complete(me) { "is complete" } else if g.archive_built(me) { "stands" } else { "is not built" }
        );
    }

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
        let q: Vec<String> = st.queue.iter().map(|b| format!("{} due t{}", b.item.name(), b.due_turn)).collect();
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
        if !q.is_empty() {
            println!("    building: {}", q.join(", "));
        }
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
        let q: Vec<String> = c.queue.iter().map(|b| format!("{} due t{}", b.item.name(), b.due_turn)).collect();
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
        println!(
            "    your Standing {}, {}",
            g.seat(me).influence.get(&Place::Colony(c.id)).copied().unwrap_or(0),
            standing_note(g, me, Place::Colony(c.id))
        );
        println!(
            "    at {} | modules: {}{}",
            c.body.name(),
            if mods.is_empty() { "none".into() } else { mods.join(", ") },
            if q.is_empty() { String::new() } else { format!(" | building: {}", q.join(", ")) }
        );
    }

    println!("\n--- FREE SLOTS ---");
    for b in BodyId::ALL {
        let ground: Vec<String> = g.free_slots_on(b).iter().map(|n| format!("{n} {} [{}]", slot_name(g, b, *n), g.slot_yields(b, *n).text())).collect();
        let orbit: Vec<String> = g.free_orbital_slots(b).iter().map(|n| format!("{n} {}", g.station_name(b, *n))).collect();
        println!(
            "{:<7} ground: {}\n        orbital: {}",
            b.name(),
            if ground.is_empty() { "none".into() } else { ground.join("; ") },
            if orbit.is_empty() { "none".into() } else { orbit.join("; ") }
        );
    }

    println!("\n--- SHIPS AND ARMIES ---");
    if g.ships.is_empty() && g.armies.is_empty() {
        println!("none anywhere");
    }
    for sh in &g.ships {
        println!(
            "ship {:<3} {:<12} seat {} at {:<26} tank {}/{} | colonists {} | army {:?} | hp {} | {}{}",
            sh.id.0,
            sh.kind.name(),
            sh.seat.0,
            ship_at_text(sh.at),
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
        "Colony Ship capacity: {} safe, {} crowded; lifting one Colonist costs {:.2} population; Emigrants a turn: {}",
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
        println!("{GRAMMAR}");
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
                match parse_line(line) {
                    Err(e) => {
                        println!("line {}: REFUSED `{line}`: {e}", n + 1);
                        bad += 1;
                    }
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
            if command == "check" {
                return;
            }
            if bad > 0 && !args.iter().any(|a| a == "--force") {
                eprintln!("\nThe turn was NOT ended: {bad} line(s) were refused. Fix them, or pass --force to end the turn with the rest.");
                std::process::exit(1);
            }
            let mut all: [Vec<Order>; SEAT_COUNT] = std::array::from_fn(|_| Vec::new());
            all[0] = kept;
            game.end_turn(all);
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
