# MobileBox first-run setup (Windows)
# Downloads Android command-line tools and installs emulator + platform-tools + system image.
# Uses curl with resume support so interrupted downloads can continue.

$ErrorActionPreference = "Stop"
$install = if ($env:MOBILEBOX_INSTALL_DIR) { $env:MOBILEBOX_INSTALL_DIR } else { "$env:LOCALAPPDATA\MobileBox" }
$env:ANDROID_AVD_HOME = "$install\.android\avd"
$env:ANDROID_SDK_ROOT = $install
New-Item -ItemType Directory -Force -Path "$install\.android\avd" | Out-Null

$cmdlineUrl = "https://dl.google.com/android/repository/commandlinetools-win-11076708_latest.zip"
$zip = "$install\cmdline-tools.zip"
Write-Host "Downloading command-line tools (resumable, with retries)..."
& curl.exe -L -C - --retry 10 --retry-delay 5 --retry-all-errors -o $zip $cmdlineUrl
if ($LASTEXITCODE -ne 0) { throw "Download failed with code $LASTEXITCODE" }

# Only extract if not already extracted
if (-not (Test-Path "$install\cmdline-tools\latest\bin\sdkmanager.bat")) {
    Write-Host "Extracting..."
    Expand-Archive -Path $zip -DestinationPath "$install\cmdline-tools-tmp" -Force
    New-Item -ItemType Directory -Force -Path "$install\cmdline-tools\latest" | Out-Null
    Move-Item "$install\cmdline-tools-tmp\cmdline-tools\*" "$install\cmdline-tools\latest\" -Force
    Remove-Item "$install\cmdline-tools-tmp" -Recurse -Force
}

$sdkmanager = "$install\cmdline-tools\latest\bin\sdkmanager.bat"
Write-Host "Installing emulator + platform-tools + Android 14 system image..."
& $sdkmanager --sdk_root=$install "platform-tools" "emulator" "platforms;android-34" "system-images;android-34;google_apis_playstore;x86_64"

# Create the MobileBox AVD using a real Pixel phone profile.
$avdmanager = "$install\cmdline-tools\latest\bin\avdmanager.bat"
if (-not (Test-Path "$install\.android\avd\mobilebox.avd")) {
    Write-Host "Creating MobileBox AVD..."
    & $avdmanager create avd -n mobilebox -k "system-images;android-34;google_apis_playstore;x86_64" -d "pixel_6" --force
}

Write-Host "Done. Emulator installed to $install\emulator"
Write-Host "You can now tap the MobileBox icon to open the phone."

# Create desktop shortcut (try release build, then debug build)
$exe = $null
foreach ($p in @("$install\..\MobileBox\target\release\MobileBox.exe","$install\..\MobileBox\target\debug\MobileBox.exe","$PSScriptRoot\..\target\release\MobileBox.exe","$PSScriptRoot\..\target\debug\MobileBox.exe")) {
    if (Test-Path $p) { $exe = (Resolve-Path $p).Path; break }
}
if ($exe) {
    $desktop = [Environment]::GetFolderPath("Desktop")
    $ws = New-Object -ComObject WScript.Shell
    $sc = $ws.CreateShortcut("$desktop\MobileBox.lnk")
    $sc.TargetPath = $exe
    $sc.WorkingDirectory = Split-Path $exe
    $sc.Description = "MobileBox - Lightweight virtual Android phone"
    $sc.Save()
    Write-Host "Desktop shortcut created: $desktop\MobileBox.lnk"
} else {
    Write-Host "Build the app first (cargo build --release) to create a desktop shortcut."
}
