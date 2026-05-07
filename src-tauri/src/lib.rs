use std::fs;
use std::path::PathBuf;
use std::sync::{Condvar, Mutex};

use serde::{Deserialize, Serialize};
use tauri::{
    menu::{CheckMenuItem, Menu, MenuItem},
    tray::TrayIconBuilder,
    window::Monitor,
    AppHandle, Emitter, Manager, WebviewWindow,
};

const TRANSPARENT_SCRIPT: &str = r#"
    document.documentElement.style.background = 'transparent';
    document.documentElement.style.backgroundColor = 'transparent';
    document.body.style.background = 'transparent';
    document.body.style.backgroundColor = 'transparent';
"#;

const SCREEN_MARGIN: i32 = 16;
const DEFAULT_SIZE: u32 = 120;
const DEFAULT_OPACITY: f64 = 1.0;
const DEFAULT_CHARACTER: &str = "kael";
const DEFAULT_HYDRATION_INTERVAL_MIN: u32 = 60;

#[derive(Serialize, Deserialize, Clone)]
struct Config {
    #[serde(default = "default_opacity")]
    opacity: f64,
    #[serde(default = "default_size")]
    size: u32,
    #[serde(default)]
    click_through: bool,
    #[serde(default)]
    monitor_index: usize,
    #[serde(default = "default_character")]
    character: String,
    #[serde(default = "default_hydration_enabled")]
    hydration_enabled: bool,
    #[serde(default = "default_hydration_interval")]
    hydration_interval_min: u32,
}

fn default_opacity() -> f64 {
    DEFAULT_OPACITY
}
fn default_size() -> u32 {
    DEFAULT_SIZE
}
fn default_character() -> String {
    DEFAULT_CHARACTER.to_string()
}
fn default_hydration_enabled() -> bool {
    true
}
fn default_hydration_interval() -> u32 {
    DEFAULT_HYDRATION_INTERVAL_MIN
}

impl Default for Config {
    fn default() -> Self {
        Self {
            opacity: DEFAULT_OPACITY,
            size: DEFAULT_SIZE,
            click_through: false,
            monitor_index: 0,
            character: DEFAULT_CHARACTER.to_string(),
            hydration_enabled: true,
            hydration_interval_min: DEFAULT_HYDRATION_INTERVAL_MIN,
        }
    }
}

fn config_path(app: &AppHandle) -> Option<PathBuf> {
    let dir = app.path().app_config_dir().ok()?;
    let _ = fs::create_dir_all(&dir);
    Some(dir.join("settings.json"))
}

fn load_config(app: &AppHandle) -> Config {
    let Some(path) = config_path(app) else {
        return Config::default();
    };
    let Ok(content) = fs::read_to_string(&path) else {
        return Config::default();
    };
    serde_json::from_str::<Config>(&content).unwrap_or_default()
}

fn save_config(app: &AppHandle, config: &Config) {
    let Some(path) = config_path(app) else { return };
    if let Ok(json) = serde_json::to_string_pretty(config) {
        let _ = fs::write(path, json);
    }
}

fn update_config(app: &AppHandle, mutate: impl FnOnce(&mut Config)) {
    let state = app.state::<ConfigState>();
    let mut config = state.0.lock().unwrap();
    mutate(&mut config);
    save_config(app, &config);
}

struct ConfigState(Mutex<Config>);
struct ClickThroughState(Mutex<bool>);
struct GhostMenuState(CheckMenuItem<tauri::Wry>);

// Señal para reiniciar el ciclo del timer de hidratación. El hilo del timer
// duerme con `wait_timeout` (o `wait` indefinido si está apagado); los setters
// cambian config y luego llaman `signal()` para que el hilo despierte y rearme
// el ciclo desde cero con los nuevos valores.
struct HydrationSignal {
    flag: Mutex<bool>,
    cvar: Condvar,
}

impl HydrationSignal {
    fn new() -> Self {
        Self {
            flag: Mutex::new(false),
            cvar: Condvar::new(),
        }
    }

    fn signal(&self) {
        let mut guard = self.flag.lock().unwrap();
        *guard = true;
        self.cvar.notify_all();
    }
}

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

    update_config(app, |c| c.click_through = enabled);
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
    let next_idx = (current_idx + 1) % monitors.len();
    anchor_to_monitor(&window, &monitors[next_idx]);
    update_config(app, |c| c.monitor_index = next_idx);
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
    update_config(&app, |c| c.click_through = enabled);
}

#[tauri::command]
fn set_main_size(app: tauri::AppHandle, size: u32) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.set_size(tauri::LogicalSize::new(size as f64, size as f64));
        re_anchor_current(&window);
    }
    update_config(&app, |c| c.size = size);
}

#[tauri::command]
fn set_main_opacity(app: tauri::AppHandle, opacity: f64) {
    if let Some(window) = app.get_webview_window("main") {
        let js = format!("document.documentElement.style.opacity = '{opacity}'");
        let _ = window.eval(&js);
    }
    update_config(&app, |c| c.opacity = opacity);
}

#[tauri::command]
fn get_config(state: tauri::State<'_, ConfigState>) -> Config {
    state.0.lock().unwrap().clone()
}

#[tauri::command]
fn set_character(app: tauri::AppHandle, character: String) {
    update_config(&app, |c| c.character = character.clone());
    let _ = app.emit("character-changed", character);
}

#[tauri::command]
fn set_hydration_enabled(app: tauri::AppHandle, enabled: bool) {
    update_config(&app, |c| c.hydration_enabled = enabled);
    app.state::<HydrationSignal>().signal();
}

#[tauri::command]
fn set_hydration_interval(app: tauri::AppHandle, minutes: u32) {
    update_config(&app, |c| c.hydration_interval_min = minutes);
    app.state::<HydrationSignal>().signal();
}

#[tauri::command]
fn confirm_hydration(app: tauri::AppHandle, action: String) {
    if let Some(win) = app.get_webview_window("hydration") {
        let _ = win.close();
    }
    let _ = app.emit("hydration-confirmed", action);
}

// Posiciona la bubble a la izquierda de la ventana principal, centrada
// verticalmente. Coordenadas en lógico para coherencia con el resto.
fn open_hydration_window(app: &AppHandle) {
    if let Some(win) = app.get_webview_window("hydration") {
        let _ = win.show();
        let _ = win.set_focus();
        return;
    }

    let bubble_w: f64 = 280.0;
    let bubble_h: f64 = 110.0;
    let gap: f64 = 12.0;

    let (x, y) = app
        .get_webview_window("main")
        .and_then(|m| {
            let pos = m.outer_position().ok()?;
            let size = m.outer_size().ok()?;
            let scale = m.scale_factor().ok()?;
            let main_x = pos.x as f64 / scale;
            let main_y = pos.y as f64 / scale;
            let main_h = size.height as f64 / scale;
            Some((
                main_x - bubble_w - gap,
                main_y + (main_h - bubble_h) / 2.0,
            ))
        })
        .unwrap_or((100.0, 100.0));

    let _ = tauri::WebviewWindowBuilder::new(
        app,
        "hydration",
        tauri::WebviewUrl::App("/#hydration".into()),
    )
    .title("Moji — Hidratación")
    .inner_size(bubble_w, bubble_h)
    .position(x, y)
    .resizable(false)
    .decorations(false)
    .transparent(true)
    .shadow(false)
    .always_on_top(true)
    .skip_taskbar(true)
    .build();
}

fn start_hydration_timer(app: AppHandle) {
    std::thread::spawn(move || loop {
        let enabled: bool = {
            let state = app.state::<ConfigState>();
            let v = state.0.lock().unwrap().hydration_enabled;
            v
        };

        // Toggle off: parquear el hilo indefinidamente hasta que un setter
        // (típicamente set_hydration_enabled(true)) despierte la condvar.
        if !enabled {
            let signal = app.state::<HydrationSignal>();
            let mut guard = signal.flag.lock().unwrap();
            while !*guard {
                guard = signal.cvar.wait(guard).unwrap();
            }
            *guard = false;
            continue; // re-leer config y arrancar ciclo
        }

        let interval_secs: u64 = {
            let state = app.state::<ConfigState>();
            let mins = state.0.lock().unwrap().hydration_interval_min;
            (mins as u64).saturating_mul(60).max(60)
        };

        // Espera el intervalo o hasta que un setter despierte la condvar.
        let was_signaled: bool = {
            let signal = app.state::<HydrationSignal>();
            let mut guard = signal.flag.lock().unwrap();
            if !*guard {
                let (g, _) = signal
                    .cvar
                    .wait_timeout(guard, std::time::Duration::from_secs(interval_secs))
                    .unwrap();
                guard = g;
            }
            let signaled = *guard;
            *guard = false;
            signaled
        };

        if was_signaled {
            // Config cambió (toggle o intervalo): rearmar ciclo desde cero.
            continue;
        }

        // Timeout completo sin interrupciones → disparar.
        let still_enabled: bool = {
            let state = app.state::<ConfigState>();
            let v = state.0.lock().unwrap().hydration_enabled;
            v
        };
        if still_enabled {
            let _ = app.emit("hydration-trigger", ());
            open_hydration_window(&app);
        }
    });
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
        .inner_size(300.0, 480.0)
        .resizable(false)
        .always_on_top(true)
        .build();
    }
}

fn create_main_window(app: &tauri::App, config: &Config) -> tauri::Result<()> {
    let size_logical: f64 = config.size as f64;
    let margin_logical: f64 = SCREEN_MARGIN as f64;

    // Restaurar monitor previo si sigue conectado; si no, caer a la pantalla
    // principal. work_area es físico; lo paso a lógico dividiendo por su
    // scale factor, porque WindowBuilder::position usa coordenadas lógicas.
    let monitor = app
        .available_monitors()
        .ok()
        .and_then(|monitors| monitors.into_iter().nth(config.monitor_index))
        .or_else(|| app.primary_monitor().ok().flatten());

    let (x, y) = monitor
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

fn setup_tray(app: &tauri::App, click_through: bool) -> tauri::Result<()> {
    let ghost_item = CheckMenuItem::with_id(
        app,
        "ghost",
        "Modo Fantasma",
        true,
        click_through,
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
                    .inner_size(300.0, 480.0)
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
            let config = load_config(&app.handle());

            app.manage(ClickThroughState(Mutex::new(config.click_through)));
            app.manage(ConfigState(Mutex::new(config.clone())));
            app.manage(HydrationSignal::new());

            create_main_window(app, &config)?;
            setup_tray(app, config.click_through)?;

            if config.click_through {
                if let Some(win) = app.get_webview_window("main") {
                    let _ = win.set_ignore_cursor_events(true);
                }
            }

            start_hydration_timer(app.handle().clone());

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
            open_settings,
            get_config,
            set_character,
            set_hydration_enabled,
            set_hydration_interval,
            confirm_hydration
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
