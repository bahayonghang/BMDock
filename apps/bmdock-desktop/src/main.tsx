import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import App from "./App";
import { LOCALE } from "./i18n";
import "./styles.css";

document.documentElement.lang = LOCALE;

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <App />
  </StrictMode>,
);
