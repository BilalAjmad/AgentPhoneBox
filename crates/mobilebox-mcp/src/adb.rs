//! ADB client wrapper. All device interaction goes through `adb`.

use mobilebox_common::{MobileBoxError, MobileBoxConfig};
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct Adb {
    bin: PathBuf,
    install_dir: PathBuf,
    serial: Option<String>,
}

impl Adb {
    pub fn new(config: &MobileBoxConfig) -> Self {
        let bin = mobilebox_common::adb_path(&config.install_dir);
        Self {
            bin,
            install_dir: config.install_dir.clone(),
            serial: None,
        }
    }

    #[allow(dead_code)]
    pub fn with_bin(bin: PathBuf, install_dir: PathBuf) -> Self {
        Self { bin, install_dir, serial: None }
    }

    pub fn install_dir(&self) -> &Path {
        &self.install_dir
    }

    #[allow(dead_code)]
    pub fn serial(&self) -> Option<&str> {
        self.serial.as_deref()
    }

    pub fn set_serial(&mut self, s: String) {
        self.serial = Some(s);
    }

    fn base(&self) -> Command {
        let mut c = Command::new(&self.bin);
        if let Some(s) = &self.serial {
            c.args(["-s", s]);
        }
        c
    }

    pub fn run(&self, args: &[&str]) -> Result<String, MobileBoxError> {
        if !self.bin.exists() {
            return Err(MobileBoxError::AdbNotFound(self.bin.clone()));
        }
        let out = self.base().args(args).output()?;
        if !out.status.success() {
            let stderr = String::from_utf8_lossy(&out.stderr);
            return Err(MobileBoxError::CommandFailed(format!(
                "adb {:?} failed: {}",
                args, stderr
            )));
        }
        Ok(String::from_utf8_lossy(&out.stdout).to_string())
    }

    pub fn shell(&self, cmd: &str) -> Result<String, MobileBoxError> {
        self.run(&["shell", cmd])
    }

    pub fn devices(&self) -> Result<Vec<String>, MobileBoxError> {
        let out = self.run(&["devices"])?;
        let mut v = Vec::new();
        for line in out.lines().skip(1) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 && parts[1] == "device" {
                v.push(parts[0].to_string());
            }
        }
        Ok(v)
    }

    pub fn screenshot_png(&self) -> Result<Vec<u8>, MobileBoxError> {
        let out = self.base().args(["exec-out", "screencap", "-p"]).output()?;
        if !out.status.success() {
            return Err(MobileBoxError::CommandFailed("screencap failed".into()));
        }
        Ok(out.stdout)
    }

    pub fn tap(&self, x: i32, y: i32) -> Result<(), MobileBoxError> {
        self.shell(&format!("input tap {} {}", x, y))?;
        Ok(())
    }

    pub fn swipe(&self, x1: i32, y1: i32, x2: i32, y2: i32, dur_ms: u32) -> Result<(), MobileBoxError> {
        self.shell(&format!("input swipe {} {} {} {} {}", x1, y1, x2, y2, dur_ms))?;
        Ok(())
    }

    pub fn long_press(&self, x: i32, y: i32, dur_ms: u32) -> Result<(), MobileBoxError> {
        self.shell(&format!("input swipe {} {} {} {} {}", x, y, x, y, dur_ms))?;
        Ok(())
    }

    pub fn type_text(&self, text: &str) -> Result<(), MobileBoxError> {
        if !text.is_ascii() {
            return Err(MobileBoxError::CommandFailed(
                "adb input text supports ASCII only".into(),
            ));
        }
        let encoded = text.replace(' ', "%s");
        self.shell(&format!("input text {}", encoded))?;
        Ok(())
    }

    pub fn key_event(&self, code: i32) -> Result<(), MobileBoxError> {
        self.shell(&format!("input keyevent {}", code))?;
        Ok(())
    }

    pub fn ui_hierarchy(&self) -> Result<String, MobileBoxError> {
        if let Ok(out) = self.run(&["exec-out", "uiautomator", "dump", "/dev/tty"]) {
            if out.contains("<hierarchy") || out.contains("<?xml") {
                return Ok(out);
            }
        }
        let _ = self.shell("uiautomator dump /sdcard/window_dump.xml");
        self.run(&["exec-out", "cat", "/sdcard/window_dump.xml"])
    }

    pub fn foreground_app(&self) -> Result<String, MobileBoxError> {
        let out = self.shell("dumpsys activity activities")?;
        for line in out.lines() {
            if line.contains("ResumedActivity") || line.contains("mResumedActivity") {
                return Ok(line.trim().to_string());
            }
        }
        // fallback
        let out = self.shell("dumpsys window | grep mCurrentFocus")?;
        Ok(out.trim().to_string())
    }

    pub fn install_apk(&self, path: &Path) -> Result<String, MobileBoxError> {
        self.run(&["install", "-r", path.to_str().unwrap_or("")])
    }

    pub fn launch_app(&self, pkg: &str) -> Result<(), MobileBoxError> {
        self.shell(&format!(
            "monkey -p {} -c android.intent.category.LAUNCHER 1",
            pkg
        ))?;
        Ok(())
    }

    pub fn list_packages(&self) -> Result<Vec<String>, MobileBoxError> {
        let out = self.shell("pm list packages -3")?;
        Ok(out
            .lines()
            .filter_map(|l| l.strip_prefix("package:"))
            .map(|s| s.trim().to_string())
            .collect())
    }

    pub fn open_url(&self, url: &str) -> Result<(), MobileBoxError> {
        self.shell(&format!(
            "am start -a android.intent.action.VIEW -d '{}'",
            url
        ))?;
        Ok(())
    }

    pub fn battery(&self) -> Result<String, MobileBoxError> {
        self.shell("dumpsys battery")
    }

    pub fn logcat(&self, lines: u32) -> Result<String, MobileBoxError> {
        self.shell(&format!("logcat -d -t {}", lines))
    }
}

pub fn default_config() -> MobileBoxConfig {
    let mut cfg = MobileBoxConfig::default();
    if let Ok(dir) = std::env::var("MOBILEBOX_INSTALL_DIR") {
        cfg.install_dir = std::path::PathBuf::from(dir);
    }
    if let Ok(serial) = std::env::var("MOBILEBOX_DEVICE") {
        // stored for later; Adb picks first device anyway
        let _ = serial;
    }
    cfg
}
