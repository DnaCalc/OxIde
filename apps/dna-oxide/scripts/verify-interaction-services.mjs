import {
  BLOCKED_SERVICE_COMMANDS,
  createInteractionHarness,
  triggerBlockedServices
} from "../src/interaction-harness.js";
import { COMMAND_CLIENT_BUCKETS, DNA_OXIDE_COMMANDS } from "../src/command-client.js";

const state = createInteractionHarness();
const entries = triggerBlockedServices(state);

if (entries.length !== BLOCKED_SERVICE_COMMANDS.length || state.serviceAttempts.length !== BLOCKED_SERVICE_COMMANDS.length) {
  console.error("Blocked service interaction length mismatch");
  process.exit(1);
}

const expectedBuckets = new Map([
  [DNA_OXIDE_COMMANDS.buildCheck, COMMAND_CLIENT_BUCKETS.oxvbaFixtureEvidenced],
  [DNA_OXIDE_COMMANDS.runProject, COMMAND_CLIENT_BUCKETS.oxvbaFixtureEvidenced],
  [DNA_OXIDE_COMMANDS.evaluateImmediate, COMMAND_CLIENT_BUCKETS.oxvbaFixtureEvidenced],
  [DNA_OXIDE_COMMANDS.debugAttach, COMMAND_CLIENT_BUCKETS.oxvbaFixtureEvidenced],
  [DNA_OXIDE_COMMANDS.watchUpsert, COMMAND_CLIENT_BUCKETS.oxvbaFixtureEvidenced],
  [DNA_OXIDE_COMMANDS.breakpointSet, COMMAND_CLIENT_BUCKETS.oxvbaFixtureEvidenced],
  [DNA_OXIDE_COMMANDS.findComCandidates, COMMAND_CLIENT_BUCKETS.oxvbaFixtureEvidenced],
  [DNA_OXIDE_COMMANDS.getCompileOptions, COMMAND_CLIENT_BUCKETS.pendingOxvbaHardening],
  [DNA_OXIDE_COMMANDS.stopRuntime, COMMAND_CLIENT_BUCKETS.pendingOxvbaHardening]
]);

for (const entry of entries) {
  const expected = expectedBuckets.get(entry.commandName);
  if (entry.enabled !== false || entry.bucket !== expected) {
    console.error(`Blocked service command had unexpected state: ${JSON.stringify(entry)}`);
    process.exit(1);
  }
  if (!entry.disabledReason?.includes(entry.bucket)) {
    console.error(`Blocked service command missing bucket in disabled reason: ${JSON.stringify(entry)}`);
    process.exit(1);
  }
  if (entry.claims.realExecutionClaimed !== false
    || entry.claims.nativeRuntimeClaimed !== false
    || entry.claims.comRuntimeClaimed !== false
    || entry.claims.fakeResponses !== false
    || entry.claims.fakeDebugData !== false) {
    console.error(`Blocked service command overclaimed capability: ${JSON.stringify(entry.claims)}`);
    process.exit(1);
  }
}

const requiredMarkupTokens = [
  "role=\"host-runtime-panel\"",
  "data-output-events=\"0\"",
  "role=\"host-immediate-panel\"",
  "data-immediate-responses=\"0\"",
  "role=\"host-debug-panel\"",
  "data-callstack-frames=\"0\"",
  "data-locals=\"0\"",
  "data-watches=\"0\"",
  "data-breakpoints=\"0\"",
  "role=\"host-com-panel\"",
  "data-com-candidates=\"0\"",
  "data-com-runtime-invocation=\"false\"",
  "data-real-execution=\"false\"",
  "data-native-runtime=\"false\"",
  "data-com-runtime=\"false\"",
  "data-fake-responses=\"false\"",
  "data-fake-debug-data=\"false\""
];

for (const token of requiredMarkupTokens) {
  if (!state.markup.includes(token)) {
    console.error(`Blocked service markup missing token ${token}`);
    process.exit(1);
  }
}

console.log(`DNA OxIde blocked service interaction verification passed: ${entries.map((entry) => `${entry.commandName}:${entry.bucket}`).join(", ")}`);
