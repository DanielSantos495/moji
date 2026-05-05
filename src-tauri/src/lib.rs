use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    Manager,
};

// ── Cursor tracking (macOS) ──────────────────────────────────────────────────
// Lee la posición del cursor via CoreGraphics (no requiere Accessibility).
// Corre en un hilo separado y emite "moji:hover" cuando el cursor entra
// a la ventana principal, sin importar si la app tiene focus o no.

#[cfg(target_os = "macos")]
mod cursor_tracker {
    use std::ffi::c_void;
    use tauri::{Emitter, Manager};

    #[repr(C)]
    #[derive(Clone, Copy)]
    struct CGPoint {
        x: f64,
        y: f64,
    }

    #[link(name = "CoreGraphics", kind = "framework")]
    extern "C" {
        fn CGEventCreate(source: *const c_void) -> *mut c_void;
        fn CGEventGetLocation(event: *const c_void) -> CGPoint;
        fn CFRelease(cf: *const c_void);
    }

    fn cursor_pos() -> (f64, f64) {
        unsafe {
            let event = CGEventCreate(std::ptr::null());
            let pt = CGEventGetLocation(event);
            CFRelease(event);
            (pt.x, pt.y)
        }
    }

    fn is_over_window(window: &tauri::WebviewWindow) -> bool {
        let (cx, cy_mac) = cursor_pos();

        let monitor = match window.current_monitor() {
            Ok(Some(m)) => m,
            _ => return false,
        };
        let scale = monitor.scale_factor();
        let screen_h = monitor.size().height as f64 / scale;

        let pos = match window.outer_position() {
            Ok(p) => p,
            Err(_) => return false,
        };
        let size = match window.outer_size() {
            Ok(s) => s,
            Err(_) => return false,
        };

        // macOS: Y=0 en la parte inferior. Tauri: Y=0 en la parte superior.
        let cy = screen_h - cy_mac;
        let wx = pos.x as f64 / scale;
        let wy = pos.y as f64 / scale;
        let ww = size.width as f64 / scale;
        let wh = size.height as f64 / scale;

        cx >= wx && cx <= wx + ww && cy >= wy && cy <= wy + wh
    }

    pub fn start(app: tauri::AppHandle) {
        std::thread::spawn(move || {
            let mut was_inside = false;
            loop {
                std::thread::sleep(std::time::Duration::from_millis(50));
                if let Some(window) = app.get_webview_window("main") {
                    let inside = is_over_window(&window);
                    if inside && !was_inside {
                        let _ = window.emit("moji:hover", ());
                    }
                    was_inside = inside;
                }
            }
        });
    }
}

const TRANSPARENT_SCRIPT: &str = r#"
    document.documentElement.style.background = 'transparent';
    document.documentElement.style.backgroundColor = 'transparent';
    document.body.style.background = 'transparent';
    document.body.style.backgroundColor = 'transparent';
"#;

#[tauri::command]
fn set_click_through(app: tauri::AppHandle, enabled: bool) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.set_ignore_cursor_events(enabled);
    }
}

#[tauri::command]
fn set_main_size(app: tauri::AppHandle, size: u32) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.set_size(tauri::LogicalSize::new(size as f64, size as f64));
    }
}

#[tauri::command]
fn set_main_opacity(app: tauri::AppHandle, opacity: f64) {
    if let Some(window) = app.get_webview_window("main") {
        let js = format!("document.documentElement.style.opacity = '{opacity}'");
        let _ = window.eval(&js);
    }
}

#[tauri::command]
fn open_settings(app: tauri::AppHandle) {
    if let Some(win) = app.get_webview_window("settings") {
        let _ = win.show();
        let _ = win.set_focus();
    } else {
        let _ = tauri::WebviewWindowBuilder::new(
            &app,
            "settings",
            tauri::WebviewUrl::App("/#settings".into()),
        )
        .title("Moji — Configuración")
        .inner_size(300.0, 380.0)
        .resizable(false)
        .always_on_top(true)
        .build();
    }
}

fn create_main_window(app: &tauri::App) -> tauri::Result<()> {
    tauri::WebviewWindowBuilder::new(app, "main", tauri::WebviewUrl::App("index.html".into()))
        .title("Moji")
        .inner_size(200.0, 200.0)
        .min_inner_size(100.0, 100.0)
        .max_inner_size(400.0, 400.0)
        .position(100.0, 100.0)
        .resizable(true)
        .decorations(false)
        .transparent(true)
        .shadow(false)
        .always_on_top(true)
        .skip_taskbar(true)
        .initialization_script(TRANSPARENT_SCRIPT)
        .build()?;
    Ok(())
}

fn setup_tray(app: &tauri::App) -> tauri::Result<()> {
    let settings_item =
        MenuItem::with_id(app, "settings", "Configuración", true, None::<&str>)?;
    let quit_item = MenuItem::with_id(app, "quit", "Salir de Moji", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&settings_item, &quit_item])?;

    TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "settings" => {
                if let Some(win) = app.get_webview_window("settings") {
                    let _ = win.show();
                    let _ = win.set_focus();
                } else {
                    let _ = tauri::WebviewWindowBuilder::new(
                        app,
                        "settings",
                        tauri::WebviewUrl::App("/#settings".into()),
                    )
                    .title("Moji — Configuración")
                    .inner_size(300.0, 380.0)
                    .resizable(false)
                    .always_on_top(true)
                    .build();
                }
            }
            "quit" => app.exit(0),
            _ => {}
        })
        .build(app)?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            create_main_window(app)?;
            setup_tray(app)?;
            #[cfg(target_os = "macos")]
            cursor_tracker::start(app.handle().clone());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            set_click_through,
            set_main_size,
            set_main_opacity,
            open_settings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
