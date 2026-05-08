import { mkdirSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import {
  W350_TEST_DRIVER_GLOBAL,
  containsForbiddenClaimToken,
  createInstrumentedDnaOxIdeApp,
  installDnaOxIdeTestDriver,
  verifyInstrumentationContract
} from "../src/app-instrumentation.js";

const here = dirname(fileURLToPath(import.meta.url));
const appRoot = resolve(here, "..");
const repoRoot = resolve(appRoot, "..", "..");
const targetDir = resolve(repoRoot, "target");
const require = createRequire(import.meta.url);

const artifacts = {
  before: resolve(targetDir, "w350-b01-snapshot-before.json"),
  after: resolve(targetDir, "w350-b01-snapshot-after.json"),
  events: resolve(targetDir, "w350-b01-events.json"),
  commands: resolve(targetDir, "w350-b01-commands.json"),
  visual: resolve(targetDir, "w350-b01-app-instrumentation.html"),
  markup: resolve(targetDir, "w350-b01-app-markup.html"),
  evidence: resolve(targetDir, "w350-b01-app-instrumentation.txt")
};

function assert(condition, message) {
  if (!condition) {
    throw new Error(message);
  }
}

function assertNoOverclaimText(label, text) {
  assert(!containsForbiddenClaimToken(text), `${label} contains a forbidden overclaim token`);
}

console.log("== W350-B01 app instrumentation verifier ==");
console.log(`repoRoot=${repoRoot}`);

const playwrightPackagePath = require.resolve("@playwright/test/package.json");
assert(playwrightPackagePath.includes("node_modules"), "@playwright/test must resolve from local node_modules");
assert(verifyInstrumentationContract(), "verifyInstrumentationContract failed");

const windowLike = {};
const app = createInstrumentedDnaOxIdeApp();
installDnaOxIdeTestDriver(windowLike, app);
assert(windowLike[W350_TEST_DRIVER_GLOBAL] === app, "test driver global was not installed");

const before = app.snapshot();
assert(before.dirty === false, "initial snapshot must be clean");
assert(before.proofMode === "browser-dom-playwright-primary", "wrong proof mode");
assert(before.instrumentation.visualArtifacts === true, "visualArtifacts not enabled");
assert(before.instrumentation.domLikeSnapshots === true, "domLikeSnapshots not enabled");
assert(before.instrumentation.commandLog === true, "commandLog not enabled");
assert(before.instrumentation.eventLog === true, "eventLog not enabled");
assert(before.instrumentation.interactionInjection === true, "interactionInjection not enabled");
assert(before.instrumentation.liveTauriWebViewIpcDriven === false, "must not claim live Tauri/WebView IPC");

const editedText = `${before.sourceText}\n' W350-B01 injected edit`;
app.injectInteraction({ type: "focusEditor", via: "verify-app-instrumentation" });
app.injectInteraction({ type: "replaceSource", text: editedText, via: "verify-app-instrumentation" });
const afterEdit = app.snapshot();
assert(afterEdit.editorFocused === true, "editor focus was not observable");
assert(afterEdit.dirty === true, "edited snapshot must be dirty");
assert(afterEdit.sourceText.includes("W350-B01 injected edit"), "edited source text missing injected marker");

app.runCommand("save-active-module", { artifact: "w350-b01" });
const afterSave = app.snapshot();
assert(afterSave.dirty === false, "saved snapshot must be clean");
assert(afterSave.persistedSourceText.includes("W350-B01 injected edit"), "persisted source missing injected marker");

const events = app.eventLog();
const commands = app.commandLog();
const markup = app.renderApp();
const visual = app.visualSnapshot();

assert(events.some((event) => event.kind === "editor-focused"), "event log missing editor-focused");
assert(events.some((event) => event.kind === "source-replaced"), "event log missing source-replaced");
assert(events.some((event) => event.kind === "source-saved-to-temp-copy"), "event log missing source-saved-to-temp-copy");
assert(commands.some((command) => command.commandName === "save-active-module"), "command log missing save-active-module");
assert(markup.includes('data-testid="dnaoxide-w350-app"'), "markup missing app test id");
assert(markup.includes('data-testid="source-editor"'), "markup missing source editor test id");
assert(markup.includes('__DNA_OXIDE_TEST_DRIVER__') === false, "markup should not inline driver global name");
assert(visual.includes('role="dnaoxide-w350-visual-snapshot"'), "visual artifact missing visual snapshot role");
assert(visual.includes("W350-B01 injected edit"), "visual artifact missing injected edit");

for (const [label, value] of [
  ["before", JSON.stringify(before)],
  ["after", JSON.stringify(afterSave)],
  ["events", JSON.stringify(events)],
  ["commands", JSON.stringify(commands)],
  ["markup", markup],
  ["visual", visual]
]) {
  assertNoOverclaimText(label, value);
}

mkdirSync(targetDir, { recursive: true });
writeFileSync(artifacts.before, `${JSON.stringify(before, null, 2)}\n`, "utf8");
writeFileSync(artifacts.after, `${JSON.stringify(afterSave, null, 2)}\n`, "utf8");
writeFileSync(artifacts.events, `${JSON.stringify(events, null, 2)}\n`, "utf8");
writeFileSync(artifacts.commands, `${JSON.stringify(commands, null, 2)}\n`, "utf8");
writeFileSync(artifacts.visual, visual, "utf8");
writeFileSync(artifacts.markup, markup, "utf8");
writeFileSync(artifacts.evidence, [
  "W350-B01 app observability and interaction instrumentation evidence",
  "",
  `repoRoot=${repoRoot}`,
  "driverGlobal=window.__DNA_OXIDE_TEST_DRIVER__",
  "proofMode=browser-dom-playwright-primary",
  "playwrightPrimary=true",
  `playwrightPackage=${playwrightPackagePath}`,
  "nodeInstrumentationVerifier=true",
  "visualArtifacts=true",
  "domLikeSnapshots=true",
  "eventLog=true",
  "commandLog=true",
  "interactionInjection=true",
  "liveTauriWebViewIpcDriven=false",
  "fullDomAccessibilityAuditClaimed=false",
  "realExecutionClaimed=false",
  "nativeRuntimeClaimed=false",
  "comRuntimeClaimed=false",
  "fakeResponses=false",
  "fakeDebugData=false",
  "",
  "Artifacts:",
  `- ${artifacts.before}`,
  `- ${artifacts.after}`,
  `- ${artifacts.events}`,
  `- ${artifacts.commands}`,
  `- ${artifacts.visual}`,
  `- ${artifacts.markup}`,
  "",
  "Observed interaction loop:",
  `- before.dirty=${before.dirty}`,
  `- afterEdit.dirty=${afterEdit.dirty}`,
  `- afterSave.dirty=${afterSave.dirty}`,
  `- eventKinds=${events.map((event) => event.kind).join(",")}`,
  `- commandNames=${commands.map((command) => command.commandName).join(",")}`,
  "",
  "Status: PASS"
].join("\n"), "utf8");

console.log(`W350-B01 instrumentation verification passed: ${artifacts.evidence}`);
