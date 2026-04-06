use objc2_app_kit::NSWorkspace;

pub struct AppInfo {
    pub name: String,
    pub bundle_id: Option<String>,
}

pub fn get_frontmost_app() -> Option<AppInfo> {
    let workspace = NSWorkspace::sharedWorkspace();
    let app = workspace.frontmostApplication()?;

    let name = app.localizedName()?.to_string();
    let bundle_id = app.bundleIdentifier().map(|s| s.to_string());

    Some(AppInfo { name, bundle_id })
}

/// Returns true if the app is VS Code
pub fn is_vscode(bundle_id: &Option<String>) -> bool {
    bundle_id
        .as_ref()
        .is_some_and(|id| id == "com.microsoft.VSCode" || id.contains("VSCode"))
}

/// Returns true if the app is a terminal (Claude Code runs here)
pub fn is_terminal(name: &str, bundle_id: &Option<String>) -> bool {
    let terminal_bundles = [
        "com.apple.Terminal",
        "com.googlecode.iterm2",
        "io.alacritty",
        "com.mitchellh.ghostty",
        "dev.warp.Warp-Stable",
        "net.kovidgoyal.kitty",
    ];

    if let Some(id) = bundle_id {
        if terminal_bundles.iter().any(|t| id == *t) {
            return true;
        }
    }

    let terminal_names = ["Terminal", "iTerm2", "Alacritty", "Ghostty", "Warp", "kitty"];
    terminal_names.iter().any(|t| name == *t)
}

/// Returns true if the app is one where we should scan for git branches
/// (VS Code, terminals running Claude Code, etc.)
pub fn is_coding_app(name: &str, bundle_id: &Option<String>) -> bool {
    is_vscode(bundle_id) || is_terminal(name, bundle_id)
}
