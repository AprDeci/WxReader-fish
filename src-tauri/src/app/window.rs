use crate::app::config::PakeConfig;
use crate::util::get_data_dir;
use std::{path::PathBuf, str::FromStr};
use tauri::{
    window, App, AppHandle, Config, Manager, Url, WebviewUrl, WebviewWindow, WebviewWindowBuilder,
};

#[cfg(target_os = "macos")]
use tauri::{Theme, TitleBarStyle};

pub fn set_window(app: &mut App, config: &PakeConfig, tauri_config: &Config) -> WebviewWindow {
    let package_name = tauri_config.clone().product_name.unwrap();
    let _data_dir = get_data_dir(app.handle(), package_name);

    let window_config = config
        .windows
        .first()
        .expect("At least one window configuration is required");

    let user_agent = config.user_agent.get();

    let url = match window_config.url_type.as_str() {
        "web" => WebviewUrl::App(window_config.url.parse().unwrap()),
        "local" => WebviewUrl::App(PathBuf::from(&window_config.url)),
        _ => panic!("url type can only be web or local"),
    };

    let config_script = format!(
        "window.pakeConfig = {}",
        serde_json::to_string(&window_config).unwrap()
    );

    // Platform-specific title: macOS prefers empty, others fallback to product name
    let effective_title = window_config.title.as_deref().unwrap_or_else(|| {
        if cfg!(target_os = "macos") {
            ""
        } else {
            tauri_config.product_name.as_deref().unwrap_or("")
        }
    });

    let mut window_builder = WebviewWindowBuilder::new(app, "pake", url)
        .title(effective_title)
        .visible(false)
        .user_agent(user_agent)
        .resizable(window_config.resizable)
        .fullscreen(window_config.fullscreen)
        .inner_size(window_config.width, window_config.height)
        .always_on_top(window_config.always_on_top)
        .decorations(window_config.decorations)
        .shadow(window_config.shadow)
        .incognito(window_config.incognito);

    #[cfg(target_os = "windows")]
    {
        window_builder = window_builder.transparent(true);
    }

    // macos 设置背景颜色(未测试)
    #[cfg(target_os = "macos")]
    {
        use cocoa::appkit::{NSColor, NSWindow};
        use cocoa::base::{id, nil};
        let ns_window = window.ns_window().unwrap() as id;
        unsafe {
            let bg_color = NSColor::colorWithRed_green_blue_alpha_(
                nil,
                50.0 / 255.0,
                158.0 / 255.0,
                163.5 / 255.0,
                0.0,
            );
            ns_window.setOpaque_(0);
            ns_window.setHasShadow_(0);
            ns_window.setBackgroundColor_(bg_color);
        }
    }

    if !window_config.enable_drag_drop {
        window_builder = window_builder.disable_drag_drop_handler();
    }

    // Add initialization scripts
    window_builder = window_builder
        .initialization_script(&config_script)
        .initialization_script(include_str!("../inject/component.js"))
        .initialization_script(include_str!("../inject/event.js"))
        .initialization_script(include_str!("../inject/style.js"))
        .initialization_script(include_str!("../inject/custom.js"))
        .initialization_script(include_str!("../inject/shortcut.js"));

    if window_config.enable_wasm {
        window_builder = window_builder
            .additional_browser_args("--enable-features=SharedArrayBuffer")
            .additional_browser_args("--enable-unsafe-webgpu");
    }

    if !config.proxy_url.is_empty() {
        if let Ok(proxy_url) = Url::from_str(&config.proxy_url) {
            window_builder = window_builder.proxy_url(proxy_url);
            #[cfg(debug_assertions)]
            println!("Proxy configured: {}", config.proxy_url);
        }
    }

    #[cfg(target_os = "macos")]
    {
        let title_bar_style = if window_config.hide_title_bar {
            TitleBarStyle::Overlay
        } else {
            TitleBarStyle::Visible
        };
        window_builder = window_builder.title_bar_style(title_bar_style);

        if window_config.dark_mode {
            window_builder = window_builder.theme(Some(Theme::Dark));
        }
    }

    // Windows and Linux share the same configuration
    #[cfg(not(target_os = "macos"))]
    {
        window_builder = window_builder
            .data_directory(_data_dir)
            .additional_browser_args("--disable-blink-features=AutomationControlled")
            .theme(None);
    }

    window_builder.build().expect("Failed to build window")
}

pub fn set_config_window(app: &AppHandle) {
    if app.get_webview_window("config").is_none() {
        let config_window =
            WebviewWindowBuilder::new(app, "config", tauri::WebviewUrl::App("config.html".into()))
                .title("WxReader_fish配置")
                .inner_size(200.0, 400.0)
                .resizable(true)
                .build()
                .unwrap();

        config_window.show().unwrap();
    } else {
        let window = app.get_webview_window("config").unwrap();
        window.show().unwrap();
        window.set_focus().unwrap();
    }
}
