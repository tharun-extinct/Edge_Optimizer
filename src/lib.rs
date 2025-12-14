//! Core fan control logic (no UI). Relies on a driver that safely exposes ACPI fan profile switches.

pub mod model;
pub mod driver;
pub mod manager;
