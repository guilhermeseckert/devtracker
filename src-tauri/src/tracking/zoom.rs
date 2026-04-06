use std::process::Command;

/// Detect if the user is currently in a Zoom meeting.
/// Checks for Zoom audio/video processes that only run during active meetings.
pub fn is_in_meeting() -> bool {
    // Check for ZoomAudioDevice - only runs during active calls
    if check_process_exists("ZoomAudioDevice") {
        return true;
    }

    // Count caphost processes - baseline is ~2 when idle, more during meetings
    if let Some(count) = count_processes("caphost") {
        if count > 2 {
            return true;
        }
    }

    // Check for CptHost which spawns during meetings
    if check_process_exists("CptHost") {
        return true;
    }

    false
}

fn check_process_exists(name: &str) -> bool {
    Command::new("pgrep")
        .args(["-x", name])
        .output()
        .is_ok_and(|o| o.status.success())
}

fn count_processes(name: &str) -> Option<usize> {
    let output = Command::new("pgrep").args(["-f", name]).output().ok()?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        Some(stdout.lines().count())
    } else {
        Some(0)
    }
}
