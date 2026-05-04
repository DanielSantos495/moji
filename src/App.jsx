import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import MojiSprite from "./components/MojiSprite";
import "./App.css";

const win = getCurrentWindow();

function App() {

  function handleDrag(e) {
    e.preventDefault();
    win.startDragging();
  }

  function openSettings() {
    invoke("open_settings");
  }

  return (
    <div className="moji-window">
      <MojiSprite onClick={() => console.log("Kael touched!")} />
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
