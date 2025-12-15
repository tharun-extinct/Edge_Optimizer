use edge_optimizer::manager::FanManager;
use edge_optimizer::model::MaxFanPolicy;

#[cfg(windows)]
use edge_optimizer::hp::HpOmenDriver;

#[cfg(windows)]
use edge_optimizer::hp_wmi::HpWmiDriver;

#[cfg(windows)]
use edge_optimizer::windows_power::WindowsPowerPlanDriver;

#[cfg(not(windows))]
use edge_optimizer::driver::UnsupportedDriver;

#[cfg(windows)]
fn is_elevated() -> bool {
    use windows::Win32::Security::{GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY};
    use windows::Win32::Foundation::{CloseHandle, HANDLE};
    use windows::Win32::System::Threading::GetCurrentProcess;
    use windows::Win32::System::Threading::OpenProcessToken;
    use std::mem;

    unsafe {
        let mut token: HANDLE = HANDLE::default();
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token).is_ok() {
            let mut elevation = TOKEN_ELEVATION { TokenIsElevated: 0 };
            let mut size = 0u32;
            let result = GetTokenInformation(
                token,
                TokenElevation,
                Some(&mut elevation as *mut _ as *mut _),
                mem::size_of::<TOKEN_ELEVATION>() as u32,
                &mut size,
            );
            let _ = CloseHandle(token);
            result.is_ok() && elevation.TokenIsElevated != 0
        } else {
            false
        }
    }
}

#[cfg(windows)]
fn elevate_and_restart() -> ! {
    use std::env;
    use std::process::Command;

    println!("⚠ Administrator privileges required for HP WMI BIOS access");
    println!("→ Attempting to restart with elevation...\n");

    let exe_path = env::current_exe().expect("Failed to get exe path");
    
    let status = Command::new("powershell")
        .args(&[
            "-NoProfile",
            "-Command",
            &format!("Start-Process -FilePath '{}' -Verb RunAs -Wait", exe_path.display()),
        ])
        .status();

    match status {
        Ok(_) => std::process::exit(0),
        Err(e) => {
            eprintln!("✗ Failed to elevate: {}", e);
            eprintln!("Please run this program as Administrator manually.");
            std::process::exit(1);
        }
    }
}

fn main() {
    println!("Edge Optimizer - Max Fan Control");
    println!("=================================\n");

    #[cfg(windows)]
    if !is_elevated() {
        elevate_and_restart();
    }

    #[cfg(windows)]
    {
        // Try HP WMI BIOS method first (works on HP Victus/OMEN)
        match HpWmiDriver::new() {
            Ok(driver) => {
                println!("✓ Using HP WMI BIOS thermal control\n");
                run_with_driver(driver);
            }
            Err(wmi_err) => {
                println!("⚠ HP WMI not available: {}", wmi_err);
                
                // Try HP-specific DLL driver
                match HpOmenDriver::new() {
                    Ok(driver) => {
                        println!("✓ Using HP thermal control (NativeRpcClient.dll)\n");
                        run_with_driver(driver);
                    }
                    Err(hp_err) => {
                        println!("⚠ HP DLL control not available: {}", hp_err);
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
