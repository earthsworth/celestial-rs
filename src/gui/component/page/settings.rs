use egui::Ui;

use crate::gui::CelestialApp;

pub trait SettingsPageExt {
    fn settings_page(&mut self, ui: &mut Ui);

}

impl SettingsPageExt for CelestialApp {
    fn settings_page(&mut self, ui: &mut Ui) {
        ui.label("settings");
    }
}
