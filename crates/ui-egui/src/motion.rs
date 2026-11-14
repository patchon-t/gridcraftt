//! Small UI transitions, in the manner of Windows 11 navigation: a selection indicator that slides
//! to the chosen item, stretching on the way (its leading edge leaves first and its trailing edge
//! catches up), and content that slides in and fades up when a tab or page changes.
//!
//! Each transition keeps its state in egui's temporary memory under the caller's id, and asks for
//! repaints only while it runs. With animations off ([`enabled`] false) everything lands at once.

use egui::{Context, Id};

/// How long the indicator takes to move.
const SLIDE_SECONDS: f64 = 0.36;
/// How long new content takes to settle.
const ENTER_SECONDS: f64 = 0.30;

/// Whether transitions run: the user's Animations preference, egui's animation time (zero in
/// tests that turn it off) and no screenshot in flight (a capture shows the settled UI).
pub fn enabled(app: &crate::SheetApp, ctx: &Context) -> bool {
    app.ui.animations && ctx.global_style().animation_time > 0.0 && !app.shots.in_flight()
}

/// Windows 11's "decelerate" curve, close to cubic-bezier(0, 0, 0, 1): quick start, long settle.
pub fn decelerate(t: f32) -> f32 {
    let t = if t.is_nan() { 1.0 } else { t.clamp(0.0, 1.0) };
    1.0 - (1.0 - t).powi(5)
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Slide {
    from: (f32, f32),
    to: (f32, f32),
    start: f64,
}

/// The span (`lo`, `hi`) an indicator shows this frame on its way to `target`.
pub fn indicator(ctx: &Context, id: Id, target: (f32, f32), enabled: bool) -> (f32, f32) {
    // Every document window has its own transitions (egui's memory is shared between them).
    let id = id.with(ctx.viewport_id());
    let now = ctx.input(|i| i.time);
    let state: Option<Slide> = ctx.data(|d| d.get_temp(id));
    let next = match state {
        _ if !enabled => Slide { from: target, to: target, start: now },
        None => Slide { from: target, to: target, start: now },
        Some(s) if s.to == target => s,
        // Retargeted mid-way: start again from where it is now.
        Some(s) => Slide { from: slide_at(s, now), to: target, start: now },
    };
    ctx.data_mut(|d| d.insert_temp(id, next));
    let span = slide_at(next, now);
    if span != next.to {
        ctx.request_repaint();
    }
    span
}

/// Where a slide is at `now`: the leading edge moves first and the trailing edge follows, so the
/// indicator stretches across the gap and shrinks back onto its target.
fn slide_at(s: Slide, now: f64) -> (f32, f32) {
    let p = ((now - s.start) / SLIDE_SECONDS) as f32;
    if p.is_nan() || p >= 1.0 {
        return s.to;
    }
    let lead = decelerate(p / 0.7);
    let trail = decelerate((p - 0.15) / 0.85);
    let mix = |a: f32, b: f32, k: f32| a + (b - a) * k;
    // Moving towards larger values, `hi` leads; towards smaller ones, `lo` does.
    let (klo, khi) = if s.to.0 + s.to.1 >= s.from.0 + s.from.1 { (trail, lead) } else { (lead, trail) };
    (mix(s.from.0, s.to.0, klo), mix(s.from.1, s.to.1, khi))
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Enter {
    key: u64,
    from: Option<u64>,
    start: f64,
}

/// How far content shown for `key` has entered (0 just changed, 1 settled), and the key it
/// replaced (to slide in from that side). The first key seen is shown settled.
pub fn entrance(ctx: &Context, id: Id, key: u64, enabled: bool) -> (f32, Option<u64>) {
    // Every document window has its own transitions (egui's memory is shared between them).
    let id = id.with(ctx.viewport_id());
    let now = ctx.input(|i| i.time);
    let state: Option<Enter> = ctx.data(|d| d.get_temp(id));
    let next = match state {
        Some(s) if s.key == key => s,
        Some(s) if enabled => Enter { key, from: Some(s.key), start: now },
        _ => Enter { key, from: None, start: now - ENTER_SECONDS },
    };
    ctx.data_mut(|d| d.insert_temp(id, next));
    let p = if enabled { ((now - next.start) / ENTER_SECONDS) as f32 } else { 1.0 };
    if p < 1.0 {
        ctx.request_repaint();
        (decelerate(p), next.from)
    } else {
        (1.0, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn at(ctx: &Context, time: f64) {
        let raw = egui::RawInput { time: Some(time), ..Default::default() };
        let mut out = ctx.run_ui(raw, |_| {});
        out.textures_delta.clear();
    }

    #[test]
    fn decelerate_runs_from_zero_to_one() {
        assert_eq!(decelerate(0.0), 0.0);
        assert_eq!(decelerate(1.0), 1.0);
        assert_eq!(decelerate(-3.0), 0.0);
        assert_eq!(decelerate(7.0), 1.0);
        assert_eq!(decelerate(f32::NAN), 1.0);
        assert!(decelerate(0.3) > 0.8, "most of the way early on");
    }

    #[test]
    fn the_indicator_stretches_towards_its_target_and_lands_on_it() {
        let ctx = Context::default();
        let id = Id::unique("test.indicator");
        at(&ctx, 1.0);
        assert_eq!(indicator(&ctx, id, (0.0, 10.0), true), (0.0, 10.0), "the first target is shown at once");
        assert_eq!(indicator(&ctx, id, (100.0, 110.0), true), (0.0, 10.0), "a move starts where it was");
        at(&ctx, 1.1);
        let (lo, hi) = indicator(&ctx, id, (100.0, 110.0), true);
        assert!(hi - lo > 10.0, "it stretches on the way: {lo}..{hi}");
        assert!(hi > lo && lo > 0.0 && hi <= 110.0);
        at(&ctx, 2.0);
        assert_eq!(indicator(&ctx, id, (100.0, 110.0), true), (100.0, 110.0));
    }

    #[test]
    fn without_animations_the_indicator_jumps() {
        let ctx = Context::default();
        let id = Id::unique("test.jump");
        at(&ctx, 1.0);
        indicator(&ctx, id, (0.0, 10.0), false);
        assert_eq!(indicator(&ctx, id, (50.0, 60.0), false), (50.0, 60.0));
    }

    #[test]
    fn entrance_starts_on_a_change_and_settles() {
        let ctx = Context::default();
        let id = Id::unique("test.entrance");
        at(&ctx, 1.0);
        assert_eq!(entrance(&ctx, id, 1, true), (1.0, None), "the first page is shown settled");
        assert_eq!(entrance(&ctx, id, 2, true), (0.0, Some(1)));
        at(&ctx, 1.1);
        let (p, from) = entrance(&ctx, id, 2, true);
        assert!(p > 0.0 && p < 1.0, "{p}");
        assert_eq!(from, Some(1));
        at(&ctx, 2.0);
        assert_eq!(entrance(&ctx, id, 2, true), (1.0, None));
        assert_eq!(entrance(&ctx, id, 3, false), (1.0, None), "no transition with animations off");
    }
}
