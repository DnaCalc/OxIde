import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import {
  LIFECYCLE_SEQUENCE,
  createInteractionHarness,
  runLifecycleSequence
} from "../src/interaction-harness.js";
import { COMMAND_CLIENT_BUCKETS, DNA_OXIDE_COMMANDS } from "../src/command-client.js";

const here = dirname(fileURLToPath(import.meta.url));
const appRoot = resolve(here, "..");
const repoRoot = resolve(appRoot, "..", "..");
const fixturePath = resolve(repoRoot, "examples", "thin-slice", "Module1.bas");
const lifecycleProof = resolve(repoRoot, "target", "w345-host-lifecycle-proof.html");
const fixtureBefore = readFileSync(fixturePath, "utf8");

const state = createInteractionHarness({
  modelOverrides: {
    lifecycle: {
      dirty: true,
      provider: "proven-oxide-only-temp-copy",
      proofPath: "target/w345-host-lifecycle-proof-files",
      events: [
        "opened-temp-project-copy",
        "saved-working-source-to-temp-copy",
        "reloaded-module-from-temp-copy",
        "session-snapshot-restored-from-temp-copy",
        "checked-in-fixture-unchanged"
      ]
    }
  }
});

const entries = runLifecycleSequence(state);

if (entries.length !== LIFECYCLE_SEQUENCE.length || state.lifecycleLog.length !== LIFECYCLE_SEQUENCE.length) {
  console.error("Lifecycle interaction sequence length mismatch");
  process.exit(1);
}

for (const entry of entries) {
  if (entry.bucket !== COMMAND_CLIENT_BUCKETS.provenOxideOnly || entry.enabled !== true) {
    console.error(`Lifecycle command was not proven/enabled: ${JSON.stringify(entry)}`);
    process.exit(1);
  }
}

const requiredCommands = [
  DNA_OXIDE_COMMANDS.openProjectPath,
  DNA_OXIDE_COMMANDS.loadActiveModule,
  DNA_OXIDE_COMMANDS.saveActiveModule,
  DNA_OXIDE_COMMANDS.reloadActiveModule,
  DNA_OXIDE_COMMANDS.saveSessionSnapshot,
  DNA_OXIDE_COMMANDS.loadSessionSnapshot
];
for (const commandName of requiredCommands) {
  if (!state.lifecycleLog.some((entry) => entry.commandName === commandName)) {
    console.error(`Lifecycle interaction log missing command ${commandName}`);
    process.exit(1);
  }
}

if (!state.markup.includes("data-provider=\"proven-oxide-only-temp-copy\"")
  || !state.markup.includes("data-dirty=\"true\"")
  || !state.markup.includes("session-snapshot-restored-from-temp-copy")) {
  console.error("Lifecycle host markup did not expose temp-copy dirty/restored state");
  process.exit(1);
}

const lifecycleMarkup = readFileSync(lifecycleProof, "utf8");
for (const token of [
  "opened-temp-project-copy",
  "saved-working-source-to-temp-copy",
  "reloaded-module-from-temp-copy",
  "session-snapshot-restored-from-temp-copy",
  "checked-in-fixture-unchanged"
]) {
  if (!lifecycleMarkup.includes(token)) {
    console.error(`W345 lifecycle proof markup missing token ${token}`);
    process.exit(1);
  }
}

if (readFileSync(fixturePath, "utf8") !== fixtureBefore) {
  console.error("Checked-in fixture changed during lifecycle interaction verification");
  process.exit(1);
}

console.log(`DNA OxIde lifecycle interaction verification passed: ${state.lifecycleLog.map((entry) => entry.commandName).join(" -> ")}`);
