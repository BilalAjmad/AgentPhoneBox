# MCP Configuration

MobileBox ships an MCP server (`mobilebox-mcp`) that exposes the virtual phone
to any AI agent over stdio. It drives the phone through adb.

## Claude Code

Add to `~/.claude.json` (or project `.mcp.json`):

```json
{
  "mcpServers": {
    "mobilebox": {
      "command": "C:\\\\Users\\\\YOU\\\\.mobilebox\\\\mobilebox-mcp.exe",
      "args": [],
      "env": { "MOBILEBOX_INSTALL_DIR": "C:\\\\Users\\\\YOU\\\\AppData\\\\Local\\\\MobileBox" }
    }
  }
}
```

## Cursor

Settings → MCP → add server: command `mobilebox-mcp`, transport stdio.

## Kilo

Add to `kilo.json` under `mcp.servers`.

## Codex

`~/.codex/config.toml`:

```toml
[mcp_servers.mobilebox]
command = "mobilebox-mcp"
```

## Available tools

screenshot, describe_screen, tap, swipe, long_press, type_text, press_key,
launch_app, list_apps, install_apk, open_url, foreground_app,
wait_for_element, battery, logcat, adb_shell, device_info.

Every screen-changing tool returns a fresh screenshot so the agent observes
the result.
