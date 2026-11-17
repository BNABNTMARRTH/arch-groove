# Módulo 02: Almacenamiento, Particionado GPT y Arranque UEFI

Este documento detalla el diseño de almacenamiento y el esquema de arranque implementado para Arch Linux ARM en Apple Silicon y UTM.

---

## 1. Esquema de Particionado GPT

Para garantizar la compatibilidad estricta con las especificaciones UEFI de ARM64 y el estándar de arranque de Apple Silicon, se utiliza una tabla de particiones GUID (GPT) pura creada con `sgdisk`.

| Partición | Tamaño | Tipo GPT | Sistema de Archivos | Etiqueta | Función |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **`/dev/vdX1`** | 256 MB | `ef00` (EFI System Partition) | FAT32 | `ESP` | Contiene el cargador `BOOTAA64.EFI`, kernel y `initramfs`. |
| **`/dev/vdX2`** | Resto | `8300` (Linux Filesystem) | ext4 | `archroot` | Sistema de archivos raíz (Rootfs) de Arch Linux ARM. |

---

## 2. Configuración de `systemd-boot`

`systemd-boot` es el gestor de arranque elegido debido a su simplicidad, rendimiento nativo y ausencia de dependencias externas complejas.

### Archivo de Configuración Global (`/boot/loader/loader.conf`):
```ini
default arch-mac.conf
timeout 3
console-mode max
editor no
```

### Entrada de Arranque (`/boot/loader/entries/arch-mac.conf`):
```ini
title   Arch Linux ARM (Apple Silicon / UTM)
linux   /vmlinuz-linux
initrd  /initramfs-linux.img
options root=LABEL=archroot rw systemd.gpt_auto=0 console=tty0 console=ttyAMA0 earlycon audit=0
```

### Justificación de Parámetros de Kernel:
- **`root=LABEL=archroot rw`**: Especifica la raíz mediante etiqueta del sistema de archivos, asegurando que el arranque nunca dependa de letras volátiles de disco (`/dev/sda` vs `/dev/vda`).
- **`systemd.gpt_auto=0`**: Desactiva el montaje automático por UUID de partición de systemd para evitar tiempos de espera innecesarios en discos virtuales o almacenamiento externo.
- **`console=tty0 console=ttyAMA0`**: Habilita la salida concurrente tanto en la pantalla gráfica KMS (`tty0`) como en la terminal serie virtual (`ttyAMA0` para PL011 UART).
- **`earlycon`**: Permite recibir mensajes del kernel desde los primeros microsegundos antes de que el controlador de consola completo esté cargado.
- **`audit=0`**: Reduce la sobrecarga de logs del subsistema de auditoría de seguridad del kernel.

