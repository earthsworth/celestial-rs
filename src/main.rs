#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::{backtrace::Backtrace, sync::Arc};

use egui::TextBuffer;
use egui_inbox::UiInbox;
use gui::CelestialApp;
use log::error;
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub(crate) mod gui;

rust_i18n::i18n!("locales");


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use rust_i18n::t;
    env_logger::init();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0]),
        ..Default::default()
    };

    let inbox = Arc::new(UiInbox::new());

    if let Err(_err) = eframe::run_native(
        t!("app.title").as_str(),
        native_options,
        Box::new(|cc| Ok(Box::new(CelestialApp::new(cc, Arc::clone(&inbox))))),
    ) {
        error!("Ui ran into some errors: {}", Backtrace::capture());
    }

    Ok(())
}
