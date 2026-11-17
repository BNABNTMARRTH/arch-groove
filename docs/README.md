# Arch Linux ARM en Apple Silicon & UTM (EGM-Compliant)

Documentación técnica integral, arquitectura del sistema y guías operativas para la distribución Arch Linux ARM optimizada para hardware Apple Silicon (M1/M2/M3/M4) y virtualización acelerada por hipervisor en UTM.

---

## 📑 Tabla de Contenidos

1. [Guía Maestra Paso a Paso](arch_linux_apple_silicon_guide.md)
   - Resumen ejecutivo, especificaciones de hardware, particionado GPT, arranque UEFI, entorno KDE Plasma 6 Wayland y solución de problemas.
2. [Arquitectura de Hardware y Especificación Técnica](designs/apple_silicon_arch_spec.md)
   - Especificaciones de arquitectura ARM64, buses, GPU AGX, controladores de energía y memoria unificada.
3. [Módulos Técnicos Especializados](architecture/):
   - [01. Suite de Optimización en Rust](architecture/01_rust_suite_daemons.md): Demonios de ventilación (`asahi-fan-control`), gobernador de perfiles energéticos (`asahi-power-profile`) y limitador de batería (`asahi-battery-limiter`).
   - [02. Almacenamiento, Particionado GPT y UEFI](architecture/02_storage_efi_partitioning.md): Particionado con `sgdisk`, partición ESP FAT32 con `systemd-boot`, partición raíz `ext4` y parámetros de kernel.
   - [03. Virtualización UTM, HVF y Cores Asimétricos](architecture/03_utm_qemu_apple_silicon.md): Causa raíz del *soft lockup* SMP en Apple Silicon (P-Cores vs E-Cores), solución mediante vCPUs homogéneas y configuración de DRM/KMS `virtio-gpu`.
   - [04. Resolución del Conflicto OpenSSL ABI](architecture/04_openssl_abi_troubleshooting.md): Procedimiento forense de resolución de colisión entre `libcrypto.so.3` y `libcrypto.so.4` sin requerir herramientas externas.

---

## 🏛️ Gobernanza y Modelo Arquitectónico (EGM)

Este proyecto sigue los principios de diseño y estándares de ingeniería gobernados por el **Elephant-Goldfish Model (EGM)**:
- **Cero Parches Frágiles:** No se utilizan trampas temporales ni soluciones no estándar; cada componente se adhiere a los estándares oficiales de Arch Linux y upstream Linux.
- **Rendimiento Nativo en Rust:** Los servicios auxiliares se compilan con nivel de optimización máximo (`opt-level = 3`, LTO activado, binarios *stripped*).
- **Seguridad Criptográfica y Limpieza:** El sistema de arranque y empaquetado utiliza llaves verificadas y compatibilidad estricta con POSIX y `systemd-boot`.

