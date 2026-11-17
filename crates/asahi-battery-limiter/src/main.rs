use anyhow::{Context, Result};
use log::{info, warn};
use std::fs;
use std::path::{Path, PathBuf};

pub struct BatteryLimiter {
    threshold_paths: Vec<PathBuf>,
}

impl BatteryLimiter {
    pub fn new() -> Self {
        let mut threshold_paths = Vec::new();
        let ps_base = Path::new("/sys/class/power_supply");
        if ps_base.exists() {
            if let Ok(entries) = fs::read_dir(ps_base) {
                for entry in entries.flatten() {
                    let dir = entry.path();
                    let name = entry.file_name().to_string_lossy().to_string();
                    if name.starts_with("macsmc-battery") || name.starts_with("BAT") {
                        let end_thresh = dir.join("charge_control_end_threshold");
                        if end_thresh.exists() {
                            threshold_paths.push(end_thresh);
                        }
                    }
                }
            }
        }
        Self { threshold_paths }
    }

    pub fn set_limit_percentage(&self, limit: u8) -> Result<()> {
        if limit < 40 || limit > 100 {
            anyhow::bail!("Battery limit percentage must be between 40 and 100, received: {}", limit);
        }

        if self.threshold_paths.is_empty() {
            warn!("No battery charge threshold control sysfs node found on this system.");
            return Ok(());
        }

        for path in &self.threshold_paths {
            info!("Writing charge threshold {}% to {:?}", limit, path);
            fs::write(path, limit.to_string())
                .with_context(|| format!("Failed writing charge limit to {:?}", path))?;
        }

        Ok(())
    }
}

fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    info!("Starting asahi-battery-limiter daemon/cli (Rust native)");

    let args: Vec<String> = std::env::args().collect();
    let limit: u8 = if args.len() > 1 {
        args[1].parse().unwrap_or(80)
    } else {
        80 // Safe battery-preservation default for Mac laptops
    };

    let limiter = BatteryLimiter::new();
    limiter.set_limit_percentage(limit)?;
    info!("Battery charge threshold configuration completed.");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_battery_limiter_range_check() {
        let limiter = BatteryLimiter { threshold_paths: vec![] };
        assert!(limiter.set_limit_percentage(80).is_ok());
        assert!(limiter.set_limit_percentage(100).is_ok());
        assert!(limiter.set_limit_percentage(30).is_err());
        assert!(limiter.set_limit_percentage(105).is_err());
    }
}

