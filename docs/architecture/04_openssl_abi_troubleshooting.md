# Módulo 04: Resolución del Conflicto OpenSSL ABI (libcrypto.so.3 vs libcrypto.so.4)

Este documento detalla la investigación forense, la causa raíz y la metodología de rescate utilizada ante la colisión binaria de librerías criptográficas de OpenSSL en Arch Linux ARM.

---

## 1. Síntomas Observados

Al ejecutar cualquier comando de gestión de paquetes o servicios del sistema:
```text
pacman: error while loading shared libraries: libcrypto.so.3: cannot open shared object file: No such file or directory
startplasma-wayland: error while loading shared libraries: libcrypto.so.4: cannot open shared object file: No such file or directory
sshd.service: Failed to start OpenSSH Daemon.
```

Al intentar forzar enlaces simbólicos cruzados:
```text
pacman: /usr/lib/libcrypto.so.3: version `OPENSSL_3.0.0' not found (required by /usr/lib/libalpm.so.16)
pacman: /usr/lib/libcrypto.so.3: version `OPENSSL_3.0.0' not found (required by /usr/lib/libarchive.so.13)
pacman: /usr/lib/libcrypto.so.3: version `OPENSSL_3.0.0' not found (required by /usr/lib/libcurl.so.4)
```

---

## 2. Causa Raíz

En distribuciones GNU/Linux con versiones ABI diferentes:
- **`openssl-3` (`libcrypto.so.3`)**: Exporta el mapa de versiones de símbolos ELF `OPENSSL_3.0.0`, `OPENSSL_3.2.0`, etc.
- **`openssl-4` (`libcrypto.so.4`)**: Proviene de una versión incompatible de OpenSSL/LibreSSL y exporta un mapa de símbolos diferente.

Al utilizar banderas de sobreescritura (`--overwrite`), los archivos reales y symlinks de OpenSSL 3 fueron reemplazados por los de OpenSSL 4. Dado que el gestor de paquetes `pacman` y las librerías del sistema (`libalpm.so`, `libarchive.so`, `libcurl.so`) están enlazados dinámicamente contra los símbolos exactos de `OPENSSL_3.0.0`, `pacman` quedó inoperativo, impidiendo cualquier reinstalación a través del gestor de paquetes tradicional.

---

## 3. Metodología de Recuperación Limpia (Sin Dependencias Externas)

La herramienta `tar` es un ejecutable estático o enlazado contra libc que no requiere OpenSSL para funcionar. Además, Arch Linux almacena copias de los paquetes previamente descargados en `/var/cache/pacman/pkg/`.

### Procedimiento de Rescate:

1. **Inspección de la Caché Local:**
   ```bash
   ls /var/cache/pacman/pkg/openssl*
   ```
   Resultado:
   - `/var/cache/pacman/pkg/openssl-3-3.5.9-2-aarch64.pkg.tar.xz`
   - `/var/cache/pacman/pkg/openssl-4.1.0-1-aarch64.pkg.tar.xz`

2. **Extracción Atómica de Librerías Nativas de OpenSSL 3:**
   ```bash
   tar -xf /var/cache/pacman/pkg/openssl-3-3.5.9-2-aarch64.pkg.tar.xz -C / usr/lib/libcrypto.so.3 usr/lib/libssl.so.3
   ```

3. **Extracción Atómica de Librerías Nativas de OpenSSL 4:**
   ```bash
   tar -xf /var/cache/pacman/pkg/openssl-4.1.0-1-aarch64.pkg.tar.xz -C / usr/lib/libcrypto.so.4 usr/lib/libssl.so.4
   ```

4. **Actualización del Enlazador Dinámico de Objetos Compartidos:**
   ```bash
   ldconfig
   ```

5. **Verificación de Integridad:**
   ```bash
   pacman -V
   ```
   Resultado exitoso:
   ```text
   Pacman v7.1.0 - libalpm v16.0.1
   Copyright (C) 2006-2025 Pacman Development Team
   ```

Con este procedimiento, ambas versiones coexisten pacíficamente en `/usr/lib/`, satisfaciendo simultáneamente las demandas de `pacman`, `sshd`, Qt6 y `startplasma-wayland`.

