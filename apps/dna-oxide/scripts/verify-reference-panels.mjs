import { readFileSync } from "node:fs";
import { resolve, dirname } from "node:path";
import { fileURLToPath } from "node:url";
import {
  renderReferenceComPanels,
  verifyReferenceComPanelContract
} from "../src/placeholder-panels.js";

const here = dirname(fileURLToPath(import.meta.url));
const appRoot = resolve(here, "..");
const repoRoot = resolve(appRoot, "..", "..");
const hostRenderPath = resolve(repoRoot, "target", "w345-host-shell-render.html");

const panelMarkup = renderReferenceComPanels();
if (!verifyReferenceComPanelContract(panelMarkup)) {
  console.error("DNA OxIde reference/COM panel contract verification failed");
  process.exit(1);
}

const hostMarkup = readFileSync(hostRenderPath, "utf8");
for (const token of [
  "role=\"host-reference-com-deck\"",
  "role=\"host-references-panel\"",
  "role=\"host-com-candidate-panel\"",
  "role=\"host-reference-repair-panel\"",
  "role=\"host-com-runtime-boundary-panel\"",
  "data-state=\"oxvba-fixture-evidenced\"",
  "data-state=\"pending-oxvba-hardening\"",
  "data-state=\"unavailable-no-claim\"",
  "ComSelectionService direct Rust surface",
  "data-roster-rows=\"0\"",
  "data-candidate-rows=\"0\"",
  "data-preview-rows=\"0\"",
  "data-com-runtime-invocation=\"false\"",
  "COM runtime invocation is not claimed",
  "data-com-runtime=\"false\""
]) {
  if (!hostMarkup.includes(token)) {
    console.error(`DnaOxIde host render missing reference/COM token: ${token}`);
    process.exit(1);
  }
}

console.log("DNA OxIde reference/COM panel verification passed");
