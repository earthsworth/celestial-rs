use egui::{Ui, Widget};
use tokio::runtime::Runtime;
use celestial_rs::utils;
use crate::gui::CelestialApp;

pub trait LaunchPageExt {
    fn launch_page(&mut self, ui: &mut Ui);
}

impl LaunchPageExt for CelestialApp {
    fn launch_page(&mut self, ui: &mut Ui) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.label("test");
        });

        let progress = self.launch_state.completed_tasks as f32 / self.launch_state.total_tasks() as f32;
        egui::TopBottomPanel::bottom("bottom_panel").show_animated_inside(
            ui,
            progress < 1.0,
            |ui| {
                ui.horizontal(|ui| {
                    let status_text = match self.launch_state.status_text.as_str() {
                        "" => format!("{:.1}% - Working...", progress * 100.0),
                        v => format!("{:.1}% - {v}", progress * 100.0),
                    };
                    ui.label("Working: ");
                    egui::ProgressBar::new(progress)
                        .animate(true)
                        .text(status_text)
                        .ui(ui);
                });
            },
        );
    }
}

