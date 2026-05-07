import { invoke } from "@tauri-apps/api/core";
import "./Hydration.css";

function Hydration() {
  const respond = (action) => invoke("confirm_hydration", { action });

  return (
    <div className="hydration-bubble">
      <p className="hydration-msg">
        <span className="hydration-icon">💧</span>
        Kael lo notó — llevas un rato sin tomar agua.
      </p>
      <div className="hydration-actions">
        <button className="hydration-btn primary" onClick={() => respond("drank")}>
          Sí, ya tomé
        </button>
        <button className="hydration-btn" onClick={() => respond("later")}>
          Voy ahora
        </button>
      </div>
    </div>
  );
}

export default Hydration;
