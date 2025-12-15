# Edge Optimizer Fan Control

A Rust-based game optimizer that sets fan speed to maximum via OEM thermal profiles on Windows systems. Currently implements **HP OMEN and HP Victus** support through `NativeRpcClient.dll`.

## Architecture

### Safety-First Design
- **No EC register manipulation**: Uses OEM-provided APIs only
- **Profile-based control**: Sets thermal profiles (not raw fan speeds)
- **Graceful degradation**: Returns "unsupported" on unknown hardware
- **Telemetry validation**: Optional RPM/temp checks after profile changes

### Components
- `driver::FanDriver`: Abstract trait for OEM driver implementations
- `hp::HpOmenDriver`: HP OMEN implementation using NativeRpcClient.dll FFI
- `manager::FanManager`: Orchestrates profile selection with safety policy
- `model`: Data types for profiles, capabilities, telemetry, and safety policy

### HP OMEN & Victus Implementation
Uses dynamic loading of `NativeRpcClient.dll` (bundled with HP OMEN Gaming Hub or OMEN Command Center):
- **Thermal Profiles**: Default (0), Performance (1), Cool (2), Quiet (3), Extreme/Max (4)
- **DLL Search Paths**:
  - `NativeRpcClient.dll` (current directory or system PATH)
  - `C:\Program Files\HP\OMEN\`
  - `C:\Program Files (x86)\HP\OMEN\`
  - `C:\Program Files\OMEN\`
  - `C:\Program Files\HP\OMEN Gaming Hub\` (HP Victus)
  - `C:\Program Files (x86)\HP\OMEN Gaming Hub\` (HP Victus)
  - `C:\Program Files\HP\HPOmenGamingHub\`
  - `C:\Program Files (x86)\HP\HPOmenGamingHub\`
- **Function Bindings** (inferred, may vary by version):
  - `SetThermalProfile(profile: u32) -> i32`
  - `GetThermalProfile() -> i32`
  - `GetFanSpeed() -> i32` (optional telemetry)
  - `GetSystemTemperatureanonymous() -> i32` (optional telemetry)

**Note**: HP Victus laptops use the OMEN Gaming Hub (rebranded OMEN Command Center) for thermal management and share the same API.

## Requirements

### For HP OMEN & Victus Systems
1. **HP OMEN Gaming Hub** (recommended for all HP gaming laptops including Victus)
   - Available from Microsoft Store or HP website
   - Provides NativeRpcClient.dll
2. **Alternative**: HP OMEN Command Center (older OMEN systems)
3. **Administrator privileges** may be required depending on system configuration
4. **Windows 10/11** (x64)

### For Other OEMs
- Implement `FanDriver` trait for your OEM's API
- Examples: Dell Command, Lenovo Vantage, ASUS Armoury Crate
- Follow the same safety pattern: profile-based, no EC writes

## Usage

### Running the CLI
```bash
# Build release binary
cargo build --release --bin max_fan

# Run (requires HP OMEN Command Center)
.\target\release\max_fan.exe
```

### Sample Output (Success)
```
Edge Optimizer - Max Fan Control
=================================

✓ Fan control supported
Available profiles:
  - Default (id=0)
  - Performance (id=1)
  - Cool (id=2)
  - Quiet (id=3)
  - Extreme (Max) (id=4) (MAX)

Setting fan to maximum...
✓ Successfully set fan profile to max (id=4)

Current telemetry:
  Fan speed: 4800 RPM
  Temperature: 68.0°C
```

### Sample Output (Not HP OMEN/Victus)
```
Edge Optimizer - Max Fan Control
=================================

✗ Failed to initialize HP driver: NativeRpcClient.dll not found or failed to load
Troubleshooting:
  1. Ensure you are running on an HP OMEN or HP Victus system
  2. Install HP OMEN Gaming Hub (available from Microsoft Store or HP website)
  3. For older systems: Install HP OMEN Command Center
  4. Run this program with administrator privileges

Note: HP Victus laptops use the OMEN Gaming Hub for thermal management.
```

## Safety Policy

Default `MaxFanPolicy`:
- **Settle timeout**: 1500ms wait after profile change
- **Min RPM delta**: 500 RPM increase expected (if telemetry available)
- **Max safe temp**: 95°C threshold (reverts if exceeded)

Validation is best-effort; if telemetry is unavailable, the manager trusts the driver's return code.

## Code Structure
```
src/
├── lib.rs              # Crate root
├── driver.rs           # FanDriver trait + UnsupportedDriver
├── hp.rs               # HP OMEN-specific driver (Windows only)
├── manager.rs          # FanManager orchestration
├── model.rs            # Data types
└── bin/
    └── max_fan.rs      # CLI entry point
```

## Extending to Other OEMs

To add support for another manufacturer:

1. Create `src/<oem>.rs` (e.g., `src/dell.rs`, `src/lenovo.rs`)
2. Implement `FanDriver` trait:
   ```rust
   pub struct DellDriver;
   impl FanDriver for DellDriver {
       fn capabilities(&self) -> Result<FanCapabilities, DriverError> { ... }
       fn set_profile(&self, profile: ProfileId) -> Result<(), DriverError> { ... }
       // ...
   }
   ```
3. Use OEM's documented API (WMI, CLI tool, or native DLL)
4. Update `max_fan.rs` to detect and select the appropriate driver

## Known Limitations
- **HP OMEN & Victus only**: Other OEMs require additional drivers
- **Windows only**: Linux/macOS have different fan control mechanisms
- **Profile-based**: Cannot set exact RPM values, only predefined profiles
- **DLL dependency**: Requires HP OMEN Gaming Hub or OMEN Command Center to be installed

## Next Steps
1. Implement Dell/Alienware support via Dell Command | Monitor API
2. Add Lenovo Legion support via Lenovo Vantage SDK
3. Add ASUS ROG support via Armoury Crate SDK or ATKD ACPI methods
4. Add MSI Dragon Center support via MSI SDK
5. Create auto-detection logic to choose the correct driver at runtime
6. Build GUI wrapper (e.g., with `iced` or `egui`)
