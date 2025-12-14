use std::time::Duration;

/// Logical fan profile identifier defined by OEM whitelist.
pub type ProfileId = u32;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FanProfile {
    /// Identifier passed to the driver; must map to a whitelisted ACPI argument.
    pub id: ProfileId,
    /// Human-friendly name (e.g., "max", "turbo", "balanced").
    pub name: String,
    /// Whether this profile is intended for maximum cooling.
    pub is_maximum: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FanCapabilities {
    pub supported: bool,
    pub profiles: Vec<FanProfile>,
}

impl FanCapabilities {
    pub fn unsupported() -> Self {
        Self {
            supported: false,
            profiles: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FanTelemetry {
    pub rpm: Option<u32>,
    pub temperature_c: Option<f32>,
    pub timestamp: std::time::Instant,
}

/// Policy configuration for setting max fan via profile selection.
#[derive(Debug, Clone)]
pub struct MaxFanPolicy {
    /// How long to wait after setting the profile before validating ramp.
    pub settle_timeout: Duration,
    /// Minimum expected RPM delta to consider the change effective (if RPM exists).
    pub min_rpm_delta: Option<u32>,
    /// Maximum allowed temperature before forcing revert (fallback safety).
    pub max_safe_temp_c: Option<f32>,
}

impl Default for MaxFanPolicy {
    fn default() -> Self {
        Self {
            settle_timeout: Duration::from_millis(1500),
            min_rpm_delta: Some(500),
            max_safe_temp_c: Some(95.0),
        }
    }
}
