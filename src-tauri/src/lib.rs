mod commands;
mod db;
mod tracking;

use std::sync::Arc;
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::TrayIconBuilder,
    Manager,
};
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_positioner::{Position, WindowExt};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_positioner::init())
        .plugin(
            tauri_plugin_autostart::Builder::new()
                .macos_launcher(MacosLauncher::LaunchAgent)
                .build(),
        )
        .setup(|app| {
            // Initialize database
            let database = Arc::new(db::Database::new().expect("Failed to initialize database"));
            app.manage(database.clone());

            // Initialize tracker
            let tracker = Arc::new(tracking::Tracker::new());
            app.manage(tracker.clone());

            // Start background polling on a dedicated thread with its own tokio runtime
            std::thread::spawn({
                let tracker = tracker.clone();
                let db = database.clone();
                move || {
                    let rt = tokio::runtime::Builder::new_current_thread()
                        .enable_time()
                        .build()
                        .expect("Failed to create tokio runtime");
                    rt.block_on(async move {
                        loop {
                            tracker.poll(&db);
                            tokio::time::sleep(std::time::Duration::from_secs(30)).await;
                        }
                    });
                }
            });

            // Build tray menu
            let quit = MenuItemBuilder::with_id("quit", "Quit DevTracker").build(app)?;
            let menu = MenuBuilder::new(app).item(&quit).build()?;

            // Build tray icon
            let icon = app.default_window_icon().cloned().unwrap();
            let _tray = TrayIconBuilder::new()
                .icon(icon)
                .icon_as_template(true)
                .menu(&menu)
                .on_menu_event(move |app, event| {
                    if event.id() == "quit" {
                        app.exit(0);
                    }
                })
                .on_tray_icon_event(|tray, event| {
                    tauri_plugin_positioner::on_tray_event(tray.app_handle(), &event);

                    if let tauri::tray::TrayIconEvent::Click { button, .. } = event {
                        if button == tauri::tray::MouseButton::Left {
                            let app = tray.app_handle();
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.move_window(Position::TrayBottomCenter);
                                if window.is_visible().unwrap_or(false) {
                                    let _ = window.hide();
                                } else {
                                    let _ = window.show();
                                    let _ = window.set_focus();
                                }
                            }
                        }
                    }
                })
                .build(app)?;

            // Hide the main window on startup (it's a menubar app)
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.hide();
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::timeline::get_current_status,
            commands::timeline::get_timeline,
            commands::timeline::get_summary,
            commands::timeline::get_repo_summary,
            commands::tags::update_activity_ticket,
            commands::export::export_summary,
            commands::settings::get_autostart,
            commands::settings::set_autostart,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
