use egui::Ui;

use crate::gui::CelestialApp;

pub trait AboutPageExt {
    fn about_page(&mut self, ui: &mut Ui);

}

impl AboutPageExt for CelestialApp {
    fn about_page(&mut self, ui: &mut Ui) {
        ui.label("The 1# launcher for your browser.");
    }
}
