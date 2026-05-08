import { execFileSync } from "node:child_process";
import { existsSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { containsForbiddenClaimToken } from "../src/app-instrumentation.js";

const here = dirname(fileURLToPath(import.meta.url));
const appRoot = resolve(here, "..");
const repoRoot = resolve(appRoot, "..", "..");
const targetDir = resolve(repoRoot, "target");

const scripts = [
  ["app-instrumentation:check", resolve(appRoot, "scripts", "verify-app-instrumentation.mjs")],
  ["editable-source-boundary:check", resolve(appRoot, "scripts", "verify-editable-source-boundary.mjs")],
  ["live-host-mount:check", resolve(appRoot, "scripts", "verify-live-host-mount.mjs")],
  ["live-save-reload:check", resolve(appRoot, "scripts", "verify-live-save-reload.mjs")]
];

const requiredArtifacts = [
  "w350-b01-snapshot-before.json",
  "w350-b01-snapshot-after.json",
  "w350-b01-events.json",
  "w350-b01-commands.json",
  "w350-b01-app-instrumentation.html",
  "w350-b02-editable-source-after-reload.json",
  "w350-b03-live-host-before.json",
  "w350-b03-live-host-after-input.json",
  "w350-b03-live-host-events.json",
  "w350-b03-live-editable-host.html",
  "w350-b03-live-editable-host.png",
  "w350-b04-before-edit.json",
  "w350-b04-after-edit.json",
  "w350-b04-after-save.json",
  "w350-b04-after-divergent-edit.json",
  "w350-b04-after-reload.json",
  "w350-b04-events.json",
  "w350-b04-commands.json",
  "w350-b04-live-save-reload.html",
  "w350-b04-live-save-reload.png",
  "w350-b04-saved-Module1.bas"
];

const evidenceFile = resolve(targetDir, "w350-b05-live-edit-feedback-loop.txt");
const commandOutputFile = resolve(targetDir, "w350-b05-live-edit-feedback-loop-commands.txt");

function assert(condition, message) {
  if (!condition) {
    throw new Error(message);
  }
}

function readJson(relative) {
  return JSON.parse(readFileSync(resolve(targetDir, relative), "utf8"));
}

function readText(relative) {
  return readFileSync(resolve(targetDir, relative), "utf8");
}

console.log("== W350-B05 live edit feedback-loop regression ==");
console.log(`repoRoot=${repoRoot}`);

const commandTranscripts = [];
for (const [script, scriptPath] of scripts) {
  console.log(`running ${script}`);
  const output = execFileSync(process.execPath, [scriptPath], {
    cwd: appRoot,
    encoding: "utf8",
    stdio: ["ignore", "pipe", "pipe"]
  });
  commandTranscripts.push(`$ npm --prefix apps/dna-oxide run ${script}\n${output.trim()}`);
}
writeFileSync(commandOutputFile, `${commandTranscripts.join("\n\n")}\n`, "utf8");

for (const relative of requiredArtifacts) {
  const absolute = resolve(targetDir, relative);
  assert(existsSync(absolute), `required artifact missing: ${relative}`);
  assert(statSync(absolute).size > 0, `required artifact is empty: ${relative}`);
}

const b03Before = readJson("w350-b03-live-host-before.json");
const b03After = readJson("w350-b03-live-host-after-input.json");
const b03Events = readJson("w350-b03-live-host-events.json");
const b04Before = readJson("w350-b04-before-edit.json");
const b04AfterEdit = readJson("w350-b04-after-edit.json");
const b04AfterSave = readJson("w350-b04-after-save.json");
const b04AfterDiverge = readJson("w350-b04-after-divergent-edit.json");
const b04AfterReload = readJson("w350-b04-after-reload.json");
const b04Events = readJson("w350-b04-events.json");
const b04Commands = readJson("w350-b04-commands.json");
const savedModule = readText("w350-b04-saved-Module1.bas");
const b03Html = readText("w350-b03-live-editable-host.html");
const b04Html = readText("w350-b04-live-save-reload.html");

assert(b03Before.dirty === false, "B03 before snapshot must be clean");
assert(b03After.dirty === true, "B03 after input snapshot must be dirty");
assert(b03After.sourceText.includes("W350-B03 browser DOM input"), "B03 source missing browser input marker");
assert(b03Events.some((event) => event.kind === "source-replaced" && event.detail.via === "dom-input"), "B03 event log missing dom-input source replacement");

assert(b04Before.dirty === false, "B04 before snapshot must be clean");
assert(b04AfterEdit.dirty === true, "B04 edit snapshot must be dirty");
assert(b04AfterSave.dirty === false, "B04 save snapshot must be clean");
assert(b04AfterDiverge.dirty === true, "B04 divergent snapshot must be dirty");
assert(b04AfterReload.dirty === false, "B04 reload snapshot must be clean");
assert(b04AfterReload.sourceText.includes("W350-B04 saved through temp project copy"), "B04 reload missing saved marker");
assert(!b04AfterReload.sourceText.includes("unsaved divergent"), "B04 reload retained divergent text");
assert(savedModule.includes("W350-B04 saved through temp project copy"), "saved module missing saved marker");
assert(!savedModule.includes("unsaved divergent"), "saved module contains divergent text");
assert(b04Commands.some((command) => command.commandName === "save-active-module"), "B04 commands missing save");
assert(b04Commands.some((command) => command.commandName === "reload-active-module"), "B04 commands missing reload");
assert(b04Events.some((event) => event.kind === "source-saved-to-temp-copy"), "B04 events missing save");
assert(b04Events.some((event) => event.kind === "source-reloaded-from-temp-copy"), "B04 events missing reload");
assert(b03Html.includes('data-testid="source-editor"'), "B03 HTML missing source editor");
assert(b04Html.includes('data-testid="source-editor"'), "B04 HTML missing source editor");

for (const relative of requiredArtifacts.filter((path) => !path.endsWith(".png"))) {
  const text = readText(relative);
  assert(!containsForbiddenClaimToken(text), `${relative} contains forbidden overclaim token`);
}

execFileSync("git", ["diff", "--quiet", "--", "examples/thin-slice/Module1.bas"], {
  cwd: repoRoot,
  stdio: "pipe"
});

writeFileSync(evidenceFile, [
  "W350-B05 live edit feedback-loop regression evidence",
  "",
  `repoRoot=${repoRoot}`,
  "automaticLoopCommand=npm --prefix apps/dna-oxide run live-edit:check",
  "scriptsRun=app-instrumentation:check,editable-source-boundary:check,live-host-mount:check,live-save-reload:check",
  "browserDomInputDriven=true",
  "playwrightEdgeDriven=true",
  "visualArtifacts=true",
  "domLikeSnapshots=true",
  "commandLog=true",
  "eventLog=true",
  "saveReloadCovered=true",
  "tempProjectCopyOnly=true",
  "fixtureMutationGuard=PASS",
  "antiOverclaimScan=PASS",
  "runtimeExecutionClaimed=false",
  "nativeRuntimeClaimed=false",
  "comRuntimeClaimed=false",
  "fakeResponses=false",
  "fakeDebugData=false",
  "",
  "Observed state transitions:",
  `- B03 before dirty=${b03Before.dirty}`,
  `- B03 after input dirty=${b03After.dirty}`,
  `- B04 before dirty=${b04Before.dirty}`,
  `- B04 after edit dirty=${b04AfterEdit.dirty}`,
  `- B04 after save dirty=${b04AfterSave.dirty}`,
  `- B04 after divergent edit dirty=${b04AfterDiverge.dirty}`,
  `- B04 after reload dirty=${b04AfterReload.dirty}`,
  `- B04 command names=${b04Commands.map((command) => command.commandName).join(",")}`,
  `- B04 event kinds=${b04Events.map((event) => event.kind).join(",")}`,
  "",
  "Required artifacts verified non-empty:",
  ...requiredArtifacts.map((artifact) => `- target/${artifact}`),
  `- target/${commandOutputFile.split(/[/\\]/).pop()}`,
  "",
  "Status: PASS"
].join("\n"), "utf8");

console.log(`W350-B05 live edit feedback loop verification passed: ${evidenceFile}`);
