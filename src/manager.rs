use std::thread;
use crate::driver::{DriverError, FanDriver};
use crate::model::{FanCapabilities, FanTelemetry, MaxFanPolicy, ProfileId};

/// Orchestrates safe fan profile changes with optional verification.
pub struct FanManager<D: FanDriver> {
    driver: D,
    policy: MaxFanPolicy,
}

impl<D: FanDriver> FanManager<D> {
    pub fn new(driver: D, policy: MaxFanPolicy) -> Self {
        Self { driver, policy }
    }

    /// Attempt to set the max-capable profile (is_maximum = true) if available.
    /// Returns the profile id that was applied.
    pub fn set_max_profile(&self) -> Result<ProfileId, DriverError> {
        let caps = self.driver.capabilities()?;
        if !caps.supported {
            return Err(DriverError::Unsupported("fan profiles not supported on this platform".into()));
        }

        let Some(profile) = Self::select_max_profile(&caps) else {
            return Err(DriverError::Unsupported("no maximum profile declared".into()));
        };

        let before = self.driver.telemetry().ok();
        self.driver.set_profile(profile.id)?;

        // Optional verification: wait and check telemetry if available.
        thread::sleep(self.policy.settle_timeout);
        if let (Some(before), Some(after)) = (before, self.driver.telemetry().ok()) {
            if let Some(min_delta) = self.policy.min_rpm_delta {
                if let (Some(rpm_before), Some(rpm_after)) = (before.rpm, after.rpm) {
                    if rpm_after + 5 < rpm_before + min_delta {
                        return Err(DriverError::Internal("fan did not ramp to expected max".into()));
                    }
                }
            }
            if let Some(max_temp) = self.policy.max_safe_temp_c {
                if let Some(temp) = after.temperature_c {
                    if temp > max_temp {
                        return Err(DriverError::Internal("temperature exceeded safety threshold after profile change".into()));
                    }
                }
            }
        }

        Ok(profile.id)
    }

    /// List capabilities.
    pub fn capabilities(&self) -> Result<FanCapabilities, DriverError> {
        self.driver.capabilities()
    }

    /// Best-effort read of telemetry.
    pub fn telemetry(&self) -> Result<FanTelemetry, DriverError> {
        self.driver.telemetry()
    }

    fn select_max_profile(caps: &FanCapabilities) -> Option<&crate::model::FanProfile> {
        caps.profiles.iter().find(|p| p.is_maximum)
    }
}
