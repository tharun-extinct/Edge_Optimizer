use edge_optimizer::manager::FanManager;
use edge_optimizer::model::MaxFanPolicy;

#[cfg(windows)]
use edge_optimizer::hp::HpOmenDriver;

#[cfg(windows)]
use edge_optimizer::windows_power::WindowsPowerPlanDriver;

#[cfg(not(windows))]
use edge_optimizer::driver::UnsupportedDriver;

fn main() {
    println!("Edge Optimizer - Max Fan Control");
    println!("=================================\n");

    #[cfg(windows)]
    {
        // Try HP-specific driver first
        match HpOmenDriver::new() {
            Ok(driver) => {
                println!("✓ Using HP thermal control (NativeRpcClient.dll)\n");
                run_with_driver(driver);
            }
            Err(hp_err) => {
                println!("⚠ HP thermal control not available: {}", hp_err);
                println!("→ Falling back to Windows Power Plans\n");
                
                match WindowsPowerPlanDriver::new() {
                    Ok(driver) => run_with_driver(driver),
                    Err(e) => {
                        eprintln!("✗ Failed to initialize any driver: {e}");
                        std::process::exit(1);
                    }
                }
            }
        }
    }

    #[cfg(not(windows))]
    {
        eprintln!("✗ Only Windows is supported");
        std::process::exit(1);
    }
}

#[cfg(windows)]
fn run_with_driver<D: edge_optimizer::driver::FanDriver>(driver: D) {
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
