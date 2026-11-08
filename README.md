# AgentPhoneBox

> **Your AI Agent's Real Android Phone.**

A virtual Android 14 phone that runs on your Windows laptop. Your AI agent gets **100% full control** via 20 MCP tools — every tap, swipe, screenshot, app launch, and shell command. Not an emulator wrapper. A real Android phone.

[![Launch Video](https://img.shields.io/badge/▶_Watch_Launch_Video-00D4FF?style=for-the-badge)](https://github.com/BilalAjmad/AgentPhoneBox/raw/main/launch_video.mp4)

## 🎬 Launch Video

https://github.com/BilalAjmad/AgentPhoneBox/raw/main/launch_video.mp4

## What Is This?

AgentPhoneBox gives your AI agent (Claude, GPT, Cursor, etc.) a **real Android phone** to control. It's a full Android 14 (Pixel 6) virtual phone running on Windows via QEMU, accessible through 20 MCP (Model Context Protocol) tools.

Your AI agent can:
- 📸 Take screenshots and read the screen
- 👆 Tap, swipe, type, press keys
- 📱 Launch, install, and manage apps
- 🌐 Browse the web, download from Play Store
- 📷 Use your laptop webcam as the phone camera
- 📍 Spoof GPS location
- 🔧 Run shell commands
- 🔄 Restart the phone

## Architecture

```
AgentPhoneBox/
├── crates/
│   ├── mobilebox-common/    # Shared types, config, paths
│   ├── mobilebox-app/       # Launcher (Windows GUI, portable paths)
│   └── mobilebox-mcp/       # MCP server (20 tools, ADB-based)
├── scripts/
│   ├── setup.ps1            # One-click installer (downloads SDK, creates AVD)
│   └── setup.sh             # Linux/macOS installer
├── skins/
│   └── mobilebox_phone/     # Device skin (Pixel 6 frame)
├── docs/                    # User requirements, MCP reference, setup guide
└── MobileBox.nsi            # NSIS installer (creates desktop shortcut)
```

## MCP Tools (20)

| Tool | Description |
|------|-------------|
| `screenshot` | Capture device screen as PNG |
| `describe_screen` | UI hierarchy with tap coordinates |
| `tap` / `swipe` / `long_press` | Touch interactions |
| `type_text` / `press_key` | Keyboard input |
| `launch_app` / `list_apps` | App management |
| `install_apk` / `open_url` | Install apps, open URLs |
| `foreground_app` / `wait_for_element` | App state |
| `device_info` / `battery` / `logcat` | System info |
| `adb_shell` | Restricted shell access |
| `restart_emulator` | Restart the phone |
| `is_booted` / `rotate` | Boot status, rotation |
| `set_location_country` | GPS spoofing |

## Installation

### Option 1: NSIS Installer (Recommended)

1. Download `MobileBox-Setup.exe` from [Releases](https://github.com/BilalAjmad/AgentPhoneBox/releases)
2. Run the installer — it creates a **Desktop shortcut** automatically
3. Tap the **MobileBox** icon on your Desktop

### Option 2: PowerShell Setup

```powershell
# Clone the repo
git clone https://github.com/BilalAjmad/AgentPhoneBox.git
cd AgentPhoneBox

# Run the installer (right-click → Run with PowerShell)
.\scripts\setup.ps1
```

### Option 3: Build from Source

```bash
git clone https://github.com/BilalAjmad/AgentPhoneBox.git
cd AgentPhoneBox
cargo build --release --workspace
.\target\release\MobileBox.exe
```

### Requirements

- Windows 10/11 (64-bit)
- Hardware virtualization (VT-x/AMD-V) enabled in BIOS
- Windows Hypervisor Platform enabled
- ~2 GB free disk space
- 8 GB RAM recommended

## Connect Your AI Agent

Add to your MCP client config (Claude Desktop, Cursor, etc.):

```json
{
  "mcpServers": {
    "mobilebox": {
      "command": "H:\\AgentPhoneBox\\target\\release\\mobilebox-mcp.exe",
      "args": [],
      "env": { "MOBILEBOX_INSTALL_DIR": "H:\\AgentPhoneBox" }
    }
  }
}
```

Your AI agent now has a real Android phone.

## Quick Start

1. Install MobileBox (see above)
2. Tap the **MobileBox** desktop shortcut — phone boots in ~30 seconds
3. Connect your AI agent via MCP
4. Tell your agent: "Take a screenshot of the phone" or "Open Chrome and search for..."

## Features

- **One-tap launch** — Desktop shortcut opens the phone directly
- **Real Android 14** — Pixel 6 profile, 1080×2400, Play Store, Camera, Settings
- **100% AI control** — 20 MCP tools for full automation
- **Data isolation** — All user data lives in the phone's own storage
- **WiFi/internet** — Uses host laptop's connection
- **Laptop webcam** — Camera app uses your integrated webcam
- **Zero bloat** — No launcher, no settings page, just the phone

## Limitations

| Feature | Status |
|---------|--------|
| Play Store | ✅ Works |
| Camera | ✅ Works (laptop webcam) |
| WiFi/Internet | ✅ Works (host NAT) |
| Hotspot/Tethering | ⚠️ UI only |
| GPS Spoofing | ✅ Via MCP (`set_location_country`) |

## License

MIT License — see [LICENSE](LICENSE)

---

**AgentPhoneBox** — A real Android phone on your laptop, controlled by you or your AI.


