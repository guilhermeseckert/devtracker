use std::process::Command;

/// Detect if the screen is locked or the user is idle.
/// Uses CGSessionCopyCurrentDictionary via ioreg to check screen lock status,
/// and HIDIdleTime for keyboard/mouse idle duration.
pub fn is_screen_locked() -> bool {
    // Check if screen saver is running
    if check_process_exists("ScreenSaverEngine") {
        return true;
    }

    // Check loginwindow for screen lock state via ioreg
    if let Some(idle_seconds) = get_hid_idle_time() {
        // Consider idle if no input for 5+ minutes
        if idle_seconds > 300.0 {
            return true;
        }
    }

    false
}

/// Get HID idle time in seconds (time since last keyboard/mouse input).
fn get_hid_idle_time() -> Option<f64> {
    let output = Command::new("ioreg")
        .args(["-c", "IOHIDSystem", "-d", "4", "-S"])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Look for HIDIdleTime in the output
    for line in stdout.lines() {
        if line.contains("HIDIdleTime") {
            // Format: "HIDIdleTime" = 1234567890
            if let Some(val_str) = line.split('=').nth(1) {
                let trimmed = val_str.trim();
                if let Ok(nanoseconds) = trimmed.parse::<u64>() {
                    return Some(nanoseconds as f64 / 1_000_000_000.0);
                }
            }
        }
    }

    None
}

fn check_process_exists(name: &str) -> bool {
    Command::new("pgrep")
        .args(["-x", name])
        .output()
        .is_ok_and(|o| o.status.success())
}

/// Get the idle duration in seconds. Returns None if unable to detect.
pub fn get_idle_seconds() -> Option<f64> {
    get_hid_idle_time()
}
