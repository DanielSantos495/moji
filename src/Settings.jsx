import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { emit } from "@tauri-apps/api/event";
import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
import { LogicalSize } from "@tauri-apps/api/window";
import "./Settings.css";

function Settings() {
  const [opacity, setOpacity] = useState(1);
  const [size, setSize] = useState(200);
  const [clickThrough, setClickThrough] = useState(false);

  async function handleOpacity(val) {
    setOpacity(val);
    await emit("moji:opacity", val);
  }

  async function handleSize(val) {
    setSize(val);
    const main = await WebviewWindow.getByLabel("main");
    if (main) await main.setSize(new LogicalSize(val, val));
  }

  async function handleClickThrough(e) {
    const enabled = e.target.checked;
    setClickThrough(enabled);
    await invoke("set_click_through", { enabled });
  }

  return (
    <div className="settings">
      <div className="settings-header">
        <span className="settings-avatar">🦊</span>
        <h1>Configuración</h1>
      </div>

      <div className="settings-body">
        <div className="setting-row">
          <label>Opacidad</label>
          <div className="setting-control">
            <input
              type="range"
              min="0.2"
              max="1"
              step="0.05"
              value={opacity}
              onChange={(e) => handleOpacity(parseFloat(e.target.value))}
            />
            <span className="setting-value">{Math.round(opacity * 100)}%</span>
          </div>
        </div>

        <div className="setting-row">
          <label>Tamaño</label>
          <div className="setting-control">
            <input
              type="range"
              min="100"
              max="400"
              step="10"
              value={size}
              onChange={(e) => handleSize(parseInt(e.target.value))}
            />
            <span className="setting-value">{size}px</span>
          </div>
        </div>

        <div className="setting-row">
          <label>
            No interrumpir
            <span className="setting-hint">Kael no bloquea clics</span>
          </label>
          <label className="toggle">
            <input
              type="checkbox"
              checked={clickThrough}
              onChange={handleClickThrough}
            />
            <span className="toggle-slider" />
          </label>
        </div>
      </div>

      <p className="settings-footer">
        Para salir de Moji usa el ícono en la barra de menú.
      </p>
    </div>
  );
}

export default Settings;
