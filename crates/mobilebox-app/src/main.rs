#![windows_subsystem = "windows"]

//! MobileBox launcher: tap the icon -> the virtual Android phone opens directly.

use mobilebox_common::{MobileBoxConfig, emulator_path};
use std::process::Command;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

const DETACHED_PROCESS: u32 = 0x00000008;
const CREATE_NEW_PROCESS_GROUP: u32 = 0x00000200;

fn main() {
    let cfg = MobileBoxConfig::default();
    let emu = emulator_path(&cfg.install_dir);

    // Prevent duplicate instances: if an emulator is already running, just exit.
    if std::process::Command::new(cfg.install_dir.join("platform-tools").join(if cfg!(target_os = "windows") { "adb.exe" } else { "adb" }))
        .arg("devices").output().map(|o| String::from_utf8_lossy(&o.stdout).contains("emulator-")).unwrap_or(false) {
        return;
    }

    if !emu.exists() {
        // Not installed yet: launch the installer in a new window, then exit.
        let script = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..")
            .join("scripts")
            .join("setup.ps1");
        let script = if script.exists() {
            script
        } else {
            cfg.install_dir.join("scripts").join("setup.ps1")
        };

        #[cfg(target_os = "windows")]
        {
            let mut c = Command::new("powershell");
            c.args(["-ExecutionPolicy", "Bypass", "-File"]);
            c.arg(&script);
            c.creation_flags(DETACHED_PROCESS | CREATE_NEW_PROCESS_GROUP);
            let _ = c.spawn();
            show_message("MobileBox is installing the Android emulator. A window will guide you. Reopen MobileBox when done.");
        }
        #[cfg(not(target_os = "windows"))]
        {
            let _ = Command::new("bash").arg(&script).spawn();
            println!("MobileBox: installing emulator. Reopen when done.");
        }
        return;
    }

    // Ensure AVD exists (auto-create on first run).
    let avd_home = std::env::var("ANDROID_AVD_HOME")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| {
            let candidate = std::path::PathBuf::from("H:\\MobileBox\\.android\\avd");
            if candidate.exists() {
                candidate
            } else {
                // standard location
                #[cfg(target_os = "windows")]
                {
                    std::env::var("USERPROFILE")
                        .map(|u| std::path::PathBuf::from(u).join(".android").join("avd"))
                        .unwrap_or_else(|_| std::path::PathBuf::from(".android").join("avd"))
                }
                #[cfg(not(target_os = "windows"))]
                {
                    std::env::var("HOME")
                        .map(|h| std::path::PathBuf::from(h).join(".android").join("avd"))
                        .unwrap_or_else(|_| std::path::PathBuf::from(".android").join("avd"))
                }
            }
        });
    let avd_dir = avd_home.join(format!("{}.avd", cfg.avd_name));
    if !avd_dir.exists() {
        let avdmanager = cfg.install_dir.join("cmdline-tools").join("latest").join("bin").join("avdmanager.bat");
        if avdmanager.exists() {
            let mut c = Command::new(&avdmanager);
            c.args([
                "create",
                "avd",
                "-n",
                &cfg.avd_name,
                "-k",
                "system-images;android-34;google_apis_playstore;x86_64",
                "--abi",
                "google_apis/x86_64",
                "-d",
                "pixel_6",
                "--tag",
                "google_apis_playstore",
                "--force",
            ]);
            c.env("JAVA_HOME", "H:\\jdk");
            c.env("ANDROID_AVD_HOME", &avd_home);
            let _ = c.output();
        }
    }

    let mut cmd = Command::new(&emu);
    cmd.args([
        "-avd",
        &cfg.avd_name,
        "-no-boot-anim",
        "-gpu",
        "swiftshader_indirect",
    ]);
    cmd.env("ANDROID_AVD_HOME", &avd_home);
    cmd.env("ANDROID_SDK_ROOT", &cfg.install_dir);

    match cmd.spawn() {
        Ok(_) => {
            // Spawn hotkey listener (Ctrl+Alt+F -> fullscreen toggle).
            #[cfg(target_os = "windows")]
            {
                let mut c = Command::new("powershell");
                c.args(["-NoProfile", "-WindowStyle", "Hidden", "-ExecutionPolicy", "Bypass", "-File", "H:\\MobileBox\\MobileBox-Hotkey.ps1"]);
                let _ = c.spawn();
            }
            // Post-boot optimizations (background, delayed).
            #[cfg(target_os = "windows")]
            {
                let adb_path = cfg.install_dir.join("platform-tools").join("adb.exe");
                let script = format!(
                    "Start-Sleep -Seconds 70; $adb='{}'; & $adb shell settings put global window_animation_scale 0; & $adb shell settings put global transition_animation_scale 0; & $adb shell settings put global animator_duration_scale 0; & $adb shell settings put secure navigation_mode 0; & $adb shell settings put secure location_mode 3; & $adb shell settings put secure location_providers_allowed +gps,+network",
                    adb_path.to_string_lossy().replace("'", "''")
                );
                let mut c = Command::new("powershell");
                c.args(["-NoProfile", "-WindowStyle", "Hidden", "-Command", &script]);
                let _ = c.spawn();
            }
        }
        Err(e) => show_message(&format!("Failed to start emulator: {}", e)),
    }
}

#[cfg(target_os = "windows")]
fn show_message(msg: &str) {
    // Escape single quotes for PowerShell by doubling them.
    let escaped = msg.replace('\'', "''");
    let script = format!(
        "Add-Type -AssemblyName System.Windows.Forms; [System.Windows.Forms.MessageBox]::Show('{}','MobileBox') | Out-Null",
        escaped
    );
    let _ = Command::new("powershell")
        .args(["-NoProfile", "-Command", &script])
        .spawn();
}

#[cfg(not(target_os = "windows"))]
fn show_message(msg: &str) {
    println!("MobileBox: {}", msg);
}
