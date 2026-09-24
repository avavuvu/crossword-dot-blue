import { attachAutosave } from "./editor/autosave";
import { attachAll } from "./editor/highlight";

attachAll(document);
attachAutosave();

document.addEventListener("htmx:after:settle", () => attachAll(document));
