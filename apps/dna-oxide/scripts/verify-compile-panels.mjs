import { readFileSync } from "node:fs";
import { resolve, dirname } from "node:path";
import { fileURLToPath } from "node:url";
import {
  renderCompileOptionsPanels,
  verifyCompilePanelContract
} from "../src/placeholder-panels.js";

const here = dirname(fileURLToPath(import.meta.url));
const appRoot = resolve(here, "..");
const repoRoot = resolve(appRoot, "..", "..");
const hostRenderPath = resolve(repoRoot, "target", "w345-host-shell-render.html");

const panelMarkup = renderCompileOptionsPanels();
if (!verifyCompilePanelContract(panelMarkup)) {
  console.error("DNA OxIde compile/options panel contract verification failed");
  process.exit(1);
}

const hostMarkup = readFileSync(hostRenderPath, "utf8");
for (const token of [
  "role=\"host-project-properties-panel\"",
  "role=\"host-compile-options-panel\"",
  "role=\"host-build-check-panel\"",
  "role=\"host-run-target-panel\"",
  "data-state=\"pending-oxvba-hardening\"",
  "data-state=\"oxvba-fixture-evidenced\"",
  "data-final-oxvba-dtos-owned-here=\"false\"",
  "EmbeddedBuildRunHost::build_workspace",
  "Final OxVba compile options DTO",
  "data-output-rows=\"0\"",
  "data-real-execution=\"false\"",
  "data-native-runtime=\"false\"",
  "data-com-runtime=\"false\""
]) {
  if (!hostMarkup.includes(token)) {
    console.error(`DnaOxIde host render missing compile panel token: ${token}`);
    process.exit(1);
  }
}

console.log("DNA OxIde compile/options panel verification passed");
