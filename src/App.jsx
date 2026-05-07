import MojiSprite from "./components/MojiSprite";
import "./App.css";

function App() {
  return (
    <div className="moji-window">
      <MojiSprite onClick={() => console.log("Kael touched!")} />
    </div>
  );
}

export default App;
