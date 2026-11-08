//! MobileBox MCP server: exposes the virtual phone to any AI agent.

use crate::adb::Adb;
use rmcp::{
    handler::server::wrapper::Parameters,
    model::{CallToolResult, ContentBlock},
    schemars, tool, tool_router, ServiceExt, transport::stdio,
};
use schemars::JsonSchema;
use serde::Deserialize;
use std::sync::Mutex;

#[derive(Clone)]
pub struct MobileBoxServer {
    adb: std::sync::Arc<Mutex<Adb>>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct TapParams {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SwipeParams {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
    #[serde(default = "default_dur")]
    pub duration_ms: u32,
}

fn default_dur() -> u32 {
    300
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct LongPressParams {
    pub x: i32,
    pub y: i32,
    #[serde(default = "default_lp")]
    pub duration_ms: u32,
}

fn default_lp() -> u32 {
    800
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct TypeParams {
    pub text: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct KeyParams {
    pub key: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct LaunchParams {
    pub package: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct InstallParams {
    pub path: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UrlParams {
    pub url: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct WaitParams {
    pub text: String,
    #[serde(default = "default_timeout")]
    pub timeout_ms: u32,
}

fn default_timeout() -> u32 {
    5000
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct LogcatParams {
    #[serde(default = "default_lines")]
    pub lines: u32,
}

fn default_lines() -> u32 {
    100
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct RotateParams {
    #[serde(default)]
    pub orientation: i32,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ShellParams {
    pub command: String,
}

#[tool_router(server_handler)]
impl MobileBoxServer {
    pub fn new(adb: Adb) -> Self {
        Self {
            adb: std::sync::Arc::new(Mutex::new(adb)),
        }
    }

    #[tool(description = "Capture the device screen as a PNG image the agent can see.")]
    fn screenshot(&self) -> Result<CallToolResult, rmcp::ErrorData> {
        let adb = self.adb.lock().unwrap();
        let bytes = adb.screenshot_png().map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;
        let b64 = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &bytes);
        Ok(CallToolResult::success(vec![ContentBlock::image(b64, "image/png")]))
    }

    #[tool(description = "Dump the UI hierarchy as a list of elements with tap coordinates.")]
    fn describe_screen(&self) -> Result<String, rmcp::ErrorData> {
        let adb = self.adb.lock().unwrap();
        adb.ui_hierarchy().map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))
    }

    #[tool(description = "Tap at pixel coordinates.")]
    fn tap(&self, Parameters(p): Parameters<TapParams>) -> Result<String, rmcp::ErrorData> {
        let adb = self.adb.lock().unwrap();
        adb.tap(p.x, p.y).map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;
        Ok(format!("tapped ({},{})", p.x, p.y))
    }

    #[tool(description = "Swipe between two points.")]
    fn swipe(&self, Parameters(p): Parameters<SwipeParams>) -> Result<String, rmcp::ErrorData> {
        let adb = self.adb.lock().unwrap();
        adb.swipe(p.x1, p.y1, p.x2, p.y2, p.duration_ms)
            .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;
        Ok("swiped".to_string())
    }

    #[tool(description = "Long press at coordinates.")]
    fn long_press(&self, Parameters(p): Parameters<LongPressParams>) -> Result<String, rmcp::ErrorData> {
        let adb = self.adb.lock().unwrap();
        adb.long_press(p.x, p.y, p.duration_ms)
            .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;
        Ok("long pressed".to_string())
    }

    #[tool(description = "Type text into the focused field.")]
    fn type_text(&self, Parameters(p): Parameters<TypeParams>) -> Result<String, rmcp::ErrorData> {
        let adb = self.adb.lock().unwrap();
        adb.type_text(&p.text).map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;
        Ok(format!("typed: {}", p.text))
    }

    #[tool(description = "Press a key: HOME, BACK, ENTER, RECENTS, VOLUME_UP, VOLUME_DOWN, POWER, or a KEYCODE_* name.")]
    fn press_key(&self, Parameters(p): Parameters<KeyParams>) -> Result<String, rmcp::ErrorData> {
        let code = key_code(&p.key);
        let adb = self.adb.lock().unwrap();
        adb.key_event(code).map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;
        Ok(format!("pressed {}", p.key))
    }

    #[tool(description = "Launch an app by package name.")]
    fn launch_app(&self, Parameters(p): Parameters<LaunchParams>) -> Result<String, rmcp::ErrorData> {
        let adb = self.adb.lock().unwrap();
        adb.launch_app(&p.package)
            .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;
        Ok(format!("launched {}", p.package))
    }

    #[tool(description = "List installed third-party packages.")]
    fn list_apps(&self) -> Result<String, rmcp::ErrorData> {
        let adb = self.adb.lock().unwrap();
        let pkgs = adb.list_packages().map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;
        Ok(pkgs.join("\n"))
    }

    #[tool(description = "Install an APK from a host path.")]
    fn install_apk(&self, Parameters(p): Parameters<InstallParams>) -> Result<String, rmcp::ErrorData> {
        let adb = self.adb.lock().unwrap();
        adb.install_apk(std::path::Path::new(&p.path))
            .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))
    }

    #[tool(description = "Open a URL in the default browser.")]
    fn open_url(&self, Parameters(p): Parameters<UrlParams>) -> Result<String, rmcp::ErrorData> {
        let adb = self.adb.lock().unwrap();
        adb.open_url(&p.url).map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;
        Ok(format!("opened {}", p.url))
    }

    #[tool(description = "Get the foreground (resumed) app/activity.")]
    fn foreground_app(&self) -> Result<String, rmcp::ErrorData> {
        let adb = self.adb.lock().unwrap();
        adb.foreground_app().map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))
    }

    #[tool(description = "Wait until an element with given text appears.")]
    fn wait_for_element(&self, Parameters(p): Parameters<WaitParams>) -> Result<String, rmcp::ErrorData> {
        let deadline = std::time::Instant::now() + std::time::Duration::from_millis(p.timeout_ms as u64);
        loop {
            let adb = self.adb.lock().unwrap();
            let tree = adb.ui_hierarchy().unwrap_or_default();
            drop(adb);
            if tree.contains(&p.text) {
                return Ok(format!("found '{}'", p.text));
            }
            if std::time::Instant::now() >= deadline {
                return Err(rmcp::ErrorData::internal_error(
                    format!("timeout waiting for '{}'", p.text),
                    None,
                ));
            }
            std::thread::sleep(std::time::Duration::from_millis(300));
        }
    }

    #[tool(description = "Get battery info.")]
    fn battery(&self) -> Result<String, rmcp::ErrorData> {
        let adb = self.adb.lock().unwrap();
        adb.battery().map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))
    }

    #[tool(description = "Get recent logcat lines.")]
    fn logcat(&self, Parameters(p): Parameters<LogcatParams>) -> Result<String, rmcp::ErrorData> {
        let adb = self.adb.lock().unwrap();
        adb.logcat(p.lines).map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))
    }

    #[tool(description = "Rotate the screen: 0/1/2/3 (portrait, landscape, etc).")]
    fn rotate(&self, Parameters(p): Parameters<RotateParams>) -> Result<String, rmcp::ErrorData> {
        let adb = self.adb.lock().unwrap();
        adb.shell(&format!("settings put system user_rotation {}", p.orientation))
            .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;
        Ok(format!("rotation set to {}", p.orientation))
    }

    #[tool(description = "Check whether the device has finished booting.")]
    fn is_booted(&self) -> Result<String, rmcp::ErrorData> {
        let adb = self.adb.lock().unwrap();
        let out = adb.shell("getprop sys.boot_completed").unwrap_or_default();
        Ok(out.trim().to_string())
    }

    #[tool(description = "Run an allow-listed adb shell command.")]
    fn adb_shell(&self, Parameters(p): Parameters<ShellParams>) -> Result<String, rmcp::ErrorData> {
        // safety: block obviously destructive commands
        let lower = p.command.to_lowercase();
        for bad in ["rm -rf", "reboot", "factory_reset", "wipe", "format", "dd if=", "mkfs", ":(){:|:&};:"] {
            if lower.contains(bad) {
                return Err(rmcp::ErrorData::invalid_params(
                    format!("blocked potentially destructive command: {}", bad),
                    None,
                ));
            }
        }
        let adb = self.adb.lock().unwrap();
        adb.shell(&p.command).map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))
    }

    #[tool(description = "Restart the emulator (kill then relaunch).")]
    fn restart_emulator(&self) -> Result<String, rmcp::ErrorData> {
        let adb = self.adb.lock().unwrap();
        let install_dir = adb.install_dir().to_path_buf();
        let avd_home = std::env::var("ANDROID_AVD_HOME")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|_| install_dir.join(".android").join("avd"));
        let avd_name = "mobilebox".to_string();
        let _ = adb.run(&["emu", "kill"]);
        drop(adb);
        // Relaunch in background
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_secs(3));
            let emu = if cfg!(target_os = "windows") {
                install_dir.join("emulator").join("emulator.exe")
            } else {
                install_dir.join("emulator").join("emulator")
            };
            let _ = std::process::Command::new(&emu)
                .args(["-avd", &avd_name, "-no-boot-anim", "-gpu", "swiftshader_indirect"])
                .env("ANDROID_AVD_HOME", &avd_home)
                .env("ANDROID_SDK_ROOT", &install_dir)
                .spawn();
        });
        Ok("restarting emulator".to_string())
    }
    #[tool(description = "Get device info (model, android version, resolution).")]
    fn device_info(&self) -> Result<String, rmcp::ErrorData> {
        let adb = self.adb.lock().unwrap();
        let model = adb.shell("getprop ro.product.model").unwrap_or_default();
        let ver = adb.shell("getprop ro.build.version.release").unwrap_or_default();
        let sdk = adb.shell("getprop ro.build.version.sdk").unwrap_or_default();
        let res = adb.shell("wm size").unwrap_or_default();
        Ok(format!(
            "model: {}\nandroid: {}\nsdk: {}\nresolution: {}",
            model.trim(),
            ver.trim(),
            sdk.trim(),
            res.trim()
        ))
    }
}

fn key_code(name: &str) -> i32 {
    let name = name.strip_prefix("KEYCODE_").unwrap_or(name);
    match name.to_uppercase().as_str() {
        "HOME" => 3,
        "BACK" => 4,
        "ENTER" | "RETURN" => 66,
        "RECENTS" | "APP_SWITCH" => 187,
        "VOLUME_UP" => 24,
        "VOLUME_DOWN" => 25,
        "POWER" => 26,
        "MENU" => 82,
        "TAB" => 61,
        "SPACE" => 62,
        "DEL" | "DELETE" | "BACKSPACE" => 67,
        "SEARCH" => 84,
        other => other.parse().unwrap_or(0),
    }
}


pub async fn run() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .init();

    let cfg = crate::adb::default_config();
    let mut adb = crate::adb::Adb::new(&cfg);
    if let Ok(devs) = adb.devices() {
        if let Some(d) = devs.first() {
            adb.set_serial(d.clone());
        }
    }

    let server = MobileBoxServer::new(adb);
    let service = server.serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}
