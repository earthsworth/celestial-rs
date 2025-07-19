use egui::{Color32, Ui};

use crate::gui::CelestialApp;

pub trait IndexPageExt {
    fn index_page(&mut self, ui: &mut Ui);
}

impl IndexPageExt for CelestialApp {
    fn index_page(&mut self, ui: &mut Ui) {
        ui.label("Welcome to Celestial.rs. The Rust implementation of the Lunar Client Launcher!");
        ui.label("Click the buttons on the topbar to get startted.");
        ui.colored_label(
            Color32::from_hex("#edcf0e").unwrap(),
            "Warning: This project is work in progress.",
        );
        if ui.link("https://lunarclient.top").clicked() {
            open::that("https://lunarclient.top").ok();
        }
    }
}
