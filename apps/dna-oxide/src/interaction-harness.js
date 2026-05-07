import {
  COMMAND_CLIENT_BUCKETS,
  DNA_OXIDE_COMMANDS,
  NO_CLAIM_FLAGS,
  createBrowserFixtureCommandClient
} from "./command-client.js";
import {
  createDnaOxIdeHostShellModel,
  renderDnaOxIdeHostShell
} from "./host-shell.js";

export const DNA_OXIDE_INTERACTION_HARNESS = Object.freeze({
  harnessLayer: "frontend-interaction-model+static-dom-token-smoke",
  liveTauriWebViewIpcDriven: false,
  browserEventLoopDriven: false,
  playwrightOrWebDriverDriven: false,
  fullDomAccessibilityAuditClaimed: false,
  realRuntimeExecutionClaimed: false,
  comRuntimeInvocationClaimed: false,
  claims: NO_CLAIM_FLAGS
});

export const KEYBOARD_SHORTCUTS = Object.freeze({
  "Ctrl+Shift+P": DNA_OXIDE_COMMANDS.openCommandPalette,
  "Ctrl+O": DNA_OXIDE_COMMANDS.openProjectPath,
  "Ctrl+S": DNA_OXIDE_COMMANDS.saveActiveModule,
  "Ctrl+R": DNA_OXIDE_COMMANDS.reloadActiveModule,
  F5: DNA_OXIDE_COMMANDS.runProject,
  "Ctrl+Enter": DNA_OXIDE_COMMANDS.evaluateImmediate,
  F9: DNA_OXIDE_COMMANDS.breakpointSet,
  F10: DNA_OXIDE_COMMANDS.debugStepOver,
  F11: DNA_OXIDE_COMMANDS.debugStepInto,
  "Ctrl+F5": DNA_OXIDE_COMMANDS.debugAttach,
  "Ctrl+Shift+C": DNA_OXIDE_COMMANDS.findComCandidates
});

export const FOCUS_ROUTE = Object.freeze([
  "host-project-spine",
  "host-editor-boundary",
  "host-diagnostics-panel",
  "host-lifecycle-panel",
  "host-command-palette",
  "host-runtime-panel",
  "host-immediate-panel",
  "host-debug-panel",
  "host-com-panel",
  "host-claim-boundaries"
]);

export const LIFECYCLE_SEQUENCE = Object.freeze([
  DNA_OXIDE_COMMANDS.openProjectPath,
  DNA_OXIDE_COMMANDS.loadActiveModule,
  DNA_OXIDE_COMMANDS.saveActiveModule,
  DNA_OXIDE_COMMANDS.reloadActiveModule,
  DNA_OXIDE_COMMANDS.saveSessionSnapshot,
  DNA_OXIDE_COMMANDS.loadSessionSnapshot
]);

export const BLOCKED_SERVICE_COMMANDS = Object.freeze([
  DNA_OXIDE_COMMANDS.buildCheck,
  DNA_OXIDE_COMMANDS.runProject,
  DNA_OXIDE_COMMANDS.evaluateImmediate,
  DNA_OXIDE_COMMANDS.debugAttach,
  DNA_OXIDE_COMMANDS.watchUpsert,
  DNA_OXIDE_COMMANDS.breakpointSet,
  DNA_OXIDE_COMMANDS.findComCandidates,
  DNA_OXIDE_COMMANDS.getCompileOptions,
  DNA_OXIDE_COMMANDS.stopRuntime
]);

export function createInteractionHarness(options = {}) {
  const commandClient = options.commandClient ?? createBrowserFixtureCommandClient();
  const model = options.model ?? createDnaOxIdeHostShellModel(commandClient, options.modelOverrides ?? {});
  const markup = options.markup ?? renderDnaOxIdeHostShell(model);

  return {
    harness: DNA_OXIDE_INTERACTION_HARNESS,
    commandClient,
    model,
    markup,
    paletteOpen: false,
    focusIndex: 0,
    commandLog: [],
    focusLog: [FOCUS_ROUTE[0]],
    lifecycleLog: [],
    serviceAttempts: []
  };
}

export function pressShortcut(state, shortcut) {
  const commandName = KEYBOARD_SHORTCUTS[shortcut];
  if (!commandName) {
    const entry = commandEntry(`unmapped:${shortcut}`, COMMAND_CLIENT_BUCKETS.unavailableNoClaim, false, `No command is mapped for ${shortcut}`);
    state.commandLog.push(entry);
    return entry;
  }

  if (commandName === DNA_OXIDE_COMMANDS.openCommandPalette) {
    state.paletteOpen = !state.paletteOpen;
  }

  return invokeCommand(state, commandName, { via: "keyboard", shortcut });
}

export function invokeCommand(state, commandName, context = {}) {
  const bucket = state.commandClient.bucketForCommand(commandName);
  const enabled = bucket === COMMAND_CLIENT_BUCKETS.provenOxideOnly;
  const entry = commandEntry(
    commandName,
    bucket,
    enabled,
    enabled ? null : `${commandName} remains ${bucket} in W346 interaction harness`,
    context
  );
  state.commandLog.push(entry);

  if (LIFECYCLE_SEQUENCE.includes(commandName)) {
    state.lifecycleLog.push(entry);
  }
  if (BLOCKED_SERVICE_COMMANDS.includes(commandName)) {
    state.serviceAttempts.push(entry);
  }

  return entry;
}

export function walkFocusRoute(state) {
  while (state.focusIndex < FOCUS_ROUTE.length - 1) {
    state.focusIndex += 1;
    state.focusLog.push(FOCUS_ROUTE[state.focusIndex]);
  }
  return state.focusLog.slice();
}

export function runLifecycleSequence(state) {
  return LIFECYCLE_SEQUENCE.map((commandName) => invokeCommand(state, commandName, { via: "lifecycle-sequence" }));
}

export function triggerBlockedServices(state) {
  return BLOCKED_SERVICE_COMMANDS.map((commandName) => invokeCommand(state, commandName, { via: "blocked-service-check" }));
}

export function renderedMarkupContainsRoles(markup, roles = FOCUS_ROUTE) {
  return roles.every((role) => markup.includes(`role="${role}"`));
}

export function noClaimFlagsAreFalse(value = NO_CLAIM_FLAGS) {
  return value.realExecutionClaimed === false
    && value.nativeRuntimeClaimed === false
    && value.comRuntimeClaimed === false
    && value.fakeResponses === false
    && value.fakeDebugData === false;
}

export function commandEntry(commandName, bucket, enabled, disabledReason, context = {}) {
  return Object.freeze({
    commandName,
    bucket,
    enabled,
    disabledReason,
    context: Object.freeze({ ...context }),
    claims: NO_CLAIM_FLAGS
  });
}

export function verifyCommandKeyboardInteraction() {
  const state = createInteractionHarness();
  const paletteOpen = pressShortcut(state, "Ctrl+Shift+P");
  const openProject = pressShortcut(state, "Ctrl+O");
  const saveModule = pressShortcut(state, "Ctrl+S");
  const runProject = pressShortcut(state, "F5");
  const immediate = pressShortcut(state, "Ctrl+Enter");
  const debug = pressShortcut(state, "Ctrl+F5");
  const com = pressShortcut(state, "Ctrl+Shift+C");
  const paletteClosed = pressShortcut(state, "Ctrl+Shift+P");

  return state.paletteOpen === false
    && paletteOpen.commandName === DNA_OXIDE_COMMANDS.openCommandPalette
    && paletteClosed.commandName === DNA_OXIDE_COMMANDS.openCommandPalette
    && openProject.enabled === true
    && saveModule.enabled === true
    && runProject.enabled === false
    && runProject.bucket === COMMAND_CLIENT_BUCKETS.oxvbaFixtureEvidenced
    && immediate.enabled === false
    && immediate.bucket === COMMAND_CLIENT_BUCKETS.oxvbaFixtureEvidenced
    && debug.enabled === false
    && debug.bucket === COMMAND_CLIENT_BUCKETS.oxvbaFixtureEvidenced
    && com.enabled === false
    && com.bucket === COMMAND_CLIENT_BUCKETS.oxvbaFixtureEvidenced
    && state.commandLog.length === 8
    && noClaimFlagsAreFalse(state.harness.claims);
}
