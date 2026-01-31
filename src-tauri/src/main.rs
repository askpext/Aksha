// Prevents additional console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod indexer;
mod search;
mod commands;

use tauri::{Manager, WindowEvent};
use tauri_plugin_global_shortcut::{Code, Modifiers, Shortcut, GlobalShortcutExt};
use std::sync::{Arc, Mutex};
use indexer::FileIndex;

#[cfg(target_os = "windows")]
use windows::Win32::UI::WindowsAndMessaging::{
    GetWindowLongW, SetWindowLongW, GWL_STYLE, GWL_EXSTYLE, WS_POPUP, WS_EX_LAYERED, WS_EX_WINDOWEDGE,
    WS_EX_CLIENTEDGE, WS_EX_DLGMODALFRAME, WS_EX_STATICEDGE, SWP_FRAMECHANGED, SWP_NOMOVE, SWP_NOSIZE,
    SWP_NOZORDER, SWP_NOOWNERZORDER, SetWindowPos, WS_CAPTION, WS_THICKFRAME, WS_MINIMIZEBOX, WS_MAXIMIZEBOX,
    WS_SYSMENU, WS_BORDER, WS_DLGFRAME,
};

fn main() {
    // Initialize file index
    let file_index = Arc::new(Mutex::new(FileIndex::new()));
    let index_clone = file_index.clone();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(move |app| {
            // Get the main window
            let window = app.get_webview_window("main").unwrap();
            
            // Apply Windows-specific transparent window setup
            // #[cfg(target_os = "windows")]
            // setup_transparent_window(&window);
            
            // Register global shortcut (Ctrl+Space)
            let shortcut = Shortcut::new(Some(Modifiers::CONTROL), Code::Space);
            let window_clone = window.clone();
            
            use tauri_plugin_global_shortcut::ShortcutState;

            if let Err(e) = app.global_shortcut().on_shortcut(shortcut, move |_app, _shortcut, event| {
                if event.state == ShortcutState::Pressed {
                    if window_clone.is_visible().unwrap_or(false) {
                        let _ = window_clone.hide();
                    } else {
                        let _ = window_clone.show();
                        let _ = window_clone.set_focus();
                    }
                }
            }) {
                eprintln!("Failed to register shortcut callback: {}", e);
            }
            
            if let Err(e) = app.global_shortcut().register(shortcut) {
                eprintln!("Failed to register global shortcut: {}", e);
            }

            // Start indexing in background
            let index = index_clone.clone();
            std::thread::spawn(move || {
                // Set indexing flag to true
                if let Ok(mut idx) = index.lock() {
                    idx.set_indexing(true);
                }

                // Perform the scan (slow operation, NO LOCK held)
                let new_entries = indexer::scan_system_files();

                // Update the index with new results (fast operation, lock held briefly)
                if let Ok(mut idx) = index.lock() {
                    idx.update_entries(new_entries);
                    idx.set_indexing(false);
                }
            });

            // Hide window on focus loss
            window.on_window_event(move |event| {
                if let WindowEvent::Focused(false) = event {
                    // Optionally hide on focus loss - commented out for now
                    // let _ = window.hide();
                }
            });

            Ok(())
        })
        .manage(file_index)
        .invoke_handler(tauri::generate_handler![
            commands::search_files,
            commands::open_file,
            commands::show_in_folder,
            commands::get_index_status,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}