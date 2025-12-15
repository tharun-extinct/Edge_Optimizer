use libloading::Library;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use thiserror::Error;

use crate::driver::{DriverError, FanDriver};
use crate::model::{FanCapabilities, FanProfile, FanTelemetry, ProfileId};

/// HP-specific thermal policy profile IDs based on HP OMEN Command Center behavior.
/// These map to thermal profiles exposed by HP's native APIs.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HpThermalProfile {
    Default = 0,
    Performance = 1,
    Cool = 2,
    Quiet = 3,
    Extreme = 4,  // Max fan speed profile
}

impl HpThermalProfile {
    fn to_profile_id(self) -> ProfileId {
        self as ProfileId
    }

    fn to_fan_profile(self) -> FanProfile {
        match self {
            Self::Default => FanProfile {
                id: self.to_profile_id(),
                name: "Default".into(),
                is_maximum: false,
            },
            Self::Performance => FanProfile {
                id: self.to_profile_id(),
                name: "Performance".into(),
                is_maximum: false,
            },
            Self::Cool => FanProfile {
                id: self.to_profile_id(),
                name: "Cool".into(),
                is_maximum: false,
            },
            Self::Quiet => FanProfile {
                id: self.to_profile_id(),
                name: "Quiet".into(),
                is_maximum: false,
            },
            Self::Extreme => FanProfile {
                id: self.to_profile_id(),
                name: "Extreme (Max)".into(),
                is_maximum: true,
            },
        }
    }

    fn all_profiles() -> Vec<FanProfile> {
        vec![
            Self::Default.to_fan_profile(),
            Self::Performance.to_fan_profile(),
            Self::Cool.to_fan_profile(),
            Self::Quiet.to_fan_profile(),
            Self::Extreme.to_fan_profile(),
        ]
    }
}

#[derive(Debug, Error)]
pub enum HpDriverError {
    #[error("NativeRpcClient.dll not found or failed to load: {0}")]
    DllNotFound(String),
    #[error("function not found in DLL: {0}")]
    FunctionNotFound(String),
    #[error("rpc call failed: {0}")]
    RpcFailed(String),
}

impl From<HpDriverError> for DriverError {
    fn from(e: HpDriverError) -> Self {
        match e {
            HpDriverError::DllNotFound(msg) => DriverError::NotReady(msg),
            HpDriverError::FunctionNotFound(msg) => DriverError::Unsupported(msg),
            HpDriverError::RpcFailed(msg) => DriverError::Internal(msg),
        }
    }
}

/// FFI function signatures for NativeRpcClient.dll (inferred from HP OMEN ecosystem).
/// Actual signatures may vary; these are best-effort based on typical HP RPC patterns.
type SetThermalProfileFn = unsafe extern "C" fn(profile: u32) -> i32;
type GetThermalProfileFn = unsafe extern "C" fn() -> i32;
type GetFanSpeedFn = unsafe extern "C" fn() -> i32;
type GetSystemTemperatureFn = unsafe extern "C" fn() -> i32;

static HP_LIBRARY: Lazy<Mutex<Option<Library>>> = Lazy::new(|| Mutex::new(None));

fn load_hp_library() -> Result<(), HpDriverError> {
    let mut guard = HP_LIBRARY.lock().unwrap();
    if guard.is_some() {
        return Ok(());
    }

    // Try loading from system directories or HP OMEN install path
    let possible_paths = [
        "NativeRpcClient.dll",
        "C:\\Program Files\\HP\\OMEN\\NativeRpcClient.dll",
        "C:\\Program Files (x86)\\HP\\OMEN\\NativeRpcClient.dll",
        "C:\\Program Files\\OMEN\\NativeRpcClient.dll",
    ];

    for path in &possible_paths {
        match unsafe { Library::new(path) } {
            Ok(lib) => {
                *guard = Some(lib);
                return Ok(());
            }
            Err(_) => continue,
        }
    }

    Err(HpDriverError::DllNotFound(
        "NativeRpcClient.dll not found in known HP OMEN paths. Ensure HP OMEN Command Center is installed.".into()
    ))
}

/// HP-specific driver implementation using NativeRpcClient.dll.
pub struct HpOmenDriver;

impl HpOmenDriver {
    pub fn new() -> Result<Self, HpDriverError> {
        load_hp_library()?;
        Ok(Self)
    }

    fn call_set_thermal_profile(&self, profile: u32) -> Result<(), HpDriverError> {
        let guard = HP_LIBRARY.lock().unwrap();
        let lib = guard.as_ref().ok_or_else(|| {
            HpDriverError::DllNotFound("library not loaded".into())
        })?;

        // Try multiple common function name patterns
        let function_names: &[&[u8]] = &[
            b"SetThermalProfile\0",
            b"HP_SetThermalProfile\0",
            b"SetFanProfile\0",
            b"OMEN_SetThermalProfile\0",
        ];

        for name in function_names {
            if let Ok(func) = unsafe { lib.get::<SetThermalProfileFn>(name) } {
                let result = unsafe { func(profile) };
                if result == 0 {
                    return Ok(());
                } else {
                    return Err(HpDriverError::RpcFailed(format!("return code: {}", result)));
                }
            }
        }

        Err(HpDriverError::FunctionNotFound(
            "SetThermalProfile or similar not found in NativeRpcClient.dll".into()
        ))
    }

    fn call_get_thermal_profile(&self) -> Result<i32, HpDriverError> {
        let guard = HP_LIBRARY.lock().unwrap();
        let lib = guard.as_ref().ok_or_else(|| {
            HpDriverError::DllNotFound("library not loaded".into())
        })?;

        let function_names: &[&[u8]] = &[
            b"GetThermalProfile\0",
            b"HP_GetThermalProfile\0",
            b"GetFanProfile\0",
        ];

        for name in function_names {
            if let Ok(func) = unsafe { lib.get::<GetThermalProfileFn>(name) } {
                let result = unsafe { func() };
                if result >= 0 {
                    return Ok(result);
                }
            }
        }

        Err(HpDriverError::FunctionNotFound(
            "GetThermalProfile or similar not found".into()
        ))
    }
}

impl FanDriver for HpOmenDriver {
    fn capabilities(&self) -> Result<FanCapabilities, DriverError> {
        // Verify we can load the library
        load_hp_library().map_err(|e: HpDriverError| DriverError::from(e))?;
        
        Ok(FanCapabilities {
            supported: true,
            profiles: HpThermalProfile::all_profiles(),
        })
    }

    fn current_profile(&self) -> Result<Option<ProfileId>, DriverError> {
        match self.call_get_thermal_profile() {
            Ok(profile_id) => Ok(Some(profile_id as ProfileId)),
            Err(_) => Ok(None),
        }
    }

    fn set_profile(&self, profile: ProfileId) -> Result<(), DriverError> {
        self.call_set_thermal_profile(profile)
            .map_err(|e| e.into())
    }

    fn telemetry(&self) -> Result<FanTelemetry, DriverError> {
        // Best-effort telemetry; HP DLL may not expose these
        let guard = HP_LIBRARY.lock().unwrap();
        let lib = guard.as_ref().ok_or_else(|| {
            DriverError::NotReady("library not loaded".into())
        })?;

        let mut rpm = None;
        let mut temp = None;

        // Try to get fan RPM
        if let Ok(func) = unsafe { lib.get::<GetFanSpeedFn>(b"GetFanSpeed\0") } {
            let speed = unsafe { func() };
            if speed >= 0 {
                rpm = Some(speed as u32);
            }
        }

        // Try to get temperature
        if let Ok(func) = unsafe { lib.get::<GetSystemTemperatureFn>(b"GetSystemTemperature\0") } {
            let t = unsafe { func() };
            if t >= 0 {
                temp = Some(t as f32);
            }
        }

        Ok(FanTelemetry {
            rpm,
            temperature_c: temp,
            timestamp: std::time::Instant::now(),
        })
    }
}
