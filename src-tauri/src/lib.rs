use std::sync::{Arc, Mutex};
use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    Manager,
};

const TRANSPARENT_SCRIPT: &str = r#"
    document.documentElement.style.background = 'transparent';
    document.documentElement.style.backgroundColor = 'transparent';
    document.body.style.background = 'transparent';
    document.body.style.backgroundColor = 'transparent';
"#;

// Estado compartido — preferencia del usuario sobre click-through
struct ClickThroughState(Arc<Mutex<bool>>);

#[tauri::command]
fn set_click_through(
    app: tauri::AppHandle,
    state: tauri::State<'_, ClickThroughState>,
    enabled: bool,
) {
    *state.0.lock().unwrap() = enabled;

    // En no-macOS, aplica el toggle directo (sin smart tracking).
    // En macOS, el cursor_tracker se encarga de aplicarlo dinámicamente.
    #[cfg(not(target_os = "macos"))]
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.set_ignore_cursor_events(enabled);
    }

    #[cfg(target_os = "macos")]
    let _ = app; // suprime el warning de variable no usada
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

// ── Smart click-through (macOS) ─────────────────────────────────────────────
// Cuando el usuario activa "no interrumpir":
//   - Si el cursor está sobre el área de controles (⠿ y ⚙ en la esquina
//     superior derecha): desactiva click-through → los iconos funcionan.
//   - En cualquier otro punto de la ventana: click-through activo → los
//     clics pasan a la app de abajo.

#[cfg(target_os = "macos")]
mod cursor_tracker {
    use std::ffi::c_void;
    use std::sync::{Arc, Mutex};
    use tauri::Manager;

    // Tamaño del área interactiva en la esquina superior derecha (px lógicos)
    const CONTROLS_WIDTH: f64 = 40.0;
    const CONTROLS_HEIGHT: f64 = 60.0;

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

    fn cursor_in_controls(window: &tauri::WebviewWindow) -> bool {
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

        // macOS: Y=0 abajo. Tauri: Y=0 arriba. Convertimos.
        let cy = screen_h - cy_mac;
        let wx = pos.x as f64 / scale;
        let wy = pos.y as f64 / scale;
        let ww = size.width as f64 / scale;

        // Esquina superior derecha de la ventana
        let in_x = cx >= wx + ww - CONTROLS_WIDTH && cx <= wx + ww;
        let in_y = cy >= wy && cy <= wy + CONTROLS_HEIGHT;
        in_x && in_y
    }

    pub fn start(app: tauri::AppHandle, click_through: Arc<Mutex<bool>>) {
        std::thread::spawn(move || {
            let mut current_ignore: Option<bool> = None;

            loop {
                std::thread::sleep(std::time::Duration::from_millis(50));

                let enabled = match click_through.lock() {
                    Ok(g) => *g,
                    Err(_) => continue,
                };

                let window = match app.get_webview_window("main") {
                    Some(w) => w,
                    None => continue,
                };

                let should_ignore = enabled && !cursor_in_controls(&window);

                if Some(should_ignore) != current_ignore {
                    let _ = window.set_ignore_cursor_events(should_ignore);
                    current_ignore = Some(should_ignore);
                }
            }
        });
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let click_through = Arc::new(Mutex::new(false));
            app.manage(ClickThroughState(click_through.clone()));

            create_main_window(app)?;
            setup_tray(app)?;

            #[cfg(target_os = "macos")]
            cursor_tracker::start(app.handle().clone(), click_through);

            #[cfg(not(target_os = "macos"))]
            let _ = click_through;

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
