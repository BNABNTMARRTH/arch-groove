# GrooveCore — Arch Groove OS Harness & Host Extension

Eres un agente con control completo sobre **Arch Groove ARM64 (KDE Plasma 6 Wayland)** a través de **GrooveCore** (`groovecore-mcp`).

## Herramientas Disponibles (`groovecore-mcp`)
- **Pantalla y Entrada**: `screen_capture`, `mouse_move`, `mouse_click`, `keyboard_type`
- **Portapapeles**: `clipboard_get`, `clipboard_set`
- **Terminal Persistente**: `tmux_run`, `tmux_read`
- **Delegación Multi-Agente**: `delegate_agent` (lanza sub-agentes OpenCode en sesiones `tmux` aisladas)
- **Sistema**: `system_snapshot`

## Mapa del Sistema y Persistencia
- **Dock macOS (`org.vicko.wavetask`)**: `/usr/share/plasma/plasmoids/org.vicko.wavetask/contents/ui/{main.qml,Task.qml}`
- **Tema Kvantum**: `/usr/share/Kvantum/{WhiteSur,WhiteSurDark}/`
- **Configs de usuario**: `/home/alarm/.config/` (siempre replicar cambios a `/etc/skel/.config/` y ejecutar `sync`)
- **Apps instaladas**: Brave (`/usr/local/bin/brave`), OnlyOffice (`flatpak run org.onlyoffice.desktopeditors`), OpenCode (`/usr/local/bin/opencode`).
