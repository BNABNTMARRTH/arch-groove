---
name: arch-groove-dock-theme
description: Guía y referencia técnica para modificar el Dock macOS (org.vicko.wavetask), temas nativos de KDE Plasma 6 / KWin y temas Kvantum (WhiteSur / WhiteSurDark) en Arch Groove ARM64.
---

# Skill: Modificación de Dock y Tema en Arch Groove (KDE Plasma 6 Wayland)

## 1. Dock macOS (`org.vicko.wavetask`)
- **Archivos principales**:
  - `/usr/share/plasma/plasmoids/org.vicko.wavetask/contents/ui/main.qml`
  - `/usr/share/plasma/plasmoids/org.vicko.wavetask/contents/ui/Task.qml`
- **Configuración del usuario**:
  - `/home/alarm/.config/plasma-org.kde.plasma.desktop-appletsrc` (`iconSize=42`, `magnification=45.0`, `amplitud=1.8`)
  - `/home/alarm/.config/plasmashellrc` (`panelThickness=135`)
- **Reglas clave aplicadas**:
  - **Sin empalme lateral**: `iconCenter` en `Task.qml` debe usar `task.x + (task.width / 2)` (posición real renderizada) en lugar de `index * _baseSize`.
  - **Globo único (`activeHoverIndex`)**: `main.qml` calcula el índice más cercano al cursor en `HoverHandler` y lo expone en `taskList.activeHoverIndex`.
  - **Globo arriba del icono**: `macToolTip` se ancla con `anchors.bottom: iconBox.top` y `anchors.bottomMargin: Math.round(iconBox.height * (zoomFactor - 1.0)) + 8`.
  - **Sin recuadro de clic/hover ni círculo de carga**: `frame.imagePath: ""` y `busyIndicator` `Loader` con `active: false`.
- **Aplicar cambios**:
  ```bash
  systemctl --user -M alarm@ restart plasma-plasmashell.service
  ```

## 2. Tema Nativo (KWin, Dolphin, KDE Wallet, Bloqueo)
- **KWin (`/home/alarm/.config/kwinrc`)**:
  - `[org.kde.kdecoration2]`: `ButtonsOnLeft=XIA`, `BorderSize=Tiny`, `BorderSizeAuto=false`.
  - Recargar sin reiniciar sesión: `qdbus6 org.kde.KWin /KWin reconfigure`.
- **Reglas de opacidad (`/home/alarm/.config/kwinrulesrc`)**:
  - `systemsettings` con `opacityactive=88` y `opacityinactive=88` para transparencia uniforme en apps Kirigami.
- **Dolphin (`/home/alarm/.config/dolphinrc`)**:
  - `ShowStatusBar=FullWidth` (evita el recuadro flotante en la esquina inferior).
- **Launch Feedback (`/home/alarm/.config/klaunchrc`)**:
  - `Bouncing=false`, `BusyCursor=false`, `TaskbarButton=false`.
- **Bloqueo y KDE Wallet**:
  - `kscreenlockerrc`: `Autolock=false`.
  - `kwalletrc`: `Enabled=false`, `First Use=false`.

## 3. Tema Kvantum (`WhiteSur` / `WhiteSurDark`)
- **Rutas**:
  - `/usr/share/Kvantum/WhiteSur/WhiteSur.kvconfig`
  - `/usr/share/Kvantum/WhiteSurDark/WhiteSurDark.kvconfig`
- **Notas**:
  - `[StatusBar]` usa `frame=false` e `interior=false` para barra de estado limpia.
  - `[Window]` mantiene `frame.bottom=10` para respetar el pie de ventana en apps Kirigami.

## 4. Persistencia en la Distro / ISO
Siempre sincronizar `/home/alarm/.config/` hacia `/etc/skel/.config/` y ejecutar `sync`:
```bash
cp -a /home/alarm/.config/{kwinrc,kwinrulesrc,dolphinrc,klaunchrc,kscreenlockerrc,powerdevilrc,kwalletrc,plasmashellrc,plasma-org.kde.plasma.desktop-appletsrc} /etc/skel/.config/
sync
```

