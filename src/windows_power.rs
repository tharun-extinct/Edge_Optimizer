use std::process::Command;
use crate::driver::{DriverError, FanDriver};
use crate::model::{FanCapabilities, FanProfile, FanTelemetry, ProfileId};

/// Windows power plan based fallback driver for systems where direct thermal control isn't available.
/// Maps fan profiles to Windows power plans: Power Saver (quiet), Balanced (default), High Performance (max).
pub struct WindowsPowerPlanDriver;

impl WindowsPowerPlanDriver {
    pub fn new() -> Result<Self, DriverError> {
        // Verify powercfg is available
        match Command::new("powercfg").arg("/?").output() {
            Ok(_) => Ok(Self),
            Err(e) => Err(DriverError::NotReady(format!("powercfg not available: {}", e))),
        }
    }

    fn set_power_plan(&self, guid: &str) -> Result<(), DriverError> {
        let output = Command::new("powercfg")
            .args(&["/setactive", guid])
            .output()
            .map_err(|e| DriverError::Io(format!("Failed to run powercfg: {}", e)))?;

        if output.status.success() {
            Ok(())
        } else {
            Err(DriverError::Internal(format!(
                "powercfg failed: {}",
                String::from_utf8_lossy(&output.stderr)
            )))
        }
    }
}

impl FanDriver for WindowsPowerPlanDriver {
    fn capabilities(&self) -> Result<FanCapabilities, DriverError> {
        Ok(FanCapabilities {
            supported: true,
            profiles: vec![
                FanProfile {
                    id: 0,
                    name: "Balanced (Default)".into(),
                    is_maximum: false,
                },
                FanProfile {
                    id: 1,
                    name: "High Performance (Max)".into(),
                    is_maximum: true,
                },
                FanProfile {
                    id: 2,
                    name: "Power Saver (Quiet)".into(),
                    is_maximum: false,
                },
            ],
        })
    }

    fn current_profile(&self) -> Result<Option<ProfileId>, DriverError> {
        // Could parse powercfg /getactivescheme but return None for simplicity
        Ok(None)
    }

    fn set_profile(&self, profile: ProfileId) -> Result<(), DriverError> {
        let guid = match profile {
            0 => "381b4222-f694-41f0-9685-ff5bb260df2e", // Balanced
            1 => "8c5e7fda-e8bf-4a96-9a85-a6e23a8c635c", // High Performance
            2 => "a1841308-3541-4fab-bc81-f71556f20b4a", // Power Saver
            _ => return Err(DriverError::Unsupported(format!("Unknown profile: {}", profile))),
        };

        self.set_power_plan(guid)
    }

    fn telemetry(&self) -> Result<FanTelemetry, DriverError> {
        Err(DriverError::Unsupported("telemetry not available via power plans".into()))
    }
}
