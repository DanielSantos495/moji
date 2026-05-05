use std::sync::Mutex;
use tauri::{
    menu::{CheckMenuItem, Menu, MenuItem},
    tray::TrayIconBuilder,
    Emitter, Manager,
};

const TRANSPARENT_SCRIPT: &str = r#"
    document.documentElement.style.background = 'transparent';
    document.documentElement.style.backgroundColor = 'transparent';
    document.body.style.background = 'transparent';
    document.body.style.backgroundColor = 'transparent';
"#;

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
    let ghost_item = CheckMenuItem::with_id(
        app,
        "ghost",
        "Modo Fantasma",
        true,
        false,
        Some("CmdOrCtrl+Shift+M"),
    )?;

    app.manage(GhostMenuState(ghost_item.clone()));

    let settings_item =
        MenuItem::with_id(app, "settings", "Configuración", true, None::<&str>)?;
    let quit_item = MenuItem::with_id(app, "quit", "Salir de Moji", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&ghost_item, &settings_item, &quit_item])?;

    TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "ghost" => toggle_ghost_mode(app),
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
