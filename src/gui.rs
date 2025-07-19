use std::sync::Arc;

use component::main_window::MainWindowExt;
use egui_inbox::UiInbox;

mod component;

pub struct CelestialApp {
    state: AppState,
    inbox: Arc<UiInbox<AppState>>,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct AppState {
    page: AppPage
}

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Eq)]
pub enum AppPage {
    Index,
    Launch,
    Addons,
    Settings,
    About,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            page: AppPage::Index,
        }
    }
}

impl CelestialApp {
    pub fn new(cc: &eframe::CreationContext<'_>, inbox: Arc<UiInbox<AppState>>) -> Self {
        let state = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        };
        Self { state, inbox }
    }
}

impl eframe::App for CelestialApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, &self.state);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(1.1);
        egui::CentralPanel::default().show(ctx, |ui| {
            self.inbox.replace(ui, &mut self.state);
            self.main_window(ui);
        });
    }
}

