# Edge Optimizer Fan Control (Logic Only)

This crate wires the fan-profile logic without any UI. It assumes a minimal, signed kernel driver that exposes whitelisted ACPI fan-profile methods per OEM. No EC register pokes are used.

## Components
- `driver::FanDriver`: trait boundary to the kernel/ACPI-facing driver. Includes capability discovery, current profile, set profile, and telemetry.
- `manager::FanManager`: orchestrates selecting the max profile, applies safety policy, optionally verifies ramp via telemetry, and surfaces errors.
- `model`: data types for profiles, capabilities, telemetry, and safety policy.

## Safety and Scope
- Only OEM-whitelisted ACPI profile methods should be invoked by the driver; unknown platforms must return `Unsupported`.
- The Rust core performs best-effort verification (RPM delta, temperature threshold) when telemetry exists; otherwise it still defers safety to the driver.
- No UI and no persistence are included; this is a library for higher layers to call.

## Usage Sketch
```rust
use edge_optimizer::{driver::UnsupportedDriver, manager::FanManager, model::MaxFanPolicy};

let driver = UnsupportedDriver; // replace with real driver impl
let policy = MaxFanPolicy::default();
let mgr = FanManager::new(driver, policy);
let _ = mgr.set_max_profile();
```

## Next Steps
- Implement the actual Windows KMDF driver that issues ACPI method evals from a whitelist.
- Populate an OEM capability database mapping DMI → ACPI method + profile ids.
- Expose a small service/CLI that calls this crate and reports status to the user.
