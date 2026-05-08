import { execFileSync } from "node:child_process";
import { existsSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { containsForbiddenClaimToken } from "../src/app-instrumentation.js";

const here = dirname(fileURLToPath(import.meta.url));
const appRoot = resolve(here, "..");
const repoRoot = resolve(appRoot, "..", "..");
const targetDir = resolve(repoRoot, "target");

const appChecks = [
  ["command-client:check", "verify-command-client.mjs"],
  ["host-ui:check", "verify-host-ui.mjs"],
  ["host-lifecycle:check", "verify-host-lifecycle.mjs"],
  ["host-services:check", "verify-host-services.mjs"],
  ["interaction-command:check", "verify-interaction-command.mjs"],
  ["interaction-focus:check", "verify-interaction-focus.mjs"],
  ["interaction-lifecycle:check", "verify-interaction-lifecycle.mjs"],
  ["interaction-services:check", "verify-interaction-services.mjs"],
  ["compile-panels:check", "verify-compile-panels.mjs"],
  ["reference-panels:check", "verify-reference-panels.mjs"],
  ["placeholder-commands:check", "verify-placeholder-commands.mjs"],
  ["live-edit:check", "verify-live-edit-feedback-loop.mjs"]
];

const requiredArtifacts = [
  "w350-b01-app-instrumentation.html",
  "w350-b01-snapshot-before.json",
  "w350-b01-snapshot-after.json",
  "w350-b02-editable-source-after-reload.json",
  "w350-b03-live-editable-host.html",
  "w350-b03-live-editable-host.png",
  "w350-b04-live-save-reload.html",
  "w350-b04-live-save-reload.png",
  "w350-b04-saved-Module1.bas",
  "w350-b05-live-edit-feedback-loop.txt"
];

const commandOutputFile = resolve(targetDir, "w350-acceptance-commands.txt");
const evidenceFile = resolve(targetDir, "w350-acceptance.txt");
const handoffFile = resolve(repoRoot, "docs", "HANDOFF_W350_LIVE_EDITABLE_SOURCE_APP.md");

function assert(condition, message) {
  if (!condition) {
    throw new Error(message);
  }
}

function runCommand(label, command, args, options = {}) {
  console.log(`running ${label}`);
  const output = execFileSync(command, args, {
    cwd: repoRoot,
    encoding: "utf8",
    stdio: ["ignore", "pipe", "pipe"],
    ...options
  });
  return `$ ${label}\n${output.trim()}`;
}

function readTarget(relative) {
  return readFileSync(resolve(targetDir, relative), "utf8");
}

function readJson(relative) {
  return JSON.parse(readTarget(relative));
}

console.log("== W350 acceptance verifier ==");
console.log(`repoRoot=${repoRoot}`);

const transcripts = [];
for (const [label, script] of appChecks) {
  transcripts.push(runCommand(`npm --prefix apps/dna-oxide run ${label}`, process.execPath, [resolve(appRoot, "scripts", script)], { cwd: appRoot }));
}
transcripts.push(runCommand(
  "cargo test --manifest-path apps/dna-oxide/src-tauri/Cargo.toml",
  "cargo",
  ["test", "--manifest-path", resolve(appRoot, "src-tauri", "Cargo.toml")]
));
transcripts.push(runCommand(
  "cargo test --manifest-path crates/Cargo.toml --workspace",
  "cargo",
  ["test", "--manifest-path", resolve(repoRoot, "crates", "Cargo.toml"), "--workspace"]
));
writeFileSync(commandOutputFile, `${transcripts.join("\n\n")}\n`, "utf8");

for (const artifact of requiredArtifacts) {
  const absolute = resolve(targetDir, artifact);
  assert(existsSync(absolute), `required artifact missing: ${artifact}`);
  assert(statSync(absolute).size > 0, `required artifact empty: ${artifact}`);
}

const b03After = readJson("w350-b03-live-host-after-input.json");
const b04AfterReload = readJson("w350-b04-after-reload.json");
const b04Commands = readJson("w350-b04-commands.json");
const b04Events = readJson("w350-b04-events.json");
const savedModule = readTarget("w350-b04-saved-Module1.bas");
const b05Evidence = readTarget("w350-b05-live-edit-feedback-loop.txt");

assert(b03After.dirty === true, "B03 after input must be dirty");
assert(b03After.sourceText.includes("W350-B03 browser DOM input"), "B03 browser input marker missing");
assert(b04AfterReload.dirty === false, "B04 after reload must be clean");
assert(b04AfterReload.sourceText.includes("W350-B04 saved through temp project copy"), "B04 reload missing saved marker");
assert(!b04AfterReload.sourceText.includes("unsaved divergent"), "B04 reload retained divergent text");
assert(savedModule.includes("W350-B04 saved through temp project copy"), "saved temp module missing marker");
assert(!savedModule.includes("unsaved divergent"), "saved temp module includes divergent text");
assert(b04Commands.some((command) => command.commandName === "save-active-module"), "command log missing save");
assert(b04Commands.some((command) => command.commandName === "reload-active-module"), "command log missing reload");
assert(b04Events.some((event) => event.kind === "source-saved-to-temp-copy"), "event log missing save");
assert(b04Events.some((event) => event.kind === "source-reloaded-from-temp-copy"), "event log missing reload");
assert(b05Evidence.includes("Status: PASS"), "B05 evidence did not pass");
assert(existsSync(handoffFile), "W350 handoff file missing");
assert(readFileSync(handoffFile, "utf8").includes("Status: `w350_accepted`"), "W350 handoff status is not accepted");

for (const artifact of requiredArtifacts.filter((path) => !path.endsWith(".png"))) {
  assert(!containsForbiddenClaimToken(readTarget(artifact)), `${artifact} contains forbidden overclaim token`);
}

for (const relative of [
  "src/main.js",
  "src/app-instrumentation.js",
  "src/editable-source-boundary.js",
  "src/command-client.js"
]) {
  const text = readFileSync(resolve(appRoot, relative), "utf8");
  for (const forbidden of ["@tauri-apps", "__TAURI__", "window.__TAURI__"]) {
    assert(!text.includes(forbidden), `${relative} contains forbidden host coupling token: ${forbidden}`);
  }
}

execFileSync("git", ["diff", "--quiet", "--", "examples/thin-slice/Module1.bas"], {
  cwd: repoRoot,
  stdio: "pipe"
});

writeFileSync(evidenceFile, [
  "W350 acceptance evidence — DnaOxIde live editable source app",
  "",
  `repoRoot=${repoRoot}`,
  "objective=DnaOxIde has a reviewable basic live editable app and automatic test loop",
  "acceptanceCommand=npm --prefix apps/dna-oxide run w350:acceptance",
  "automaticLoopCommand=npm --prefix apps/dna-oxide run live-edit:check",
  "",
  "Checklist:",
  "- live editable source loop: PASS (Playwright fills source editor; dirty state changes)",
  "- visual artifacts: PASS (B03/B04 HTML and PNG artifacts non-empty)",
  "- DOM-like snapshots: PASS (B01/B03/B04 JSON snapshots verified)",
  "- injected effects visible: PASS (B03/B04 source markers and dirty transitions verified)",
  "- save/reload temp copy: PASS (B04 temp Module1.bas saved; reload discards divergent text)",
  "- command/event logs: PASS (B04 save/reload commands and events verified)",
  "- automatic test loop: PASS (live-edit:check runs B01-B04 checks and audits artifacts)",
  "- W344 command boundary check: PASS (command-client:check)",
  "- W345 host UI checks: PASS (host-ui/lifecycle/services)",
  "- W346 interaction checks: PASS (interaction command/focus/lifecycle/services)",
  "- compile/reference placeholder no-claim checks: PASS",
  "- Tauri scaffold cargo tests: PASS",
  "- crates workspace tests: PASS",
  "- checked-in fixture unchanged: PASS",
  "- anti-overclaim scan: PASS",
  "",
  "Review artifacts:",
  "- target/w350-b03-live-editable-host.html",
  "- target/w350-b03-live-editable-host.png",
  "- target/w350-b04-live-save-reload.html",
  "- target/w350-b04-live-save-reload.png",
  "- target/w350-b05-live-edit-feedback-loop.txt",
  "- target/w350-acceptance-commands.txt",
  "- docs/HANDOFF_W350_LIVE_EDITABLE_SOURCE_APP.md",
  "",
  "Explicit non-claims:",
  "- live Tauri/WebView IPC: false",
  "- real/native OxVba runtime execution: false",
  "- real Immediate evaluation: false",
  "- real debug/watch/breakpoint behavior: false",
  "- COM runtime invocation: false",
  "- real DnaOneCalc product mount: false",
  "",
  "Next unblocked work:",
  "- W355 compile/build adapter contract and implementation work may start after this acceptance.",
  "- W352 Tauri/WebView automation remains future desktop automation work and does not block W355.",
  "",
  "Status: PASS"
].join("\n"), "utf8");

console.log(`W350 acceptance verification passed: ${evidenceFile}`);
