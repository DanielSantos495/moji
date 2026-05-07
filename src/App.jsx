import { useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import MojiSprite from "./components/MojiSprite";
import { DEFAULT_CHARACTER } from "./components/MojiSprite/registry";
import "./App.css";

function App() {
  const [character, setCharacter] = useState(DEFAULT_CHARACTER);
  const mojiRef = useRef(null);

  useEffect(() => {
    invoke("get_config").then((c) => {
      if (c?.opacity != null) {
        document.documentElement.style.opacity = String(c.opacity);
      }
      if (c?.character) setCharacter(c.character);
    });

    const unlistenCharacter = listen("character-changed", (event) => {
      setCharacter(event.payload);
    });
    const unlistenHydration = listen("hydration-trigger", () => {
      mojiRef.current?.trigger("hydration");
    });
    const unlistenConfirmed = listen("hydration-confirmed", (event) => {
      if (event.payload === "drank") {
        mojiRef.current?.trigger("celebrate");
      } else {
        mojiRef.current?.trigger("default");
      }
    });

    return () => {
      unlistenCharacter.then((fn) => fn());
      unlistenHydration.then((fn) => fn());
      unlistenConfirmed.then((fn) => fn());
    };
  }, []);

  return (
    <div className="moji-window">
      <MojiSprite ref={mojiRef} character={character} />
    </div>
  );
}

export default App;
