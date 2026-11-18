# Guía Completa: Montar la ISO / VM desde un Router por Wi-Fi (Red Local y Cualquier Red)

## 1. Preparación en el Router (Disco USB)
1. Conecta tu disco duro externo al puerto USB del router.
2. En el panel web del router (ej. `192.168.1.1`), activa **USB Sharing / Samba (SMB)** (o **NFS**) y anota:
   - IP local del router (ej. `192.168.1.1`)
   - Nombre del recurso compartido (ej. `Backup` o `sda1`)
   - Usuario y contraseña (si configuraste)

---

## 2. Uso en Red Local (Wi-Fi de Casa / Oficina)

### Desde macOS (para abrir la VM o ISO en UTM)
1. En **Finder**, presiona `Cmd + K`.
2. Escribe `smb://192.168.1.1/Backup` y presiona **Conectar**.
3. El disco aparecerá en `/Volumes/Backup/`. Desde UTM puedes abrir la VM o montar la ISO directamente.

### Desde dentro de Arch Groove (VM)
```bash
# Montar recurso SMB del router en /mnt/router
sudo mount -t cifs //192.168.1.1/Backup /mnt/router -o username=admin,password=TU_PASS,uid=1000,gid=1000

# Montar una ISO ubicada dentro del disco del router
sudo mkdir -p /mnt/iso
sudo mount -o loop /mnt/router/ruta/arch-groove.iso /mnt/iso
```

---

## 3. Uso desde Cualquier Red (Por Internet, Fuera de Casa)

> **Importante**: Nunca abras el puerto 445 (SMB) directamente a Internet. Usa un túnel VPN cifrado.

### Opción A: Tailscale (Más fácil, sin abrir puertos)
1. En un dispositivo dentro de tu red local (router compatible, Raspberry Pi o tu Mac en casa):
   ```bash
   tailscale up --advertise-routes=192.168.1.0/24
   ```
   (Aprueba la ruta de subred en el panel web de Tailscale).
2. En la VM Arch Groove (ya tiene `tailscaled` instalado y activo):
   ```bash
   sudo tailscale up
   ```
3. Desde cualquier Wi-Fi del mundo, monta `//192.168.1.1/Backup` en `/mnt/router` exactamente igual que en red local.

### Opción B: WireGuard (Si tu router tiene servidor WireGuard)
1. Activa **WireGuard Server** en el router y descarga el archivo `.conf` del cliente.
2. En la VM Arch Groove:
   ```bash
   sudo cp cliente.conf /etc/wireguard/wg0.conf
   sudo systemctl enable --now wg-quick@wg0
   ```
3. Monta `//192.168.1.1/Backup` normalmente.
