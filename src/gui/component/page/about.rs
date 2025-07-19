use std::backtrace::Backtrace;

use egui::Ui;
use log::error;
use rust_i18n::t;

use crate::{VERSION, gui::CelestialApp};

pub trait AboutPageExt {
    fn about_page(&mut self, ui: &mut Ui);
}

impl AboutPageExt for CelestialApp {
    fn about_page(&mut self, ui: &mut Ui) {
        ui.label(t!("about.content"));
        ui.label(t!("about.metadata", version = VERSION));
        // the open operation wont be executed if the clicked() condition is false
        if ui.link("https://lunarclient.top").clicked()
            && open::that("https://lunarclient.top").is_err()
        {
            error!("Failed to open link: {}", Backtrace::capture());
        }
    }
}
