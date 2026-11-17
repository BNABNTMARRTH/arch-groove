use anyhow::{Context, Result};
use log::{info, warn};
use std::fs;
use std::path::{Path, PathBuf};
use std::thread::sleep;
use std::time::Duration;

#[derive(Debug)]
pub struct ThermalSensor {
    pub name: String,
    pub path: PathBuf,
}

impl ThermalSensor {
    pub fn read_temp_celsius(&self) -> Result<f32> {
        let content = fs::read_to_string(&self.path)
            .with_context(|| format!("Failed to read temperature from {:?}", self.path))?;
        let millidegrees: f32 = content.trim().parse()
            .with_context(|| format!("Failed to parse temperature from {:?}", self.path))?;
        Ok(millidegrees / 1000.0)
    }
}

pub struct FanController {
    pwm_target_path: Option<PathBuf>,
}

impl FanController {
    pub fn new() -> Self {
        let pwm_target_path = Self::find_fan_pwm();
        Self { pwm_target_path }
    }

    fn find_fan_pwm() -> Option<PathBuf> {
        let hwmon_base = Path::new("/sys/class/hwmon");
        if !hwmon_base.exists() {
            return None;
        }

        if let Ok(entries) = fs::read_dir(hwmon_base) {
            for entry in entries.flatten() {
                let path = entry.path();
                let fan1_target = path.join("fan1_target");
                if fan1_target.exists() {
                    return Some(fan1_target);
                }
                let pwm1 = path.join("pwm1");
                if pwm1.exists() {
                    return Some(pwm1);
                }
            }
        }
        None
    }

    pub fn set_speed_rpm(&self, rpm: u32) -> Result<()> {
        if let Some(ref path) = self.pwm_target_path {
            fs::write(path, rpm.to_string())
                .with_context(|| format!("Failed to write fan RPM {} to {:?}", rpm, path))?;
        } else {
            warn!("No controllable fan interface found in /sys/class/hwmon (Mac may be passively cooled like MacBook Air)");
        }
        Ok(())
    }
}

pub fn discover_sensors() -> Vec<ThermalSensor> {
    let mut sensors = Vec::new();
    let hwmon_base = Path::new("/sys/class/hwmon");
    if !hwmon_base.exists() {
        return sensors;
    }

    if let Ok(entries) = fs::read_dir(hwmon_base) {
        for entry in entries.flatten() {
            let dir = entry.path();
            let name = fs::read_to_string(dir.join("name"))
                .unwrap_or_else(|_| "unknown_hwmon".to_string())
                .trim()
                .to_string();

            if let Ok(hwmon_entries) = fs::read_dir(&dir) {
                for file_entry in hwmon_entries.flatten() {
                    let file_name = file_entry.file_name().to_string_lossy().to_string();
                    if file_name.starts_with("temp") && file_name.ends_with("_input") {
                        sensors.push(ThermalSensor {
                            name: format!("{}:{}", name, file_name),
                            path: file_entry.path(),
                        });
                    }
                }
            }
        }
    }
    sensors
}

pub fn calculate_target_fan_rpm(temp: f32) -> u32 {
    if temp < 55.0 {
        0 // Silent / passive mode under low workload
    } else if temp < 70.0 {
        1800 // Gentle cooling
    } else if temp < 85.0 {
        3200 // Moderate cooling
    } else {
        5200 // Maximum safety cooling
    }
}

fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    info!("Starting asahi-fan-control daemon for Apple Silicon (Rust native)");

    let fan = FanController::new();
    let sensors = discover_sensors();
    info!("Discovered {} thermal sensors", sensors.len());

    let is_daemon = std::env::var("DAEMON_ONCE").is_err();

    loop {
        let mut max_temp: f32 = 0.0;
        for sensor in &sensors {
            if let Ok(temp) = sensor.read_temp_celsius() {
                if temp > max_temp && temp < 125.0 {
                    max_temp = temp;
                }
            }
        }

        let target_rpm = calculate_target_fan_rpm(max_temp);
        if max_temp > 0.0 {
            info!("Max SoC/CPU temperature: {:.1}°C -> Target RPM: {}", max_temp, target_rpm);
            let _ = fan.set_speed_rpm(target_rpm);
        }

        if !is_daemon {
            break;
        }
        sleep(Duration::from_secs(3));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_target_fan_rpm() {
        assert_eq!(calculate_target_fan_rpm(40.0), 0);
        assert_eq!(calculate_target_fan_rpm(60.0), 1800);
        assert_eq!(calculate_target_fan_rpm(75.0), 3200);
        assert_eq!(calculate_target_fan_rpm(90.0), 5200);
    }
}

