//! The desktop app sets its context up with `setup_context_for_language` and then lets the app
//! apply its fonts; the installed-font scan must start on that path, or no installed font is ever
//! found (cells in them fall back to the default and the Font list shows only the standard
//! names). Its own test binary: the scan is process-wide and starts once, so no other test may
//! start it first.
#![cfg(not(target_arch = "wasm32"))]

use gridcraft_engine::Session;
use gridcraft_ui_egui::{Services, SheetApp, fonts};

#[test]
fn the_production_font_setup_starts_the_installed_font_scan() {
    let ctx = egui::Context::default();
    // As the desktop app does before its first frame.
    SheetApp::setup_context_for_language(&ctx, false, gridcraft_ui_egui::i18n::Language::En);
    assert!(!fonts::scan_started(), "setting up the context alone doesn't start it");
    let mut session = Session::new();
    session.new_workbook();
    let mut app = SheetApp::new(session, Services::default());
    let mut out = ctx.run_ui(egui::RawInput::default(), |ui| {
        app.logic(ui.ctx());
        app.ui(ui);
    });
    out.textures_delta.clear();
    assert!(fonts::scan_started(), "the app's first frame starts the installed-font scan");
}
