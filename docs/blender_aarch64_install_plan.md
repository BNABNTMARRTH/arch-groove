# Plan de Instalación de Blender en Arch Linux ARM (`aarch64`)

## Contexto
Blender Foundation no distribuye binarios precompilados de Linux para `aarch64`, y los repositorios de Arch Linux ARM (`extra`) tampoco lo incluyen. Además, Arch Linux ARM utiliza **Python 3.14**, mientras que Blender 4.3 requiere la API C de **Python 3.11/3.12**.

Las dependencias base y el código fuente ya se encuentran preparados en la VM:
- Código fuente: `/opt/blender-src`
- Directorio de compilación: `/opt/blender-build`
- Espacio disponible en `/dev/vda2`: **49 GB libres**

---

## Opción 1: Compilar Blender 4.3 con Python 3.11 en paralelo (Recomendada)
Mantiene el código estable de Blender 4.3 sin conflictos con Python 3.14 del sistema.

1. **Instalar Python 3.11 en `/opt/python311`**:
   ```bash
   curl -sLO https://www.python.org/ftp/python/3.11.11/Python-3.11.11.tar.xz
   tar -xf Python-3.11.11.tar.xz && cd Python-3.11.11
   ./configure --prefix=/opt/python311 --enable-shared
   make -j$(nproc) && make install
   echo "/opt/python311/lib" > /etc/ld.so.conf.d/python311.conf
   ldconfig
   ```
2. **Reconfigurar CMake apuntando a Python 3.11**:
   ```bash
   cmake -S /opt/blender-src -B /opt/blender-build -G Ninja \
     -DCMAKE_BUILD_TYPE=Release \
     -DCMAKE_INSTALL_PREFIX=/usr/local \
     -DWITH_LIBS_PRECOMPILED=OFF \
     -DWITH_INSTALL_PORTABLE=OFF \
     -DWITH_PYTHON_INSTALL=OFF \
     -DPYTHON_VERSION=3.11 \
     -DPYTHON_ROOT_DIR=/opt/python311 \
     -DWITH_CYCLES=OFF \
     -DWITH_CODEC_FFMPEG=OFF \
     -DWITH_USD=OFF \
     -DWITH_OPENVDB=OFF \
     -DWITH_OPENIMAGEDENOISE=OFF \
     -DWITH_OPENCOLORIO=OFF
   ```
3. **Compilar e instalar**:
   ```bash
   cmake --build /opt/blender-build --target install -j$(nproc)
   update-desktop-database /usr/local/share/applications
   sync
   ```

---

## Opción 2: Compilar la rama `main` de Blender (4.4 / 4.5+)
Utiliza el Python del sistema si la rama `main` ya soporta Python 3.14:

1. **Actualizar fuentes a `main`**:
   ```bash
   cd /opt/blender-src
   git fetch --depth 1 origin main
   git checkout -B main FETCH_HEAD
   ```
2. **Reconfigurar y compilar**:
   ```bash
   rm -rf /opt/blender-build/*
   cmake -S /opt/blender-src -B /opt/blender-build -G Ninja \
     -DCMAKE_BUILD_TYPE=Release \
     -DCMAKE_INSTALL_PREFIX=/usr/local \
     -DWITH_LIBS_PRECOMPILED=OFF \
     -DWITH_INSTALL_PORTABLE=OFF \
     -DWITH_PYTHON_INSTALL=OFF \
     -DPYTHON_VERSION=3.14 \
     -DWITH_CYCLES=OFF \
     -DWITH_CODEC_FFMPEG=OFF
   cmake --build /opt/blender-build --target install -j$(nproc)
   ```
