use anyhow::{Context, Result};
use log::{info, warn};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerProfile {
    PowerSave,
    Balanced,
    Performance,
}

impl PowerProfile {
    pub fn as_epp(&self) -> &'static str {
        match self {
            PowerProfile::PowerSave => "power",
            PowerProfile::Balanced => "balance_performance",
            PowerProfile::Performance => "performance",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().trim() {
            "powersave" | "power" | "battery" => Some(PowerProfile::PowerSave),
            "balanced" | "balance" | "default" => Some(PowerProfile::Balanced),
            "performance" | "perf" | "turbo" => Some(PowerProfile::Performance),
            _ => None,
        }
    }
}

pub struct CpuGovernorManager {
    cpu_dirs: Vec<PathBuf>,
}

impl CpuGovernorManager {
    pub fn new() -> Self {
        let mut cpu_dirs = Vec::new();
        let cpu_base = Path::new("/sys/devices/system/cpu");
        if cpu_base.exists() {
            if let Ok(entries) = fs::read_dir(cpu_base) {
                for entry in entries.flatten() {
                    let file_name = entry.file_name().to_string_lossy().to_string();
                    if file_name.starts_with("cpu") && file_name[3..].chars().all(|c| c.is_ascii_digit()) {
                        cpu_dirs.push(entry.path());
                    }
                }
            }
        }
        Self { cpu_dirs }
    }

    pub fn apply_profile(&self, profile: PowerProfile) -> Result<()> {
        let epp_str = profile.as_epp();
        info!("Applying power profile {:?} (EPP: {}) across {} CPUs", profile, epp_str, self.cpu_dirs.len());

        let mut applied_count = 0;
        for cpu in &self.cpu_dirs {
            let cpufreq_dir = cpu.join("cpufreq");
            if !cpufreq_dir.exists() {
                continue;
            }

            let epp_file = cpufreq_dir.join("energy_performance_preference");
            if epp_file.exists() {
                if let Err(e) = fs::write(&epp_file, epp_str) {
                    warn!("Failed writing to {:?}: {}", epp_file, e);
                } else {
                    applied_count += 1;
                }
            }

            let scaling_gov = cpufreq_dir.join("scaling_governor");
            if scaling_gov.exists() {
                let gov = match profile {
                    PowerProfile::PowerSave => "powersave",
                    PowerProfile::Balanced | PowerProfile::Performance => "schedutil",
                };
                let _ = fs::write(&scaling_gov, gov);
            }
        }

        info!("Profile successfully updated on {} core endpoints", applied_count);
        Ok(())
    }
}

fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    info!("Starting asahi-power-profile utility for Apple Silicon");

    let args: Vec<String> = std::env::args().collect();
    let profile = if args.len() > 1 {
        PowerProfile::from_str(&args[1]).unwrap_or(PowerProfile::Balanced)
    } else {
        PowerProfile::Balanced
    };

    let manager = CpuGovernorManager::new();
    manager.apply_profile(profile).context("Failed applying CPU power profile")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_profile_parsing() {
        assert_eq!(PowerProfile::from_str("battery"), Some(PowerProfile::PowerSave));
        assert_eq!(PowerProfile::from_str("balanced"), Some(PowerProfile::Balanced));
        assert_eq!(PowerProfile::from_str("performance"), Some(PowerProfile::Performance));
        assert_eq!(PowerProfile::from_str("invalid"), None);
    }

    #[test]
    fn test_power_profile_epp() {
        assert_eq!(PowerProfile::PowerSave.as_epp(), "power");
        assert_eq!(PowerProfile::Balanced.as_epp(), "balance_performance");
        assert_eq!(PowerProfile::Performance.as_epp(), "performance");
    }
}

