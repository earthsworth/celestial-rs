use egui::{TextBuffer, Ui};
use rust_i18n::t;

use crate::gui::{AppPage, AppState, CelestialApp};

use super::page::{
    about::AboutPageExt, addons::AddonsPageExt, index::IndexPageExt, launch::LaunchPageExt,
    settings::SettingsPageExt,
};

pub trait MainWindowExt {
    fn navigate_bar(&mut self, ui: &mut Ui);
    fn main_window(&mut self, ui: &mut Ui);
}

impl MainWindowExt for CelestialApp {
    fn main_window(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                self.navigate_bar(ui);
            });

            match self.state.page {
                AppPage::Index => self.index_page(ui),
                AppPage::Launch => self.launch_page(ui),
                AppPage::Addons => self.addons_page(ui),
                AppPage::Settings => self.settings_page(ui),
                AppPage::About => self.about_page(ui),
            }
        });
    }

    fn navigate_bar(&mut self, ui: &mut Ui) {
        ui.label(t!("app.title").as_str());
        navigate_btn(&mut self.state, ui, AppPage::Index, t!("page.index.title").as_str());
        navigate_btn(&mut self.state, ui, AppPage::Launch, t!("page.launch.title").as_str());
        navigate_btn(&mut self.state, ui, AppPage::Settings, t!("page.settings.title").as_str());
        navigate_btn(&mut self.state, ui, AppPage::About, t!("page.about.title").as_str());
    }
}

fn navigate_btn(state: &mut AppState, ui: &mut Ui, page: AppPage, text: &str) {
    let btn = ui.selectable_label(state.page == page, text);
    if btn.clicked() {
        state.page = page;
    }
}
