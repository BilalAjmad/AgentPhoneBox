# MobileBox Settings
$mcp = "H:\MobileBox\target\debug\mobilebox-mcp.exe"

function Invoke-Mcp($method, $params) {
    $req = '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"settings","version":"1"}}}' + "`n" +
           '{"jsonrpc":"2.0","method":"notifications/initialized"}' + "`n" +
           ('{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"' + $method + '","arguments":' + $params + '}}') + "`n"
    $out = $req | & $mcp 2>$null
    $last = $out | Select-Object -Last 1
    try { ($last | ConvertFrom-Json).result.content[0].text } catch { $last }
}

while ($true) {
    Write-Host ""
    Write-Host "=== MobileBox Settings ===" -ForegroundColor Cyan
    Write-Host "1) Restart phone"
    Write-Host "2) Toggle fullscreen (Alt+Enter)"
    Write-Host "3) Exit"
    $c = Read-Host "Choice"
    switch ($c) {
        "1" {
            $r = Invoke-Mcp "restart_emulator" '{}'
            Write-Host "Result: $r" -ForegroundColor Green
        }
        "2" {
            $wsh = New-Object -ComObject WScript.Shell
            if ($wsh.AppActivate('Android Emulator')) {
                Start-Sleep -Milliseconds 500
                $wsh.SendKeys('%{ENTER}')
                Write-Host "Fullscreen toggled" -ForegroundColor Green
            } else {
                Write-Host "Emulator window not found" -ForegroundColor Yellow
            }
        }
        "3" { exit }
        default { Write-Host "Invalid choice" }
    }
}
