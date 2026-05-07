import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import MojiSprite from "./components/MojiSprite";
import { DEFAULT_CHARACTER } from "./components/MojiSprite/registry";
import "./App.css";

function App() {
  const [character, setCharacter] = useState(DEFAULT_CHARACTER);

  useEffect(() => {
    invoke("get_config").then((c) => {
      if (c?.opacity != null) {
        document.documentElement.style.opacity = String(c.opacity);
      }
      if (c?.character) setCharacter(c.character);
    });

    const unlisten = listen("character-changed", (event) => {
      setCharacter(event.payload);
    });
    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  return (
    <div className="moji-window">
      <MojiSprite character={character} />
    </div>
  );
}

export default App;
