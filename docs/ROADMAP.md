# MobileBox — Completion Roadmap

## Done (Windows core)

- Headless one-tap launcher: tap icon -> phone opens directly
- Unique MobileBox phone skin (custom bezel, 1200x2580, transparent screen hole)
- Fingerprint hardening: ro.product.* = MobileBox (not Pixel)
- Auto-installer (setup.ps1 / setup.sh) with AVD creation (Pixel 6 hardware profile)
- MCP server with 20+ tools (screenshot, UI tree, tap/swipe/type/keys, apps, location, logcat, shell, device info)
- VT detection, settings removed, version locked to Android 14

## Remaining work

| Item | Effort | Notes |
|------|--------|-------|
| First successful end-to-end boot test | 0.5 day | Run setup.ps1 (network), then tap icon, verify phone boots |
| MCP agent integration test (Claude Code/Cursor) | 1 day | Connect, screenshot, tap, type, verify loop |
| Boot animation / splash branding | 1 day | Custom bootanimation.zip |
| Multi-instance support | 2 days | Multiple AVDs, port allocation |
| Snapshot save/load (reset) | 1 day | emulator snapshot commands |
| Clipboard sync (host <-> guest) | 1 day | ADB clipboard bridge |
| Keyboard mapping profiles | 1 day | keymaps like Habumi |
| Windows installer (MSI) | 2 days | cargo-wix or WiX |
| macOS build + (dmg) | 3 days | emulator binary + signing |
| Linux build + (AppImage) | 3 days | KVM + packaging |
| Auto-update mechanism | 2 days | GitHub releases + in-app check |
| Telemetry opt-in | 1 day | crash reports only |

## Total estimate

- Windows feature-complete: ~1.5 weeks
- Cross-platform (Win+Mac+Linux): ~3-4 weeks
- Polish + release: ~1 week

## Known limitations

- Requires VT-x/AMD-V + WHPX (no no-VT fallback yet)
- Banking/DRM apps may refuse emulators (FLAG_SECURE / attestation)
- First run needs ~2 GB download
- Network-dependent installer
