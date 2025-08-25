use std::path::PathBuf;

use tauri::{AppHandle, LogicalPosition, Manager, Position, WebviewWindowBuilder};
use tauri_nspanel::{
    panel::{NSWindowCollectionBehavior, NSWindowStyleMask},
    tauri_panel, ManagerExt, PanelLevel, WebviewWindowExt,
};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn close_panel(app: AppHandle) {
    if let Ok(panel) = app.get_webview_panel("pop_panel") {
        panel.set_released_when_closed(true);
        // release the event handler if any
        panel.set_event_handler(None);
        panel.close(&app);
    }
}

tauri_panel! {
    panel!(ConvertedPanel {
        config: {
            isFloatingPanel: true,
            canBecomeKeyWindow: true,
        }
    })
}

fn transform_webview_window_to_panel(window: tauri::WebviewWindow, width: f64, height: f64) {
    let panel = window.to_panel::<ConvertedPanel>().unwrap();

    panel.set_level(PanelLevel::Dock.value());

    panel.set_style_mask(NSWindowStyleMask::NonactivatingPanel);

    panel.set_collection_behavior(
        NSWindowCollectionBehavior::MoveToActiveSpace
            | NSWindowCollectionBehavior::Stationary
            | NSWindowCollectionBehavior::FullScreenAuxiliary,
    );

    panel.set_content_size(width, height);
}

fn build_pop_panel(
    app: &AppHandle,
    label: String,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    url: PathBuf,
) {
    let builder = WebviewWindowBuilder::new(app, &label, tauri::WebviewUrl::App(url))
        .minimizable(false)
        .maximizable(false)
        .maximized(false)
        .closable(false)
        .resizable(false)
        .hidden_title(true)
        .title_bar_style(tauri::TitleBarStyle::Overlay)
        .decorations(false);

    let webview_window = builder.build().unwrap();

    webview_window
        .set_position(Position::Logical(LogicalPosition { x, y }))
        .unwrap();

    transform_webview_window_to_panel(webview_window.clone(), width, height);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_nspanel::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .invoke_handler(tauri::generate_handler![close_panel])
        .setup(|app| {
            build_pop_panel(
                &app.handle(),
                "pop_panel".to_string(),
                300.0,
                200.0,
                800.0,
                600.0,
                PathBuf::from("dist/index.html"),
            );

            let gs = app.global_shortcut();
            gs.on_shortcut("CommandOrControl+Shift+X", |app, _shortcut, event| {
                let handle = app.app_handle().clone();
                if event.state == ShortcutState::Pressed {
                    println!("CommandOrControl+Shift+X pressed");
                    let window = handle.get_webview_window("pop_panel");
                    if window.is_some() {
                        println!("panel already exists!");
                        return;
                    } else {
                        let handle_clone = handle.clone();
                        handle
                            .run_on_main_thread(move || {
                                build_pop_panel(
                                    &handle_clone,
                                    "pop_panel".to_string(),
                                    300.0,
                                    200.0,
                                    800.0,
                                    600.0,
                                    PathBuf::from("dist/index.html"),
                                );
                            })
                            .unwrap();
                    }
                }
            })
            .unwrap();

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
