//! Ticket #548 (version 0.1.0.2): **map labels that give way.** Every map label is laid out again
//! each frame for where it lands in this view; where two would collide the one ranked lower first
//! shrinks to its glyph and name, and hides if even that collides. Fixed marks (shields, Battle
//! marks, glyphs, the orbit band) never give way, and a label does not give way to its own place's
//! marks. The selected label and the one under the pointer are laid over the marks rather than
//! giving way to them, so pointing at a place always shows its name (the designer, 2026-10-19). This module decides the forms; `ui.rs` gathers the labels and paints them.
//!
//! Two dampers keep a turning view from flickering: a label that has given way grows back only when
//! the larger form would clear everything by `MARGIN`, and a change of form fades over `FADE`.

use bevy_egui::egui::{Id, Rect};
use std::collections::HashMap;

/// The pixels a label that has given way must have to spare before it grows back.
pub const MARGIN: f32 = 6.0;
/// The pixels two blocks may overlap and still count as clear: a block pads its text by two pixels
/// a side, so two backgrounds that kiss leave both texts whole. Without it the Solar System Map's
/// stack labels, set 16 pixels apart in blocks 19 tall, knocked each other out.
pub const SLACK: f32 = 2.0;
/// The seconds a change of form takes to fade.
pub const FADE: f64 = 0.2;

/// Rank, highest first: the selected place, the one under the pointer, the player's own, rivals',
/// neutral and nobody's.
pub const SELECTED: u8 = 0;
pub const HOVERED: u8 = 1;
pub const OWN: u8 = 2;
pub const RIVAL: u8 = 3;
pub const NEUTRAL: u8 = 4;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Form {
    #[default]
    Full,
    Short,
    Hidden,
}

/// One label as the layout sees it: where each form would land, its rank, and how far its anchor is
/// from the middle of the view, which breaks ties within a rank.
pub struct Candidate {
    pub id: Id,
    pub full: Rect,
    pub short: Rect,
    pub tier: u8,
    pub dist: f32,
}

/// A fixed mark. `owner` is the label it is set beside, which does not give way to it.
pub struct Obstacle {
    pub rect: Rect,
    pub owner: Option<Id>,
}

/// Overlap with area, past the slack each block is allowed. Two blocks that only touch along an
/// edge leave each other legible.
fn overlaps(a: Rect, b: Rect) -> bool {
    let (a, b) = (a.shrink(SLACK), b.shrink(SLACK));
    a.min.x < b.max.x && b.min.x < a.max.x && a.min.y < b.max.y && b.min.y < a.max.y
}

/// The form each candidate takes, in the candidates' order. `prev` is the form a label held last
/// frame, `None` for a label new to the view.
pub fn give_way(cands: &[Candidate], fixed: &[Obstacle], prev: &dyn Fn(Id) -> Option<Form>) -> Vec<Form> {
    let mut order: Vec<usize> = (0..cands.len()).collect();
    order.sort_by(|a, b| (cands[*a].tier, cands[*a].dist).partial_cmp(&(cands[*b].tier, cands[*b].dist)).unwrap_or(std::cmp::Ordering::Equal));
    let mut placed: Vec<Rect> = Vec::new();
    let mut forms = vec![Form::Hidden; cands.len()];
    for i in order {
        let c = &cands[i];
        let was = prev(c.id).unwrap_or(Form::Full);
        let fits = |rect: Rect, growing: bool| {
            let rect = if growing { rect.expand(MARGIN) } else { rect };
            !placed.iter().any(|p| overlaps(*p, rect)) && (c.tier <= HOVERED || !fixed.iter().any(|o| o.owner != Some(c.id) && overlaps(o.rect, rect)))
        };
        let form = if fits(c.full, was != Form::Full) {
            Form::Full
        } else if fits(c.short, was == Form::Hidden) {
            Form::Short
        } else {
            Form::Hidden
        };
        match form {
            Form::Full => placed.push(c.full),
            Form::Short => placed.push(c.short),
            Form::Hidden => {}
        }
        forms[i] = form;
    }
    forms
}

#[derive(Clone, Copy, Debug)]
pub struct State {
    pub form: Form,
    /// The form it is fading out of, and when the fade began.
    pub was: Form,
    pub since: f64,
}

impl State {
    /// How far the fade has gone, 0 to 1.
    pub fn faded(&self, now: f64) -> f32 {
        ((now - self.since) / FADE).clamp(0.0, 1.0) as f32
    }
}

/// What the layout remembers between frames, one entry a label seen last frame.
#[derive(Clone, Default)]
pub struct Memory {
    states: HashMap<Id, State>,
}

impl Memory {
    pub fn form(&self, id: Id) -> Option<Form> {
        self.states.get(&id).map(|s| s.form)
    }

    /// Records this frame's forms, starting a fade where one changed, and forgets every label not
    /// laid out this frame. A label new to the view fades in from nothing.
    pub fn settle(&mut self, laid: &[(Id, Form)], now: f64) -> Vec<State> {
        let mut next = HashMap::new();
        let out = laid
            .iter()
            .map(|(id, form)| {
                let state = match self.states.get(id) {
                    Some(s) if s.form == *form => *s,
                    Some(s) => State { form: *form, was: s.form, since: now },
                    None => State { form: *form, was: Form::Hidden, since: now },
                };
                next.insert(*id, state);
                state
            })
            .collect();
        self.states = next;
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_egui::egui::{pos2, vec2};

    fn block(x: f32, y: f32, w: f32, h: f32) -> Rect {
        Rect::from_center_size(pos2(x, y), vec2(w, h))
    }

    fn label(name: &str, x: f32, tier: u8, dist: f32) -> Candidate {
        Candidate { id: Id::new(name), full: block(x, 0.0, 100.0, 40.0), short: block(x, 0.0, 40.0, 14.0), tier, dist }
    }

    fn fresh(_: Id) -> Option<Form> {
        None
    }

    #[test]
    fn apart_both_stand_full() {
        let forms = give_way(&[label("a", 0.0, OWN, 0.0), label("b", 200.0, OWN, 0.0)], &[], &fresh);
        assert_eq!(forms, vec![Form::Full, Form::Full]);
    }

    #[test]
    fn the_lower_rank_shrinks_whatever_the_order() {
        // b overlaps a's full block but its short block clears it.
        let forms = give_way(&[label("neutral", 75.0, NEUTRAL, 0.0), label("own", 0.0, OWN, 50.0)], &[], &fresh);
        assert_eq!(forms, vec![Form::Short, Form::Full]);
    }

    #[test]
    fn within_a_rank_the_nearer_the_middle_wins() {
        let forms = give_way(&[label("far", 0.0, RIVAL, 90.0), label("near", 75.0, RIVAL, 10.0)], &[], &fresh);
        assert_eq!(forms, vec![Form::Short, Form::Full]);
    }

    #[test]
    fn hides_when_even_the_name_collides() {
        let forms = give_way(&[label("a", 0.0, SELECTED, 0.0), label("b", 10.0, NEUTRAL, 0.0)], &[], &fresh);
        assert_eq!(forms, vec![Form::Full, Form::Hidden]);
    }

    #[test]
    fn gives_way_to_a_fixed_mark_but_not_its_own() {
        let mark = |owner: Option<&str>| Obstacle { rect: block(30.0, 15.0, 20.0, 24.0), owner: owner.map(Id::new) };
        assert_eq!(give_way(&[label("a", 0.0, OWN, 0.0)], &[mark(None)], &fresh), vec![Form::Short]);
        assert_eq!(give_way(&[label("a", 0.0, OWN, 0.0)], &[mark(Some("a"))], &fresh), vec![Form::Full]);
    }

    #[test]
    fn the_selected_and_hovered_are_laid_over_the_marks() {
        let mark = Obstacle { rect: block(0.0, 0.0, 20.0, 24.0), owner: None };
        assert_eq!(give_way(&[label("a", 0.0, RIVAL, 0.0)], std::slice::from_ref(&mark), &fresh), vec![Form::Hidden]);
        assert_eq!(give_way(&[label("a", 0.0, HOVERED, 0.0)], std::slice::from_ref(&mark), &fresh), vec![Form::Full]);
        assert_eq!(give_way(&[label("a", 0.0, SELECTED, 0.0)], std::slice::from_ref(&mark), &fresh), vec![Form::Full]);
        // They still give way to each other.
        assert_eq!(give_way(&[label("a", 0.0, SELECTED, 0.0), label("b", 10.0, HOVERED, 0.0)], &[mark], &fresh), vec![Form::Full, Form::Hidden]);
    }

    #[test]
    fn touching_edges_and_kissing_backgrounds_are_no_collision() {
        let forms = give_way(&[label("a", 0.0, OWN, 0.0), label("b", 100.0, NEUTRAL, 0.0)], &[], &fresh);
        assert_eq!(forms, vec![Form::Full, Form::Full]);
        // Within the slack: the blocks overlap by three pixels.
        let forms = give_way(&[label("a", 0.0, OWN, 0.0), label("b", 97.0, NEUTRAL, 0.0)], &[], &fresh);
        assert_eq!(forms, vec![Form::Full, Form::Full]);
        // Past it: five.
        let forms = give_way(&[label("a", 0.0, OWN, 0.0), label("b", 95.0, NEUTRAL, 0.0)], &[], &fresh);
        assert_eq!(forms, vec![Form::Full, Form::Short]);
    }

    #[test]
    fn grows_back_only_with_the_margin_to_spare() {
        // 99 apart: the full blocks clear, with the slack, by three pixels, under the margin.
        let cands = [label("a", 0.0, OWN, 0.0), label("b", 99.0, NEUTRAL, 0.0)];
        let shrunk = |id: Id| if id == Id::new("b") { Some(Form::Short) } else { Some(Form::Full) };
        assert_eq!(give_way(&cands, &[], &shrunk), vec![Form::Full, Form::Short]);
        let full = |_: Id| Some(Form::Full);
        assert_eq!(give_way(&cands, &[], &full), vec![Form::Full, Form::Full]);
        // Past the margin it grows back.
        let cands = [label("a", 0.0, OWN, 0.0), label("b", 103.0, NEUTRAL, 0.0)];
        assert_eq!(give_way(&cands, &[], &shrunk), vec![Form::Full, Form::Full]);
    }

    #[test]
    fn a_change_of_form_starts_a_fade_and_unseen_labels_are_forgotten() {
        let (a, b) = (Id::new("a"), Id::new("b"));
        let mut m = Memory::default();
        let first = m.settle(&[(a, Form::Full), (b, Form::Full)], 1.0);
        assert_eq!((first[0].was, first[0].since), (Form::Hidden, 1.0));
        let held = m.settle(&[(a, Form::Full), (b, Form::Short)], 2.0);
        assert_eq!(held[0].since, 1.0);
        assert_eq!((held[1].form, held[1].was, held[1].since), (Form::Short, Form::Full, 2.0));
        assert!((held[1].faded(2.1) - 0.5).abs() < 1e-4);
        m.settle(&[(a, Form::Full)], 3.0);
        assert_eq!(m.form(b), None);
    }
}
