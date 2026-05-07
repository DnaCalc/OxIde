export const DNA_OXIDE_COMMANDS = Object.freeze({
  getHostCapabilities: "dna_oxide_get_host_capabilities",
  openProjectPath: "dna_oxide_open_project_path",
  inspectProject: "dna_oxide_inspect_project",
  loadActiveModule: "dna_oxide_load_active_module",
  saveActiveModule: "dna_oxide_save_active_module",
  reloadActiveModule: "dna_oxide_reload_active_module",
  revertActiveModule: "dna_oxide_revert_active_module",
  saveSessionSnapshot: "dna_oxide_save_session_snapshot",
  loadSessionSnapshot: "dna_oxide_load_session_snapshot",
  languageDiagnostics: "dna_oxide_language_diagnostics",
  languageHover: "dna_oxide_language_hover",
  languageDefinition: "dna_oxide_language_definition",
  languageReferences: "dna_oxide_language_references",
  getCompileOptions: "dna_oxide_get_compile_options",
  applyCompileOptions: "dna_oxide_apply_compile_options",
  buildCheck: "dna_oxide_build_check",
  getReferences: "dna_oxide_get_references",
  findComCandidates: "dna_oxide_find_com_candidates",
  applyReferencePlan: "dna_oxide_apply_reference_plan",
  runProject: "dna_oxide_run_project",
  stopRuntime: "dna_oxide_stop_runtime",
  resetRuntime: "dna_oxide_reset_runtime",
  evaluateImmediate: "dna_oxide_evaluate_immediate",
  debugAttach: "dna_oxide_debug_attach",
  debugContinue: "dna_oxide_debug_continue",
  debugStepInto: "dna_oxide_debug_step_into",
  debugStepOver: "dna_oxide_debug_step_over",
  debugStepOut: "dna_oxide_debug_step_out",
  debugStop: "dna_oxide_debug_stop",
  watchUpsert: "dna_oxide_watch_upsert",
  watchRemove: "dna_oxide_watch_remove",
  breakpointSet: "dna_oxide_breakpoint_set",
  breakpointClear: "dna_oxide_breakpoint_clear",
  openSettings: "dna_oxide_open_settings",
  openCommandPalette: "dna_oxide_open_command_palette"
});

export const COMMAND_CLIENT_BUCKETS = Object.freeze({
  provenOxideOnly: "proven-oxide-only",
  oxvbaAvailableSubset: "oxvba-available-subset",
  oxvbaFixtureEvidenced: "oxvba-fixture-evidenced",
  pendingOxvbaHardening: "pending-oxvba-hardening",
  unavailableNoClaim: "unavailable-no-claim"
});

export const NO_CLAIM_FLAGS = Object.freeze({
  realExecutionClaimed: false,
  nativeRuntimeClaimed: false,
  comRuntimeClaimed: false,
  fakeResponses: false,
  fakeDebugData: false
});

const PROVEN_COMMANDS = new Set([
  DNA_OXIDE_COMMANDS.getHostCapabilities,
  DNA_OXIDE_COMMANDS.openProjectPath,
  DNA_OXIDE_COMMANDS.loadActiveModule,
  DNA_OXIDE_COMMANDS.saveActiveModule,
  DNA_OXIDE_COMMANDS.reloadActiveModule,
  DNA_OXIDE_COMMANDS.revertActiveModule,
  DNA_OXIDE_COMMANDS.saveSessionSnapshot,
  DNA_OXIDE_COMMANDS.loadSessionSnapshot,
  DNA_OXIDE_COMMANDS.openSettings,
  DNA_OXIDE_COMMANDS.openCommandPalette
]);

const AVAILABLE_SUBSET_COMMANDS = new Set([
  DNA_OXIDE_COMMANDS.inspectProject,
  DNA_OXIDE_COMMANDS.languageDiagnostics,
  DNA_OXIDE_COMMANDS.languageHover,
  DNA_OXIDE_COMMANDS.languageDefinition,
  DNA_OXIDE_COMMANDS.languageReferences,
  DNA_OXIDE_COMMANDS.debugContinue,
  DNA_OXIDE_COMMANDS.debugStepInto,
  DNA_OXIDE_COMMANDS.debugStepOver,
  DNA_OXIDE_COMMANDS.debugStepOut
]);

const FIXTURE_EVIDENCED_COMMANDS = new Set([
  DNA_OXIDE_COMMANDS.buildCheck,
  DNA_OXIDE_COMMANDS.getReferences,
  DNA_OXIDE_COMMANDS.findComCandidates,
  DNA_OXIDE_COMMANDS.runProject,
  DNA_OXIDE_COMMANDS.evaluateImmediate,
  DNA_OXIDE_COMMANDS.debugAttach,
  DNA_OXIDE_COMMANDS.watchUpsert,
  DNA_OXIDE_COMMANDS.breakpointSet
]);

const PENDING_COMMANDS = new Set([
  DNA_OXIDE_COMMANDS.getCompileOptions,
  DNA_OXIDE_COMMANDS.applyCompileOptions,
  DNA_OXIDE_COMMANDS.applyReferencePlan,
  DNA_OXIDE_COMMANDS.stopRuntime,
  DNA_OXIDE_COMMANDS.resetRuntime,
  DNA_OXIDE_COMMANDS.debugStop,
  DNA_OXIDE_COMMANDS.watchRemove,
  DNA_OXIDE_COMMANDS.breakpointClear
]);

export function bucketForCommand(commandName) {
  if (PROVEN_COMMANDS.has(commandName)) {
    return COMMAND_CLIENT_BUCKETS.provenOxideOnly;
  }
  if (AVAILABLE_SUBSET_COMMANDS.has(commandName)) {
    return COMMAND_CLIENT_BUCKETS.oxvbaAvailableSubset;
  }
  if (FIXTURE_EVIDENCED_COMMANDS.has(commandName)) {
    return COMMAND_CLIENT_BUCKETS.oxvbaFixtureEvidenced;
  }
  if (PENDING_COMMANDS.has(commandName)) {
    return COMMAND_CLIENT_BUCKETS.pendingOxvbaHardening;
  }
  return COMMAND_CLIENT_BUCKETS.unavailableNoClaim;
}

export function createDnaOxIdeCommandClient(invokeImpl) {
  if (typeof invokeImpl !== "function") {
    throw new TypeError("createDnaOxIdeCommandClient requires an injected invoke function");
  }

  return Object.freeze({
    kind: "dnaoxide-injected-invoke-client",
    tauriImportedHere: false,
    sharedUiCoupledToTauri: false,
    invoke(commandName, payload = {}) {
      return invokeImpl(commandName, payload);
    },
    commandNames() {
      return Object.values(DNA_OXIDE_COMMANDS);
    },
    bucketForCommand,
    noClaimFlags() {
      return { ...NO_CLAIM_FLAGS };
    }
  });
}

export function createBrowserFixtureCommandClient() {
  return Object.freeze({
    kind: "browser-fixture-command-client",
    tauriImportedHere: false,
    sharedUiCoupledToTauri: false,
    invoke(commandName, payload = {}) {
      return Promise.resolve(unavailableFixtureResponse(commandName, payload));
    },
    commandNames() {
      return Object.values(DNA_OXIDE_COMMANDS);
    },
    bucketForCommand,
    noClaimFlags() {
      return { ...NO_CLAIM_FLAGS };
    }
  });
}

export function unavailableFixtureResponse(commandName, payload = {}) {
  const bucket = bucketForCommand(commandName);
  return Object.freeze({
    commandName,
    bucket,
    enabled: bucket === COMMAND_CLIENT_BUCKETS.provenOxideOnly,
    disabledReason: bucket === COMMAND_CLIENT_BUCKETS.provenOxideOnly
      ? null
      : `${commandName} is ${bucket}; browser fixture does not execute native services`,
    payloadEchoed: Object.keys(payload).sort(),
    claims: { ...NO_CLAIM_FLAGS }
  });
}

export function verifyCommandClientContract() {
  const commandNames = Object.values(DNA_OXIDE_COMMANDS);
  const uniqueNames = new Set(commandNames);
  const browserClient = createBrowserFixtureCommandClient();
  const runtimeResponse = unavailableFixtureResponse(DNA_OXIDE_COMMANDS.runProject);
  const compileResponse = unavailableFixtureResponse(DNA_OXIDE_COMMANDS.getCompileOptions);

  return commandNames.length === uniqueNames.size
    && commandNames.length >= 30
    && browserClient.tauriImportedHere === false
    && browserClient.sharedUiCoupledToTauri === false
    && browserClient.bucketForCommand(DNA_OXIDE_COMMANDS.runProject) === COMMAND_CLIENT_BUCKETS.oxvbaFixtureEvidenced
    && browserClient.bucketForCommand(DNA_OXIDE_COMMANDS.getCompileOptions) === COMMAND_CLIENT_BUCKETS.pendingOxvbaHardening
    && runtimeResponse.claims.realExecutionClaimed === false
    && runtimeResponse.claims.nativeRuntimeClaimed === false
    && runtimeResponse.claims.comRuntimeClaimed === false
    && runtimeResponse.claims.fakeResponses === false
    && runtimeResponse.claims.fakeDebugData === false
    && compileResponse.enabled === false;
}
