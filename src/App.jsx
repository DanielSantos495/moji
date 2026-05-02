import { useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { listen } from "@tauri-apps/api/event";
import "./App.css";

const win = getCurrentWindow();

function App() {
  useEffect(() => {
    const unlisten = listen("moji:opacity", (e) => {
      document.documentElement.style.opacity = e.payload;
    });
    return () => unlisten.then((fn) => fn());
  }, []);

  function handleDrag(e) {
    e.preventDefault();
    win.startDragging();
  }

  function openSettings() {
    invoke("open_settings");
  }

  return (
    <div className="moji-window">
      <div className="kael" onClick={() => console.log("Kael touched!")}>
        🦊
      </div>
      <div className="corner-handle">
        <div className="drag-handle" onMouseDown={handleDrag} title="Mover">
          ⠿
        </div>
        <button className="settings-btn" onClick={openSettings} title="Configuración">
          ⚙
        </button>
      </div>
    </div>
  );
}

export default App;
