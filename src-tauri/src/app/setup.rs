use std::collections::HashMap;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::TrayIconBuilder,
    AppHandle, Manager,
};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut};
use tauri_plugin_window_state::{AppHandleExt, StateFlags};

use crate::app::window::set_config_window;
use crate::util::show_toast;

pub fn set_system_tray(app: &AppHandle, show_system_tray: bool) -> tauri::Result<()> {
    if !show_system_tray {
        app.remove_tray_by_id("pake-tray");
        return Ok(());
    }

    let hide_app = MenuItemBuilder::with_id("hide_app", "Hide").build(app)?;
    let show_app = MenuItemBuilder::with_id("show_app", "Show").build(app)?;
    let config = MenuItemBuilder::with_id("config", "Config").build(app)?;
    let quit = MenuItemBuilder::with_id("quit", "Quit").build(app)?;

    let menu = MenuBuilder::new(app)
        .items(&[&hide_app, &show_app, &config, &quit])
        .build()?;

    app.app_handle().remove_tray_by_id("pake-tray");

    let tray = TrayIconBuilder::new()
        .menu(&menu)
        .on_menu_event(move |app, event| match event.id().as_ref() {
            "hide_app" => {
                if let Some(window) = app.get_webview_window("pake") {
                    window.minimize().unwrap();
                }
            }
            "show_app" => {
                if let Some(window) = app.get_webview_window("pake") {
                    window.show().unwrap();
                }
            }
            "config" => {
                // 检查配置窗口是否已存在
                set_config_window(app);
            }
            "quit" => {
                app.save_window_state(StateFlags::all()).unwrap();
                std::process::exit(0);
            }
            _ => (),
        })
        .icon(app.default_window_icon().unwrap().clone())
        .build(app)?;

    tray.set_icon_as_template(false)?;
    Ok(())
}

// pub fn set_global_shortcut(app: &AppHandle, shortcut: String) -> tauri::Result<()> {
//     if shortcut.is_empty() {
//         return Ok(());
//     }

//     let app_handle = app.clone();
//     let shortcut_hotkey = Shortcut::from_str(&shortcut).unwrap();
//     let last_triggered = Arc::new(Mutex::new(Instant::now()));

//     app_handle
//         .plugin(
//             tauri_plugin_global_shortcut::Builder::new()
//                 .with_handler({
//                     let last_triggered = Arc::clone(&last_triggered);
//                     move |app, event, _shortcut| {
//                         let mut last_triggered = last_triggered.lock().unwrap();
//                         if Instant::now().duration_since(*last_triggered)
//                             < Duration::from_millis(300)
//                         {
//                             return;
//                         }
//                         *last_triggered = Instant::now();

//                         if shortcut_hotkey.eq(event) {
//                             if let Some(window) = app.get_webview_window("pake") {
//                                 let is_visible = window.is_visible().unwrap();
//                                 if is_visible {
//                                     window.hide().unwrap();
//                                 } else {
//                                     window.show().unwrap();
//                                     window.set_focus().unwrap();
//                                 }
//                             }
//                         }
//                     }
//                 })
//                 .build(),
//         )
//         .expect("Failed to set global shortcut");

//     app.global_shortcut().register(shortcut_hotkey).unwrap();

//     Ok(())
// }

pub fn set_multiple_global_shortcuts(
    app: &AppHandle,
    config: &HashMap<String, String>,
) -> tauri::Result<()> {
    if config.is_empty() {
        return Ok(());
    }

    let app_handle = app.clone();
    let last_triggered = Arc::new(Mutex::new(Instant::now()));
    let shortcuts_clone = config.clone();

    // 注册全局快捷键插件处理器
    app_handle
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(move |app, event, _shortcut| {
                    let mut last_triggered = last_triggered.lock().unwrap();
                    if Instant::now().duration_since(*last_triggered) < Duration::from_millis(300) {
                        return;
                    }
                    *last_triggered = Instant::now();

                    // 检查哪个快捷键被触发
                    for (action, shortcut_str) in &shortcuts_clone {
                        if let Ok(shortcut_hotkey) = Shortcut::from_str(shortcut_str) {
                            if shortcut_hotkey.eq(event) {
                                handle_shortcut_action(app, action);
                                break;
                            }
                        }
                    }
                })
                .build(),
        )
        .expect("Failed to set global shortcuts");

    // 注册所有快捷键
    for (action, shortcut_str) in config {
        if let Ok(shortcut_hotkey) = Shortcut::from_str(shortcut_str) {
            match app.global_shortcut().register(shortcut_hotkey) {
                Ok(_) => {
                    println!("Registered global shortcut '{}': {}", action, shortcut_str);
                }
                Err(e) => {
                    println!(
                        "Failed to register global shortcut '{}': {}",
                        shortcut_str, e
                    );
                    show_toast(
                        &app.get_webview_window("pake").unwrap(),
                        &format!(
                            "Failed to register global shortcut '{}': {}",
                            shortcut_str, e
                        ),
                    );
                }
            }
        } else {
            println!("Invalid shortcut format: {}", shortcut_str);
            show_toast(
                &app.get_webview_window("pake").unwrap(),
                &format!("Invalid shortcut format: {}", shortcut_str),
            );
        }
    }

    Ok(())
}

fn handle_shortcut_action(app: &AppHandle, action: &str) {
    match action {
        "toggle_window" => {
            if let Some(window) = app.get_webview_window("pake") {
                let is_visible = window.is_visible().unwrap();
                if is_visible {
                    window.hide().unwrap();
                } else {
                    window.show().unwrap();
                    window.set_focus().unwrap();
                }
            }
        }
        "next_page" => {
            if let Some(window) = app.get_webview_window("pake") {
                let script = r#"nextPage();"#;
                window.eval(script).unwrap();
            }
        }
        "prev_page" => {
            if let Some(window) = app.get_webview_window("pake") {
                let script = r#"prevPage();"#;
                window.eval(script).unwrap();
            }
        }
        _ => {
            println!("Unknown action: {}", action);
            show_toast(
                &app.get_webview_window("pake").unwrap(),
                &format!("Unknown action: {}", action),
            );
        }
    }
}
