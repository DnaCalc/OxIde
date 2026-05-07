import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
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
const targetDir = resolve(repoRoot, "target");
const targetFile = resolve(targetDir, "w345-host-shell-render.html");
const thinSliceModule = resolve(repoRoot, "examples", "thin-slice", "Module1.bas");

const sourceText = readFileSync(thinSliceModule, "utf8");
const model = createDnaOxIdeHostShellModel(createBrowserFixtureCommandClient(), {
  sourceText,
  sourceProvenance: "read-only-check-of-examples/thin-slice/Module1.bas"
});
const markup = renderDnaOxIdeHostShell(model);

if (!verifyHostShellContract(markup)) {
  console.error("DNA OxIde host shell contract verification failed");
  process.exit(1);
}

const requiredTokens = [
  "role=\"dnaoxide-host-ui-proof\"",
  "data-proof-mode=\"static-frontend-host-fixture\"",
  "data-shared-ui-crate=\"oxide-ui-leptos\"",
  "data-host-bridge-crate=\"oxide-host-bridge\"",
  "DNA OxIde",
  "ThinSliceHello",
  "Module1.bas",
  "Public Sub Main()",
  "role=\"host-lifecycle-panel\"",
  "role=\"host-command-palette\"",
  "role=\"host-runtime-panel\"",
  "role=\"host-immediate-panel\"",
  "role=\"host-debug-panel\"",
  "role=\"host-com-panel\"",
  "native-service",
  "oxvba-fixture-evidenced",
  "pending-oxvba-hardening",
  "data-real-execution=\"false\"",
  "data-native-runtime=\"false\"",
  "data-com-runtime=\"false\"",
  "data-fake-responses=\"false\"",
  "data-fake-debug-data=\"false\""
];

for (const token of requiredTokens) {
  if (!markup.includes(token)) {
    console.error(`DNA OxIde host shell markup missing token: ${token}`);
    process.exit(1);
  }
}

const forbiddenTokens = [
  "data-real-execution=\"true\"",
  "data-native-runtime=\"true\"",
  "data-com-runtime=\"true\"",
  "data-fake-responses=\"true\"",
  "data-fake-debug-data=\"true\""
];

for (const token of forbiddenTokens) {
  if (markup.includes(token)) {
    console.error(`DNA OxIde host shell markup contains forbidden token: ${token}`);
    process.exit(1);
  }
}

mkdirSync(targetDir, { recursive: true });
writeFileSync(targetFile, markup, "utf8");
console.log(`DNA OxIde host shell verification passed: ${targetFile}`);
