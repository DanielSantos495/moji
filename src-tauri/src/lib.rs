use std::sync::Mutex;
use tauri::{
    menu::{CheckMenuItem, Menu, MenuItem},
    tray::TrayIconBuilder,
    window::Monitor,
    Emitter, Manager, WebviewWindow,
};

const TRANSPARENT_SCRIPT: &str = r#"
    document.documentElement.style.background = 'transparent';
    document.documentElement.style.backgroundColor = 'transparent';
    document.body.style.background = 'transparent';
    document.body.style.backgroundColor = 'transparent';
"#;

const SCREEN_MARGIN: i32 = 16;

struct ClickThroughState(Mutex<bool>);
struct GhostMenuState(CheckMenuItem<tauri::Wry>);

fn apply_click_through(app: &tauri::AppHandle, enabled: bool) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.set_ignore_cursor_events(enabled);
    }
    let _ = app.emit("click-through-changed", enabled);
}

fn toggle_ghost_mode(app: &tauri::AppHandle) {
    let state = app.state::<ClickThroughState>();
    let mut guard = state.0.lock().unwrap();
    *guard = !*guard;
    let enabled = *guard;
    drop(guard);

    apply_click_through(app, enabled);

    let ghost = app.state::<GhostMenuState>();
    let _ = ghost.0.set_checked(enabled);
}

// Ancla la ventana a la esquina inferior derecha del work_area del monitor
// (work_area excluye dock/menu bar/taskbar). Trabaja todo en píxeles
// lógicos: en macOS las posiciones físicas no son consistentes entre
// monitores con scale distinto, pero el sistema de coordenadas lógico
// (puntos NSScreen) sí es unificado globalmente, así que LogicalPosition
// cruza monitores correctamente.
fn anchor_to_monitor(window: &WebviewWindow, monitor: &Monitor) {
    let Ok(outer) = window.outer_size() else { return };
    let Ok(current_scale) = window.scale_factor() else { return };
    let logical_w = outer.width as f64 / current_scale;
    let logical_h = outer.height as f64 / current_scale;

    let target_scale = monitor.scale_factor();
    let work_area = monitor.work_area();
    let wa_x = work_area.position.x as f64 / target_scale;
    let wa_y = work_area.position.y as f64 / target_scale;
    let wa_w = work_area.size.width as f64 / target_scale;
    let wa_h = work_area.size.height as f64 / target_scale;

    let margin = SCREEN_MARGIN as f64;
    let x = wa_x + wa_w - logical_w - margin;
    let y = wa_y + wa_h - logical_h - margin;
    let _ = window.set_position(tauri::LogicalPosition::new(x, y));
}

// Re-ancla en el monitor donde ya vive la ventana (tras un resize).
fn re_anchor_current(window: &WebviewWindow) {
    if let Ok(Some(monitor)) = window.current_monitor() {
        anchor_to_monitor(window, &monitor);
    }
}

fn move_main_to_next_monitor(app: &tauri::AppHandle) {
    let Some(window) = app.get_webview_window("main") else { return };
    let Ok(monitors) = window.available_monitors() else { return };
    if monitors.len() < 2 {
        return;
    }
    let current_pos = window
        .current_monitor()
        .ok()
        .flatten()
        .map(|m| *m.position());
    let current_idx = current_pos
        .and_then(|p| monitors.iter().position(|m| *m.position() == p))
        .unwrap_or(0);
    let next = &monitors[(current_idx + 1) % monitors.len()];
    anchor_to_monitor(&window, next);
}

#[tauri::command]
fn get_click_through(state: tauri::State<'_, ClickThroughState>) -> bool {
    *state.0.lock().unwrap()
}

#[tauri::command]
fn set_click_through(
    app: tauri::AppHandle,
    state: tauri::State<'_, ClickThroughState>,
    ghost: tauri::State<'_, GhostMenuState>,
    enabled: bool,
) {
    *state.0.lock().unwrap() = enabled;
    apply_click_through(&app, enabled);
    let _ = ghost.0.set_checked(enabled);
}

#[tauri::command]
fn set_main_size(app: tauri::AppHandle, size: u32) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.set_size(tauri::LogicalSize::new(size as f64, size as f64));
        re_anchor_current(&window);
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
    let size_logical: f64 = 120.0;
    let margin_logical: f64 = SCREEN_MARGIN as f64;

    // Calcular posición inicial sobre el monitor principal antes de construir
    // la ventana. work_area es físico; lo paso a lógico dividiendo por su
    // scale factor, porque WindowBuilder::position usa coordenadas lógicas.
    let (x, y) = app
        .primary_monitor()
        .ok()
        .flatten()
        .map(|m| {
            let scale = m.scale_factor();
            let wa = m.work_area();
            let wa_x = wa.position.x as f64 / scale;
            let wa_y = wa.position.y as f64 / scale;
            let wa_w = wa.size.width as f64 / scale;
            let wa_h = wa.size.height as f64 / scale;
            (
                wa_x + wa_w - size_logical - margin_logical,
                wa_y + wa_h - size_logical - margin_logical,
            )
        })
        .unwrap_or((100.0, 100.0));

    tauri::WebviewWindowBuilder::new(app, "main", tauri::WebviewUrl::App("index.html".into()))
        .title("Moji")
        .inner_size(size_logical, size_logical)
        .min_inner_size(120.0, 120.0)
        .max_inner_size(150.0, 150.0)
        .position(x, y)
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
    let ghost_item = CheckMenuItem::with_id(
        app,
        "ghost",
        "Modo Fantasma",
        true,
        false,
        Some("CmdOrCtrl+Shift+M"),
    )?;

    app.manage(GhostMenuState(ghost_item.clone()));

    let move_screen_item =
        MenuItem::with_id(app, "move_screen", "Mover a otra pantalla", true, None::<&str>)?;
    let settings_item =
        MenuItem::with_id(app, "settings", "Configuración", true, None::<&str>)?;
    let quit_item = MenuItem::with_id(app, "quit", "Salir de Moji", true, None::<&str>)?;
    let menu = Menu::with_items(
        app,
        &[&ghost_item, &move_screen_item, &settings_item, &quit_item],
    )?;

    TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "ghost" => toggle_ghost_mode(app),
            "move_screen" => move_main_to_next_monitor(app),
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
            app.manage(ClickThroughState(Mutex::new(false)));

            create_main_window(app)?;
            setup_tray(app)?;

            #[cfg(desktop)]
            {
                use tauri_plugin_global_shortcut::ShortcutState;

                app.handle().plugin(
                    tauri_plugin_global_shortcut::Builder::new()
                        .with_shortcuts(["CmdOrCtrl+Shift+M"])?
                        .with_handler(|app, _shortcut, event| {
                            if event.state == ShortcutState::Pressed {
                                toggle_ghost_mode(app);
                            }
                        })
                        .build(),
                )?;
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_click_through,
            set_click_through,
            set_main_size,
            set_main_opacity,
            open_settings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
