import {
  COMMAND_CLIENT_BUCKETS,
  DNA_OXIDE_COMMANDS,
  createBrowserFixtureCommandClient,
  unavailableFixtureResponse
} from "../src/command-client.js";
import { createInteractionHarness, invokeCommand } from "../src/interaction-harness.js";

const client = createBrowserFixtureCommandClient();
const state = createInteractionHarness({ commandClient: client });
const expected = new Map([
  [DNA_OXIDE_COMMANDS.getCompileOptions, COMMAND_CLIENT_BUCKETS.pendingOxvbaHardening],
  [DNA_OXIDE_COMMANDS.applyCompileOptions, COMMAND_CLIENT_BUCKETS.pendingOxvbaHardening],
  [DNA_OXIDE_COMMANDS.buildCheck, COMMAND_CLIENT_BUCKETS.oxvbaFixtureEvidenced],
  [DNA_OXIDE_COMMANDS.getReferences, COMMAND_CLIENT_BUCKETS.oxvbaFixtureEvidenced],
  [DNA_OXIDE_COMMANDS.findComCandidates, COMMAND_CLIENT_BUCKETS.oxvbaFixtureEvidenced],
  [DNA_OXIDE_COMMANDS.applyReferencePlan, COMMAND_CLIENT_BUCKETS.pendingOxvbaHardening]
]);

for (const [commandName, bucket] of expected.entries()) {
  const response = unavailableFixtureResponse(commandName, { panel: "compile-reference-placeholder" });
  const interaction = invokeCommand(state, commandName, { via: "placeholder-panel" });
  if (response.bucket !== bucket || interaction.bucket !== bucket) {
    console.error(`Placeholder command ${commandName} had unexpected bucket: ${response.bucket}/${interaction.bucket}`);
    process.exit(1);
  }
  if (response.enabled !== false || interaction.enabled !== false) {
    console.error(`Placeholder command ${commandName} unexpectedly enabled`);
    process.exit(1);
  }
  if (!response.disabledReason?.includes(bucket) || !interaction.disabledReason?.includes(bucket)) {
    console.error(`Placeholder command ${commandName} missing disabled reason bucket`);
    process.exit(1);
  }
  if (response.claims.realExecutionClaimed !== false
    || response.claims.comRuntimeClaimed !== false
    || response.claims.fakeResponses !== false
    || interaction.claims.fakeDebugData !== false) {
    console.error(`Placeholder command ${commandName} overclaimed capability`);
    process.exit(1);
  }
}

for (const token of [
  "role=\"host-compile-options-panel\"",
  "role=\"host-build-check-panel\"",
  "role=\"host-references-panel\"",
  "role=\"host-com-candidate-panel\"",
  "role=\"host-reference-repair-panel\"",
  "data-output-rows=\"0\"",
  "data-candidate-rows=\"0\"",
  "data-preview-rows=\"0\"",
  "data-com-runtime-invocation=\"false\""
]) {
  if (!state.markup.includes(token)) {
    console.error(`Placeholder command host markup missing token ${token}`);
    process.exit(1);
  }
}

console.log("DNA OxIde placeholder command verification passed");
