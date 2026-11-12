//! Community links: the NOVA Workspace suite never sends the user to another site.
//!
//! These tests are the guard for that: no link button in the interface, no community
//! entry in the Help menu, and an empty link table in the engine.

use egui_kittest::Harness;
use pdfcraft_engine::links;
use pdfcraft_ui_egui::{Dialog, NovaPdfApp};

fn harness(setup: impl FnOnce(&mut NovaPdfApp) + 'static) -> Harness<'static, NovaPdfApp> {
    let mut h = Harness::builder().with_size(egui::vec2(1400.0, 900.0)).build_eframe(move |_cc| {
        let mut app = NovaPdfApp::new();
        app.set_option("language", "en").unwrap();
        setup(&mut app);
        app
    });
    h.run_steps(4);
    h
}

#[test]
fn the_engine_has_no_community_links() {
    assert!(links::LINKS.is_empty(), "the suite offers no link that leaves the app");
    assert_eq!(links::COMMUNITY, "");
    assert_eq!(links::APP_PAGE, "");
    assert_eq!(links::GITHUB, "");
    assert_eq!(links::RELEASES, "");
}

#[test]
fn the_title_bar_has_no_github_button() {
    let mut h = harness(|_| {});
    assert!(h.query_by_label("GitHub").is_none(), "no link that leaves the app");
}

#[test]
fn the_home_screen_has_no_community_card() {
    let mut h = harness(|_| {});
    assert!(h.query_by_label("Join the NOVA community").is_none());
    assert!(h.query_by_label("Project home").is_none());
}

#[test]
fn the_about_dialog_has_no_links() {
    let mut h = harness(|app| app.dialog = Some(Dialog::About));
    for label in ["Project home", "NOVA PDF project page", "NOVA PDF on GitHub", "Source code of the office suite"] {
        assert!(h.query_by_label(label).is_none(), "{label} would be a link that leaves the app");
    }
}

/// Runs frames until the node labelled `label` exists and has stopped moving, then returns it.
///
/// The About dialog is a modal centred on its own size, and switching tabs changes that size, so
/// for a few frames afterwards the modal grows and recentres and every control in it moves. A
/// fixed number of frames is a guess at how long that takes: it was too few after the 0.5.0
/// contributors refresh (78 names instead of 5) on some machines, the click on Table landed where
/// the button had been, and the table never opened (#671).
fn settled<'a>(h: &'a mut Harness<'static, NovaPdfApp>, label: &'a str) -> egui_kittest::Node<'a> {
    let mut last = None;
    for _ in 0..60 {
        h.run_steps(1);
        let now = h.query_by_label(label).map(|n| n.rect());
        if now.is_some() && now == last {
            return h.get_by_label(label);
        }
        last = now;
    }
    panic!("{label:?} never appeared or never stopped moving");
}

#[test]
fn about_dialog_has_contributors_and_models_tabs() {
    let mut h = harness(|app| app.dialog = Some(Dialog::About));
    settled(&mut h, "Contributors").click();
    // The owner is always in the compiled-in credits (contributors/contributors.json), shown by username.
    settled(&mut h, "@echelon");
    settled(&mut h, "Table").click();
    settled(&mut h, "PRs");
    settled(&mut h, "Display name").click();
    settled(&mut h, "Brandon Thomas");
    settled(&mut h, "Models").click();
    h.run_steps(4);
    assert!(h.query_all_by_label("Anthropic").count() >= 1);
}
