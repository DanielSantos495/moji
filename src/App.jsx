import { useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import MojiSprite from "./components/MojiSprite";
import "./App.css";

function App() {
  useEffect(() => {
    invoke("get_config").then((c) => {
      if (c?.opacity != null) {
        document.documentElement.style.opacity = String(c.opacity);
      }
    });
  }, []);

  return (
    <div className="moji-window">
      <MojiSprite onClick={() => console.log("Kael touched!")} />
    </div>
  );
}

export default App;
