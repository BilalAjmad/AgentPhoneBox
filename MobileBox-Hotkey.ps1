# MobileBox Hotkey Listener - Ctrl+Alt+F toggles fullscreen via Alt+Enter
Add-Type -TypeDefinition @"
using System;
using System.Runtime.InteropServices;
public class HK {
    [DllImport("user32.dll")] public static extern short GetAsyncKeyState(int vKey);
}
"@

$VK_CONTROL = 0x11
$VK_MENU = 0x12
$VK_F = 0x46
$VK_F11 = 0x7A

$wsh = New-Object -ComObject WScript.Shell
$wasPressed = $false

Write-Host "MobileBox Hotkey Listener started - Press Ctrl+Alt+F or F11 for fullscreen"

while ($true) {
    # Check if emulator is still running
    $emu = Get-Process -Name "qemu-system-x86_64" -ErrorAction SilentlyContinue
    if (-not $emu) { 
        Write-Host "Emulator not running, exiting..."
        break 
    }

    # Check for Ctrl+Alt+F or F11
    $ctrl = [HK]::GetAsyncKeyState(0x11) -band 0x8000
    $alt = [HK]::GetAsyncKeyState(0x12) -band 0x8000
    $f = [HK]::GetAsyncKeyState(0x46) -band 0x8000
    $f11 = [HK]::GetAsyncKeyState(0x7A) -band 0x8000

    if (($ctrl -and $alt -and $f) -or $f11) {
        if (-not $wasPressed) {
            $wasPressed = $true
            Write-Host "Fullscreen toggle triggered"
            
            # Find and activate emulator window
            $emuProc = Get-Process -Name "qemu-system-x86_64" -ErrorAction SilentlyContinue | Select-Object -First 1
            if ($emuProc -and $emuProc.MainWindowHandle -ne 0) {
                # Use SetForegroundWindow via P/Invoke
                Add-Type -TypeDefinition @"
using System;
using System.Runtime.InteropServices;
public class FG {
    [DllImport("user32.dll")] public static extern bool SetForegroundWindow(IntPtr hWnd);
    [DllImport("user32.dll")] public static extern bool ShowWindow(IntPtr hWnd, int nCmdShow);
}
"@ -ErrorAction SilentlyContinue
                
                $emuProc = Get-Process -Name "qemu-system-x86_64" -ErrorAction SilentlyContinue | Select-Object -First 1
                if ($emuProc -and $emuProc.MainWindowHandle -ne 0) {
                    Add-Type -TypeDefinition @"
using System;
using System.Runtime.InteropServices;
public class FG {
    [DllImport("user32.dll")] public static extern bool SetForegroundWindow(IntPtr hWnd);
    [DllImport("user32.dll")] public static extern bool ShowWindow(IntPtr hWnd, int nCmdShow);
}
"@ -ErrorAction SilentlyContinue
                    [FG]::SetForegroundWindow($emuProc.MainWindowHandle) | Out-Null
                    Start-Sleep -Milliseconds 300
                    $wsh = New-Object -ComObject WScript.Shell
                    $wsh.SendKeys('%{ENTER}')
                    Write-Host "Sent Alt+Enter to toggle fullscreen"
                }
            }
        }
    } else {
        $wasPressed = $false
    }
    Start-Sleep -Milliseconds 100
}