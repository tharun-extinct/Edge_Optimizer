use edge_optimizer::manager::FanManager;
use edge_optimizer::model::MaxFanPolicy;

#[cfg(windows)]
use edge_optimizer::hp::HpOmenDriver;

#[cfg(not(windows))]
use edge_optimizer::driver::UnsupportedDriver;

fn main() {
    println!("Edge Optimizer - Max Fan Control");
    println!("=================================\n");

    #[cfg(windows)]
    let driver_result = HpOmenDriver::new();

    #[cfg(not(windows))]
    let driver_result: Result<UnsupportedDriver, _> = Ok(UnsupportedDriver);

    match driver_result {
        Ok(driver) => {
            let policy = MaxFanPolicy::default();
            let manager = FanManager::new(driver, policy);

            // Show capabilities
            match manager.capabilities() {
                Ok(caps) => {
                    if caps.supported {
                        println!("✓ Fan control supported");
                        println!("Available profiles:");
                        for profile in &caps.profiles {
                            let marker = if profile.is_maximum { " (MAX)" } else { "" };
                            println!("  - {} (id={}){}",  profile.name, profile.id, marker);
                        }
                        println!();
                    } else {
                        eprintln!("✗ Fan control not supported on this platform");
                        std::process::exit(1);
                    }
                }
                Err(e) => {
                    eprintln!("Failed to query capabilities: {e}");
                    std::process::exit(1);
                }
            }

            // Set max fan profile
            println!("Setting fan to maximum...");
            match manager.set_max_profile() {
                Ok(id) => {
                    println!("✓ Successfully set fan profile to max (id={id})");
                    
                    // Try to read telemetry
                    if let Ok(telemetry) = manager.telemetry() {
                        println!("\nCurrent telemetry:");
                        if let Some(rpm) = telemetry.rpm {
                            println!("  Fan speed: {} RPM", rpm);
                        }
                        if let Some(temp) = telemetry.temperature_c {
                            println!("  Temperature: {:.1}°C", temp);
                        }
                    }
                }
                Err(err) => {
                    eprintln!("✗ Failed to set max fan profile: {err}");
                    std::process::exit(1);
                }
            }
        }
        Err(err) => {
            eprintln!("✗ Failed to initialize HP OMEN driver: {err}");
            eprintln!("\nTroubleshooting:");
            eprintln!("  1. Ensure you are running on an HP OMEN system");
            eprintln!("  2. Install HP OMEN Command Center (includes NativeRpcClient.dll)");
            eprintln!("  3. Run this program with administrator privileges");
            std::process::exit(1);
        }
    }
}
