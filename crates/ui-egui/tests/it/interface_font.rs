//! File › Options › Interface font (`options.interfaceFont`) and the transition between the File
//! view and the sheet.

use egui_kittest::kittest::Queryable as _;
use gridcraft_engine::Session;
use gridcraft_ui_egui::backstage::Page;
use serde_json::json;

fn harness() -> crate::common::Harness {
    let mut session = Session::new();
    session.new_workbook();
    let builder = crate::common::builder().with_size(egui::vec2(1200.0, 800.0)).with_step_dt(1.0 / 60.0);
    let h = crate::common::harness_with(builder, crate::common::app(session), 4);
    // The harness turns egui's animations off; transitions follow them, as in the app.
    h.ctx.all_styles_mut(|s| s.animation_time = 0.1);
    h
}

#[test]
fn the_interface_font_command_resets_and_refuses_what_it_cannot_use() {
    let mut h = harness();
    assert_eq!(h.state_mut().run("options.interfaceFont", json!({})), Ok(json!({"font": null})), "no parameters: the default font");
    assert_eq!(h.state().ui.interface_font, None);
    assert!(h.state_mut().run("options.interfaceFont", json!({"family": "No Such Font Family 1234"})).is_err());
    assert!(h.state_mut().run("options.interfaceFont", json!({"file": "/no/such/dir/font.ttf"})).is_err());
    assert_eq!(h.state().ui.interface_font, None, "a refused font changes nothing");
    assert_eq!(h.state_mut().run("options.interfaceFont", json!({"family": "default"})), Ok(json!({"font": null})));
    h.run_steps(3);
    assert!(h.query_by_label("AutoSave").is_some(), "the window still draws after the font set is rebuilt");
}

#[test]
fn the_options_page_offers_the_interface_font() {
    let mut h = harness();
    h.state_mut().open_backstage(Page::Options);
    h.run_steps(12);
    assert!(h.query_by_label("Interface font").is_some(), "File › Options › General has an Interface font row");
}

/// Where the File view's Back button is, `steps` frames after the File view opened.
fn back_button_left(animations: bool, steps: usize) -> f32 {
    let mut h = harness();
    h.state_mut().ui.animations = animations;
    h.state_mut().open_backstage(Page::Home);
    h.run_steps(steps);
    h.get_by_label("Back").rect().left()
}

#[test]
fn the_file_view_slides_in_from_the_left_and_settles() {
    let settled = back_button_left(true, 40);
    let entering = back_button_left(true, 1);
    assert!(entering < settled - 1.0, "the File view comes in from the left: {entering} then {settled}");
    assert_eq!(back_button_left(false, 1), settled, "with animations off it is in place at once");
}

#[test]
fn leaving_the_file_view_slides_the_sheet_in_from_the_right() {
    let paste_left = |animations: bool, steps: usize| {
        let mut h = harness();
        h.state_mut().ui.animations = animations;
        h.state_mut().open_backstage(Page::Home);
        h.run_steps(40);
        h.state_mut().backstage = None;
        h.run_steps(steps);
        h.get_by_label("Paste").rect().left()
    };
    let settled = paste_left(true, 40);
    let entering = paste_left(true, 1);
    assert!(entering > settled + 1.0, "the sheet comes back in from the right: {entering} then {settled}");
    assert_eq!(paste_left(false, 1), settled);
}
