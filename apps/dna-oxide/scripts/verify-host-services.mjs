import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import {
  createDnaOxIdeHostShellModel,
  renderDnaOxIdeHostShell,
  verifyHostShellContract
} from "../src/host-shell.js";
import { createBrowserFixtureCommandClient } from "../src/command-client.js";

const here = dirname(fileURLToPath(import.meta.url));
const appRoot = resolve(here, "..");
const repoRoot = resolve(appRoot, "..", "..");
const thinSliceModule = resolve(repoRoot, "examples", "thin-slice", "Module1.bas");

const model = createDnaOxIdeHostShellModel(createBrowserFixtureCommandClient(), {
  sourceText: readFileSync(thinSliceModule, "utf8"),
  sourceProvenance: "read-only-service-proof-source"
});
const markup = renderDnaOxIdeHostShell(model);

if (!verifyHostShellContract(markup)) {
  console.error("DNA OxIde unavailable service proof failed base host shell contract");
  process.exit(1);
}

const requiredTokens = [
  "role=\"host-runtime-panel\"",
  "data-command=\"dna_oxide_run_project\"",
  "data-state=\"oxvba-fixture-evidenced\"",
  "data-provider=\"native-service-missing\"",
  "data-output-events=\"0\"",
  "data-runtime-id=\"\"",
  "role=\"host-immediate-panel\"",
  "data-command=\"dna_oxide_evaluate_immediate\"",
  "data-immediate-responses=\"0\"",
  "data-immediate-session-id=\"\"",
  "No Immediate responses are synthesized",
  "role=\"host-debug-panel\"",
  "data-command=\"dna_oxide_debug_attach\"",
  "data-callstack-frames=\"0\"",
  "data-locals=\"0\"",
  "data-watches=\"0\"",
  "data-breakpoints=\"0\"",
  "data-debug-session-id=\"\"",
  "No callstack, locals, watches, breakpoints",
  "role=\"host-com-panel\"",
  "data-command=\"dna_oxide_find_com_candidates\"",
  "data-com-candidates=\"0\"",
  "data-com-runtime-invocation=\"false\"",
  "COM runtime invocation is not claimed",
  "data-real-execution=\"false\"",
  "data-native-runtime=\"false\"",
  "data-com-runtime=\"false\"",
  "data-fake-responses=\"false\"",
  "data-fake-debug-data=\"false\""
];

for (const token of requiredTokens) {
  if (!markup.includes(token)) {
    console.error(`DNA OxIde unavailable service proof missing token: ${token}`);
    process.exit(1);
  }
}

const forbiddenTokens = [
  "data-output-events=\"1\"",
  "data-immediate-responses=\"1\"",
  "data-callstack-frames=\"1\"",
  "data-locals=\"1\"",
  "data-watches=\"1\"",
  "data-breakpoints=\"1\"",
  "data-com-runtime-invocation=\"true\"",
  "data-real-execution=\"true\"",
  "data-native-runtime=\"true\"",
  "data-com-runtime=\"true\"",
  "data-fake-responses=\"true\"",
  "data-fake-debug-data=\"true\""
];

for (const token of forbiddenTokens) {
  if (markup.includes(token)) {
    console.error(`DNA OxIde unavailable service proof contains forbidden token: ${token}`);
    process.exit(1);
  }
}

console.log("DNA OxIde unavailable service proof passed");
