//! GridCraft's icon set, drawn in code on a 16×16 design grid (original artwork; see
//! ATTRIBUTION.md). Large ribbon icons use the same drawings at 32×32 with accent colours.
//! Accent and paper colours come from the theme tokens, so icons follow light and dark mode;
//! icons placed on the sheet ([`paint_on_sheet`]) keep the light colours, as the sheet does.

use egui::epaint::{CubicBezierShape, PathShape, PathStroke};
use egui::{Color32, Painter, Pos2, Rect, Stroke, pos2, vec2};

use crate::theme::{Cat, Tokens};

/// The glyph on a Function Library book ([`Icon::FnBook`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FnGlyph {
    /// Recently Used: a star.
    Star,
    /// Financial: a stack of coins.
    Coins,
    /// Logical: a question mark.
    Question,
    /// Text: a letter.
    Letter,
    /// Date & Time: a clock face.
    Clock,
    /// Lookup & Reference: a magnifier.
    Magnifier,
    /// Math & Trig: theta.
    Theta,
    /// More Functions: three dots.
    Dots,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Icon {
    Paste,
    Cut,
    Copy,
    Brush,
    Bold,
    Italic,
    Underline,
    Strike,
    Borders,
    BorderAll,
    BorderNone,
    BorderOutside,
    BorderThick,
    Fill,
    FontColor,
    AlignTop,
    AlignMiddle,
    AlignBottom,
    AlignLeft,
    AlignCenter,
    AlignRight,
    Orientation,
    IndentDec,
    IndentInc,
    Wrap,
    Merge,
    Currency,
    Percent,
    Comma,
    DecInc,
    DecDec,
    CondFormat,
    FormatTable,
    CellStyles,
    Insert,
    Delete,
    Format,
    Sum,
    FillDown,
    Eraser,
    SortFilter,
    Find,
    Undo,
    Redo,
    Save,
    Home,
    More,
    Chevron,
    ChevronUp,
    Close,
    Check,
    Fx,
    Plus,
    Left,
    Right,
    Normal,
    PageLayout,
    PageBreak,
    Minus,
    Table,
    PivotTable,
    Chart,
    ChartBar,
    ChartLine,
    ChartPie,
    ChartScatter,
    Picture,
    Shapes,
    Link,
    Comment,
    Note,
    TextBox,
    WordArt,
    Symbol,
    /// An equation: a radical sign over an italic x.
    Equation,
    Sparkline,
    Checkbox,
    /// A form button with a pointer.
    Button,
    /// A form slider (scroll bar).
    Slider,
    /// A form spin button.
    Spin,
    Function,
    Name,
    Trace,
    Calc,
    Filter,
    SortAsc,
    SortDesc,
    Duplicates,
    TextColumns,
    Validation,
    Group,
    Ungroup,
    Subtotal,
    Lock,
    Spell,
    Freeze,
    Zoom,
    Gridlines,
    Theme,
    Margins,
    Orient,
    SheetRtl,
    Paper,
    PrintArea,
    Script,
    Pen,
    Share,
    Search,
    Settings,
    /// Light/dark appearance: a circle, its right half filled.
    Appearance,
    Sheet,
    /// All Sheets navigator next to the sheet tabs: a list of rows with a bullet each.
    SheetList,
    Book,
    Folder,
    /// A book with a coloured spine and a category glyph (Formulas › Function Library). The
    /// spine and glyph take the accent colour of [`paint_accent`].
    FnBook(FnGlyph),
    /// Header & Footer: a page with its top / bottom band marked.
    Header,
    Footer,
    PageNumber,
    PageCount,
    Calendar,
    Clock,
    FileName,
    /// File › Backstage navigation (line art, drawn in the navigation colour).
    Back,
    NewFile,
    OpenFolder,
    Info,
    Print,
    /// A macro: a script page with a run arrow.
    Macro,
    /// Record Macro: a ring around the recording dot.
    Record,
    /// Use Relative References: a cell and an arrow to a cell offset from it.
    Relative,
    /// A reference box's button: shrink the window to the box (Collapse) or restore it (Expand).
    Collapse,
    Expand,
    /// The macro debugger's Run: a play triangle.
    Play,
    /// Step: an arrow down onto a line.
    Step,
    /// Continue: a bar and a play triangle.
    Continue,
    /// Run to Here: an arrow to a marked line.
    RunTo,
    /// Stop: a square.
    StopSquare,
}

struct G<'a> {
    p: &'a Painter,
    r: Rect,
    s: f32,
}

impl G<'_> {
    fn pt(&self, x: f32, y: f32) -> Pos2 {
        pos2(self.r.left() + x * self.s, self.r.top() + y * self.s)
    }
    fn line(&self, pts: &[(f32, f32)], c: Color32, w: f32) {
        let v: Vec<Pos2> = pts.iter().map(|(x, y)| self.pt(*x, *y)).collect();
        self.p.add(PathShape::line(v, PathStroke::new(w * self.s.clamp(1.0, 2.0), c)));
    }
    fn closed(&self, pts: &[(f32, f32)], c: Color32, w: f32) {
        let v: Vec<Pos2> = pts.iter().map(|(x, y)| self.pt(*x, *y)).collect();
        self.p.add(PathShape::closed_line(v, PathStroke::new(w * self.s.clamp(1.0, 2.0), c)));
    }
    fn stroke(&self, pts: &[(f32, f32)], c: Color32) {
        self.line(pts, c, 1.15);
    }
    fn rect(&self, x: f32, y: f32, w: f32, h: f32, fill: Option<Color32>, stroke: Option<Color32>) {
        let r = Rect::from_min_size(self.pt(x, y), vec2(w * self.s, h * self.s));
        if let Some(f) = fill {
            self.p.rect_filled(r, 1.0, f);
        }
        if let Some(c) = stroke {
            self.p.rect_stroke(r, 1.0, Stroke::new(1.0, c), egui::StrokeKind::Inside);
        }
    }
    fn poly(&self, pts: &[(f32, f32)], fill: Color32) {
        let v: Vec<Pos2> = pts.iter().map(|(x, y)| self.pt(*x, *y)).collect();
        self.p.add(PathShape::convex_polygon(v, fill, Stroke::NONE));
    }
    fn circle(&self, x: f32, y: f32, r: f32, fill: Option<Color32>, stroke: Option<Color32>) {
        if let Some(f) = fill {
            self.p.circle_filled(self.pt(x, y), r * self.s, f);
        }
        if let Some(c) = stroke {
            self.p.circle_stroke(self.pt(x, y), r * self.s, Stroke::new(1.0, c));
        }
    }
    fn text(&self, x: f32, y: f32, t: &str, size: f32, c: Color32, style: (bool, bool)) {
        let fam = if style.0 {
            crate::theme::CELL_BOLD
        } else if style.1 {
            crate::theme::CELL_ITALIC
        } else {
            crate::theme::CELL
        };
        let font = egui::FontId::new(size * self.s, egui::FontFamily::Name(fam.into()));
        self.p.text(self.pt(x, y), egui::Align2::CENTER_CENTER, t, font, c);
    }
}

/// The colours an icon draws with besides its main colour: the paper inside documents and cells,
/// the category palette for its coloured parts, and the accent a button asked for.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Palette {
    paper: Color32,
    cat: [Color32; 8],
    recording: Color32,
    /// The accent colour: the coloured part of an icon that has one (a book's spine and glyph),
    /// otherwise the icon's main colour. `None` draws the icon as it is.
    accent: Option<Color32>,
}

impl Palette {
    fn of(t: &Tokens) -> Palette {
        Palette { paper: t.paper, cat: t.cat, recording: t.recording, accent: None }
    }

    fn cat(&self, c: Cat) -> Color32 {
        self.cat.get(c as usize).copied().unwrap_or(Color32::GRAY)
    }

    /// A pale wash of a category colour over the paper (cell fills inside icons).
    fn wash(&self, c: Cat) -> Color32 {
        self.cat(c).gamma_multiply(0.3)
    }
}

/// The code-drawn icon that stands in for a character the current fonts cannot draw (`✕` and
/// `✓` have no bundled egui face). Interface chrome uses this instead of painting a tofu box; a
/// workbook cell still needs a real glyph because its characters are user data.
pub fn fallback_for_symbol(c: char) -> Option<Icon> {
    match c {
        '✕' => Some(Icon::Close),
        '✓' => Some(Icon::Check),
        _ => None,
    }
}

/// Whether an icon has a part of its own for the accent colour (otherwise the accent recolours
/// the whole icon).
fn has_accent_part(icon: Icon) -> bool {
    matches!(icon, Icon::FnBook(_))
}

/// Draws `icon` into `rect` (square-ish), in colour `c`, with the current theme's palette.
pub fn paint(p: &Painter, rect: Rect, icon: Icon, c: Color32) {
    paint_with(p, rect, icon, c, &Palette::of(&Tokens::get(p.ctx())));
}

/// Draws `icon` like [`paint`], with an accent colour: the icon's coloured part when it has one
/// (the spine and glyph of [`Icon::FnBook`]), otherwise the whole icon. `None` is [`paint`].
pub fn paint_accent(p: &Painter, rect: Rect, icon: Icon, c: Color32, accent: Option<Color32>) {
    paint_with(p, rect, icon, c, &Palette { accent, ..Palette::of(&Tokens::get(p.ctx())) });
}

/// Draws `icon` on the sheet, which stays white in both themes, so with the light palette.
pub fn paint_on_sheet(p: &Painter, rect: Rect, icon: Icon, c: Color32) {
    paint_with(p, rect, icon, c, &Palette::of(&Tokens::light()));
}

/// The egui font family of the Fluent System Icons subset (registered by
/// `theme::font_definitions*`).
pub const FLUENT_FAMILY: &str = "fluent-icons";

/// The Fluent System Icons subset: Microsoft's Regular icon font, cut down to the glyphs GridCraft
/// uses (MIT; see `assets/icons/fluent/LICENSE` and ATTRIBUTION.md).
pub const FLUENT_FONT: &[u8] = include_bytes!("../../../assets/icons/fluent/FluentSystemIcons-GridCraft.ttf");

/// The Fluent System Icons glyph that draws `icon`, or `None` for the icons still drawn in code
/// (the Function Library books, Percent).
fn fluent_glyph(icon: Icon) -> Option<char> {
    Some(match icon {
        Icon::Paste => '\u{F2D5}',         // ic_fluent_clipboard_paste_20_regular
        Icon::Cut => '\u{F33A}',           // ic_fluent_cut_20_regular
        Icon::Copy => '\u{F32B}',          // ic_fluent_copy_20_regular
        Icon::Brush => '\u{F592}',         // ic_fluent_paint_brush_20_regular
        Icon::Bold => '\u{F7A4}',          // ic_fluent_text_bold_20_regular
        Icon::Italic => '\u{F7F4}',        // ic_fluent_text_italic_20_regular
        Icon::Underline => '\u{F80A}',     // ic_fluent_text_underline_20_regular
        Icon::Strike => '\u{ED5F}',        // ic_fluent_text_strikethrough_20_regular
        Icon::Borders => '\u{E19C}',       // ic_fluent_border_all_20_regular
        Icon::BorderAll => '\u{E19C}',     // ic_fluent_border_all_20_regular
        Icon::BorderNone => '\u{E1A8}',    // ic_fluent_border_none_20_regular
        Icon::BorderOutside => '\u{E1AA}', // ic_fluent_border_outside_20_regular
        Icon::BorderThick => '\u{E1AC}',   // ic_fluent_border_outside_thick_20_regular
        Icon::Fill => '\u{F595}',          // ic_fluent_paint_bucket_20_regular
        Icon::FontColor => '\u{F7BF}',     // ic_fluent_text_color_20_regular
        Icon::AlignTop => '\u{E04A}',      // ic_fluent_align_top_20_regular
        Icon::AlignMiddle => '\u{E02B}',   // ic_fluent_align_center_vertical_20_regular
        Icon::AlignBottom => '\u{E01F}',   // ic_fluent_align_bottom_20_regular
        Icon::AlignLeft => '\u{F79F}',     // ic_fluent_text_align_left_20_regular
        Icon::AlignCenter => '\u{F799}',   // ic_fluent_text_align_center_20_regular
        Icon::AlignRight => '\u{F7A1}',    // ic_fluent_text_align_right_20_regular
        Icon::Orientation => '\u{ED06}',   // ic_fluent_text_direction_rotate_270_right_20_regular
        Icon::IndentDec => '\u{F0298}',    // ic_fluent_text_indent_decrease_20_regular
        Icon::IndentInc => '\u{F029B}',    // ic_fluent_text_indent_increase_20_regular
        Icon::Wrap => '\u{ED69}',          // ic_fluent_text_wrap_20_regular
        Icon::Merge => '\u{F760}',         // ic_fluent_table_cells_merge_20_regular
        Icon::Currency => '\u{F54F}',      // ic_fluent_money_20_regular
        Icon::Comma => '\u{F57D}',         // ic_fluent_number_symbol_20_regular
        Icon::DecInc => '\u{E477}',        // ic_fluent_decimal_arrow_left_20_regular
        Icon::DecDec => '\u{E479}',        // ic_fluent_decimal_arrow_right_20_regular
        Icon::CondFormat => '\u{EC2E}',    // ic_fluent_table_lightning_20_regular
        Icon::FormatTable => '\u{F75D}',   // ic_fluent_table_20_regular
        Icon::CellStyles => '\u{EC02}',    // ic_fluent_table_cell_edit_20_regular
        Icon::Insert => '\u{EC2A}',        // ic_fluent_table_insert_row_20_regular
        Icon::Delete => '\u{EC10}',        // ic_fluent_table_delete_row_20_regular
        Icon::Format => '\u{EC4E}',        // ic_fluent_table_settings_20_regular
        Icon::Sum => '\u{F1AF}',           // ic_fluent_autosum_20_regular
        Icon::FillDown => '\u{F150}',      // ic_fluent_arrow_download_20_regular
        Icon::Eraser => '\u{E5E4}',        // ic_fluent_eraser_20_regular
        Icon::SortFilter => '\u{F18A}',    // ic_fluent_arrow_sort_20_regular
        Icon::Find => '\u{F68F}',          // ic_fluent_search_20_regular
        Icon::Undo => '\u{F199}',          // ic_fluent_arrow_undo_20_regular
        Icon::Redo => '\u{F16E}',          // ic_fluent_arrow_redo_20_regular
        Icon::Save => '\u{F67F}',          // ic_fluent_save_20_regular
        Icon::Home => '\u{F480}',          // ic_fluent_home_20_regular
        Icon::More => '\u{E824}',          // ic_fluent_more_horizontal_20_regular
        Icon::Chevron => '\u{F2A3}',       // ic_fluent_chevron_down_20_regular
        Icon::ChevronUp => '\u{F2B6}',     // ic_fluent_chevron_up_20_regular
        Icon::Close => '\u{F369}',         // ic_fluent_dismiss_20_regular
        Icon::Check => '\u{F294}',         // ic_fluent_checkmark_20_regular
        Icon::Fx => '\u{E7E2}',            // ic_fluent_math_formula_20_regular
        Icon::Plus => '\u{F109}',          // ic_fluent_add_20_regular
        Icon::Left => '\u{F2AA}',          // ic_fluent_chevron_left_20_regular
        Icon::Right => '\u{F2B0}',         // ic_fluent_chevron_right_20_regular
        Icon::Normal => '\u{F462}',        // ic_fluent_grid_20_regular
        Icon::PageLayout => '\u{F39C}',    // ic_fluent_document_page_bottom_center_20_regular
        Icon::PageBreak => '\u{E527}',     // ic_fluent_document_page_break_20_regular
        Icon::Minus => '\u{EBD0}',         // ic_fluent_subtract_20_regular
        Icon::Table => '\u{F75D}',         // ic_fluent_table_20_regular
        Icon::PivotTable => '\u{EC1E}',    // ic_fluent_table_freeze_column_and_row_20_regular
        Icon::Chart => '\u{F33F}',         // ic_fluent_data_bar_vertical_20_regular
        Icon::ChartBar => '\u{F33F}',      // ic_fluent_data_bar_vertical_20_regular
        Icon::ChartLine => '\u{E458}',     // ic_fluent_data_line_20_regular
        Icon::ChartPie => '\u{F344}',      // ic_fluent_data_pie_20_regular
        Icon::ChartScatter => '\u{E459}',  // ic_fluent_data_scatter_20_regular
        Icon::Picture => '\u{F488}',       // ic_fluent_image_20_regular
        Icon::Shapes => '\u{F6AD}',        // ic_fluent_shapes_20_regular
        Icon::Link => '\u{F4E4}',          // ic_fluent_link_20_regular
        Icon::Comment => '\u{F2FF}',       // ic_fluent_comment_20_regular
        Icon::Note => '\u{F56B}',          // ic_fluent_note_20_regular
        Icon::TextBox => '\u{F80F}',       // ic_fluent_textbox_20_regular
        Icon::WordArt => '\u{F7DB}',       // ic_fluent_text_effects_20_regular
        Icon::Symbol => '\u{EBE1}',        // ic_fluent_symbols_20_regular
        Icon::Equation => '\u{E7E6}',      // ic_fluent_math_symbols_20_regular
        Icon::Sparkline => '\u{E45D}',     // ic_fluent_data_trending_20_regular
        Icon::Checkbox => '\u{F28D}',      // ic_fluent_checkbox_checked_20_regular
        Icon::Button => '\u{F0168}',       // ic_fluent_button_20_regular
        Icon::Slider => '\u{EB25}',        // ic_fluent_slide_size_20_regular
        Icon::Spin => '\u{F18A}',          // ic_fluent_arrow_sort_20_regular
        Icon::Function => '\u{E7E2}',      // ic_fluent_math_formula_20_regular
        Icon::Name => '\u{F77C}',          // ic_fluent_tag_20_regular
        Icon::Trace => '\u{E637}',         // ic_fluent_flow_20_regular
        Icon::Calc => '\u{F20A}',          // ic_fluent_calculator_20_regular
        Icon::Filter => '\u{F406}',        // ic_fluent_filter_20_regular
        Icon::SortAsc => '\u{F802}',       // ic_fluent_text_sort_ascending_20_regular
        Icon::SortDesc => '\u{F803}',      // ic_fluent_text_sort_descending_20_regular
        Icon::Duplicates => '\u{E424}',    // ic_fluent_copy_select_20_regular
        Icon::TextColumns => '\u{F7C5}',   // ic_fluent_text_column_two_20_regular
        Icon::Validation => '\u{F298}',    // ic_fluent_checkmark_circle_20_regular
        Icon::Group => '\u{F465}',         // ic_fluent_group_20_regular
        Icon::Ungroup => '\u{E6C8}',       // ic_fluent_group_dismiss_20_regular
        Icon::Subtotal => '\u{E236}',      // ic_fluent_calculator_multiple_20_regular
        Icon::Lock => '\u{E78F}',          // ic_fluent_lock_closed_20_regular
        Icon::Spell => '\u{F7FE}',         // ic_fluent_text_proofing_tools_20_regular
        Icon::Freeze => '\u{EC22}',        // ic_fluent_table_freeze_row_20_regular
        Icon::Zoom => '\u{F8C4}',          // ic_fluent_zoom_in_20_regular
        Icon::Gridlines => '\u{F462}',     // ic_fluent_grid_20_regular
        Icon::Theme => '\u{F2F5}',         // ic_fluent_color_20_regular
        Icon::Margins => '\u{F395}',       // ic_fluent_document_margins_20_regular
        Icon::Orient => '\u{E89F}',        // ic_fluent_orientation_20_regular
        Icon::SheetRtl => '\u{ED04}',      // ic_fluent_text_direction_horizontal_right_20_regular
        Icon::Paper => '\u{F378}',         // ic_fluent_document_20_regular
        Icon::PrintArea => '\u{F62A}',     // ic_fluent_print_20_regular
        Icon::Script => '\u{F2EF}',        // ic_fluent_code_20_regular
        Icon::Pen => '\u{E8D7}',           // ic_fluent_pen_20_regular
        Icon::Share => '\u{F6AF}',         // ic_fluent_share_20_regular
        Icon::Search => '\u{F68F}',        // ic_fluent_search_20_regular
        Icon::Settings => '\u{F6A9}',      // ic_fluent_settings_20_regular
        Icon::Appearance => '\u{E452}',    // ic_fluent_dark_theme_20_regular
        Icon::Sheet => '\u{F75D}',         // ic_fluent_table_20_regular
        Icon::SheetList => '\u{F4ED}',     // ic_fluent_list_20_regular
        Icon::Book => '\u{E54B}',          // ic_fluent_document_table_20_regular
        Icon::Folder => '\u{F418}',        // ic_fluent_folder_20_regular
        Icon::Header => '\u{E501}',        // ic_fluent_document_header_20_regular
        Icon::Footer => '\u{E4FD}',        // ic_fluent_document_footer_20_regular
        Icon::PageNumber => '\u{E88C}',    // ic_fluent_number_symbol_square_20_regular
        Icon::PageCount => '\u{F3A3}',     // ic_fluent_document_page_number_20_regular
        Icon::Calendar => '\u{E24E}',      // ic_fluent_calendar_ltr_20_regular
        Icon::Clock => '\u{F2DD}',         // ic_fluent_clock_20_regular
        Icon::FileName => '\u{E557}',      // ic_fluent_document_text_20_regular
        Icon::Back => '\u{E094}',          // ic_fluent_arrow_circle_left_20_regular
        Icon::NewFile => '\u{E4DB}',       // ic_fluent_document_add_20_regular
        Icon::OpenFolder => '\u{F42E}',    // ic_fluent_folder_open_20_regular
        Icon::Info => '\u{F4A3}',          // ic_fluent_info_20_regular
        Icon::Print => '\u{F62A}',         // ic_fluent_print_20_regular
        Icon::Macro => '\u{EFBB}',         // ic_fluent_code_block_20_regular
        Icon::Record => '\u{F661}',        // ic_fluent_record_20_regular
        Icon::Relative => '\u{F01E6}',     // ic_fluent_arrow_routing_20_regular
        Icon::Collapse => '\u{E32D}',      // ic_fluent_chevron_double_up_20_regular
        Icon::Expand => '\u{E329}',        // ic_fluent_chevron_double_down_20_regular
        Icon::Play => '\u{F605}',          // ic_fluent_play_20_regular
        Icon::Step => '\u{E0FC}',          // ic_fluent_arrow_step_in_20_regular
        Icon::Continue => '\u{F569}',      // ic_fluent_next_20_regular
        Icon::RunTo => '\u{E10F}',         // ic_fluent_arrow_step_over_20_regular
        Icon::StopSquare => '\u{F72A}',    // ic_fluent_stop_20_regular
        _ => return None,
    })
}

/// Whether the context's fonts include the Fluent icons (a bare `egui::Context` in a unit test
/// has only egui's defaults: icons are then drawn in code, never with a missing family).
fn fluent_loaded(ctx: &egui::Context) -> bool {
    ctx.fonts(|f| f.families().iter().any(|fam| matches!(fam, egui::FontFamily::Name(n) if &**n == FLUENT_FAMILY)))
}

/// Draws `icon` into `rect` (square-ish), in colour `c`, with palette `pal`.
fn paint_with(p: &Painter, rect: Rect, icon: Icon, c: Color32, pal: &Palette) {
    if let Some(glyph) = fluent_glyph(icon)
        && fluent_loaded(p.ctx())
    {
        let side = rect.width().min(rect.height());
        let font = egui::FontId::new(side, egui::FontFamily::Name(FLUENT_FAMILY.into()));
        p.text(rect.center(), egui::Align2::CENTER_CENTER, glyph, font, pal.accent.unwrap_or(c));
        return;
    }
    paint_drawn(p, rect, icon, c, pal);
}

/// Draws `icon` with GridCraft's own code-drawn artwork.
fn paint_drawn(p: &Painter, rect: Rect, icon: Icon, c: Color32, pal: &Palette) {
    let side = rect.width().min(rect.height());
    let r = Rect::from_center_size(rect.center(), vec2(side, side));
    let g = G { p, r, s: side / 16.0 };
    let c = match pal.accent {
        Some(a) if !has_accent_part(icon) => a,
        _ => c,
    };
    let light = c.gamma_multiply(0.45);
    let paper = pal.paper;
    let blue = pal.cat(Cat::Blue);
    let green = pal.cat(Cat::Green);
    let red = pal.cat(Cat::Red);
    let orange = pal.cat(Cat::Orange);
    let purple = pal.cat(Cat::Purple);
    let yellow = pal.cat(Cat::Amber);
    match icon {
        Icon::Paste => {
            g.rect(3.0, 2.5, 9.0, 12.0, Some(yellow), None);
            g.rect(5.0, 1.5, 5.0, 2.5, Some(pal.cat(Cat::Slate)), None);
            g.rect(6.5, 6.0, 7.5, 9.0, Some(paper), Some(c));
            for y in [8.5, 10.5, 12.5] {
                g.stroke(&[(8.0, y), (12.5, y)], light);
            }
        }
        Icon::Cut => {
            g.circle(5.0, 12.0, 2.0, None, Some(c));
            g.circle(11.0, 12.0, 2.0, None, Some(c));
            g.stroke(&[(6.3, 10.6), (11.5, 2.0)], c);
            g.stroke(&[(9.7, 10.6), (4.5, 2.0)], c);
        }
        Icon::Copy => {
            g.rect(2.5, 2.5, 7.5, 9.0, Some(paper), Some(c));
            g.rect(6.0, 5.5, 7.5, 9.0, Some(paper), Some(c));
        }
        Icon::Brush => {
            g.rect(3.0, 2.0, 10.0, 4.0, Some(yellow), Some(c));
            g.stroke(&[(8.0, 6.0), (8.0, 8.5), (5.0, 8.5), (5.0, 10.0)], c);
            g.rect(4.0, 10.0, 2.0, 4.5, Some(c), None);
        }
        Icon::Bold => g.text(8.0, 8.5, "B", 12.0, c, (true, false)),
        Icon::Italic => g.text(8.0, 8.5, "I", 12.0, c, (false, true)),
        Icon::Underline => {
            g.text(8.0, 7.5, "U", 11.0, c, (false, false));
            g.stroke(&[(4.0, 14.0), (12.0, 14.0)], c);
        }
        Icon::Strike => {
            g.text(8.0, 8.0, "ab", 10.0, c, (false, false));
            g.stroke(&[(2.5, 8.5), (13.5, 8.5)], c);
        }
        Icon::Borders | Icon::BorderAll => {
            g.rect(2.5, 2.5, 11.0, 11.0, None, Some(light));
            g.stroke(&[(8.0, 2.5), (8.0, 13.5)], light);
            g.stroke(&[(2.5, 8.0), (13.5, 8.0)], light);
            if icon == Icon::Borders {
                g.line(&[(2.0, 13.5), (14.0, 13.5)], c, 1.6);
            } else {
                g.rect(2.5, 2.5, 11.0, 11.0, None, Some(c));
                g.stroke(&[(8.0, 2.5), (8.0, 13.5)], c);
                g.stroke(&[(2.5, 8.0), (13.5, 8.0)], c);
            }
        }
        Icon::BorderNone => {
            for i in 0..6 {
                let t = 2.5 + i as f32 * 2.2;
                g.circle(t, 2.5, 0.4, Some(light), None);
                g.circle(t, 13.5, 0.4, Some(light), None);
                g.circle(2.5, t, 0.4, Some(light), None);
                g.circle(13.5, t, 0.4, Some(light), None);
            }
        }
        Icon::BorderOutside | Icon::BorderThick => {
            g.stroke(&[(8.0, 2.5), (8.0, 13.5)], light);
            g.stroke(&[(2.5, 8.0), (13.5, 8.0)], light);
            g.line(&[(2.5, 2.5), (13.5, 2.5), (13.5, 13.5), (2.5, 13.5), (2.5, 2.5)], c, if icon == Icon::BorderThick { 2.2 } else { 1.2 });
        }
        Icon::Fill => {
            g.poly(&[(3.0, 7.0), (8.0, 2.0), (13.0, 7.0), (8.0, 12.0)], paper);
            g.line(&[(3.0, 7.0), (8.0, 2.0), (13.0, 7.0), (8.0, 12.0), (3.0, 7.0)], c, 1.1);
            g.circle(13.5, 10.0, 1.2, Some(c), None);
        }
        Icon::FontColor => {
            g.text(8.0, 7.0, "A", 11.0, c, (false, false));
        }
        Icon::AlignTop
        | Icon::AlignMiddle
        | Icon::AlignBottom
        | Icon::AlignLeft
        | Icon::AlignCenter
        | Icon::AlignRight
        | Icon::IndentDec
        | Icon::IndentInc => {
            paint_alignment(&g, icon, c, light, blue);
        }
        Icon::Orientation => {
            g.text(6.0, 9.0, "ab", 8.5, c, (false, false));
            g.stroke(&[(9.0, 13.0), (14.0, 3.0)], blue);
            g.poly(&[(14.5, 2.0), (12.0, 4.0), (14.6, 5.2)], blue);
        }
        Icon::Wrap => {
            g.stroke(&[(2.0, 4.0), (14.0, 4.0)], c);
            g.stroke(&[(2.0, 8.0), (12.0, 8.0)], c);
            g.p.add(CubicBezierShape::from_points_stroke(
                [g.pt(12.0, 8.0), g.pt(15.0, 8.0), g.pt(15.0, 12.0), g.pt(11.0, 12.0)],
                false,
                Color32::TRANSPARENT,
                Stroke::new(1.1, blue),
            ));
            g.poly(&[(9.0, 12.0), (11.5, 10.5), (11.5, 13.5)], blue);
            g.stroke(&[(2.0, 12.0), (7.0, 12.0)], c);
        }
        Icon::Merge => {
            g.rect(1.5, 3.5, 13.0, 9.0, None, Some(c));
            g.stroke(&[(4.0, 8.0), (12.0, 8.0)], blue);
            g.poly(&[(3.0, 8.0), (5.5, 6.5), (5.5, 9.5)], blue);
            g.poly(&[(13.0, 8.0), (10.5, 6.5), (10.5, 9.5)], blue);
        }
        Icon::Currency => g.text(8.0, 8.5, "$", 12.0, c, (false, false)),
        Icon::Percent => g.text(8.0, 8.5, "%", 11.0, c, (false, false)),
        Icon::Comma => g.text(8.0, 7.0, ",", 14.0, c, (true, false)),
        Icon::DecInc => {
            g.text(5.0, 5.0, ".0", 6.5, c, (false, false));
            g.text(10.0, 11.0, ".00", 6.5, c, (false, false));
            g.stroke(&[(2.0, 12.0), (5.0, 12.0)], blue);
            g.poly(&[(1.0, 12.0), (2.8, 10.6), (2.8, 13.4)], blue);
        }
        Icon::DecDec => {
            g.text(6.0, 5.0, ".00", 6.5, c, (false, false));
            g.text(11.0, 11.0, ".0", 6.5, c, (false, false));
            g.stroke(&[(2.0, 12.0), (5.5, 12.0)], blue);
            g.poly(&[(7.0, 12.0), (5.2, 10.6), (5.2, 13.4)], blue);
        }
        Icon::CondFormat => {
            g.rect(2.0, 2.0, 12.0, 12.0, Some(paper), Some(light));
            g.rect(3.0, 3.5, 4.0, 2.5, Some(red), None);
            g.rect(3.0, 7.0, 7.0, 2.5, Some(yellow), None);
            g.rect(3.0, 10.5, 10.0, 2.5, Some(blue), None);
        }
        Icon::FormatTable => {
            g.rect(2.0, 2.5, 12.0, 11.0, Some(paper), Some(light));
            g.rect(2.0, 2.5, 12.0, 3.0, Some(blue), None);
            g.rect(2.0, 8.0, 12.0, 2.5, Some(pal.wash(Cat::Blue)), None);
            g.stroke(&[(6.0, 2.5), (6.0, 13.5)], light);
            g.stroke(&[(10.0, 2.5), (10.0, 13.5)], light);
        }
        Icon::CellStyles => {
            g.rect(2.0, 3.0, 12.0, 4.5, Some(pal.wash(Cat::Green)), Some(light));
            g.rect(2.0, 8.5, 12.0, 4.5, Some(pal.wash(Cat::Red)), Some(light));
            g.stroke(&[(4.0, 5.2), (9.0, 5.2)], green);
            g.stroke(&[(4.0, 10.7), (9.0, 10.7)], red);
        }
        Icon::Insert | Icon::Delete | Icon::Format => {
            g.rect(2.0, 3.0, 12.0, 10.0, Some(paper), Some(light));
            g.stroke(&[(2.0, 6.5), (14.0, 6.5)], light);
            g.stroke(&[(2.0, 10.0), (14.0, 10.0)], light);
            g.stroke(&[(6.0, 3.0), (6.0, 13.0)], light);
            match icon {
                Icon::Insert => {
                    g.rect(6.0, 6.5, 8.0, 3.5, Some(pal.wash(Cat::Blue)), Some(blue));
                    g.line(&[(1.0, 8.2), (4.5, 8.2)], green, 1.6);
                    g.line(&[(2.75, 6.5), (2.75, 10.0)], green, 1.6);
                }
                Icon::Delete => {
                    g.line(&[(8.0, 6.0), (13.0, 11.0)], red, 1.6);
                    g.line(&[(13.0, 6.0), (8.0, 11.0)], red, 1.6);
                }
                _ => {
                    g.rect(6.0, 6.5, 8.0, 3.5, Some(pal.wash(Cat::Blue)), None);
                }
            }
        }
        Icon::Sum => g.text(8.0, 8.0, "Σ", 13.0, c, (false, false)),
        Icon::FillDown => {
            g.rect(4.0, 2.0, 8.0, 12.0, Some(paper), Some(light));
            g.stroke(&[(8.0, 4.0), (8.0, 11.0)], blue);
            g.poly(&[(5.5, 9.5), (10.5, 9.5), (8.0, 13.0)], blue);
        }
        Icon::Eraser => {
            g.poly(&[(2.0, 10.0), (8.0, 4.0), (12.0, 8.0), (6.0, 14.0)], red.gamma_multiply(0.55));
            g.poly(&[(8.0, 4.0), (10.5, 1.5), (14.5, 5.5), (12.0, 8.0)], red);
        }
        Icon::SortFilter => {
            g.text(4.5, 4.5, "A", 6.0, c, (false, false));
            g.text(4.5, 11.5, "Z", 6.0, c, (false, false));
            g.stroke(&[(8.0, 3.0), (8.0, 13.0)], blue);
            g.poly(&[(6.5, 11.0), (9.5, 11.0), (8.0, 13.5)], blue);
            g.poly(&[(10.0, 3.0), (15.0, 3.0), (13.0, 7.5), (13.0, 12.0), (12.0, 11.0), (12.0, 7.5)], light);
        }
        Icon::Find | Icon::Search | Icon::Zoom => {
            g.circle(6.5, 6.5, 4.0, None, Some(c));
            g.line(&[(9.5, 9.5), (14.0, 14.0)], c, 1.6);
        }
        Icon::Undo | Icon::Redo => {
            let flip = icon == Icon::Redo;
            let x = |v: f32| if flip { 16.0 - v } else { v };
            g.p.add(CubicBezierShape::from_points_stroke(
                [g.pt(x(4.0), 7.0), g.pt(x(9.0), 3.0), g.pt(x(15.0), 6.0), g.pt(x(12.0), 12.5)],
                false,
                Color32::TRANSPARENT,
                Stroke::new(1.2, c),
            ));
            g.poly(&[(x(2.5), 7.5), (x(6.5), 4.5), (x(6.5), 9.5)], c);
        }
        Icon::Save => {
            g.rect(2.5, 2.5, 11.0, 11.0, None, Some(c));
            g.rect(5.0, 2.5, 6.0, 4.0, None, Some(c));
            g.rect(4.5, 9.0, 7.0, 4.5, None, Some(c));
        }
        Icon::Home => {
            g.line(&[(2.0, 8.0), (8.0, 2.5), (14.0, 8.0)], c, 1.2);
            g.line(&[(4.0, 6.5), (4.0, 13.5), (12.0, 13.5), (12.0, 6.5)], c, 1.2);
        }
        Icon::SheetList => {
            for y in [4.0, 8.0, 12.0] {
                g.rect(2.5, y - 1.0, 2.0, 2.0, Some(c), None);
                g.stroke(&[(6.5, y), (13.5, y)], c);
            }
        }
        Icon::More => {
            for x in [4.0, 8.0, 12.0] {
                g.circle(x, 8.0, 1.0, Some(c), None);
            }
        }
        Icon::Chevron => g.line(&[(4.5, 6.5), (8.0, 10.0), (11.5, 6.5)], c, 1.2),
        Icon::ChevronUp => g.line(&[(4.5, 10.0), (8.0, 6.5), (11.5, 10.0)], c, 1.2),
        Icon::Left => g.line(&[(10.0, 4.0), (6.0, 8.0), (10.0, 12.0)], c, 1.2),
        Icon::Right => g.line(&[(6.0, 4.0), (10.0, 8.0), (6.0, 12.0)], c, 1.2),
        Icon::Close => {
            g.line(&[(4.0, 4.0), (12.0, 12.0)], c, 1.3);
            g.line(&[(12.0, 4.0), (4.0, 12.0)], c, 1.3);
        }
        Icon::Check => g.line(&[(3.0, 8.5), (6.5, 12.0), (13.0, 4.5)], c, 1.3),
        Icon::Checkbox => {
            g.rect(2.0, 2.0, 12.0, 12.0, None, Some(c));
            g.line(&[(4.5, 8.0), (7.0, 10.5), (11.5, 5.0)], c, 1.4);
        }
        Icon::Button => {
            let frame = Rect::from_min_max(g.pt(1.5, 3.0), g.pt(14.5, 11.0));
            g.p.rect_filled(frame, 2.0 * g.s, paper);
            g.p.rect_stroke(frame, 2.0 * g.s, Stroke::new(1.0, c), egui::StrokeKind::Inside);
            g.poly(&[(9.0, 7.5), (9.0, 15.0), (13.5, 12.5)], c);
        }
        Icon::Slider => {
            g.line(&[(2.0, 8.0), (14.0, 8.0)], light, 1.6);
            g.line(&[(2.0, 8.0), (7.0, 8.0)], blue, 1.6);
            g.circle(7.0, 8.0, 2.4, Some(paper), Some(c));
        }
        Icon::Spin => {
            g.rect(4.0, 1.5, 8.0, 13.0, Some(paper), Some(c));
            g.stroke(&[(4.0, 8.0), (12.0, 8.0)], c);
            g.poly(&[(8.0, 3.2), (10.5, 6.3), (5.5, 6.3)], c);
            g.poly(&[(5.5, 9.7), (10.5, 9.7), (8.0, 12.8)], c);
        }
        Icon::Fx => g.text(8.0, 8.0, "fx", 10.5, c, (false, true)),
        Icon::Plus => {
            g.line(&[(8.0, 3.0), (8.0, 13.0)], c, 1.3);
            g.line(&[(3.0, 8.0), (13.0, 8.0)], c, 1.3);
        }
        Icon::Minus => g.line(&[(3.0, 8.0), (13.0, 8.0)], c, 1.3),
        Icon::Normal | Icon::Gridlines | Icon::Sheet => {
            g.rect(2.0, 2.5, 12.0, 11.0, None, Some(c));
            g.stroke(&[(6.0, 2.5), (6.0, 13.5)], c);
            g.stroke(&[(10.0, 2.5), (10.0, 13.5)], c);
            g.stroke(&[(2.0, 6.2), (14.0, 6.2)], c);
            g.stroke(&[(2.0, 9.8), (14.0, 9.8)], c);
        }
        Icon::PageLayout | Icon::Paper => {
            g.rect(3.5, 1.5, 9.0, 13.0, None, Some(c));
            for y in [5.0, 7.5, 10.0] {
                g.stroke(&[(5.5, y), (10.5, y)], light);
            }
        }
        Icon::PageBreak => {
            g.rect(2.5, 2.0, 11.0, 12.0, None, Some(c));
            g.line(&[(2.5, 8.0), (13.5, 8.0)], blue, 1.6);
        }
        Icon::Table | Icon::PivotTable => {
            g.rect(2.0, 2.5, 12.0, 11.0, Some(paper), Some(c));
            g.rect(2.0, 2.5, 12.0, 3.0, Some(if icon == Icon::Table { blue } else { green }), None);
            g.stroke(&[(6.0, 2.5), (6.0, 13.5)], light);
            g.stroke(&[(2.0, 9.5), (14.0, 9.5)], light);
            if icon == Icon::PivotTable {
                g.poly(&[(9.0, 7.0), (12.5, 7.0), (12.5, 10.5)], orange);
            }
        }
        Icon::Chart | Icon::ChartBar => {
            g.stroke(&[(2.0, 14.0), (14.0, 14.0)], c);
            g.rect(3.0, 8.0, 2.5, 6.0, Some(blue), None);
            g.rect(6.8, 4.0, 2.5, 10.0, Some(green), None);
            g.rect(10.6, 6.0, 2.5, 8.0, Some(orange), None);
        }
        Icon::ChartLine | Icon::Sparkline => {
            g.stroke(&[(2.0, 14.0), (14.0, 14.0)], light);
            g.line(&[(2.0, 11.0), (5.5, 7.0), (8.5, 9.5), (13.5, 3.5)], blue, 1.5);
        }
        Icon::ChartPie => {
            g.circle(8.0, 8.0, 6.0, Some(blue), None);
            g.poly(&[(8.0, 8.0), (8.0, 2.0), (12.0, 3.4), (14.0, 8.0)], orange);
        }
        Icon::ChartScatter => {
            g.stroke(&[(2.0, 2.0), (2.0, 14.0), (14.0, 14.0)], c);
            for (x, y) in [(5.0, 11.0), (7.0, 8.0), (9.5, 9.5), (11.0, 5.0), (13.0, 4.0)] {
                g.circle(x, y, 1.1, Some(blue), None);
            }
        }
        Icon::Picture => {
            g.rect(2.0, 3.0, 12.0, 10.0, Some(pal.wash(Cat::Blue)), Some(c));
            g.poly(&[(3.0, 12.0), (7.0, 7.0), (10.0, 10.0), (11.5, 8.5), (13.0, 12.0)], green);
            g.circle(11.0, 5.5, 1.2, Some(yellow), None);
        }
        Icon::Shapes => {
            g.circle(5.5, 5.5, 3.5, Some(blue), None);
            g.rect(7.0, 7.0, 7.0, 7.0, Some(orange), None);
        }
        Icon::Link => {
            g.line(&[(6.0, 10.0), (10.0, 6.0)], c, 1.2);
            g.rect(2.0, 8.0, 6.0, 4.0, None, Some(c));
            g.rect(8.0, 4.0, 6.0, 4.0, None, Some(c));
        }
        Icon::Comment | Icon::Note => {
            g.poly(&[(2.0, 3.0), (14.0, 3.0), (14.0, 11.0), (2.0, 11.0)], if icon == Icon::Note { pal.wash(Cat::Amber) } else { paper });
            g.line(&[(2.0, 3.0), (14.0, 3.0), (14.0, 11.0), (7.0, 11.0), (4.0, 14.0), (4.0, 11.0), (2.0, 11.0), (2.0, 3.0)], c, 1.1);
        }
        Icon::TextBox => {
            g.rect(2.0, 3.0, 12.0, 10.0, None, Some(c));
            g.text(8.0, 8.0, "A", 8.0, c, (false, false));
        }
        Icon::WordArt => {
            g.text(9.0, 9.0, "A", 13.0, Color32::from_rgb(0x8E, 0x4F, 0xB5), (true, false));
            g.text(8.0, 8.0, "A", 13.0, c, (true, false));
        }
        Icon::Symbol => g.text(8.0, 8.0, "Ω", 12.0, c, (false, false)),
        Icon::Equation => {
            g.line(&[(1.5, 9.0), (3.5, 8.0), (5.5, 13.5), (8.5, 2.5), (14.5, 2.5)], c, 1.2);
            g.text(11.0, 8.5, "x", 8.0, blue, (false, true));
        }
        Icon::Function => g.text(8.0, 8.0, "fx", 11.0, c, (false, true)),
        Icon::Name => {
            g.rect(1.5, 4.0, 13.0, 8.0, None, Some(c));
            g.text(8.0, 8.0, "ab", 8.0, c, (false, false));
        }
        Icon::Trace => {
            g.circle(4.0, 12.0, 1.6, Some(blue), None);
            g.circle(12.0, 4.0, 1.6, Some(blue), None);
            g.line(&[(5.0, 11.0), (11.0, 5.0)], blue, 1.2);
            g.poly(&[(12.5, 3.5), (9.0, 4.5), (11.5, 7.0)], blue);
        }
        Icon::Calc => {
            g.rect(3.0, 1.5, 10.0, 13.0, None, Some(c));
            g.rect(4.5, 3.0, 7.0, 3.0, Some(light), None);
            for (x, y) in [(5.5, 8.5), (8.0, 8.5), (10.5, 8.5), (5.5, 11.5), (8.0, 11.5), (10.5, 11.5)] {
                g.circle(x, y, 0.8, Some(c), None);
            }
        }
        Icon::Filter => g.poly(&[(2.0, 3.0), (14.0, 3.0), (9.5, 8.5), (9.5, 13.5), (6.5, 12.0), (6.5, 8.5)], c),
        Icon::SortAsc | Icon::SortDesc => {
            let (a, b) = if icon == Icon::SortAsc { ("A", "Z") } else { ("Z", "A") };
            g.text(5.0, 4.5, a, 6.5, c, (false, false));
            g.text(5.0, 11.5, b, 6.5, c, (false, false));
            g.stroke(&[(11.0, 2.5), (11.0, 13.0)], blue);
            g.poly(&[(9.0, 11.0), (13.0, 11.0), (11.0, 14.0)], blue);
        }
        Icon::Duplicates => {
            g.rect(2.0, 2.0, 9.0, 4.0, Some(paper), Some(c));
            g.rect(5.0, 7.0, 9.0, 4.0, Some(paper), Some(c));
            g.line(&[(9.0, 12.0), (14.0, 15.0)], red, 1.4);
            g.line(&[(14.0, 12.0), (9.0, 15.0)], red, 1.4);
        }
        Icon::TextColumns => {
            g.rect(1.5, 3.0, 5.5, 10.0, None, Some(c));
            g.rect(9.0, 3.0, 5.5, 10.0, None, Some(c));
            g.poly(&[(7.0, 6.0), (9.0, 8.0), (7.0, 10.0)], blue);
        }
        Icon::Validation => {
            g.rect(2.0, 3.0, 12.0, 10.0, None, Some(c));
            g.line(&[(4.0, 8.0), (6.5, 10.5), (11.5, 5.5)], green, 1.4);
        }
        Icon::Group | Icon::Ungroup | Icon::Subtotal => {
            g.stroke(&[(3.0, 3.0), (3.0, 13.0), (6.0, 13.0)], c);
            for y in [4.0, 7.0, 10.0] {
                g.stroke(&[(7.0, y), (14.0, y)], light);
            }
            match icon {
                Icon::Group => g.text(3.0, 2.5, "+", 6.0, blue, (true, false)),
                Icon::Ungroup => g.text(3.0, 2.5, "−", 6.0, blue, (true, false)),
                _ => g.text(11.0, 13.0, "Σ", 6.0, blue, (false, false)),
            }
        }
        Icon::Lock => {
            g.rect(3.5, 7.0, 9.0, 7.0, Some(yellow), Some(c));
            g.line(&[(5.5, 7.0), (5.5, 4.5), (8.0, 2.5), (10.5, 4.5), (10.5, 7.0)], c, 1.2);
        }
        Icon::Spell => {
            g.text(7.0, 6.0, "abc", 7.0, c, (false, false));
            g.line(&[(4.0, 11.0), (6.5, 13.5), (12.0, 8.5)], green, 1.4);
        }
        Icon::Freeze => {
            g.rect(2.0, 2.5, 12.0, 11.0, None, Some(c));
            g.rect(2.0, 2.5, 12.0, 3.0, Some(pal.wash(Cat::Blue)), None);
            g.rect(2.0, 2.5, 3.5, 11.0, Some(pal.wash(Cat::Blue)), None);
            g.line(&[(2.0, 5.5), (14.0, 5.5)], blue, 1.2);
            g.line(&[(5.5, 2.5), (5.5, 13.5)], blue, 1.2);
        }
        Icon::Theme => {
            g.rect(2.0, 2.0, 5.5, 5.5, Some(blue), None);
            g.rect(8.5, 2.0, 5.5, 5.5, Some(orange), None);
            g.rect(2.0, 8.5, 5.5, 5.5, Some(green), None);
            g.rect(8.5, 8.5, 5.5, 5.5, Some(purple), None);
        }
        Icon::Margins => {
            g.rect(3.0, 1.5, 10.0, 13.0, None, Some(c));
            g.rect(5.0, 3.5, 6.0, 9.0, None, Some(blue));
        }
        Icon::Orient => {
            g.rect(2.0, 4.0, 8.0, 10.0, None, Some(c));
            g.rect(6.0, 2.0, 8.0, 6.0, Some(paper), Some(blue));
        }
        Icon::SheetRtl => {
            // A sheet with an arrow running right to left.
            g.rect(2.0, 2.5, 12.0, 8.0, None, Some(c));
            g.stroke(&[(6.0, 2.5), (6.0, 10.5)], light);
            g.stroke(&[(10.0, 2.5), (10.0, 10.5)], light);
            g.line(&[(13.5, 13.0), (4.5, 13.0)], blue, 1.4);
            g.poly(&[(2.0, 13.0), (5.0, 10.5), (5.0, 15.5)], blue);
        }
        Icon::PrintArea => {
            g.rect(2.5, 2.5, 11.0, 11.0, None, Some(light));
            g.line(&[(4.0, 4.0), (11.0, 4.0), (11.0, 10.0), (4.0, 10.0), (4.0, 4.0)], blue, 1.4);
        }
        Icon::Script => {
            g.rect(2.0, 2.0, 12.0, 12.0, None, Some(c));
            g.line(&[(4.5, 6.0), (7.0, 8.0), (4.5, 10.0)], green, 1.3);
            g.stroke(&[(8.0, 10.5), (11.5, 10.5)], c);
        }
        Icon::Pen => {
            g.poly(&[(3.0, 13.0), (4.0, 10.0), (11.0, 3.0), (13.0, 5.0), (6.0, 12.0)], blue);
            g.stroke(&[(3.0, 13.0), (4.0, 10.0)], c);
        }
        Icon::Share => {
            g.line(&[(4.0, 7.0), (4.0, 13.0), (12.0, 13.0), (12.0, 7.0)], c, 1.2);
            g.line(&[(8.0, 10.0), (8.0, 2.5)], c, 1.2);
            g.line(&[(5.5, 5.0), (8.0, 2.5), (10.5, 5.0)], c, 1.2);
        }
        Icon::Settings => {
            g.circle(8.0, 8.0, 5.0, None, Some(c));
            g.circle(8.0, 8.0, 2.0, Some(c), None);
        }
        Icon::Appearance => {
            g.circle(8.0, 8.0, 5.5, None, Some(c));
            let half: Vec<(f32, f32)> = (0..=12)
                .map(|i| {
                    let a = std::f32::consts::PI * (i as f32 / 12.0 - 0.5);
                    (8.0 + 5.5 * a.cos(), 8.0 + 5.5 * a.sin())
                })
                .collect();
            g.poly(&half, c);
        }
        Icon::Book => {
            g.rect(2.0, 2.0, 12.0, 12.0, Some(green), None);
            g.rect(4.0, 4.0, 8.0, 8.0, Some(paper), None);
            g.stroke(&[(8.0, 4.0), (8.0, 12.0)], green);
            g.stroke(&[(4.0, 8.0), (12.0, 8.0)], green);
        }
        Icon::Folder => {
            g.poly(&[(1.5, 4.0), (6.0, 4.0), (7.5, 5.5), (14.5, 5.5), (14.5, 13.0), (1.5, 13.0)], yellow);
        }
        Icon::FnBook(glyph) => {
            let spine = pal.accent.unwrap_or(c);
            g.rect(3.0, 1.5, 10.5, 13.0, Some(paper), Some(c));
            g.rect(3.0, 1.5, 2.5, 13.0, Some(spine), None);
            let (x, y) = (9.5, 7.8);
            match glyph {
                FnGlyph::Star => {
                    let pts: Vec<(f32, f32)> = (0..10)
                        .map(|i| {
                            let a = std::f32::consts::PI * (i as f32 / 5.0 - 0.5);
                            let rad = if i % 2 == 0 { 3.4 } else { 1.4 };
                            (x + rad * a.cos(), y + rad * a.sin())
                        })
                        .collect();
                    g.closed(&pts, spine, 1.0);
                }
                FnGlyph::Coins => {
                    for k in 0..3 {
                        let top = y + 1.6 - k as f32 * 2.3;
                        g.p.rect_filled(Rect::from_min_max(g.pt(x - 2.8, top), g.pt(x + 2.8, top + 1.7)), 0.9 * g.s, spine);
                    }
                }
                FnGlyph::Question => g.text(x, y, "?", 8.5, spine, (true, false)),
                FnGlyph::Letter => g.text(x, y, "A", 8.0, spine, (true, false)),
                FnGlyph::Clock => {
                    g.circle(x, y, 3.2, None, Some(spine));
                    g.stroke(&[(x, y - 2.0), (x, y), (x + 1.6, y + 0.9)], spine);
                }
                FnGlyph::Magnifier => {
                    g.circle(x - 0.6, y - 0.6, 2.3, None, Some(spine));
                    g.line(&[(x + 1.0, y + 1.0), (x + 3.0, y + 3.0)], spine, 1.5);
                }
                FnGlyph::Theta => g.text(x, y, "θ", 8.5, spine, (false, false)),
                FnGlyph::Dots => {
                    for dx in [-2.4, 0.0, 2.4] {
                        g.circle(x + dx, y, 0.9, Some(spine), None);
                    }
                }
            }
        }
        Icon::Header | Icon::Footer => {
            g.rect(3.5, 1.5, 9.0, 13.0, None, Some(c));
            let y = if icon == Icon::Header { 2.5 } else { 11.0 };
            g.rect(4.5, y, 7.0, 2.5, Some(blue), None);
            for yy in if icon == Icon::Header { [7.0, 9.5] } else { [4.5, 7.0] } {
                g.stroke(&[(5.5, yy), (10.5, yy)], light);
            }
        }
        Icon::PageNumber | Icon::PageCount => {
            if icon == Icon::PageCount {
                g.rect(5.5, 1.0, 8.5, 11.5, None, Some(light));
            }
            g.rect(2.5, 3.0, 8.5, 12.0, Some(paper), Some(c));
            // A "#" mark.
            g.stroke(&[(5.3, 6.5), (4.8, 12.5)], blue);
            g.stroke(&[(8.3, 6.5), (7.8, 12.5)], blue);
            g.stroke(&[(4.0, 8.4), (9.6, 8.4)], blue);
            g.stroke(&[(3.8, 10.8), (9.4, 10.8)], blue);
        }
        Icon::Calendar => {
            g.rect(2.0, 3.0, 12.0, 11.0, Some(paper), Some(c));
            g.rect(2.0, 3.0, 12.0, 3.0, Some(blue), None);
            for (x, y) in [(4.5, 8.5), (7.5, 8.5), (10.5, 8.5), (4.5, 11.5), (7.5, 11.5)] {
                g.rect(x - 0.6, y - 0.6, 1.4, 1.4, Some(c), None);
            }
        }
        Icon::Clock => {
            let center = g.pt(8.0, 8.0);
            g.p.circle_stroke(center, 6.0 * g.s, Stroke::new(1.15 * g.s.clamp(1.0, 2.0), c));
            g.stroke(&[(8.0, 4.5), (8.0, 8.0), (10.8, 9.6)], blue);
        }
        Icon::FileName => {
            g.poly(&[(3.0, 1.5), (10.0, 1.5), (13.0, 4.5), (13.0, 14.5), (3.0, 14.5)], paper);
            g.line(&[(3.0, 1.5), (10.0, 1.5), (13.0, 4.5), (13.0, 14.5), (3.0, 14.5), (3.0, 1.5)], c, 1.15);
            g.stroke(&[(10.0, 1.5), (10.0, 4.5), (13.0, 4.5)], c);
            g.line(&[(5.0, 9.0), (11.0, 9.0)], blue, 1.4);
            g.stroke(&[(5.0, 11.5), (9.0, 11.5)], light);
        }
        Icon::Back => {
            g.circle(8.0, 8.0, 6.5, None, Some(c));
            g.line(&[(11.5, 8.0), (4.8, 8.0)], c, 1.2);
            g.line(&[(7.5, 5.3), (4.8, 8.0), (7.5, 10.7)], c, 1.2);
        }
        Icon::NewFile => {
            g.line(&[(9.5, 1.5), (3.5, 1.5), (3.5, 14.5), (12.5, 14.5), (12.5, 4.5), (9.5, 1.5), (9.5, 4.5), (12.5, 4.5)], c, 1.1);
        }
        Icon::OpenFolder => {
            g.line(&[(13.0, 6.5), (13.0, 4.5), (7.5, 4.5), (6.0, 3.0), (1.5, 3.0), (1.5, 13.0)], c, 1.1);
            g.line(&[(1.5, 13.0), (4.0, 7.0), (15.0, 7.0), (12.5, 13.0), (1.5, 13.0)], c, 1.1);
        }
        Icon::Info => {
            g.circle(8.0, 8.0, 6.5, None, Some(c));
            g.line(&[(8.0, 7.0), (8.0, 11.8)], c, 1.3);
            g.circle(8.0, 4.8, 0.8, Some(c), None);
        }
        Icon::Print => {
            g.line(&[(4.5, 5.5), (4.5, 1.5), (11.5, 1.5), (11.5, 5.5)], c, 1.1);
            g.line(&[(4.0, 11.5), (1.5, 11.5), (1.5, 5.5), (14.5, 5.5), (14.5, 11.5), (12.0, 11.5)], c, 1.1);
            g.line(&[(4.5, 9.0), (4.5, 14.5), (11.5, 14.5), (11.5, 9.0), (4.5, 9.0)], c, 1.1);
            g.circle(12.3, 7.6, 0.6, Some(c), None);
        }
        Icon::Macro => {
            g.rect(2.5, 1.5, 9.0, 12.0, Some(paper), Some(c));
            for y in [4.5, 7.0, 9.5] {
                g.stroke(&[(4.5, y), (9.5, y)], light);
            }
            g.poly(&[(9.5, 8.5), (15.0, 11.75), (9.5, 15.0)], green);
        }
        Icon::Record => {
            g.circle(8.0, 8.0, 6.0, None, Some(c));
            g.circle(8.0, 8.0, 3.5, Some(pal.recording), None);
        }
        Icon::Relative => {
            g.rect(1.5, 1.5, 5.5, 5.0, Some(paper), Some(c));
            g.rect(9.0, 9.5, 5.5, 5.0, None, Some(c));
            g.line(&[(5.0, 7.5), (5.0, 12.0), (8.0, 12.0)], c, 1.1);
            g.poly(&[(7.5, 10.2), (9.6, 12.0), (7.5, 13.8)], c);
        }
        Icon::Collapse | Icon::Expand => {
            // A small sheet with a selected block and an arrow pointing up (collapse) or down.
            g.rect(2.0, 6.5, 12.0, 7.5, None, Some(c));
            g.rect(4.0, 9.0, 4.0, 3.0, Some(c), None);
            if icon == Icon::Collapse {
                g.poly(&[(8.0, 1.0), (11.5, 4.5), (4.5, 4.5)], c);
            } else {
                g.poly(&[(8.0, 5.5), (11.5, 2.0), (4.5, 2.0)], c);
            }
        }
        Icon::Play => g.poly(&[(4.0, 2.5), (13.0, 8.0), (4.0, 13.5)], green),
        Icon::Step => {
            g.line(&[(8.0, 1.5), (8.0, 9.0)], c, 1.3);
            g.poly(&[(4.8, 7.0), (11.2, 7.0), (8.0, 10.8)], c);
            g.line(&[(3.0, 13.5), (13.0, 13.5)], c, 1.3);
        }
        Icon::Continue => {
            g.rect(2.5, 2.5, 2.2, 11.0, Some(c), None);
            g.poly(&[(6.5, 2.5), (14.0, 8.0), (6.5, 13.5)], green);
        }
        Icon::RunTo => {
            g.line(&[(2.0, 5.0), (9.0, 5.0)], c, 1.3);
            g.poly(&[(8.0, 2.0), (12.0, 5.0), (8.0, 8.0)], c);
            g.line(&[(2.0, 11.5), (14.0, 11.5)], c, 1.6);
            g.line(&[(13.0, 3.0), (13.0, 9.0)], c, 1.1);
        }
        Icon::StopSquare => g.rect(3.0, 3.0, 10.0, 10.0, Some(pal.recording), None),
    }
}

/// Icons offered by Insert › Icons, by name.
pub const LIBRARY: &[(&str, Icon)] = &[
    ("check", Icon::Check),
    ("close", Icon::Close),
    ("plus", Icon::Plus),
    ("minus", Icon::Minus),
    ("chart", Icon::Chart),
    ("line chart", Icon::ChartLine),
    ("pie chart", Icon::ChartPie),
    ("scatter", Icon::ChartScatter),
    ("table", Icon::Table),
    ("sum", Icon::Sum),
    ("filter", Icon::Filter),
    ("lock", Icon::Lock),
    ("comment", Icon::Comment),
    ("note", Icon::Note),
    ("picture", Icon::Picture),
    ("book", Icon::Book),
    ("folder", Icon::Folder),
    ("search", Icon::Search),
    ("calculator", Icon::Calc),
    ("link", Icon::Link),
    ("pen", Icon::Pen),
    ("share", Icon::Share),
    ("settings", Icon::Settings),
    ("home", Icon::Home),
    ("save", Icon::Save),
    ("theme", Icon::Theme),
    ("shapes", Icon::Shapes),
    ("text", Icon::TextBox),
    ("trace", Icon::Trace),
    ("validation", Icon::Validation),
];

pub fn from_name(name: &str) -> Option<Icon> {
    LIBRARY.iter().find(|(n, _)| n.eq_ignore_ascii_case(name)).map(|(_, i)| *i)
}

fn paint_alignment(g: &G<'_>, icon: Icon, c: Color32, light: Color32, blue: Color32) {
    if matches!(icon, Icon::AlignTop | Icon::AlignMiddle | Icon::AlignBottom) {
        let ys: [f32; 3] = match icon {
            Icon::AlignTop => [3.0, 5.5, 8.0],
            Icon::AlignMiddle => [5.5, 8.0, 10.5],
            _ => [8.0, 10.5, 13.0],
        };
        let edge = match icon {
            Icon::AlignTop => 1.5,
            Icon::AlignMiddle => -1.0,
            _ => 14.5,
        };
        if edge > 0.0 {
            g.stroke(&[(2.0, edge), (14.0, edge)], light);
        }
        for (i, y) in ys.iter().enumerate() {
            let w = if i == 1 { 8.0 } else { 10.0 };
            g.stroke(&[(8.0 - w / 2.0, *y), (8.0 + w / 2.0, *y)], c);
        }
    } else {
        for (i, y) in [3.5f32, 6.5, 9.5, 12.5].iter().enumerate() {
            let w = if i % 2 == 0 { 12.0 } else { 8.0 };
            let (x0, x1) = match icon {
                Icon::AlignLeft => (2.0, 2.0 + w),
                Icon::AlignRight => (14.0 - w, 14.0),
                Icon::AlignCenter => (8.0 - w / 2.0, 8.0 + w / 2.0),
                _ => (if i == 0 || i == 3 { 2.0 } else { 7.0 }, 14.0),
            };
            g.stroke(&[(x0, *y), (x1, *y)], c);
        }
        if icon == Icon::IndentInc {
            g.poly(&[(2.0, 6.0), (5.0, 8.0), (2.0, 10.0)], blue);
        }
        if icon == Icon::IndentDec {
            g.poly(&[(5.0, 6.0), (2.0, 8.0), (5.0, 10.0)], blue);
        }
    }
}

#[cfg(test)]
mod fluent_tests {
    use super::*;

    /// Every icon variant without a payload, plus a Function Library book.
    fn sample_icons() -> Vec<Icon> {
        vec![Icon::Paste, Icon::Save, Icon::Close, Icon::Check, Icon::ChartPie, Icon::Settings, Icon::Percent, Icon::FnBook(FnGlyph::Star)]
    }

    #[test]
    fn every_fluent_glyph_is_in_the_subset_font() {
        use skrifa::MetadataProvider as _;
        let font = skrifa::FontRef::new(FLUENT_FONT).expect("the subset is a font");
        let charmap = font.charmap();
        // Walk every icon the table names (the match covers them all; `sample_icons` alone would
        // miss most): build each variant through the glyph table's own source.
        let src = include_str!("icons.rs");
        let mut checked = 0;
        for line in src.lines().filter(|l| l.trim_start().starts_with("Icon::") && l.contains("=> '\\u{")) {
            let hex = line.split("\\u{").nth(1).and_then(|r| r.split('}').next()).unwrap();
            let ch = char::from_u32(u32::from_str_radix(hex, 16).unwrap()).unwrap();
            assert!(charmap.map(ch).is_some(), "the subset has no glyph for {line}");
            checked += 1;
        }
        assert!(checked > 100, "the glyph table was read: {checked} entries");
    }

    /// The shapes painting `icon` adds, with `fonts` installed (or egui's defaults).
    fn painted(icon: Icon, fluent: bool) -> Vec<egui::Shape> {
        let ctx = egui::Context::default();
        if fluent {
            ctx.set_fonts(crate::theme::pinned_font_definitions());
        }
        let mut shapes = Vec::new();
        // Fonts set now apply from the next pass.
        for _ in 0..2 {
            let mut out = ctx.run_ui(egui::RawInput::default(), |ui| {
                let layer = egui::LayerId::new(egui::Order::Foreground, egui::Id::unique("icon-test"));
                let p = ui.ctx().layer_painter(layer);
                paint(&p, Rect::from_min_size(pos2(10.0, 10.0), vec2(20.0, 20.0)), icon, Color32::BLACK);
            });
            out.textures_delta.clear();
            shapes = out.shapes.into_iter().map(|c| c.shape).collect();
        }
        shapes
    }

    /// Whether any of `shapes` is text set in the Fluent icon family.
    fn has_text(shapes: &[egui::Shape]) -> bool {
        shapes.iter().any(|s| match s {
            egui::Shape::Text(t) => {
                t.galley.job.sections.iter().any(|sec| matches!(&sec.format.font_id.family, egui::FontFamily::Name(n) if &**n == FLUENT_FAMILY))
            }
            _ => false,
        })
    }

    #[test]
    fn icons_draw_from_fluent_when_its_font_is_loaded_and_in_code_otherwise() {
        for icon in sample_icons() {
            let mapped = fluent_glyph(icon).is_some();
            assert_eq!(has_text(&painted(icon, true)), mapped, "{icon:?} with the Fluent font");
            if mapped {
                assert!(!has_text(&painted(icon, false)), "{icon:?} without it is drawn in code, never from a missing family");
            }
        }
        assert_eq!(fluent_glyph(Icon::Percent), None, "Fluent has no plain percent: still drawn in code");
        assert_eq!(fluent_glyph(Icon::FnBook(FnGlyph::Coins)), None);
    }
}
