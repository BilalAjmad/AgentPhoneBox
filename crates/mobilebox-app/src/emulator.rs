//! Emulator process management.

use mobilebox_common::{MobileBoxConfig, MobileBoxError, emulator_path};
use std::process::{Child, Command};
use std::sync::Mutex;

static CHILD: Mutex<Option<Child>> = Mutex::new(None);

pub fn start(cfg: &MobileBoxConfig) -> Result<(), MobileBoxError> {
    let emu = emulator_path(&cfg.install_dir);
    if !emu.exists() {
        return Err(MobileBoxError::EmulatorNotFound(emu));
    }
    let skin_parent = cfg.install_dir.join("skins");
    let mut cmd = Command::new(&emu);
    cmd.args([
        "-avd",
        &cfg.avd_name,
        "-skin",
        "mobilebox_phone",
        "-skin-dir",
        skin_parent.to_str().unwrap_or("."),
        "-no-snapshot-load",
        "-no-boot-anim",
    ]);
    let child = cmd.spawn().map_err(MobileBoxError::Io)?;
    *CHILD.lock().unwrap() = Some(child);
    Ok(())
}

pub fn stop(_cfg: &MobileBoxConfig) -> Result<(), MobileBoxError> {
    if let Some(mut c) = CHILD.lock().unwrap().take() {
        let _ = c.kill();
    }
    Ok(())
}
