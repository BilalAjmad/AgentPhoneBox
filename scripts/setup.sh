#!/usr/bin/env bash
set -euo pipefail
INSTALL="${MOBILEBOX_INSTALL_DIR:-$HOME/.local/share/MobileBox}"
mkdir -p "$INSTALL"
cd "$INSTALL"
URL="https://dl.google.com/android/repository/commandlinetools-linux-11076708_latest.zip"
curl -L -o cmdline-tools.zip "$URL"
unzip -q cmdline-tools.zip -d cmdline-tmp
mkdir -p cmdline-tools/latest
mv cmdline-tmp/cmdline-tools/* cmdline-tools/latest/
rm -rf cmdline-tmp cmdline-tools.zip
SDKMANAGER="$INSTALL/cmdline-tools/latest/bin/sdkmanager"
"$SDKMANAGER" --sdk_root="$INSTALL" "platform-tools" "emulator" "platforms;android-34" "system-images;android-34;google_apis;x86_64"
AVDMANAGER="$INSTALL/cmdline-tools/latest/bin/avdmanager"
"$AVDMANAGER" create avd -n mobilebox -k "system-images;android-34;google_apis;x86_64" -d "pixel_6" --force
echo "Done."
