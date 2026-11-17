# Arch Linux Documentation Compliant Architecture Specification: Apple Silicon (ARM64)

## 1. Boot Chain Architecture
According to the official upstream Asahi Linux and Arch Linux specifications:
1. **ROM / iBoot Stage 1**: Verifies boot image on the internal NVMe storage in the APFS container.
2. **m1n1 stage 1 & 2**: Initializes Apple SoC clocks, memory controller, display controller, and hands over device trees.
3. **U-Boot**: Implements standard UEFI interfaces (`EFI_SYSTEM_TABLE`, runtime services).
4. **systemd-boot**: Official, zero-patch bootloader compliant with UEFI specifications.
5. **Kernel (linux-asahi)**: Kernel compiled for aarch64 with 16k page size support (standard Apple Silicon MMU) or 4k pages.

## 2. Desktop Environment: KDE Plasma 6 on Wayland
- **Compositor**: KWin Wayland (`kwin_wayland`).
- **Drivers**: Mesa with Asahi AGX Honeykrisp driver providing full Vulkan 1.3 and OpenGL 4.6 acceleration.
- **Fractional Scaling**: Configured natively via KWin Wayland display settings (recommended 150% or 175% on 13"/14"/16" Retina displays).
- **DSP Audio**: Handled by PipeWire + WirePlumber + `asahi-audio` profiles. This is critical to prevent hardware speaker damage on MacBook Pro/Air.

## 3. High Performance Optimization Daemons (Rust)
- **asahi-fan-control**: Reads `/sys/class/hwmon` endpoints and modulates fan speed with hysteresis to eliminate noisy fan ramps while preventing thermal throttling.
- **asahi-power-profile**: Controls dynamic frequency switching between Efficiency Cores and Performance Cores through `energy_performance_preference`.
- **asahi-battery-limiter**: Applies a safety charging limit (default 80%) using `charge_control_end_threshold`.

