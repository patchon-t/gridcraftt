//! Home ▸ Font lists the fonts installed on the machine, not only the standard names, and on the
//! desktop ends with Add font file… for a downloaded font.

use egui::{Event, Key, Modifiers};
use egui_kittest::kittest::Queryable as _;
use gridcraft_engine::Session;
use gridcraft_ui_egui::{Services, SheetApp};
use serde_json::json;

type Harness = egui_kittest::Harness<'static, SheetApp>;

fn harness(services: Services) -> Harness {
    let mut s = Session::new();
    s.new_workbook();
    s.execute("selection.set", json!({"range": "A1"})).unwrap();
    let app = crate::common::prepare(SheetApp::new(s, services));
    let mut h = crate::common::builder().with_size(egui::vec2(1200.0, 800.0)).build_ui_state(
        |ui, app: &mut SheetApp| {
            app.logic(ui.ctx());
            app.ui(ui);
        },
        app,
    );
    h.run_steps(4);
    h
}

/// Opens the Font list from the box showing A1's font, and types `filter` into it.
fn open_font_list(h: &mut Harness, filter: &str) {
    let font = h.state().session.active().unwrap().wb.active().unwrap().style_id(gridcraft_engine::core::CellRef::parse("A1").unwrap());
    let name = h.state().session.active().unwrap().wb.styles.get(font).font.name.clone();
    h.get_by_label(&name).click();
    h.run_steps(4);
    h.input_mut().events.push(Event::Text(filter.to_string()));
    h.run_steps(3);
}

#[test]
fn a_font_added_from_a_file_is_in_the_font_list_and_applies_to_the_selection() {
    // egui's bundled face stands in for a downloaded font; the copy goes to a temporary folder.
    let defs = egui::FontDefinitions::default();
    let bytes = (*defs.font_data.get("Ubuntu-Light").expect("bundled face").font).as_ref().to_vec();
    let dir = std::env::temp_dir().join(format!("gridcraft-font-list-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let source = dir.join("download.ttf");
    std::fs::write(&source, bytes).unwrap();
    let family = gridcraft_ui_egui::fonts::add_font_file(source.to_str().unwrap(), Some(&dir.join("installed"))).unwrap();

    let mut h = harness(Services::default());
    open_font_list(&mut h, &family);
    assert!(h.get_all_by_label(&family).count() > 0, "the list offers the added font");
    h.input_mut().events.extend([
        Event::Key { key: Key::Enter, physical_key: None, pressed: true, repeat: false, modifiers: Modifiers::NONE },
        Event::Key { key: Key::Enter, physical_key: None, pressed: false, repeat: false, modifiers: Modifiers::NONE },
    ]);
    h.run_steps(4);
    let d = h.state().session.active().unwrap();
    let style = d.wb.active().unwrap().style_id(gridcraft_engine::core::CellRef::parse("A1").unwrap());
    assert_eq!(d.wb.styles.get(style).font.name, family, "picking it sets the selection's font");
}

#[test]
fn add_font_file_is_offered_only_where_a_file_can_be_picked() {
    let mut h = harness(Services::default());
    open_font_list(&mut h, "Add font");
    assert!(h.query_by_label("Add font file…").is_none(), "no file picker (the web build): no row");

    let mut h = harness(Services { pick_font: Some(Box::new(|| None)), ..Default::default() });
    open_font_list(&mut h, "Add font");
    assert!(h.query_by_label("Add font file…").is_some(), "the desktop offers it");
}

#[test]
fn the_add_font_command_refuses_what_it_cannot_use() {
    let mut h = harness(Services::default());
    assert!(h.state_mut().run("fonts.addFile", json!({})).is_err(), "a file is required");
    assert!(h.state_mut().run("fonts.addFile", json!({"file": "/no/such/dir/font.ttf"})).is_err());
}
