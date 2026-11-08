//! Virtualization detection.

use std::sync::OnceLock;

static CACHED: OnceLock<bool> = OnceLock::new();

fn compute() -> bool {
    #[cfg(target_os = "windows")]
    {
        let out = std::process::Command::new("powershell")
            .args([
                "-NoProfile",
                "-Command",
                "(Get-WindowsOptionalFeature -Online -FeatureName HypervisorPlatform).State",
            ])
            .output();
        match out {
            Ok(o) => String::from_utf8_lossy(&o.stdout).trim().eq_ignore_ascii_case("enabled"),
            Err(_) => false,
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        std::path::Path::new("/dev/kvm").exists()
    }
}

pub fn whpx_available() -> bool {
    *CACHED.get_or_init(compute)
}

pub fn message() -> &'static str {
    if whpx_available() {
        "Hardware virtualization (WHPX) available."
    } else {
        "WHPX not detected. Enable 'Windows Hypervisor Platform' in Windows Features."
    }
}
