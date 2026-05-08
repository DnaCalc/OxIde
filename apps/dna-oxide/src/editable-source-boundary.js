import { NO_CLAIM_FLAGS } from "./command-client.js";

export const DNA_OXIDE_EDITABLE_SOURCE_BOUNDARY = Object.freeze({
  version: "w350-editable-source-v1",
  owner: "OxIde editor lifecycle model",
  hostNeutral: true,
  tauriCoupled: false,
  sharedUiCoupledToTauri: false,
  oxvbaSemanticTruthClaimed: false,
  runtimeExecutionClaimed: false,
  tempProjectCopiesOnly: true,
  claims: NO_CLAIM_FLAGS
});

export function createEditableSourceBoundary(options = {}) {
  const state = {
    projectName: options.projectName ?? "ThinSliceHello",
    projectFile: options.projectFile ?? "ThinSliceHello.basproj",
    activeModule: options.activeModule ?? "Module1.bas",
    sourceProvenance: options.sourceProvenance ?? "w350-editable-source-boundary",
    tempProjectRoot: options.tempProjectRoot ?? "target/w350-temp-project-copy",
    workingSourceText: options.sourceText ?? "",
    persistedSourceText: options.persistedSourceText ?? options.sourceText ?? "",
    lastReloadedSourceText: options.persistedSourceText ?? options.sourceText ?? "",
    editRevision: 0,
    savedRevision: 0,
    reloadedRevision: 0
  };

  function dirty() {
    return state.workingSourceText !== state.persistedSourceText;
  }

  function snapshot() {
    return Object.freeze({
      boundaryVersion: DNA_OXIDE_EDITABLE_SOURCE_BOUNDARY.version,
      projectName: state.projectName,
      projectFile: state.projectFile,
      activeModule: state.activeModule,
      sourceProvenance: state.sourceProvenance,
      tempProjectRoot: state.tempProjectRoot,
      sourceText: state.workingSourceText,
      sourceTextLength: state.workingSourceText.length,
      sourceTextHash: stableSourceHash(state.workingSourceText),
      persistedSourceText: state.persistedSourceText,
      persistedSourceTextLength: state.persistedSourceText.length,
      persistedSourceTextHash: stableSourceHash(state.persistedSourceText),
      lastReloadedSourceText: state.lastReloadedSourceText,
      lastReloadedSourceTextHash: stableSourceHash(state.lastReloadedSourceText),
      dirty: dirty(),
      lifecycleStatus: dirty() ? "dirty" : "clean",
      editRevision: state.editRevision,
      savedRevision: state.savedRevision,
      reloadedRevision: state.reloadedRevision,
      commandStates: lifecycleCommandStates(dirty()),
      noClaimFlags: { ...NO_CLAIM_FLAGS },
      boundary: { ...DNA_OXIDE_EDITABLE_SOURCE_BOUNDARY }
    });
  }

  function replaceSource(text, metadata = {}) {
    requireString(text, "replaceSource text");
    const beforeHash = stableSourceHash(state.workingSourceText);
    state.workingSourceText = text;
    state.editRevision += 1;
    return outcome("source-replaced", metadata, beforeHash);
  }

  function appendSource(text, metadata = {}) {
    requireString(text, "appendSource text");
    const beforeHash = stableSourceHash(state.workingSourceText);
    state.workingSourceText += text;
    state.editRevision += 1;
    return outcome("source-appended", metadata, beforeHash);
  }

  function applyInputEvent(event, metadata = {}) {
    if (!event || typeof event !== "object") {
      throw new TypeError("applyInputEvent requires an event-like object");
    }
    if (event.type === "replaceSource" || event.type === "input") {
      return replaceSource(event.text ?? event.value ?? "", metadata);
    }
    if (event.type === "appendSource") {
      return appendSource(event.text ?? "", metadata);
    }
    throw new Error(`Unsupported editable source event: ${event.type}`);
  }

  function saveToPersisted(metadata = {}) {
    const beforeHash = stableSourceHash(state.persistedSourceText);
    state.persistedSourceText = state.workingSourceText;
    state.savedRevision += 1;
    return outcome("source-saved", metadata, beforeHash);
  }

  function reloadFromPersisted(metadata = {}) {
    const beforeHash = stableSourceHash(state.workingSourceText);
    state.workingSourceText = state.persistedSourceText;
    state.lastReloadedSourceText = state.persistedSourceText;
    state.reloadedRevision += 1;
    return outcome("source-reloaded", metadata, beforeHash);
  }

  function revertToPersisted(metadata = {}) {
    const beforeHash = stableSourceHash(state.workingSourceText);
    state.workingSourceText = state.persistedSourceText;
    return outcome("source-reverted", metadata, beforeHash);
  }

  function outcome(kind, metadata, beforeHash) {
    const snap = snapshot();
    return Object.freeze({
      kind,
      metadata: { ...metadata },
      activeModule: state.activeModule,
      dirty: snap.dirty,
      beforeHash,
      afterHash: snap.sourceTextHash,
      persistedHash: snap.persistedSourceTextHash,
      sourceTextLength: snap.sourceTextLength,
      commandStates: snap.commandStates,
      noClaimFlags: { ...NO_CLAIM_FLAGS }
    });
  }

  return Object.freeze({
    boundary: DNA_OXIDE_EDITABLE_SOURCE_BOUNDARY,
    snapshot,
    replaceSource,
    appendSource,
    applyInputEvent,
    saveToPersisted,
    reloadFromPersisted,
    revertToPersisted
  });
}

export function verifyEditableSourceBoundaryContract() {
  const boundary = createEditableSourceBoundary({
    sourceText: "Attribute VB_Name = \"Module1\"\nOption Explicit\n"
  });
  const before = boundary.snapshot();
  const edit = boundary.appendSource("\n' W350-B02 edit");
  const afterEdit = boundary.snapshot();
  const save = boundary.saveToPersisted();
  const afterSave = boundary.snapshot();
  boundary.replaceSource("temporary divergent text");
  const afterDiverge = boundary.snapshot();
  const reload = boundary.reloadFromPersisted();
  const afterReload = boundary.snapshot();

  return before.dirty === false
    && before.boundary.hostNeutral === true
    && before.boundary.tauriCoupled === false
    && before.commandStates.saveActiveModule === "enabled-clean-noop"
    && edit.kind === "source-appended"
    && afterEdit.dirty === true
    && afterEdit.commandStates.saveActiveModule === "enabled-dirty"
    && afterEdit.sourceText.includes("W350-B02 edit")
    && save.kind === "source-saved"
    && afterSave.dirty === false
    && afterSave.persistedSourceText.includes("W350-B02 edit")
    && afterDiverge.dirty === true
    && reload.kind === "source-reloaded"
    && afterReload.dirty === false
    && afterReload.sourceText === afterSave.persistedSourceText
    && afterReload.noClaimFlags.realExecutionClaimed === false
    && afterReload.noClaimFlags.nativeRuntimeClaimed === false
    && afterReload.noClaimFlags.comRuntimeClaimed === false
    && afterReload.noClaimFlags.fakeResponses === false
    && afterReload.noClaimFlags.fakeDebugData === false;
}

export function lifecycleCommandStates(isDirty) {
  return Object.freeze({
    focusEditor: "enabled",
    saveActiveModule: isDirty ? "enabled-dirty" : "enabled-clean-noop",
    reloadActiveModule: "enabled-temp-copy",
    revertActiveModule: isDirty ? "enabled-dirty" : "enabled-clean-noop",
    runProject: "unavailable-no-runtime-claim",
    debugAttach: "unavailable-no-debug-claim",
    evaluateImmediate: "unavailable-no-immediate-claim",
    findComCandidates: "unavailable-no-com-runtime-claim"
  });
}

export function stableSourceHash(text) {
  let hash = 0x811c9dc5;
  for (const char of String(text)) {
    hash ^= char.charCodeAt(0);
    hash = Math.imul(hash, 0x01000193) >>> 0;
  }
  return hash.toString(16).padStart(8, "0");
}

function requireString(value, label) {
  if (typeof value !== "string") {
    throw new TypeError(`${label} must be a string`);
  }
}
