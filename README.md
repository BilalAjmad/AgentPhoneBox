# AgentPhoneBox

> **Your AI Agent's Real Android Phone.**

A virtual Android 14 phone that runs on your Windows laptop. Your AI agent gets **100% full control** via 20 MCP tools â€” every tap, swipe, screenshot, app launch, and shell command. Not an emulator wrapper. A real Android phone.

[![Launch Video](https://img.shields.io/badge/â–¶_Watch_Launch_Video-00D4FF?style=for-the-badge)](https://github.com/BilalAjmad/AgentPhoneBox/raw/main/launch_video.mp4)

## ðŸŽ¬ Launch Video

![AgentPhoneBox Launch Video](https://github.com/BilalAjmad/AgentPhoneBox/raw/main/launch_video.gif)

## What Is This?

AgentPhoneBox gives your AI agent (Claude, GPT, Cursor, etc.) a **real Android phone** to control. It's a full Android 14 (Pixel 6) virtual phone running on Windows via QEMU, accessible through 20 MCP (Model Context Protocol) tools.

Your AI agent can:
- ðŸ“¸ Take screenshots and read the screen
- ðŸ‘† Tap, swipe, type, press keys
- ðŸ“± Launch, install, and manage apps
- ðŸŒ Browse the web, download from Play Store
- ðŸ“· Use your laptop webcam as the phone camera
- ðŸ”§ Run shell commands
- ðŸ”„ Restart the phone

## Architecture

```
AgentPhoneBox/
â”œâ”€â”€ crates/
â”‚   â”œâ”€â”€ mobilebox-common/    # Shared types, config, paths
â”‚   â”œâ”€â”€ mobilebox-app/       # Launcher (Windows GUI, portable paths)
â”‚   â””â”€â”€ mobilebox-mcp/       # MCP server (20 tools, ADB-based)
â”œâ”€â”€ scripts/
â”‚   â”œâ”€â”€ setup.ps1            # One-click installer (downloads SDK, creates AVD)
â”‚   â””â”€â”€ setup.sh             # Linux/macOS installer
â”œâ”€â”€ skins/
â”‚   â””â”€â”€ mobilebox_phone/     # Device skin (Pixel 6 frame)
â”œâ”€â”€ docs/                    # User requirements, MCP reference, setup guide
â””â”€â”€ MobileBox.nsi            # NSIS installer (creates desktop shortcut)
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

## Installation

### Option 1: NSIS Installer (Recommended)

1. Download `MobileBox-Setup.exe` from [Releases](https://github.com/BilalAjmad/AgentPhoneBox/releases)
2. Run the installer â€” it creates a **Desktop shortcut** automatically
3. Tap the **MobileBox** icon on your Desktop

### Option 2: PowerShell Setup

```powershell
# Clone the repo
git clone https://github.com/BilalAjmad/AgentPhoneBox.git
cd AgentPhoneBox

# Run the installer (right-click â†’ Run with PowerShell)
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
2. Tap the **MobileBox** desktop shortcut â€” phone boots in ~30 seconds
3. Connect your AI agent via MCP
4. Tell your agent: "Take a screenshot of the phone" or "Open Chrome and search for..."

## Features

- **One-tap launch** â€” Desktop shortcut opens the phone directly
- **Real Android 14** â€” Pixel 6 profile, 1080Ã—2400, Play Store, Camera, Settings
- **100% AI control** â€” 20 MCP tools for full automation
- **Data isolation** â€” All user data lives in the phone's own storage
- **WiFi/internet** â€” Uses host laptop's connection
- **Laptop webcam** â€” Camera app uses your integrated webcam
- **Zero bloat** â€” No launcher, no settings page, just the phone

## Limitations

| Feature | Status |
|---------|--------|
| Play Store | âœ… Works |
| Camera | âœ… Works (laptop webcam) |
| WiFi/Internet | âœ… Works (host NAT) |
| Hotspot/Tethering | âš ï¸ UI only |

## License

MIT License â€” see [LICENSE](LICENSE)

---

**AgentPhoneBox** â€” A real Android phone on your laptop, controlled by you or your AI.





