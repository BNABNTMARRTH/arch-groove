# Arch Groove ARM64 — OS Harness & Host Extension

**Arch Groove** es una distribución **Arch Linux ARM (`aarch64`)** optimizada para **Apple Silicon (M1/M2/M3/M4)** y virtualización acelerada en **UTM**, con escritorio **KDE Plasma 6 Wayland** estilizado al estilo macOS y diseñada nativamente para operar como **Harness y Extensión de Sistema Operativo para modelos de IA**.

---

## - Objetivos Principales

### 1. Virtualización Portátil vía Thunderbolt y Wi-Fi
- **Thunderbolt / USB-C Directo**: Ejecuta tu VM o ISO directamente desde un SSD externo a máxima velocidad, con todas las configuraciones persistidas en `/etc/skel/.config/`.
- **Montaje Remoto por Wi-Fi (Local y Cualquier Red)**: Preparada con `cifs-utils`, `nfs-utils`, `tailscale` y `wireguard-tools` para montar discos e ISOs compartidos desde tu router o red privada desde cualquier lugar.
- Guía detallada: [`docs/remote_iso_mount_wifi.md`](docs/remote_iso_mount_wifi.md)

### 2. GrooveCore: El Sistema Operativo como Harness y Extensión de IA
**GrooveCore** (`harness/groovecore-mcp`) convierte al sistema operativo completo en un entorno ejecutable para cualquier modelo de IA (OpenCode, Antigravity, Claude, etc.):
- **Harness Interno**: Modelos corriendo *dentro* de la VM (ej. OpenCode) operan todo el SO vía MCP local (`stdio`).
- **Extensión Externa**: Modelos corriendo *fuera* (en el Mac host) se conectan vía [`harness/groovecore-bridge.sh`](harness/groovecore-bridge.sh) usando la VM como una extensión Linux ARM64 con entorno gráfico real.

**Capacidades expuestas por `groovecore-mcp`:**
- **Visión y Control GUI**: Capturas de pantalla (`spectacle`), control de mouse/teclado en Wayland (`ydotool`) y portapapeles compartido (`wl-clipboard` + `spice-vdagent`).
- **Terminal Persistente**: Ejecución y lectura en sesiones `tmux`.
- **Delegación Multi-Agente**: Lanzamiento de sub-agentes autónomos (`delegate_agent`) en sesiones aisladas.
- **Apps Listas**: Brave Web Browser (`aarch64`), OnlyOffice Desktop Editors, OpenCode (`1.19.6`) y Konsole.

---

## 📁 Estructura del Repositorio

| Ruta | Descripción |
| :--- | :--- |
| [`harness/`](harness/) | Servidor MCP `groovecore-mcp`, puente SSH `groovecore-bridge.sh` y contexto `GROOVECORE.md` |
| [`.agent/skills/`](.agent/skills/) | Skills para IAs (`arch-groove-dock-theme`, `arch-groove-remote-mount`) |
| [`configs/`](configs/) | Configuraciones persistentes de KDE Plasma 6, KWin, Dolphin, Dock `wavetask` y `/etc/skel/` |
| [`crates/`](crates/) & [`systemd/`](systemd/) | Suite de demonios en Rust (`asahi-fan-control`, `asahi-power-profile`, `asahi-battery-limiter`) |
| [`docs/`](docs/) | Guías técnicas de arquitectura, montaje Wi-Fi, Dock macOS y compilación de Blender `aarch64` |
| [`installer/`](installer/) & [`pkgbuilds/`](pkgbuilds/) | Perfil de paquetes `aarch64` y PKGBUILDs |

---

## 📚 Documentación Rápida
- [Guía Maestra Apple Silicon & UTM](docs/README.md)
- [Montaje Remoto de ISO por Wi-Fi (SMB / Tailscale / WireGuard)](docs/remote_iso_mount_wifi.md)
- [Plan de Compilación de Blender en `aarch64`](docs/blender_aarch64_install_plan.md)

---

## 🚀 Inicio Rápido

1. **Clonar el repositorio:**
   ```bash
   git clone https://github.com/BNABNTMARRTH/arch-groove.git
   cd arch-groove
   ```
2. **Desplegar base en UTM / Apple Silicon:** seguir [`docs/README.md`](docs/README.md) e instalar [`installer/profile/packages.aarch64`](installer/profile/packages.aarch64).
3. **Aplicar entorno macOS (Dock, KWin, Dolphin, idioma):**
   ```bash
   cp -a configs/skel/* ~/.config/
   sudo cp -a configs/skel/* /etc/skel/.config/
   sudo cp configs/wavetask/{main.qml,Task.qml} /usr/share/plasma/plasmoids/org.vicko.wavetask/contents/ui/
   systemctl --user restart plasma-plasmashell.service
   ```
4. **Activar GrooveCore (Harness + Extensión):**
   ```bash
   sudo pacman -S --noconfirm --needed spectacle ydotool wl-clipboard tmux spice-vdagent cifs-utils tailscale
   sudo cp harness/groovecore-mcp /usr/local/bin/groovecore-mcp && sudo chmod +x /usr/local/bin/groovecore-mcp
   sudo mkdir -p /etc/groovecore && sudo cp harness/GROOVECORE.md /etc/groovecore/GROOVECORE.md
   ```
   Desde el Mac host:
   ```bash
   GROOVECORE_VM_IP=<IP_DE_TU_VM> ./harness/groovecore-bridge.sh
   ```

