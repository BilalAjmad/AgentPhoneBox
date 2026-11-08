# MobileBox Fullscreen Toggle - Uses Windows API for true fullscreen
Add-Type -TypeDefinition @"
using System;
using System.Runtime.InteropServices;
public class WinAPI {
    [DllImport("user32.dll")] public static extern IntPtr GetForegroundWindow();
    [DllImport("user32.dll")] public static extern bool SetForegroundWindow(IntPtr hWnd);
    [DllImport("user32.dll")] public static extern bool ShowWindow(IntPtr hWnd, int nCmdShow);
    [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr hWnd, out RECT lpRect);
    [DllImport("user32.dll")] public static extern int GetWindowLong(IntPtr hWnd, int nIndex);
    [DllImport("user32.dll")] public static extern int SetWindowLong(IntPtr hWnd, int nIndex, int dwNewLong);
    [DllImport("user32.dll")] public static extern bool SetWindowPos(IntPtr hWnd, IntPtr hWndInsertAfter, int X, int Y, int cx, int cy, uint uFlags);
    [DllImport("user32.dll")] public static extern bool GetMonitorInfo(IntPtr hMonitor, ref MONITORINFO lpmi);
    [DllImport("user32.dll")] public static extern IntPtr MonitorFromWindow(IntPtr hwnd, uint dwFlags);
    [DllImport("user32.dll")] public static extern int GetSystemMetrics(int nIndex);

    public const int SW_MAXIMIZE = 3;
    public const int SW_RESTORE = 9;
    public const int GWL_STYLE = -16;
    public const int GWL_EXSTYLE = -20;
    public const int WS_CAPTION = 0x00C00000;
    public const int WS_THICKFRAME = 0x00040000;
    public const int WS_MINIMIZEBOX = 0x00020000;
    public const int WS_MAXIMIZEBOX = 0x00010000;
    public const int WS_SYSMENU = 0x00080000;
    public const int WS_OVERLAPPEDWINDOW = 0x00CF0000;
    public const int WS_POPUP = 0x80000000;
    public const uint SWP_NOSIZE = 0x0001;
    public const uint SWP_NOMOVE = 0x0002;
    public const uint SWP_NOZORDER = 0x0004;
    public const uint SWP_FRAMECHANGED = 0x0020;
    public const int MONITOR_DEFAULTTOPRIMARY = 0x00000001;

    [StructLayout(LayoutKind.Sequential)]
    public struct RECT { public int Left, Top, Right, Bottom; }

    [StructLayout(LayoutKind.Sequential, CharSet = CharSet.Auto)]
    public class MONITORINFO {
        public int cbSize = System.Runtime.InteropServices.Marshal.SizeOf(typeof(MONITORINFO));
        public RECT rcMonitor = new RECT();
        public RECT rcWork = new RECT();
        public int dwFlags = 0;
    }
}
"@

$VK_CONTROL = 0x11
$VK_MENU = 0x12
$VK_F = 0x46
$VK_F11 = 0x7A

$wsh = New-Object -ComObject WScript.Shell
$wasPressed = $false
$isFullscreen = $false

Write-Host "MobileBox Fullscreen Toggle - Press Ctrl+Alt+F or F11"

while ($true) {
    $emu = Get-Process -Name "qemu-system-x86_64" -ErrorAction SilentlyContinue
    if (-not $emu) { 
        Write-Host "Emulator not running, exiting..."
        break 
    }

    # Check for Ctrl+Alt+F
    $ctrl = [HK]::GetAsyncKeyState(0x11) -band 0x8000
    $alt = [HK]::GetAsyncKeyState(0x12) -band 0x8000
    $f = [HK]::GetAsyncKeyState(0x46) -band 0x8000
    $f11 = [HK]::GetAsyncKeyState(0x7A) -band 0x8000

    if (($ctrl -and $alt -and $f) -or $f11) {
        if (-not $wasPressed) {
            $wasPressed = $true
            Write-Host "Fullscreen toggle triggered"

            $emuProc = Get-Process -Name "qemu-system-x86_64" -ErrorAction SilentlyContinue | Select-Object -First 1
            if ($emuProc -and $emuProc.MainWindowHandle -ne 0) {
                $hWnd = $emuProc.MainWindowHandle
                
                if (-not $isFullscreen) {
                    # Enter fullscreen: remove window decorations and maximize
                    $style = [WinAPI]::GetWindowLong($hWnd, [WinAPI]::GWL_STYLE)
                    $newStyle = $style -band -bnot ([WinAPI]::WS_CAPTION -bor [WinAPI]::WS_THICKFRAME -bor [WinAPI]::WS_MINIMIZEBOX -bor [WinAPI]::WS_MAXIMIZEBOX -bor [WinAPI]::WS_SYSMENU)
                    [WinAPI]::SetWindowLong($hWnd, [WinAPI]::GWL_STYLE, $newStyle) | Out-Null
                    
                    # Get screen dimensions
                    $screenWidth = [WinAPI]::GetSystemMetrics(0)  # SM_CXSCREEN
                    $screenHeight = [WinAPI]::GetSystemMetrics(1) # SM_CYSCREEN
                    
                    [WinAPI]::SetWindowPos($hWnd, [IntPtr]::Zero, 0, 0, $screenWidth, $screenHeight, 
                        [WinAPI]::SWP_NOZORDER -bor [WinAPI]::SWP_FRAMECHANGED) | Out-Null
                    [WinAPI]::ShowWindow($hWnd, [WinAPI]::SW_MAXIMIZE) | Out-Null
                    $isFullscreen = $true
                    Write-Host "Entered fullscreen mode"
                } else {
                    # Exit fullscreen: restore window style
                    $style = [WinAPI]::GetWindowLong($hWnd, [WinAPI]::GWL_STYLE)
                    $newStyle = $style -bor [WinAPI]::WS_OVERLAPPEDWINDOW
                    [WinAPI]::SetWindowLong($hWnd, [WinAPI]::GWL_STYLE, $newStyle) | Out-Null
                    [WinAPI]::SetWindowPos($hWnd, [IntPtr]::Zero, 100, 100, 800, 600, 
                        [WinAPI]::SWP_NOZORDER -bor [WinAPI]::SWP_FRAMECHANGED) | Out-Null
                    [WinAPI]::ShowWindow($hWnd, [WinAPI]::SW_RESTORE) | Out-Null
                    $isFullscreen = $false
                    Write-Host "Exited fullscreen mode"
                }
            }
        }
    } else {
        $wasPressed = $false
    }
    Start-Sleep -Milliseconds 100
}