import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [opacity, setOpacity] = useState(1);
  const [clickThrough, setClickThrough] = useState(false);

  useEffect(() => {
    document.documentElement.style.opacity = opacity;
  }, [opacity]);

  function toggleClickThrough() {
    const next = !clickThrough;
    setClickThrough(next);
    invoke("set_click_through", { enabled: next });
  }

  return (
    <div className="moji-window">
      <div
        className="kael-placeholder"
        onClick={() => console.log("Kael touched!")}
        title="Kael"
      >
        🦊
      </div>

      <div className="controls">
        <input
          type="range"
          min="0.2"
          max="1"
          step="0.05"
          value={opacity}
          onChange={(e) => setOpacity(parseFloat(e.target.value))}
          title="Opacidad"
        />
        <button onClick={toggleClickThrough}>
          {clickThrough ? "🔓 Click-through ON" : "🔒 Click-through OFF"}
        </button>
      </div>
    </div>
  );
}

export default App;
