use std::backtrace::Backtrace;

use egui::Ui;
use log::error;
use rust_i18n::t;

use crate::{CelestialApp, consts};

pub trait AboutPageExt {
    fn about_page(&mut self, ui: &mut Ui);
}

impl AboutPageExt for CelestialApp {
    fn about_page(&mut self, ui: &mut Ui) {
        ui.label(t!("about.content"));
        ui.label(t!(
            "about.metadata",
            version = consts::VERSION,
            git_latest_commit = consts::GIT_COMMIT_HASH,
            git_remote = consts::GIT_REMOTE
        ));
        // the open operation wont be executed if the clicked() condition is false
        if ui.link("https://lunarclient.top").clicked()
            && let Err(err) = open::that("https://lunarclient.top")
        {
            error!("Failed to open link: {err}\n{}", Backtrace::capture());
        }
    }
}
