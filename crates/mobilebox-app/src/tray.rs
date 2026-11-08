//! System tray icon.

use tray_icon::{TrayIcon, TrayIconBuilder};

pub struct Tray {
    _icon: TrayIcon,
}

impl Tray {
    pub fn new() -> Option<Self> {
        // 1x1 transparent-ish icon bytes (RGBA)
        let icon = TrayIconBuilder::new()
            .with_tooltip("MobileBox")
            .build()
            .ok()?;
        Some(Self { _icon: icon })
    }
}
