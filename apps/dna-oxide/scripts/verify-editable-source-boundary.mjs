import { execFileSync } from "node:child_process";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { containsForbiddenClaimToken, createInstrumentedDnaOxIdeApp } from "../src/app-instrumentation.js";
import {
  createEditableSourceBoundary,
  verifyEditableSourceBoundaryContract
} from "../src/editable-source-boundary.js";

const here = dirname(fileURLToPath(import.meta.url));
const appRoot = resolve(here, "..");
const repoRoot = resolve(appRoot, "..", "..");
const targetDir = resolve(repoRoot, "target");

const artifacts = {
  before: resolve(targetDir, "w350-b02-editable-source-before.json"),
  afterEdit: resolve(targetDir, "w350-b02-editable-source-after-edit.json"),
  afterSave: resolve(targetDir, "w350-b02-editable-source-after-save.json"),
  afterReload: resolve(targetDir, "w350-b02-editable-source-after-reload.json"),
  appSnapshot: resolve(targetDir, "w350-b02-instrumented-app-source-snapshot.json"),
  evidence: resolve(targetDir, "w350-b02-editable-source-boundary.txt")
};

function assert(condition, message) {
  if (!condition) {
    throw new Error(message);
  }
}

function assertNoOverclaim(label, value) {
  assert(!containsForbiddenClaimToken(value), `${label} contains forbidden overclaim token`);
}

function assertNoHostCoupling(relativePath) {
  const text = readFileSync(resolve(appRoot, relativePath), "utf8");
  const forbidden = ["@tauri-apps", "__TAURI__", "invoke(", "window.__TAURI__"];
  for (const token of forbidden) {
    assert(!text.includes(token), `${relativePath} contains forbidden host coupling token: ${token}`);
  }
}

console.log("== W350-B02 editable source boundary verifier ==");
console.log(`repoRoot=${repoRoot}`);

assert(verifyEditableSourceBoundaryContract(), "editable source boundary contract failed");

const initialSource = "Attribute VB_Name = \"Module1\"\nOption Explicit\n\nPublic Sub Main()\nEnd Sub\n";
const boundary = createEditableSourceBoundary({ sourceText: initialSource });
const before = boundary.snapshot();
assert(before.dirty === false, "initial source boundary must be clean");
assert(before.boundary.hostNeutral === true, "source boundary must be host-neutral");
assert(before.boundary.tauriCoupled === false, "source boundary must not be Tauri-coupled");
assert(before.commandStates.saveActiveModule === "enabled-clean-noop", "clean save command state mismatch");

boundary.applyInputEvent({ type: "input", text: `${initialSource}\n' B02 edit` }, { via: "input-event" });
const afterEdit = boundary.snapshot();
assert(afterEdit.dirty === true, "input event must make source dirty");
assert(afterEdit.sourceText.includes("B02 edit"), "edited source missing marker");
assert(afterEdit.commandStates.saveActiveModule === "enabled-dirty", "dirty save command state mismatch");

boundary.saveToPersisted({ commandName: "save-active-module" });
const afterSave = boundary.snapshot();
assert(afterSave.dirty === false, "save must make boundary clean");
assert(afterSave.persistedSourceText.includes("B02 edit"), "persisted source missing marker");

boundary.replaceSource("temporary divergent text", { via: "reload-check" });
assert(boundary.snapshot().dirty === true, "divergent text must be dirty before reload");
boundary.reloadFromPersisted({ commandName: "reload-active-module" });
const afterReload = boundary.snapshot();
assert(afterReload.dirty === false, "reload must restore clean state");
assert(afterReload.sourceText === afterSave.persistedSourceText, "reload must restore persisted text");
assert(afterReload.lastReloadedSourceText === afterSave.persistedSourceText, "lastReloadedSourceText mismatch");

const app = createInstrumentedDnaOxIdeApp({ sourceText: initialSource });
app.injectInteraction({ type: "replaceSource", text: `${initialSource}\n' app B02 edit` });
const appSnapshot = app.snapshot();
assert(appSnapshot.dirty === true, "instrumented app must expose dirty source boundary state");
assert(appSnapshot.editableSourceBoundary.hostNeutral === true, "app snapshot missing host-neutral source boundary");
assert(appSnapshot.lifecycleCommandStates.saveActiveModule === "enabled-dirty", "app lifecycle state did not come from editable source boundary");

assertNoHostCoupling("src/editable-source-boundary.js");
assertNoHostCoupling("src/app-instrumentation.js");

for (const [label, value] of [
  ["before", JSON.stringify(before)],
  ["afterEdit", JSON.stringify(afterEdit)],
  ["afterSave", JSON.stringify(afterSave)],
  ["afterReload", JSON.stringify(afterReload)],
  ["appSnapshot", JSON.stringify(appSnapshot)]
]) {
  assertNoOverclaim(label, value);
}

console.log("== Rust editor/lifecycle core smoke ==");
const rustEditorOutput = execFileSync("cargo", ["test", "--manifest-path", resolve(repoRoot, "crates", "Cargo.toml"), "-p", "oxide-editor-core"], {
  cwd: repoRoot,
  encoding: "utf8"
});
const rustCoreOutput = execFileSync("cargo", ["test", "--manifest-path", resolve(repoRoot, "crates", "Cargo.toml"), "-p", "oxide-core"], {
  cwd: repoRoot,
  encoding: "utf8"
});

mkdirSync(targetDir, { recursive: true });
writeFileSync(artifacts.before, `${JSON.stringify(before, null, 2)}\n`, "utf8");
writeFileSync(artifacts.afterEdit, `${JSON.stringify(afterEdit, null, 2)}\n`, "utf8");
writeFileSync(artifacts.afterSave, `${JSON.stringify(afterSave, null, 2)}\n`, "utf8");
writeFileSync(artifacts.afterReload, `${JSON.stringify(afterReload, null, 2)}\n`, "utf8");
writeFileSync(artifacts.appSnapshot, `${JSON.stringify(appSnapshot, null, 2)}\n`, "utf8");
writeFileSync(artifacts.evidence, [
  "W350-B02 editable source component/model boundary evidence",
  "",
  `repoRoot=${repoRoot}`,
  "boundary=apps/dna-oxide/src/editable-source-boundary.js",
  "hostNeutral=true",
  "tauriCoupled=false",
  "sharedUiCoupledToTauri=false",
  "editableSourceStateModeled=true",
  "inputEventDirtyState=true",
  "saveCleanState=true",
  "reloadRestoresPersistedSource=true",
  "instrumentationSnapshotCarriesBoundary=true",
  "runtimeExecutionClaimed=false",
  "nativeRuntimeClaimed=false",
  "comRuntimeClaimed=false",
  "fakeResponses=false",
  "fakeDebugData=false",
  "",
  "Artifacts:",
  `- ${artifacts.before}`,
  `- ${artifacts.afterEdit}`,
  `- ${artifacts.afterSave}`,
  `- ${artifacts.afterReload}`,
  `- ${artifacts.appSnapshot}`,
  "",
  "Rust checks:",
  "cargo test --manifest-path crates/Cargo.toml -p oxide-editor-core",
  rustEditorOutput.trim(),
  "cargo test --manifest-path crates/Cargo.toml -p oxide-core",
  rustCoreOutput.trim(),
  "",
  "Status: PASS"
].join("\n"), "utf8");

console.log(`W350-B02 editable source boundary verification passed: ${artifacts.evidence}`);
