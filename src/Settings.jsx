import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { CHARACTERS, DEFAULT_CHARACTER } from "./components/MojiSprite/registry";
import "./Settings.css";

function Settings() {
  const [opacity, setOpacity] = useState(1);
  const [size, setSize] = useState(120);
  const [clickThrough, setClickThrough] = useState(false);
  const [character, setCharacter] = useState(DEFAULT_CHARACTER);

  useEffect(() => {
    invoke("get_config").then((c) => {
      if (!c) return;
      setOpacity(c.opacity);
      setSize(c.size);
      setClickThrough(c.click_through);
      if (c.character) setCharacter(c.character);
    });

    const unlisten = listen("click-through-changed", (event) => {
      setClickThrough(event.payload);
    });
    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  async function handleOpacity(val) {
    setOpacity(val);
    await invoke("set_main_opacity", { opacity: val });
  }

  async function handleSize(val) {
    setSize(val);
    await invoke("set_main_size", { size: val });
  }

  async function handleClickThrough(e) {
    const enabled = e.target.checked;
    setClickThrough(enabled);
    await invoke("set_click_through", { enabled });
  }

  async function handleCharacter(value) {
    setCharacter(value);
    await invoke("set_character", { character: value });
  }

  return (
    <div className="settings">
      <div className="settings-header">
        <span className="settings-avatar">🦊</span>
        <h1>Configuración</h1>
      </div>

      <div className="settings-body">
        <div className="setting-row">
          <label>Personaje</label>
          <div className="setting-control">
            <select
              className="setting-select"
              value={character}
              onChange={(e) => handleCharacter(e.target.value)}
            >
              {CHARACTERS.map((name) => (
                <option key={name} value={name}>
                  {name.charAt(0).toUpperCase() + name.slice(1)}
                </option>
              ))}
            </select>
          </div>
        </div>

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
              min="120"
              max="150"
              step="1"
              value={size}
              onChange={(e) => handleSize(parseInt(e.target.value))}
            />
            <span className="setting-value">{size}px</span>
          </div>
        </div>

        <div className="setting-row">
          <label>
            Modo Fantasma
            <span className="setting-hint">Los clics atraviesan a Kael</span>
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
