import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import Settings from "./Settings";
import Hydration from "./Hydration";

const route = window.location.hash.replace("#", "");

const view =
  route === "settings" ? <Settings /> :
  route === "hydration" ? <Hydration /> :
  <App />;

ReactDOM.createRoot(document.getElementById("root")).render(
  <React.StrictMode>{view}</React.StrictMode>,
);
