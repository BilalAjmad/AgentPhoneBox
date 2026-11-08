# MobileBox Phone Skin

This directory is an Android emulator device skin. It gives the emulator a
phone-shaped window with a bezel.

- `layout` — skin layout config (display rect + parts).
- `frame.png` — the bezel image (1200x2520) with a transparent hole where
  the 1080x2280 display shows through. **Not committed** (binary). Generate
  or drop your own phone frame PNG here named `frame.png`.

To use a different frame, replace `frame.png` and adjust the display `x`/`y`
in `layout` accordingly.

The emulator is launched with:

```
emulator -avd mobilebox -skin mobilebox_phone -skin-dir <install_dir>/skins
```
