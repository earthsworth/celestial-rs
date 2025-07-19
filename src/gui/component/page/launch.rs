use egui::Ui;

use crate::gui::CelestialApp;

pub trait LaunchPageExt {
    fn launch_page(&mut self, ui: &mut Ui);

}

impl LaunchPageExt for CelestialApp {
    fn launch_page(&mut self, ui: &mut Ui) {
        ui.label("launch page");
    }
}
