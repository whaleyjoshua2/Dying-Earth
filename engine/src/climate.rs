//! The Climate phase and the Climate Model (spec 11, 13.1).

use crate::data::{BreakCard, BreakEffect};
use crate::ids::*;
use crate::state::*;

/// Ticket #55: what the Climate Panel says about the last turn on which cutting net Emissions to
/// zero still avoids Collapse.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LastTurn {
    /// Cut from this turn's Climate phase onward and the Temperature stays under the Collapse Line
    /// to the last turn; cut any later and it does not.
    Turn(u32),
    /// No turn works any more, this one included: cuts alone no longer avoid Collapse.
    TooLate,
    /// The path as it stands never reaches the Collapse Line.
    NoCollapse,
}

#[derive(Debug, Clone)]
pub struct Projection {
    pub temperature_at_last_turn: f64,
    pub collapse_turn: Option<u32>,
}

impl Game {
    /// Phase 2: Climate.
    pub fn climate_phase(&mut self) {
        // Ticket #52: the turn's Unrest bookkeeping starts here, since the Climate phase opens the
        // turn's rises and the falls are settled at the end of its Resolution.
        for s in &mut self.states {
            s.refugees_in = 0.0;
        }
        let t = self.tables.clone();
        let c = &t.climate;
        let breakdown = self.emissions_now();
        let net = breakdown.net();
        self.climate.co2 += net;
        // Ticket #53: Blame. Each Faction takes on what the sources it controls emitted this turn
        // and is credited with what it removed; both totals stand for the whole game. Ticket #54:
        // what it removed is its Scrubbers, read off the same board `emissions_now` just read.
        let removed_by_seat = self.scrubber_removal_by_seat();
        for seat in Seat::ALL {
            let i = seat.index();
            let (emitted, removed) = (breakdown.by_seat[i], removed_by_seat[i]);
            let s = self.seat_mut(seat);
            s.blame_emitted += emitted;
            s.blame_removed += removed;
        }
        self.climate.launches_pending = [0; SEAT_COUNT];
        self.climate.card_emissions_next = 0.0;
        for s in &mut self.states {
            s.wildfire_emissions_next = 0.0;
        }
        // Stabilization runs (spec 15): net counted Emissions below the Sink.
        let stabilized = breakdown.counted() < breakdown.total_sink();
        for seat in Seat::ALL {
            let s = self.seat_mut(seat);
            if stabilized {
                s.stabilization_run += 1;
            } else {
                s.stabilization_run = 0;
            }
        }
        // Temperature follows the stock with a lag.
        let target = self.target_temperature();
        let temp = self.climate.temperature + (target - self.climate.temperature) * c.temperature_lag_fraction;
        self.climate.temperature = temp.max(c.base_temperature);
        self.climate.last = breakdown.clone();
        self.log(format!(
            "Climate: emissions {:.1} (industry {:.1}, factories {:.1}, power {:.1}, refineries {:.1}, launches {:.1}, population {:.1}, cards {:.1}, permafrost {:.1}), sink {:.1} (Scrubbers {:.1}), net {:+.1}; CO2 {:.1} ppm; temperature {:+.2} heading to {:+.2}.",
            breakdown.total(),
            breakdown.state_industry,
            breakdown.factories,
            breakdown.power_plants,
            breakdown.refineries,
            breakdown.launches,
            breakdown.population,
            breakdown.cards,
            breakdown.permafrost,
            breakdown.total_sink(),
            breakdown.scrubbers,
            net,
            self.climate.co2,
            self.climate.temperature,
            target
        ));
        // Ticket #55: the Breaks, read off the Temperature the phase has just settled, before the
        // scheduled Sea Level check, so an Ice Sheets threshold and a scheduled one can both land
        // in the same phase.
        self.break_check();
        self.sea_level_check();
        self.population_change();
        self.neutral_development();
        // Ticket #52: a Resettle order steers only the flows of the Climate phase that follows it.
        for seat in Seat::ALL {
            self.seat_mut(seat).resettle_to = None;
        }
    }

    /// Ticket #53, Neutral Development: a neutral Nation State below the ceiling raises its
    /// Industry Level by one every `turns` turns of unbroken neutrality, and brings the first idle
    /// Facility in its list online with it. The clock is per state and does not run while a Faction
    /// holds the place. A world at `stops_at_temperature` or above develops nothing, and neither
    /// does a state whose Unrest has passed `no_development_at` (`may_develop`); a state held back
    /// that way keeps its place in the queue and develops the moment the block lifts.
    pub fn neutral_development(&mut self) {
        let d = self.tables.development.clone();
        let turn = self.turn;
        let hot = self.climate.temperature >= d.stops_at_temperature;
        for sid in StateId::ALL {
            if self.state(sid).control != Control::Neutral {
                self.state_mut(sid).neutral_since = None;
                continue;
            }
            let since = *self.state_mut(sid).neutral_since.get_or_insert(turn);
            if (turn + 1).saturating_sub(since) < d.turns {
                continue;
            }
            if hot || self.state(sid).industry_level >= d.max_level || !self.may_develop(sid) {
                continue;
            }
            self.state_mut(sid).industry_level += 1;
            self.state_mut(sid).neutral_since = Some(turn + 1);
            let level = self.state(sid).industry_level;
            // The state takes one of its start Facilities into its own hands: it comes online and
            // runs itself, emitting at x1.0 to nobody's Blame while the state stays neutral.
            let woken = self.state_mut(sid).facilities.iter_mut().find(|f| !f.self_run).map(|f| {
                f.online = true;
                f.self_run = true;
                f.kind.name()
            });
            let name = self.tables.state(sid).name.clone();
            let line = match woken {
                Some(k) => format!("{name} raised its Industry Level to {level} and brought a {k} online."),
                None => format!("{name} raised its Industry Level to {level}."),
            };
            self.log(line.clone());
            self.report.lines.push(line);
        }
    }

    pub fn target_temperature(&self) -> f64 {
        let c = &self.tables.climate;
        c.base_temperature + c.degrees_per_ppm_step * (self.climate.co2 - c.starting_co2) / c.ppm_step
    }

    /// Emissions from what stands now, by source (spec 11.2).
    pub fn emissions_now(&self) -> EmissionsBreakdown {
        let t = &self.tables;
        let c = &t.climate;
        let mut b = EmissionsBreakdown::default();
        let mult = |seat: Option<Seat>| seat.map(|s| t.faction(self.kind(s)).emissions_multiplier).unwrap_or(1.0);
        let pop_mult = if self.has_tech(TechId::GreenConsensus) { t.tech(TechId::GreenConsensus).value } else { 1.0 };
        let pp_mult = if self.has_tech(TechId::CleanPower) { t.tech(TechId::CleanPower).value } else { 1.0 };
        let fr_mult = if self.has_tech(TechId::CleanManufacturing) { t.tech(TechId::CleanManufacturing).value } else { 1.0 };
        for st in &self.states {
            let director = st.control.director();
            let m = mult(director);
            // Ticket #53: whoever directs the state at this Climate phase wears its figure; a
            // neutral state's industry and people are nobody's Blame.
            // Ticket #54: the state's own Baseline Emissions, which a spent Strip Permit raises for
            // good, and its own per-person coefficient, which Leapfrog lowers for good.
            let industry = self.baseline_emissions(st.id) * st.industry_level as f64 * m;
            let people = self.population_coefficient(st.id) * st.population * pop_mult * m;
            b.state_industry += industry;
            b.population += people;
            let mut worn = industry + people;
            // A Facility nobody directs stands idle: it makes nothing and emits nothing (ticket #24),
            // unless the state developed itself and runs it (ticket #53): then it emits at x1.0, to
            // nobody's Blame.
            if director.is_none() {
                let half = if self.facilities_at_half(st.id) { 0.5 } else { 1.0 };
                for f in st.facilities.iter().filter(|f| f.self_run && f.working()) {
                    let e = t.facility(f.kind).emissions * half;
                    match f.kind {
                        FacilityKind::Factory => b.factories += e * fr_mult,
                        FacilityKind::Refinery => b.refineries += e * fr_mult,
                        FacilityKind::PowerPlant => b.power_plants += e * pp_mult,
                        _ => {}
                    }
                }
            }
            if let Some(d) = director {
                for f in &st.facilities {
                    // Ticket #54: a mothballed Facility emits nothing at all.
                    if !f.working() {
                        continue;
                    }
                    // Ticket #52: at Unrest 7 every Facility in the state emits at half.
                    let e = t.facility(f.kind).emissions * if self.facilities_at_half(st.id) { 0.5 } else { 1.0 };
                    let charged = match f.kind {
                        FacilityKind::Factory | FacilityKind::Refinery => e * fr_mult * m,
                        FacilityKind::PowerPlant => e * pp_mult * m,
                        _ => 0.0,
                    };
                    match f.kind {
                        FacilityKind::Factory => b.factories += charged,
                        FacilityKind::PowerPlant => b.power_plants += charged,
                        FacilityKind::Refinery => b.refineries += charged,
                        _ => {}
                    }
                    worn += charged;
                }
                b.by_seat[d.index()] += worn;
                // An Event card is the world's doing, not a Faction's, so it is nobody's Blame.
                // It stays inside the directed branch, where it has been since ticket #24: a
                // Wildfire on a state nobody holds charges nothing at all today. That is almost
                // certainly an accident of where the old `continue` sat rather than a rule, but
                // moving it would change the Climate Model, so it is left for its own ticket.
                b.cards += st.wildfire_emissions_next;
            }
        }
        // Version 0.04 (ticket #44): a Module on Earth (Antarctica) emits as its counterpart Facility does.
        for col in self.colonies.iter().filter(|c| c.body == BodyId::Earth) {
            let Some(d) = col.control.director() else { continue };
            let m = mult(Some(d));
            for md in col.modules.iter().filter(|md| md.working()) {
                let e = t.module(md.kind).earth_emissions;
                let charged = match md.kind {
                    ModuleKind::Mine | ModuleKind::Refinery => e * fr_mult * m,
                    ModuleKind::Generator => e * pp_mult * m,
                    _ => 0.0,
                };
                match md.kind {
                    ModuleKind::Mine => b.factories += charged,
                    ModuleKind::Generator => b.power_plants += charged,
                    ModuleKind::Refinery => b.refineries += charged,
                    _ => {}
                }
                // Ticket #53: an Antarctic Module is its Faction's, as a Facility is.
                b.by_seat[d.index()] += charged;
            }
        }
        b.cards += self.climate.card_emissions_next;
        let per_launch = if self.has_tech(TechId::CleanPropellant) { t.tech(TechId::CleanPropellant).value } else { c.launch_emissions };
        for seat in Seat::ALL {
            let charged = self.climate.launches_pending[seat.index()] as f64 * per_launch * mult(Some(seat));
            b.launches += charged;
            // Ticket #53: a Faction wears its own launches, wherever they lifted from.
            b.by_seat[seat.index()] += charged;
        }
        // Ticket #55: the live Sink, which the Sink Weakens Break lowers for good, and the
        // Permafrost line, which is the world's: nobody's Blame and outside `counted()`.
        b.sink = self.climate.natural_sink;
        b.permafrost = self.climate.permafrost;
        // Ticket #54: the Scrubbers standing and online enlarge the Natural Sink this phase.
        b.scrubbers = self.scrubber_removal();
        b
    }

    /// Sea level (spec 11.4): each threshold fires once per state.
    fn sea_level_check(&mut self) {
        let thresholds = self.tables.climate.sea_level_thresholds.clone();
        let temp = self.climate.temperature;
        for (i, thr) in thresholds.iter().enumerate() {
            if temp < *thr {
                continue;
            }
            for sid in StateId::ALL {
                if !self.state(sid).thresholds_fired[i] {
                    self.apply_sea_threshold(sid, i);
                }
            }
        }
    }

    /// Apply sea-level threshold `i` to one state now (also used by Storm Surge): it is marked
    /// fired for that state, so it never fires again there.
    pub fn apply_sea_threshold(&mut self, sid: StateId, i: usize) {
        let thr = self.tables.climate.sea_level_thresholds[i];
        self.state_mut(sid).thresholds_fired[i] = true;
        self.apply_sea_loss(sid, thr);
    }

    /// Ticket #55: one Sea Level threshold's slot loss, displacement and Unrest, without marking a
    /// scheduled index. The Ice Sheets Committed Break fires one out of sequence this way, so the
    /// three scheduled thresholds still fire on their own turns and a game reaching +2.8 takes four.
    pub fn apply_sea_loss(&mut self, sid: StateId, thr: f64) {
        let exposure = self.tables.state(sid).coastal_exposure;
        let name = self.tables.state(sid).name.clone();
        if exposure == 0 {
            return;
        }
        self.state_mut(sid).lost_slots += exposure;
        // Ticket #52: two Unrest per build slot the sea took, and 5% of the people per point of
        // Coastal Exposure driven out, half of them to the neighbours.
        let u = self.tables.unrest.clone();
        let per_slot = u.per_sea_level_slot * exposure as f64;
        let rose = self.raise_unrest(sid, per_slot, UnrestSource::Climate);
        let displaced = self.state(sid).population * u.sea_loss_per_exposure * exposure as f64;
        if displaced > 0.0 {
            let p = &mut self.state_mut(sid).population;
            *p = (*p - displaced).max(0.0);
            self.move_refugees(sid, displaced * u.sea_share, "the sea");
        }
        let slots = self.build_slots(sid);
        let mut destroyed = Vec::new();
        loop {
            let st = self.state(sid);
            if (st.facilities.len() as u32) <= slots {
                break;
            }
            // Highest upkeep first.
            let (idx, _) = st
                .facilities
                .iter()
                .enumerate()
                .max_by_key(|(_, f)| self.tables.facility(f.kind).energy_upkeep)
                .unwrap();
            let f = self.state_mut(sid).facilities.remove(idx);
            destroyed.push(f.kind.name().to_string());
        }
        // Queued Facilities beyond the slots are lost too, without refund.
        loop {
            if self.slots_used(sid) <= slots {
                break;
            }
            let st = self.state_mut(sid);
            if let Some(pos) = st.queue.iter().rposition(|b| matches!(b.item, BuildItem::Facility(_))) {
                let b = st.queue.remove(pos);
                destroyed.push(format!("{} under construction", b.item.name()));
            } else {
                break;
            }
        }
        let mut line = if destroyed.is_empty() {
            format!("Sea level at {thr:+.1} C: {name} lost {exposure} build slots.")
        } else {
            format!("Sea level at {thr:+.1} C: {name} lost {exposure} build slots; destroyed {}.", destroyed.join(", "))
        };
        if rose > 0.0 {
            line.push_str(&format!(" Unrest there rose by {} to {}.", Game::unrest_figure(rose), self.unrest_text(sid)));
        }
        self.report.lines.push(line.clone());
        self.log(line);
    }

    /// Spec 11.3: growth less 0.15% per full 0.1 C above +1.2.
    pub fn population_growth_rate(&self) -> f64 {
        let c = &self.tables.climate;
        let tenths = ((self.climate.temperature - c.base_temperature) / 0.1 + 1e-9).floor().max(0.0);
        c.population_growth - c.population_loss_per_tenth_degree * tenths
    }

    fn population_change(&mut self) {
        let rate = self.population_growth_rate();
        let u = self.tables.unrest.clone();
        // Every state's own change first, so what arrives as refugees is not scaled again by the
        // same turn's growth; then the Unrest and the flow, state by state, so the Report reads
        // "population fell here, and this many left for there".
        let mut fell: Vec<(StateId, f64, f64)> = Vec::new();
        for sid in StateId::ALL {
            let before = self.state(sid).population;
            let after = (before * (1.0 + rate)).max(0.0);
            self.state_mut(sid).population = after;
            if after < before && before > 0.0 {
                fell.push((sid, before, after));
            }
        }
        for (sid, before, after) in fell {
            // Ticket #52: a fall raises Unrest by one, by two when it is more than one per cent.
            let lost = before - after;
            let big = lost / before > u.population_fall_big_fraction;
            let n = if big { u.population_fall_big } else { u.population_fall };
            let rose = self.raise_unrest(sid, n, UnrestSource::Climate);
            if rose > 0.0 {
                let line = format!(
                    "{}: population fell {:.1}% to {:.1}; Unrest rose by {} to {}.",
                    self.tables.state(sid).name,
                    100.0 * lost / before,
                    after,
                    Game::unrest_figure(rose),
                    self.unrest_text(sid)
                );
                self.log(line.clone());
                self.report.lines.push(line);
            }
            // Ticket #52: half of what the heat took moves to the neighbours instead of vanishing.
            self.move_refugees(sid, lost * u.heat_share, "the heat");
        }
    }

    /// Ticket #52: `amount` population leaves `from` for its neighbours, split in proportion to
    /// Industry Level (equally if every neighbour is at 0), or entirely to the state a Resettle
    /// order named this turn. Flows go to neutral and controlled neighbours alike; what arrives is
    /// added to the receiver, so its Population Emissions and Research weight follow it.
    pub fn move_refugees(&mut self, from: StateId, amount: f64, why: &str) {
        if amount <= 0.0 {
            return;
        }
        // Resettle (rule 9): whoever directs `from` may have named one state for all its flows.
        let steered = self
            .state(from)
            .control
            .director()
            .and_then(|s| self.seat(s).resettle_to)
            .filter(|t| *t != from);
        let targets: Vec<StateId> = match steered {
            Some(t) => vec![t],
            None => self.tables.state(from).neighbours.clone(),
        };
        if targets.is_empty() {
            return;
        }
        let weights: Vec<f64> = targets.iter().map(|t| self.state(*t).industry_level as f64).collect();
        let total: f64 = weights.iter().sum();
        let each = 1.0 / targets.len() as f64;
        let mut names: Vec<String> = Vec::new();
        for (i, t) in targets.iter().enumerate() {
            let share = if total > 0.0 { weights[i] / total } else { each };
            let n = amount * share;
            if n <= 0.0 {
                continue;
            }
            let st = self.state_mut(*t);
            st.population += n;
            st.refugees_in += n;
            names.push(self.tables.state(*t).name.clone());
        }
        if names.is_empty() || amount < 0.05 {
            return;
        }
        let list = match names.len() {
            1 => names[0].clone(),
            n => format!("{} and {}", names[..n - 1].join(", "), names[n - 1]),
        };
        let line = format!("{:.1} population left {} for {} ({}).", amount, self.tables.state(from).name, list, why);
        self.log(line.clone());
        self.report.lines.push(line);
    }

    /// Repeat the current net to the last turn (spec 11.5).
    pub fn projection(&self) -> Projection {
        let c = &self.tables.climate;
        let net = self.climate.last.net();
        let mut co2 = self.climate.co2;
        let mut temp = self.climate.temperature;
        let mut collapse = None;
        let last = self.tables.victory.turns;
        if temp >= c.collapse_line {
            collapse = Some(self.turn);
        }
        for turn in (self.turn + 1)..=last {
            co2 += net;
            let target = c.base_temperature + c.degrees_per_ppm_step * (co2 - c.starting_co2) / c.ppm_step;
            temp += (target - temp) * c.temperature_lag_fraction;
            temp = temp.max(c.base_temperature);
            if collapse.is_none() && temp >= c.collapse_line {
                collapse = Some(turn);
            }
        }
        Projection { temperature_at_last_turn: temp, collapse_turn: collapse }
    }

    // ---------------------------------------------------------------- Ticket #55: the Breaks

    /// The next Break ahead of the Temperature now, for the Climate Panel's bar.
    pub fn next_break(&self) -> Option<&crate::data::BreakCard> {
        self.tables
            .climate
            .breaks
            .iter()
            .enumerate()
            .find(|(i, _)| !self.climate.breaks_fired[*i])
            .map(|(_, b)| b)
    }

    /// Ticket #55: each Break fires once, in the first Climate phase whose Temperature stands at or
    /// above its figure, and what it does is permanent. A log line and a Report line name it.
    fn break_check(&mut self) {
        let breaks = self.tables.climate.breaks.clone();
        for (i, b) in breaks.iter().enumerate() {
            if self.climate.breaks_fired[i] || self.climate.temperature < b.temperature {
                continue;
            }
            self.climate.breaks_fired[i] = true;
            let line = format!("Break at {:+.1} C - {}. {} {}", b.temperature, b.name, b.happened, b.text);
            self.log(line.clone());
            self.report.lines.push(line);
            self.apply_break(b);
        }
    }

    fn apply_break(&mut self, b: &BreakCard) {
        match b.effect {
            // Coral Die-off: no climate effect at all. It costs the exposed coasts their people and
            // their calm, and what leaves flows as refugees by the population-fall rule of #52.
            BreakEffect::CoastalUnrest => {
                let share = self.tables.unrest.heat_share;
                let mut hit: Vec<String> = Vec::new();
                for sid in StateId::ALL {
                    if self.tables.state(sid).coastal_exposure != b.exposure {
                        continue;
                    }
                    self.raise_unrest(sid, b.unrest, UnrestSource::Climate);
                    hit.push(self.tables.state(sid).name.clone());
                    let lost = self.state(sid).population * b.population_loss;
                    if lost <= 0.0 {
                        continue;
                    }
                    let p = &mut self.state_mut(sid).population;
                    *p = (*p - lost).max(0.0);
                    self.move_refugees(sid, lost * share, "the reefs");
                }
                if !hit.is_empty() {
                    let line = format!(
                        "{}: every state at Coastal Exposure {} lost {:.0}% of its people and took {} Unrest.",
                        hit.join(", "),
                        b.exposure,
                        b.population_loss * 100.0,
                        Game::unrest_figure(b.unrest)
                    );
                    self.log(line.clone());
                    self.report.lines.push(line);
                }
            }
            // Permafrost Thaw: its own Emissions line from now on, the world's and nobody's Blame.
            BreakEffect::EmissionsPerTurn => self.climate.permafrost += b.emissions,
            // The Sink Weakens: the bar Stabilization is measured against comes down with it.
            BreakEffect::WeakenSink => self.climate.natural_sink = b.sink_after,
            // Ice Sheets Committed: a Sea Level threshold at once, for every state, out of sequence.
            BreakEffect::SeaLevelThreshold => {
                for sid in StateId::ALL {
                    self.apply_sea_loss(sid, b.temperature);
                }
            }
            // Amazon Dieback: a pulse into the CO2 Stock and a state left dirtier for good.
            BreakEffect::CarbonPulse => {
                self.climate.co2 += b.co2;
                if let Some(sid) = b.state {
                    self.state_mut(sid).baseline_rise += b.baseline_rise;
                    let line = format!("{}: its Baseline Emissions rise by {:.1} for good.", self.tables.state(sid).name, b.baseline_rise);
                    self.log(line.clone());
                    self.report.lines.push(line);
                }
            }
        }
    }

    // ---------------------------------------------------------------- Ticket #55: the Last Turn

    /// Ticket #55: the latest turn on which cutting net Emissions to zero from that turn onward
    /// still keeps the Temperature under the Collapse Line to the last turn.
    pub fn last_turn_to_act(&self) -> LastTurn {
        if !self.collapses_with_cut_at(None) {
            return LastTurn::NoCollapse;
        }
        let mut best = None;
        for turn in (self.turn + 1)..=self.tables.victory.turns {
            if !self.collapses_with_cut_at(Some(turn)) {
                best = Some(turn);
            }
        }
        match best {
            Some(turn) => LastTurn::Turn(turn),
            None => LastTurn::TooLate,
        }
    }

    /// Run the projection forward to the last turn and say whether the Temperature reaches the
    /// Collapse Line, with the world's Emissions cut to the Sink from the Climate phase of `cut`
    /// (None: no cut at all, the path as it stands). Every Break the run crosses fires in it as it
    /// would in play: the Sink Weakens lowers the Sink, Permafrost adds its line, Amazon its pulse.
    /// A cut is a cut of NET Emissions, so it holds the world's output at the Sink as it stood at
    /// that turn -- which is why a Sink that weakens afterwards puts the world back over the line.
    fn collapses_with_cut_at(&self, cut: Option<u32>) -> bool {
        let c = &self.tables.climate;
        let e = &self.climate.last;
        let mut co2 = self.climate.co2;
        let mut temp = self.climate.temperature;
        let mut sink = self.climate.natural_sink + e.scrubbers;
        let mut permafrost = self.climate.permafrost;
        // Everything the world emits but the Permafrost line, which is not anybody's to cut.
        let mut gross = e.total() - e.permafrost;
        let mut fired = self.climate.breaks_fired.clone();
        if temp >= c.collapse_line {
            return true;
        }
        for turn in (self.turn + 1)..=self.tables.victory.turns {
            if cut == Some(turn) {
                gross = sink;
            }
            co2 += gross + permafrost - sink;
            let target = c.base_temperature + c.degrees_per_ppm_step * (co2 - c.starting_co2) / c.ppm_step;
            temp = (temp + (target - temp) * c.temperature_lag_fraction).max(c.base_temperature);
            if temp >= c.collapse_line {
                return true;
            }
            for (i, b) in c.breaks.iter().enumerate() {
                if fired[i] || temp < b.temperature {
                    continue;
                }
                fired[i] = true;
                match b.effect {
                    BreakEffect::EmissionsPerTurn => permafrost += b.emissions,
                    BreakEffect::WeakenSink => sink = b.sink_after + e.scrubbers,
                    BreakEffect::CarbonPulse => co2 += b.co2,
                    // Neither the reefs nor the ice sheets move the carbon; they cost people and coasts.
                    BreakEffect::CoastalUnrest | BreakEffect::SeaLevelThreshold => {}
                }
            }
        }
        false
    }
}
