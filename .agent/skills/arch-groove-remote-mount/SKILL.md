---
name: arch-groove-remote-mount
description: Skill para configurar, montar y automatizar el acceso remoto a discos USB de un router e ISOs por Wi-Fi (SMB/CIFS, NFS, Tailscale y WireGuard) en Arch Groove ARM64.
---

# Skill: Montaje Remoto de Disco e ISO por Wi-Fi (`arch-groove-remote-mount`)

## Herramientas Preinstaladas en la VM
- `cifs-utils` (`mount.cifs`)
- `nfs-utils` (`mount.nfs`)
- `tailscale` (`tailscaled.service` habilitado)
- `wireguard-tools` (`wg`, `wg-quick`)
- Punto de montaje listo: `/mnt/router`

## Flujo para cualquier IA

### 1. Montar recurso SMB del router
```bash
sudo mount -t cifs //<ROUTER_IP>/<SHARE> /mnt/router -o username=<USER>,password=<PASS>,uid=1000,gid=1000,vers=3.0
```
*(Si falla por versión antigua del router, probar `vers=2.0` o `vers=1.0`).*

### 2. Montar recurso NFS del router
```bash
sudo mount -t nfs <ROUTER_IP>:/<EXPORT_PATH> /mnt/router
```

### 3. Montar archivo `.iso` dentro del recurso remoto
```bash
sudo mkdir -p /mnt/iso
sudo mount -o loop /mnt/router/<RUTA_ISO>.iso /mnt/iso
```

### 4. Habilitar acceso desde cualquier red externa
- **Tailscale**:
  ```bash
  sudo tailscale up
  ```
- **WireGuard** (con `/etc/wireguard/wg0.conf` provisto):
  ```bash
  sudo systemctl enable --now wg-quick@wg0
  ```

### 5. Hacer montaje persistente en `/etc/fstab` (opcional)
```text
//<ROUTER_IP>/<SHARE> /mnt/router cifs username=<USER>,password=<PASS>,uid=1000,gid=1000,_netdev,x-systemd.automount 0 0
```

