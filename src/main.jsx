import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import Settings from "./Settings";

const isSettings = window.location.hash === "#settings";

ReactDOM.createRoot(document.getElementById("root")).render(
  <React.StrictMode>
    {isSettings ? <Settings /> : <App />}
  </React.StrictMode>,
);
