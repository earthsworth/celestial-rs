use egui::Ui;

use crate::gui::CelestialApp;

pub trait AddonsPageExt {
    fn addons_page(&mut self, ui: &mut Ui);

}

impl AddonsPageExt for CelestialApp {
    fn addons_page(&mut self, ui: &mut Ui) {
        ui.label("addons page");
    }
}
