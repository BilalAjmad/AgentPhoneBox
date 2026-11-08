//! Shared types and helpers for MobileBox.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AndroidVersion {
    V13,
    V14,
    V15,
}

impl AndroidVersion {
    pub fn api_level(&self) -> u32 {
        match self {
            Self::V13 => 33,
            Self::V14 => 34,
            Self::V15 => 35,
        }
    }
    pub fn label(&self) -> &'static str {
        match self {
            Self::V13 => "Android 13 (API 33)",
            Self::V14 => "Android 14 (API 34)",
            Self::V15 => "Android 15 (API 35)",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobileBoxConfig {
    pub android_version: AndroidVersion,
    pub avd_name: String,
    pub install_dir: PathBuf,
    pub adb_port: u16,
    pub emulator_console_port: u16,
    pub phone_width: u32,
    pub phone_height: u32,
    pub always_on_top: bool,
}

impl Default for MobileBoxConfig {
    fn default() -> Self {
        Self {
            android_version: AndroidVersion::V14,
            avd_name: "mobilebox".to_string(),
            install_dir: default_install_dir(),
            adb_port: 5555,
            emulator_console_port: 5554,
            phone_width: 1080,
            phone_height: 2280,
            always_on_top: false,
        }
    }
}

pub fn default_install_dir() -> PathBuf {
    // Env override takes priority.
    if let Ok(d) = std::env::var("MOBILEBOX_INSTALL_DIR") {
        return PathBuf::from(d);
    }
    // Portable: walk up from the executable to find an SDK (emulator/emulator.exe).
    if let Ok(exe) = std::env::current_exe() {
        let mut cur = exe.parent().map(|p| p.to_path_buf());
        for _ in 0..4 {
            if let Some(dir) = &cur {
                if dir.join("emulator").join(if cfg!(target_os = "windows") { "emulator.exe" } else { "emulator" }).exists() {
                    return dir.clone();
                }
                cur = dir.parent().map(|p| p.to_path_buf());
            } else {
                break;
            }
        }
    }
    // Dev fallback: H:\MobileBox
    #[cfg(target_os = "windows")]
    {
        if std::path::Path::new("H:\\MobileBox\\emulator\\emulator.exe").exists() {
            return PathBuf::from("H:\\MobileBox");
        }
    }
    // Standard install location.
    #[cfg(target_os = "windows")]
    {
        std::env::var("LOCALAPPDATA")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("."))
            .join("MobileBox")
    }
    #[cfg(target_os = "macos")]
    {
        dirs_home().join("Library/Application Support/MobileBox")
    }
    #[cfg(target_os = "linux")]
    {
        dirs_home().join(".local/share/MobileBox")
    }
}

#[allow(dead_code)]
fn dirs_home() -> PathBuf {
    std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."))
}

pub fn adb_path(install_dir: &Path) -> PathBuf {
    install_dir.join("platform-tools").join(if cfg!(target_os = "windows") {
        "adb.exe"
    } else {
        "adb"
    })
}

pub fn emulator_path(install_dir: &Path) -> PathBuf {
    install_dir.join("emulator").join(if cfg!(target_os = "windows") {
        "emulator.exe"
    } else {
        "emulator"
    })
}

pub fn skin_dir(install_dir: &Path) -> PathBuf {
    install_dir.join("skins").join("mobilebox_phone")
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenPoint {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, thiserror::Error)]
pub enum MobileBoxError {
    #[error("adb not found at {0}")]
    AdbNotFound(PathBuf),
    #[error("emulator not found at {0}")]
    EmulatorNotFound(PathBuf),
    #[error("device not connected: {0}")]
    NotConnected(String),
    #[error("command failed: {0}")]
    CommandFailed(String),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
}
