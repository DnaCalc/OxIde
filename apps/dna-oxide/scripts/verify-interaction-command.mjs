import {
  createInteractionHarness,
  pressShortcut,
  verifyCommandKeyboardInteraction
} from "../src/interaction-harness.js";
import {
  COMMAND_CLIENT_BUCKETS,
  DNA_OXIDE_COMMANDS
} from "../src/command-client.js";

if (!verifyCommandKeyboardInteraction()) {
  console.error("DNA OxIde command/keyboard interaction verification failed");
  process.exit(1);
}

const state = createInteractionHarness();
const checks = [
  ["Ctrl+Shift+P", DNA_OXIDE_COMMANDS.openCommandPalette, COMMAND_CLIENT_BUCKETS.provenOxideOnly, true],
  ["Ctrl+O", DNA_OXIDE_COMMANDS.openProjectPath, COMMAND_CLIENT_BUCKETS.provenOxideOnly, true],
  ["Ctrl+S", DNA_OXIDE_COMMANDS.saveActiveModule, COMMAND_CLIENT_BUCKETS.provenOxideOnly, true],
  ["F5", DNA_OXIDE_COMMANDS.runProject, COMMAND_CLIENT_BUCKETS.oxvbaFixtureEvidenced, false],
  ["Ctrl+Enter", DNA_OXIDE_COMMANDS.evaluateImmediate, COMMAND_CLIENT_BUCKETS.oxvbaFixtureEvidenced, false],
  ["Ctrl+F5", DNA_OXIDE_COMMANDS.debugAttach, COMMAND_CLIENT_BUCKETS.oxvbaFixtureEvidenced, false],
  ["Ctrl+Shift+C", DNA_OXIDE_COMMANDS.findComCandidates, COMMAND_CLIENT_BUCKETS.oxvbaFixtureEvidenced, false]
];

for (const [shortcut, commandName, bucket, enabled] of checks) {
  const entry = pressShortcut(state, shortcut);
  if (entry.commandName !== commandName || entry.bucket !== bucket || entry.enabled !== enabled) {
    console.error(`Shortcut ${shortcut} routed incorrectly: ${JSON.stringify(entry)}`);
    process.exit(1);
  }
  if (entry.claims.realExecutionClaimed !== false || entry.claims.fakeDebugData !== false) {
    console.error(`Shortcut ${shortcut} produced an unsupported claim: ${JSON.stringify(entry.claims)}`);
    process.exit(1);
  }
}

const blocked = state.commandLog.filter((entry) => !entry.enabled);
if (blocked.length < 4 || !blocked.every((entry) => entry.disabledReason?.includes(entry.bucket))) {
  console.error("Blocked command entries did not preserve disabled reasons and bucket labels");
  process.exit(1);
}

console.log("DNA OxIde command/keyboard interaction verification passed");
