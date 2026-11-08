//! Settings UI.

use eframe::egui;
use mobilebox_common::{MobileBoxConfig, AndroidVersion};
use std::sync::{Arc, Mutex};

pub fn ui(ui: &mut egui::Ui, config: &Arc<Mutex<MobileBoxConfig>>) {
    let mut cfg = config.lock().unwrap();
    ui.label("Android version: Android 14 (API 34) — fixed");
    cfg.android_version = AndroidVersion::V14;
    ui.label(format!("Install dir: {}", cfg.install_dir.display()));
    ui.label(format!("Phone: {}x{}", cfg.phone_width, cfg.phone_height));
    ui.checkbox(&mut cfg.always_on_top, "Always on top");
}
