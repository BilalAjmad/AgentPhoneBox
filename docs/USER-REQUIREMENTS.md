# MobileBox — User Requirements (remembered)

Source: user directives during development. These are hard constraints.

1. Strictly a REAL virtual mobile — not fake. Phone-shaped window, real Android 14, real settings, real Play Store, real apps.
2. Ultra hyper super FAST + LIGHTWEIGHT. No hang, no lag. Swiftshader GPU, animations disabled, snappy boot.
3. Use the laptop's available camera (Integrated Webcam via webcam0) for the camera app.
4. Navigation = 3-button layout (back/home/recents), not gesture nav (easier).
5. Typing with the DESKTOP real keyboard (hardware keyboard enabled; soft keyboard auto-hides).
6. Real settings, real use. Play Store (or a store) to download apps. Real internet (host network) used inside; works offline too. Real sound. Volume buttons must work correctly.
7. Side buttons (volume/power) must work like a real phone. Long-press Power = restart/power menu.
8. One-tap launch: tap icon → phone opens directly. No launcher/settings/start page.
9. Everything on H: drive (C: has no space). Project, SDK, system image, AVD, JDK, cargo cache all on H:.
10. Android 14 only (locked, not user-selectable).
11. MCP server for 100% AI agent control (screenshot, tap, type, swipe, keys, restart, etc.) — AI can use the phone exactly like a human.
12. Desktop shortcut "MobileBox" on desktop + "MobileBox Settings".
13. Fullscreen auto-toggles after boot (Alt+Enter also toggles manually).
14. Hotspot/tethering is an Android settings feature; emulator shares host internet via WiFi/NAT (no real radio for broadcasting).
15. Restart available via MCP restart_emulator tool, Settings menu, and long-press Power button on device frame.
16. NO location spoofing feature (removed per user request). Real GPS/location from the emulator's default.
17. NO personal data in the project (clean for GitHub distribution; users set up their own accounts).
18. Setup must be ultra hyper super easiest — fully automated.

Do NOT regress on: lightweight, fast, real feel, 3-button nav, desktop keyboard, camera, sound, volume buttons, one-tap launch, H: drive only, AI 100% control, fullscreen, no personal data.
