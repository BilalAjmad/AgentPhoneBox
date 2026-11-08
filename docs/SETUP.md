# Setup Guide

## Quick Start

1. **Clone & Build**
   ```bash
   git clone https://github.com/yourusername/MobileBox.git
   cd MobileBox
   .\scripts\setup.ps1
   ```

2. **Run**
   - Double-click `MobileBox` on Desktop
   - Or run `.\target\release\MobileBox.exe`

## Prerequisites

- **Windows 10/11 (64-bit)**
- **Hardware Virtualization** enabled in BIOS (VT-x/AMD-V)
- **Windows Hypervisor Platform** enabled:
  ```powershell
  Enable-WindowsOptionalFeature -Online -FeatureName HypervisorPlatform -All
  ```
- **Disk Space**: ~2 GB free (on H: if C: is full)
- **RAM**: 8 GB+ recommended

## Automatic Setup (setup.ps1)

Run once:
```powershell
.\scripts\setup.ps1
```

What it does:
1. Downloads Android command-line tools (~100 MB)
2. Installs platform-tools (adb), emulator, Android 14 system image (google_apis_playstore)
3. Creates AVD `mobilebox` with Pixel 6 profile (1080x2400)
5. Creates Desktop + Start Menu shortcuts
6. Applies AVD optimizations (camera, keyboard, sensors, WiFi)

**Time**: 5-15 minutes depending on internet speed.

## Manual Build

```bash
git clone https://github.com/yourusername/MobileBox.git
cd MobileBox
cargo build --release --workspace
```

Binaries: `target/release/MobileBox.exe`, `target/release/mobilebox-mcp.exe`

## First Launch

1. Double-click **MobileBox** on Desktop (or run `target/release/MobileBox.exe`)
2. Phone boots in ~20 seconds (quick boot enabled)
3. Tap the screen to unlock

## Configuration

### Environment Variables
| Variable | Default | Description |
|----------|---------|-------------|
| `MOBILEBOX_INSTALL_DIR` | Auto-detected | Override install directory |
| `ANDROID_AVD_HOME` | `~/.android/avd` | AVD location |
| `ANDROID_SDK_ROOT` | Auto-detected | SDK root |

### AVD Location
- Default: `~/.android/avd` (Windows: `%USERPROFILE%\.android\avd`)
- Override: `ANDROID_AVD_HOME=H:\.android\avd`
- Dev fallback: `H:\.android\avd` (if exists)

### SDK Location
Auto-detected (walk up from exe to find `emulator/emulator.exe`), then:
1. `MOBILEBOX_INSTALL_DIR` env var
2. Walk up from exe to find `emulator/emulator.exe`
3. `H:\MobileBox` (dev fallback)
4. `%LOCALAPPDATA%\MobileBox` (standard)

## Customization

### Fullscreen
- Auto: disabled (windowed by default)
- Toggle: `Ctrl+Alt+F` (hotkey) or `Alt+Enter` / `F11` (manual)

### Animation Scales
Auto-disabled after boot (window_animation_scale, transition_animation_scale, animator_duration_scale = 0)

### Navigation
- Default: 3-button navigation (back/home/recents)
- Change in Settings → System → Gestures → System navigation

### Keyboard
- Desktop keyboard works natively (hw.keyboard=yes)
- Soft keyboard auto-hides when hardware keyboard present

### Camera
- Default: laptop Integrated Webcam (webcam0)
- Change in AVD config: `hw.camera.back = webcam0|emulated|none`

### Location
- Default: real GPS from emulator
- Spoof via MCP: `set_location_country("Pakistan")` or `set_location(lat, lon)`
- Persists to `location.txt` (applied on boot)

## Keyboard Shortcuts

| Keys | Action |
|------|--------|
| `Ctrl+Alt+F` | Toggle fullscreen |
| `Alt+Enter` | Toggle fullscreen |
| `F11` | Toggle fullscreen (alt) |
| Long-press Power | Power menu (restart/off) |
| `Esc` | Back |
| `Win` / `Home` | Home |
| `Ctrl+Alt+Del` | Host OS (breaks out) |

## Troubleshooting

### Emulator won't start
- Check virtualization: Task Manager → Performance → CPU → Virtualization: Enabled
- Enable Hyper-V: `Enable-WindowsOptionalFeature -Online -FeatureName HypervisorPlatform`
- Check antivirus exclusions: `H:\MobileBox\emulator\emulator.exe`

### "Not Responding" at boot
Normal — Windows marks the window "Not Responding" while the UI thread boots. Wait 30-60s.

### No internet in emulator
- Host must have internet
- Emulator uses host NAT (WiFi/Ethernet)
- Try: `adb shell ping 8.8.8.8` (ICMP may be blocked, try HTTP)

### "Not enough space"
- Free up C: or set `MOBILEBOX_INSTALL_DIR=H:\MobileBox`
- Clear SDK cache: `%LOCALAPPDATA%\Android\sdk\` (if using default)

### Hotkey not working (Ctrl+Alt+F)
- Ensure hotkey listener running: check Task Manager for PowerShell
- Run `MobileBox Settings.lnk` → "Toggle fullscreen"
- Manual: `Alt+Enter` or `F11`

### Play Store not working
- Use google_apis_playstore image (default)
- Sign in with Google account
- Update Play Store from within app

### Camera not working
- Check `hw.camera.back = webcam0` in config.ini
- Grant camera permission to app
- Close other apps using webcam

### "Not enough space" on boot
- Increase AVD data partition: `disk.dataPartition.size=4096M` in config.ini
- Free space on H:

### ADB not found
- Run `H:\MobileBox\scripts\setup.ps1` (installs platform-tools)
- Or add `H:\MobileBox\platform-tools` to PATH

### "ADB server version mismatch"
```bash
adb kill-server
adb start-server
```

## Logs

| Component | Location |
|-----------|----------|
| Launcher | stderr (run from terminal) |
| Emulator | `%TEMP%\AndroidEmulator\*.log` |
| ADB | `adb logcat` |
| MCP Server | stderr |

## Uninstall

1. Delete Desktop shortcut
2. Delete Start Menu entry
3. Delete `H:\MobileBox` (or `%LOCALAPPDATA%\MobileBox`)
4. Delete `H:\.android\avd\mobilebox.avd` (or `%USERPROFILE%\.android\avd\mobilebox.avd`)
5. Remove `H:\jdk` if not needed
5. Remove `H:\.cargo\registry` if not needed (cargo cache)

## Updates

```bash
cd MobileBox
git pull
cargo build --release --workspace
```

## License

MIT — see [LICENSE](LICENSE)