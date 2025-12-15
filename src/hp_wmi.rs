//! HP WMI BIOS-based thermal control for HP Victus and OMEN systems.
//! Calls PowerShell script that uses the hpqBIntM WMI class.

use std::process::Command;
use crate::driver::{DriverError, FanDriver};
use crate::model::{FanCapabilities, FanProfile, FanTelemetry, ProfileId};

pub struct HpWmiDriver {
    script_path: String,
}

impl HpWmiDriver {
    pub fn new() -> Result<Self, DriverError> {
        // Check if we're on an HP system by looking for the WMI class
        let check = Command::new("powershell")
            .args(&["-NoProfile", "-Command", 
                    "Get-CimInstance -ClassName 'hpqBIntM' -Namespace 'root\\wmi' -ErrorAction SilentlyContinue | Select-Object -First 1"])
            .output();

        match check {
            Ok(output) if output.status.success() && !output.stdout.is_empty() => {
                Ok(Self {
                    script_path: ".\\SetMaxFan.ps1".to_string(),
                })
            }
            _ => Err(DriverError::NotReady(
                "hpqBIntM WMI class not found - not an HP OMEN/Victus system?".into()
            ))
        }
    }

    fn run_script(&self, script_name: &str) -> Result<(), DriverError> {
        let output = Command::new("powershell")
            .args(&[
                "-NoProfile",
                "-ExecutionPolicy", "Bypass",
                "-File", script_name
            ])
            .output()
            .map_err(|e| DriverError::Internal(format!("Failed to run PowerShell: {}", e)))?;

        if output.status.success() {
            Ok(())
        } else {
            Err(DriverError::Internal(format!(
                "Script failed: {}",
                String::from_utf8_lossy(&output.stderr)
            )))
        }
    }
}

impl FanDriver for HpWmiDriver {
    fn capabilities(&self) -> Result<FanCapabilities, DriverError> {
        Ok(FanCapabilities {
            supported: true,
            profiles: vec![
                FanProfile {
                    id: 0,
                    name: "Normal (Default)".into(),
                    is_maximum: false,
                },
                FanProfile {
                    id: 1,
                    name: "Max Fan (Performance)".into(),
                    is_maximum: true,
                },
            ],
        })
    }

    fn current_profile(&self) -> Result<Option<ProfileId>, DriverError> {
        Ok(None)
    }

    fn set_profile(&self, profile: ProfileId) -> Result<(), DriverError> {
        let script = if profile == 1 {
            "SetMaxFan.ps1"
        } else {
            "SetNormalFan.ps1"
        };
        
        self.run_script(script)
    }

    fn telemetry(&self) -> Result<FanTelemetry, DriverError> {
        Err(DriverError::Unsupported("telemetry not available via WMI BIOS".into()))
    }
}
