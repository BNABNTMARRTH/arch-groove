# Módulo 03: Virtualización en UTM, Hipervisor HVF y Optimización de Cores en Apple Silicon

Este documento aborda los aspectos críticos del funcionamiento de máquinas virtuales ARM64 sobre el hipervisor de macOS (**Hypervisor.framework / HVF**) en procesadores Apple Silicon (M1/M2/M3/M4).

---

## 1. El Fenómeno del *Soft Lockup* en Cores Asimétricos (M4 / M-Series)

### Diagnóstico Forense
En logs del kernel se observaba:
```text
watchdog: BUG: soft lockup - CPU#0 stuck for 49s! [kworker/0:1]
CPU#0 Utilization every 4000ms during lockup:
#1: 100% system, #2: 100% system, #3: 100% system, #4: 100% system
Call trace:
smp_call_function_many_cond+0x148/0x3cc
kick_all_cpus_sync+0x38/0x70
jump_label_update+0xd0/0x120
```

### Explicación Física y de Arquitectura
El hardware de Apple Silicon no utiliza un diseño SMP homogéneo tradicional. Los núcleos están divididos en dos clústeres físicos dispares:
1. **Núcleos de Rendimiento (P-Cores):** Disponen de relojes dinámicos más altos, cachés L1/L2 significativamente mayores y decodificadores de instrucciones más anchos.
2. **Núcleos de Eficiencia (E-Cores):** Diseñados para bajo consumo energético con relojes y cachés reducidos.

Cuando QEMU solicita 4 CPUs virtuales sobre HVF sin afinidad de pines de hilo estricta, el planificador del host distribuye los hilos de QEMU entre núcleos P y núcleos E. Cuando el kernel de Linux dentro de la máquina virtual emite una instrucción IPI (Inter-Processor Interrupt) con barrera de sincronización (`smp_call_function`), los núcleos rápidos terminan su ciclo y quedan atrapados en un bucle de giro (*spinlock*) esperando que los núcleos lentos respondan. El temporizador del perro guardián (*watchdog*) interpreta este retraso como un bloqueo del sistema (`soft lockup`).

### Mitigación y Solución Definitiva
La solución óptima consiste en configurar la máquina virtual con **2 vCPUs**:
```xml
<key>System</key>
<dict>
    <key>Architecture</key>
    <string>aarch64</string>
    <key>CPU</key>
    <string>host</string>
    <key>CPUCount</key>
    <integer>2</integer>
    <key>Hypervisor</key>
    <true/>
</dict>
```
Al reducir la cuenta a 2 vCPUs, el hipervisor del host mantiene los hilos en el mismo clúster homogéneo de cómputo, eliminando la contención de sincronización IPI y reduciendo el tiempo de arranque de **53 segundos** a solo **6 segundos**.

---

## 2. Subsistema Gráfico VirtIO GPU y DRM/KMS

### El Problema de la Pantalla Negra ("Display output is not active")
KWin Wayland y SDDM requieren un dispositivo de renderizado directo (**DRM / KMS**) accesible en `/dev/dri/card0`. Por defecto, si el módulo de kernel `virtio-gpu` no está cargado, el kernel solo expone un framebuffer estático ciego en `/dev/fb0`.

### Carga Automática de Controladores:
Archivo `/etc/modules-load.d/virtio.conf`:
```ini
virtio-gpu
virtio_net
```

Una vez cargado el controlador:
- `virtio-gpu` inicializa el nodo `/dev/dri/card0` para la gestión de modos de pantalla (KMS).
- Inicializa el nodo `/dev/dri/renderD128` para la aceleración gráfica por software/hardware mediante Mesa.
- SDDM y Wayland se conectan automáticamente al scanout activo, eliminando el aviso de "Display output is not active".

