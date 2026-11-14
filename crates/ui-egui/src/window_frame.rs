//! The app's own window frame, for hosts that open the window without the system's decorations
//! ([`crate::Chrome::window_controls`]): minimize, maximize/restore and close buttons at the
//! right end of the title bar, rounded window corners while the window is neither maximized nor
//! full screen, a thin outline, and resizing from the window's edges.
//!
//! The corners are rounded by the panels on the window's outer edges (the title bar on top, the
//! status bar or sheet tabs below, the Backstage columns), which ask [`corners`] for their radius;
//! the window itself is transparent, so what they leave unpainted shows the desktop.

use egui::{Color32, Context, CornerRadius, CursorIcon, Pos2, Rect, ResizeDirection, Sense, Stroke, StrokeKind, Ui, ViewportCommand, pos2, vec2};

use crate::theme::Tokens;

/// Radius of the window's corners, the same as a Windows 11 top-level window.
pub const RADIUS: u8 = 8;

/// Width of a caption button.
pub const CAPTION_W: f32 = 46.0;

/// How far inside the window's edge a press starts a resize.
const GRIP: f32 = 5.0;

/// Caption-button close-hover red and its pressed shade.
const CLOSE_HOVER: Color32 = Color32::from_rgb(0xc4, 0x2b, 0x1c);
const CLOSE_PRESSED: Color32 = Color32::from_rgb(0xc8, 0x40, 0x31);

fn state_id() -> egui::Id {
    egui::Id::unique("gridcraft.window_frame.framed")
}

/// Records, for this frame of this viewport, whether the app draws the window frame and the
/// window is in a state with rounded corners. Called once at the start of the window's pass.
pub fn begin(ctx: &Context, window_controls: bool) {
    let framed = window_controls && rounded(ctx);
    ctx.data_mut(|d| d.insert_temp(state_id(), (window_controls, framed)));
}

/// Whether the window's corners are rounded: not maximized and not full screen.
fn rounded(ctx: &Context) -> bool {
    ctx.input(|i| !i.viewport().maximized.unwrap_or(false) && !i.viewport().fullscreen.unwrap_or(false))
}

/// Whether the app draws the window frame in this viewport (set by [`begin`]).
pub fn controls(ctx: &Context) -> bool {
    ctx.data(|d| d.get_temp::<(bool, bool)>(state_id())).is_some_and(|(c, _)| c)
}

/// Whether the corners are rounded in this frame.
pub fn framed(ctx: &Context) -> bool {
    ctx.data(|d| d.get_temp::<(bool, bool)>(state_id())).is_some_and(|(_, f)| f)
}

/// Which corners of the window a panel touches.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Edge {
    Top,
    Bottom,
    BottomLeft,
    BottomRight,
}

/// The corner radius a panel on `edge` of the window gives its frame: rounded where it meets a
/// window corner while the window is framed, square otherwise.
pub fn corners(ctx: &Context, edge: Edge) -> CornerRadius {
    if !framed(ctx) {
        return CornerRadius::ZERO;
    }
    let r = RADIUS;
    match edge {
        Edge::Top => CornerRadius { nw: r, ne: r, sw: 0, se: 0 },
        Edge::Bottom => CornerRadius { nw: 0, ne: 0, sw: r, se: r },
        Edge::BottomLeft => CornerRadius { nw: 0, ne: 0, sw: r, se: 0 },
        Edge::BottomRight => CornerRadius { nw: 0, ne: 0, sw: 0, se: r },
    }
}

/// What a caption button does.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Caption {
    Minimize,
    Maximize,
    Restore,
    Close,
}

impl Caption {
    fn label(self) -> &'static str {
        match self {
            Caption::Minimize => "Minimize",
            Caption::Maximize => "Maximize",
            Caption::Restore => "Restore Down",
            Caption::Close => "Close",
        }
    }
}

/// Draws close, maximize/restore and minimize, right to left, the full height of the title bar.
/// Call inside a right-to-left layout at the bar's right end.
pub fn caption_buttons(ui: &mut Ui, height: f32) {
    let maximized = ui.ctx().input(|i| i.viewport().maximized.unwrap_or(false));
    let saved = ui.spacing().item_spacing.x;
    ui.spacing_mut().item_spacing.x = 0.0;
    for caption in [Caption::Close, if maximized { Caption::Restore } else { Caption::Maximize }, Caption::Minimize] {
        if caption_button(ui, caption, height).clicked() {
            let cmd = match caption {
                Caption::Minimize => ViewportCommand::Minimized(true),
                Caption::Maximize => ViewportCommand::Maximized(true),
                Caption::Restore => ViewportCommand::Maximized(false),
                // The usual close path: unsaved workbooks are asked about first.
                Caption::Close => ViewportCommand::Close,
            };
            ui.ctx().send_viewport_cmd(cmd);
        }
    }
    ui.spacing_mut().item_spacing.x = saved;
}

fn caption_button(ui: &mut Ui, caption: Caption, height: f32) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let (rect, resp) = ui.allocate_exact_size(vec2(CAPTION_W, height), Sense::click());
    let lang = crate::i18n::current(ui.ctx());
    let label = lang.tr(caption.label());
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, true, label));
    let resp = resp.on_hover_text(label);
    let pressed = resp.is_pointer_button_down_on();
    let hovered = resp.hovered();
    let close = caption == Caption::Close;
    let fill = match (close, hovered, pressed) {
        (true, _, true) => CLOSE_PRESSED,
        (true, true, false) => CLOSE_HOVER,
        (false, _, true) => t.text.gamma_multiply(0.06),
        (false, true, false) => t.text.gamma_multiply(0.1),
        _ => Color32::TRANSPARENT,
    };
    let p = ui.painter();
    if fill != Color32::TRANSPARENT {
        // The close button sits in the window's top-right corner and follows its rounding.
        let radius = if close { CornerRadius { ne: corners(ui.ctx(), Edge::Top).ne, ..CornerRadius::ZERO } } else { CornerRadius::ZERO };
        p.rect_filled(rect, radius, fill);
    }
    let ink = if close && (hovered || pressed) { Color32::WHITE } else { t.text };
    paint_glyph(p, rect.center(), caption, ink);
    resp
}

/// The caption glyphs, drawn in code on a 10-point grid.
fn paint_glyph(p: &egui::Painter, c: Pos2, caption: Caption, ink: Color32) {
    let s = Stroke::new(1.0, ink);
    let h = 5.0;
    match caption {
        Caption::Minimize => {
            p.line_segment([pos2(c.x - h, c.y), pos2(c.x + h, c.y)], s);
        }
        Caption::Maximize => {
            p.rect_stroke(Rect::from_center_size(c, vec2(2.0 * h, 2.0 * h)), 1.5, s, StrokeKind::Middle);
        }
        Caption::Restore => {
            // A square with a second one peeking out behind it, up and to the right.
            let front = Rect::from_min_size(pos2(c.x - h, c.y - h + 2.0), vec2(2.0 * h - 2.0, 2.0 * h - 2.0));
            p.rect_stroke(front, 1.5, s, StrokeKind::Middle);
            let (l, t, r, b) = (c.x - h + 2.0, c.y - h, c.x + h, c.y + h - 2.0);
            p.line_segment([pos2(l, t + 0.5), pos2(r - 1.5, t + 0.5)], s);
            p.line_segment([pos2(r - 0.5, t + 1.5), pos2(r - 0.5, b)], s);
        }
        Caption::Close => {
            p.line_segment([pos2(c.x - h, c.y - h), pos2(c.x + h, c.y + h)], s);
            p.line_segment([pos2(c.x + h, c.y - h), pos2(c.x - h, c.y + h)], s);
        }
    }
}

/// The resize direction for a pointer at `pos` in a window `rect`, if it is on an edge.
pub fn edge_at(rect: Rect, pos: Pos2) -> Option<ResizeDirection> {
    if !rect.contains(pos) {
        return None;
    }
    let corner = GRIP * 2.0;
    let left = pos.x - rect.left() < GRIP;
    let right = rect.right() - pos.x < GRIP;
    let top = pos.y - rect.top() < GRIP;
    let bottom = rect.bottom() - pos.y < GRIP;
    // Near a corner, the edge zone widens along both sides, as a system frame's does.
    let near_left = pos.x - rect.left() < corner;
    let near_right = rect.right() - pos.x < corner;
    let near_top = pos.y - rect.top() < corner;
    let near_bottom = rect.bottom() - pos.y < corner;
    Some(match () {
        _ if (top && near_left) || (left && near_top) => ResizeDirection::NorthWest,
        _ if (top && near_right) || (right && near_top) => ResizeDirection::NorthEast,
        _ if (bottom && near_left) || (left && near_bottom) => ResizeDirection::SouthWest,
        _ if (bottom && near_right) || (right && near_bottom) => ResizeDirection::SouthEast,
        _ if top => ResizeDirection::North,
        _ if bottom => ResizeDirection::South,
        _ if left => ResizeDirection::West,
        _ if right => ResizeDirection::East,
        _ => return None,
    })
}

fn cursor_for(dir: ResizeDirection) -> CursorIcon {
    match dir {
        ResizeDirection::North => CursorIcon::ResizeNorth,
        ResizeDirection::South => CursorIcon::ResizeSouth,
        ResizeDirection::East => CursorIcon::ResizeEast,
        ResizeDirection::West => CursorIcon::ResizeWest,
        ResizeDirection::NorthEast => CursorIcon::ResizeNorthEast,
        ResizeDirection::SouthEast => CursorIcon::ResizeSouthEast,
        ResizeDirection::NorthWest => CursorIcon::ResizeNorthWest,
        ResizeDirection::SouthWest => CursorIcon::ResizeSouthWest,
    }
}

/// After the window's content: the outline, and resizing from the edges. Does nothing while the
/// window is maximized or full screen (no edges to grab) or when the system draws the frame.
pub fn end(ctx: &Context) {
    if !framed(ctx) {
        return;
    }
    let rect = ctx.content_rect();
    let t = Tokens::get(ctx);
    let outline = if t.dark { Color32::from_white_alpha(28) } else { Color32::from_black_alpha(40) };
    ctx.layer_painter(egui::LayerId::new(egui::Order::Foreground, egui::Id::unique("gridcraft.window_frame.outline"))).rect_stroke(
        rect,
        RADIUS,
        Stroke::new(1.0, outline),
        StrokeKind::Inside,
    );
    // A drag that started inside the window (a selection, a splitter) keeps its own cursor.
    let (pos, pressed, any_down) = ctx.input(|i| (i.pointer.hover_pos(), i.pointer.primary_pressed(), i.pointer.any_down()));
    let Some(dir) = pos.and_then(|p| edge_at(rect, p)) else { return };
    if any_down && !pressed {
        return;
    }
    ctx.set_cursor_icon(cursor_for(dir));
    if pressed {
        ctx.send_viewport_cmd(ViewportCommand::BeginResize(dir));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const WINDOW: Rect = Rect { min: Pos2 { x: 0.0, y: 0.0 }, max: Pos2 { x: 800.0, y: 600.0 } };

    #[test]
    fn edges_and_corners_resize_in_their_direction() {
        assert_eq!(edge_at(WINDOW, pos2(1.0, 300.0)), Some(ResizeDirection::West));
        assert_eq!(edge_at(WINDOW, pos2(799.0, 300.0)), Some(ResizeDirection::East));
        assert_eq!(edge_at(WINDOW, pos2(400.0, 1.0)), Some(ResizeDirection::North));
        assert_eq!(edge_at(WINDOW, pos2(400.0, 599.0)), Some(ResizeDirection::South));
        assert_eq!(edge_at(WINDOW, pos2(1.0, 1.0)), Some(ResizeDirection::NorthWest));
        assert_eq!(edge_at(WINDOW, pos2(8.0, 2.0)), Some(ResizeDirection::NorthWest), "the corner zone reaches along the edge");
        assert_eq!(edge_at(WINDOW, pos2(798.0, 598.0)), Some(ResizeDirection::SouthEast));
        assert_eq!(edge_at(WINDOW, pos2(2.0, 595.0)), Some(ResizeDirection::SouthWest));
        assert_eq!(edge_at(WINDOW, pos2(795.0, 2.0)), Some(ResizeDirection::NorthEast));
    }

    #[test]
    fn the_inside_and_the_outside_do_not_resize() {
        assert_eq!(edge_at(WINDOW, pos2(400.0, 300.0)), None);
        assert_eq!(edge_at(WINDOW, pos2(6.0, 300.0)), None);
        assert_eq!(edge_at(WINDOW, pos2(-3.0, 300.0)), None);
        assert_eq!(edge_at(WINDOW, pos2(f32::NAN, 300.0)), None);
    }

    #[test]
    fn corners_are_square_unless_the_window_is_framed() {
        let ctx = Context::default();
        begin(&ctx, false);
        assert_eq!(corners(&ctx, Edge::Top), CornerRadius::ZERO);
        begin(&ctx, true);
        assert!(controls(&ctx));
        assert_eq!(corners(&ctx, Edge::Top), CornerRadius { nw: RADIUS, ne: RADIUS, sw: 0, se: 0 });
        assert_eq!(corners(&ctx, Edge::BottomRight), CornerRadius { nw: 0, ne: 0, sw: 0, se: RADIUS });
    }
}
