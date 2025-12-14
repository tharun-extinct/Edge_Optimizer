use crate::model::{FanCapabilities, FanTelemetry, ProfileId};
use thiserror::Error;

/// Errors surfaced by the driver boundary.
#[derive(Debug, Error)]
pub enum DriverError {
    #[error("driver not ready: {0}")]
    NotReady(String),
    #[error("unsupported operation: {0}")]
    Unsupported(String),
    #[error("io failure: {0}")]
    Io(String),
    #[error("internal error: {0}")]
    Internal(String),
}

/// Minimal contract the kernel/ACPI-facing driver must satisfy.
pub trait FanDriver {
    /// Return the known fan capabilities for the current platform.
    fn capabilities(&self) -> Result<FanCapabilities, DriverError>;

    /// Return the currently selected profile if the platform exposes it.
    fn current_profile(&self) -> Result<Option<ProfileId>, DriverError>;

    /// Request a profile change; driver enforces whitelist and safe ACPI invocation.
    fn set_profile(&self, profile: ProfileId) -> Result<(), DriverError>;

    /// Optional: fetch current telemetry to aid verification.
    fn telemetry(&self) -> Result<FanTelemetry, DriverError>;
}

/// A no-op driver for platforms without support; allows graceful degradation.
pub struct UnsupportedDriver;

impl FanDriver for UnsupportedDriver {
    fn capabilities(&self) -> Result<FanCapabilities, DriverError> {
        Ok(FanCapabilities::unsupported())
    }

    fn current_profile(&self) -> Result<Option<ProfileId>, DriverError> {
        Ok(None)
    }

    fn set_profile(&self, _profile: ProfileId) -> Result<(), DriverError> {
        Err(DriverError::Unsupported("fan profiles not exposed".into()))
    }

    fn telemetry(&self) -> Result<FanTelemetry, DriverError> {
        Err(DriverError::Unsupported("telemetry not exposed".into()))
    }
}
