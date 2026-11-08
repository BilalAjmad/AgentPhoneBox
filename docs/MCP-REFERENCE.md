# MCP Reference

Complete reference for MobileBox MCP server tools.

## Connection

```json
{
  "mcpServers": {
    "mobilebox": {
      "command": "H:\\MobileBox\\target\\release\\mobilebox-mcp.exe",
      "args": [],
      "env": {
        "MOBILEBOX_INSTALL_DIR": "H:\\MobileBox"
      }
    }
  }
}
```

## Tools

### Screen & UI

#### `screenshot`
Capture current screen as PNG (base64).
```
params: {}
returns: { type: "image", data: "base64...", mimeType: "image/png" }
```

#### `describe_screen`
Dump UI hierarchy as text with element bounds, text, clickability.
```
params: {}
returns: "element1: text='Button', bounds=[100,200,300,250], clickable=true\n..."
```

#### `tap`
Tap at pixel coordinates.
```
params: { "x": 540, "y": 1200 }
returns: "tapped (540,1200)"
```

#### `swipe`
Swipe from (x1,y1) to (x2,y2).
```
params: { "x1": 540, "y1": 1800, "x2": 540, "y2": 600, "duration_ms": 300 }
```

#### `long_press`
Long press at coordinates.
```
params: { "x": 540, "y": 1200, "duration_ms": 800 }
```

#### `drag`
Drag from (x1,y1) to (x2,y2).
```
params: { "x1": 540, "y1": 1800, "x2": 540, "y2": 600, "duration_ms": 500 }
```

#### `pinch`
Pinch/zoom gesture.
```
params: { "x": 540, "y": 960, "scale": 2.0, "duration_ms": 500 }
```

#### `multi_touch`
Multi-touch gesture (2 points).
```
params: { "points": [[x1,y1], [x2,y2]], "duration_ms": 300 }
```

### Input

#### `type_text`
Type text into focused field (ASCII only).
```
params: { "text": "Hello World" }
```

#### `press_key`
Press a key: `HOME`, `BACK`, `ENTER`, `RECENTS`, `VOLUME_UP`, `VOLUME_DOWN`, `POWER`, `MENU`, `TAB`, `SPACE`, `DELETE`, `SEARCH`, or numeric keycode.
```
params: { "key": "HOME" }
```

#### `clear_field`
Clear focused text field.
```
params: {}
```

### Apps

#### `launch_app`
Launch app by package name.
```
params: { "package": "com.android.chrome" }
```

#### `list_apps`
List installed third-party packages.
```
params: {}
returns: "com.android.chrome\ncom.google.android.gm\n..."
```

#### `install_apk`
Install APK from host path.
```
params: { "path": "H:\\Downloads\\app.apk" }
```

#### `open_url`
Open URL in default browser.
```
params: { "url": "https://github.com" }
```

#### `foreground_app`
Get foreground (resumed) app/activity.
```
params: {}
returns: "com.android.chrome/com.google.android.apps.chrome.Main"
```

#### `wait_for_element`
Wait until element with text appears.
```
params: { "text": "Sign in", "timeout_ms": 5000 }
```

#### `scroll_to`
Scroll until element with text appears.
```
params: { "text": "Settings", "direction": "down", "max_swipes": 10 }
```

### Device

#### `device_info`
Get device info (model, Android version, SDK, resolution).
```
params: {}
returns: "model: sdk_gphone64_x86_64\nandroid: 14\nsdk: 34\nresolution: Physical size: 1080x2400"
```

#### `battery`
Get battery info.
```
params: {}
```

#### `logcat`
Get recent logcat lines.
```
params: { "lines": 100 }
```

#### `adb_shell`
Run allow-listed adb shell command.
```
params: { "command": "getprop ro.build.version.sdk" }
```

#### `restart_emulator`
Restart the emulator (kill and relaunch).
```
params: {}
```

#### `is_booted`
Check if device finished booting.
```
params: {}
returns: "1" (booted) or "0" (booting)
```

#### `set_location`
Set GPS location (emulator only). Persists to location.txt.
```
params: { "lat": 35.6762, "lon": 139.6503 }
```

#### `set_location_country`
Set device location by country name (capital coordinates). Persists.
```
params: { "country": "Japan" }
```

#### `rotate`
Rotate screen: 0=portrait, 1=landscape, 2=reverse portrait, 3=reverse landscape.
```
params: { "orientation": 1 }
```

#### `set_location`
Set GPS location (emulator only). Persists.
```
params: { "lat": 35.6762, "lon": 139.6503 }
```

#### `set_location_country`
Set device location by country name (capital coordinates). Persists.
```
params: { "country": "Japan" }
```

#### `rotate`
Rotate screen: 0=portrait, 1=landscape, 2=reverse portrait, 3=reverse landscape.
```
params: { "orientation": 1 }
```

#### `foreground_app`
Get foreground (resumed) app/activity.
```
params: {}
returns: "com.android.chrome/com.google.android.apps.chrome.Main"
```

#### `wait_for_element`
Wait until element with text appears.
```
params: { "text": "Sign in", "timeout_ms": 5000 }
```

#### `scroll_to`
Scroll until element with text appears.
```
params: { "text": "Settings", "direction": "down", "max_swipes": 10 }
```

#### `device_info`
Get device info (model, Android version, SDK, resolution).
```
params: {}
returns: "model: sdk_gphone64_x86_64\nandroid: 14\nsdk: 34\nresolution: Physical size: 1080x2400"
```

#### `battery`
Get battery info.
```
params: {}
```

#### `logcat`
Get recent logcat lines.
```
params: { "lines": 100 }
```

#### `adb_shell`
Run allow-listed adb shell command.
```
params: { "command": "getprop ro.build.version.sdk" }
```

#### `restart_emulator`
Restart the emulator (kill and relaunch).
```
params: {}
```

#### `is_booted`
Check if device finished booting.
```
params: {}
returns: "1" (booted) or "0" (booting)
```

## Error Handling

All tools return:
- Success: `{ "content": [{ "type": "text", "text": "..." }], "isError": false }`
- Error: `{ "content": [{ "type": "text", "text": "error message" }], "isError": true, "code": -32603 }`

## Security

- `adb_shell` blocks destructive commands (`rm -rf`, `reboot`, `factory_reset`, `wipe`)
- `install_apk` only accepts local file paths
- `adb_shell` command is executed via `adb shell` (not direct shell)

## Version

Current: 0.1.0 (2026-11-02)