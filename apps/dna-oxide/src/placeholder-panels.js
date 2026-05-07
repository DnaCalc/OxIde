import { NO_CLAIM_FLAGS } from "./command-client.js";

export const PLACEHOLDER_PANEL_STATES = Object.freeze({
  provenOxideOnly: "proven-oxide-only",
  oxvbaAvailableSubset: "oxvba-available-subset",
  oxvbaFixtureEvidenced: "oxvba-fixture-evidenced",
  pendingOxvbaHardening: "pending-oxvba-hardening",
  unavailableNoClaim: "unavailable-no-claim"
});

export const DNA_OXIDE_PLACEHOLDER_PANEL_CONTRACT = Object.freeze({
  contract: "docs/DNAOXIDE_COMPILE_REFERENCE_PANEL_CONTRACT.md",
  ownsFinalOxVbaDtos: false,
  claims: NO_CLAIM_FLAGS
});

export function createCompileOptionsPanelModel(overrides = {}) {
  return Object.freeze({
    project: Object.freeze({
      name: overrides.projectName ?? "ThinSliceHello",
      projectFile: overrides.projectFile ?? "ThinSliceHello.basproj",
      moduleName: overrides.moduleName ?? "Module1.bas",
      sourcePolicy: overrides.sourcePolicy ?? "WorkspaceOverlay placeholder / final OxVba source-policy DTO pending"
    }),
    compileOptions: Object.freeze({
      state: PLACEHOLDER_PANEL_STATES.pendingOxvbaHardening,
      disabledReason: "Final OxVba compile options DTO and mutation semantics are pending.",
      rows: Object.freeze([
        "Option Explicit: displayed from source text only",
        "Compiler constants: pending OxVba DTO",
        "Reference resolution mode: pending OxVba DTO"
      ])
    }),
    buildCheck: Object.freeze({
      state: PLACEHOLDER_PANEL_STATES.oxvbaFixtureEvidenced,
      disabledReason: "ThinSliceHello covers EmbeddedBuildRunHost::build_workspace, but DnaOxIde adapter proof is pending.",
      outputRows: 0
    }),
    runTarget: Object.freeze({
      state: PLACEHOLDER_PANEL_STATES.pendingOxvbaHardening,
      displayTarget: "Module1.Main",
      disabledReason: "Final OxVba run target DTO and command availability taxonomy are pending."
    }),
    claims: Object.freeze({ ...NO_CLAIM_FLAGS })
  });
}

export function createReferenceComPanelModel(overrides = {}) {
  return Object.freeze({
    references: Object.freeze({
      state: PLACEHOLDER_PANEL_STATES.oxvbaFixtureEvidenced,
      rosterRows: 0,
      disabledReason: "ThinSliceHello fixture evidence covers reference state seams; local DnaOxIde adapter proof is pending."
    }),
    comCandidates: Object.freeze({
      state: PLACEHOLDER_PANEL_STATES.oxvbaFixtureEvidenced,
      availableSubset: "ComSelectionService direct Rust surface",
      candidateRows: 0,
      disabledReason: "COM candidate/capability profile is fixture-evidenced; native boundary and local adapter proof are pending."
    }),
    referenceRepair: Object.freeze({
      state: PLACEHOLDER_PANEL_STATES.pendingOxvbaHardening,
      previewRows: 0,
      disabledReason: "Reference repair/apply DTO is pending OxVba hardening."
    }),
    comRuntime: Object.freeze({
      state: PLACEHOLDER_PANEL_STATES.unavailableNoClaim,
      invocationClaimed: false,
      disabledReason: "COM runtime invocation is not claimed by DnaOxIde."
    }),
    claims: Object.freeze({ ...NO_CLAIM_FLAGS }),
    ...overrides
  });
}

export function renderCompileOptionsPanels(model = createCompileOptionsPanelModel()) {
  return `<section role="host-compile-reference-deck" class="panel placeholder-deck" data-contract="DNAOXIDE_COMPILE_REFERENCE_PANEL_CONTRACT" data-final-oxvba-dtos-owned-here="false" data-real-execution="${String(model.claims.realExecutionClaimed)}" data-native-runtime="${String(model.claims.nativeRuntimeClaimed)}" data-com-runtime="${String(model.claims.comRuntimeClaimed)}" data-fake-responses="${String(model.claims.fakeResponses)}" data-fake-debug-data="${String(model.claims.fakeDebugData)}">
  <section role="host-project-properties-panel" class="panel" data-state="${PLACEHOLDER_PANEL_STATES.provenOxideOnly}" data-project="${escapeHtml(model.project.name)}" data-module="${escapeHtml(model.project.moduleName)}">
    <h2>Project properties</h2>
    <p>${escapeHtml(model.project.name)} / ${escapeHtml(model.project.projectFile)} / ${escapeHtml(model.project.moduleName)}</p>
    <p role="host-source-policy-placeholder">${escapeHtml(model.project.sourcePolicy)}</p>
  </section>
  <section role="host-compile-options-panel" class="panel" data-state="${model.compileOptions.state}" data-enabled="false" data-row-count="${model.compileOptions.rows.length}">
    <h2>Compile options</h2>
    <p role="host-compile-options-disabled-reason">${escapeHtml(model.compileOptions.disabledReason)}</p>
    <ul>${model.compileOptions.rows.map((row) => `<li>${escapeHtml(row)}</li>`).join("")}</ul>
  </section>
  <section role="host-build-check-panel" class="panel" data-state="${model.buildCheck.state}" data-enabled="false" data-output-rows="${model.buildCheck.outputRows}">
    <h2>Build / check</h2>
    <p role="host-build-check-disabled-reason">${escapeHtml(model.buildCheck.disabledReason)}</p>
  </section>
  <section role="host-run-target-panel" class="panel" data-state="${model.runTarget.state}" data-enabled="false" data-display-target="${escapeHtml(model.runTarget.displayTarget)}">
    <h2>Run target</h2>
    <p>${escapeHtml(model.runTarget.displayTarget)}</p>
    <p role="host-run-target-disabled-reason">${escapeHtml(model.runTarget.disabledReason)}</p>
  </section>
</section>`;
}

export function renderReferenceComPanels(model = createReferenceComPanelModel()) {
  return `<section role="host-reference-com-deck" class="panel placeholder-deck" data-contract="DNAOXIDE_COMPILE_REFERENCE_PANEL_CONTRACT" data-final-oxvba-dtos-owned-here="false" data-real-execution="${String(model.claims.realExecutionClaimed)}" data-native-runtime="${String(model.claims.nativeRuntimeClaimed)}" data-com-runtime="${String(model.claims.comRuntimeClaimed)}" data-fake-responses="${String(model.claims.fakeResponses)}" data-fake-debug-data="${String(model.claims.fakeDebugData)}">
  <section role="host-references-panel" class="panel" data-state="${model.references.state}" data-roster-rows="${model.references.rosterRows}" data-enabled="false">
    <h2>References</h2>
    <p role="host-references-disabled-reason">${escapeHtml(model.references.disabledReason)}</p>
  </section>
  <section role="host-com-candidate-panel" class="panel" data-state="${model.comCandidates.state}" data-available-subset="${escapeHtml(model.comCandidates.availableSubset)}" data-candidate-rows="${model.comCandidates.candidateRows}" data-enabled="false">
    <h2>COM candidates</h2>
    <p role="host-com-candidate-disabled-reason">${escapeHtml(model.comCandidates.disabledReason)}</p>
  </section>
  <section role="host-reference-repair-panel" class="panel" data-state="${model.referenceRepair.state}" data-preview-rows="${model.referenceRepair.previewRows}" data-enabled="false">
    <h2>Reference repair/apply preview</h2>
    <p role="host-reference-repair-disabled-reason">${escapeHtml(model.referenceRepair.disabledReason)}</p>
  </section>
  <section role="host-com-runtime-boundary-panel" class="panel" data-state="${model.comRuntime.state}" data-com-runtime-invocation="${String(model.comRuntime.invocationClaimed)}" data-enabled="false">
    <h2>COM runtime boundary</h2>
    <p role="host-com-runtime-disabled-reason">${escapeHtml(model.comRuntime.disabledReason)}</p>
  </section>
</section>`;
}

export function renderPlaceholderPanelDeck() {
  return `${renderCompileOptionsPanels()}\n${renderReferenceComPanels()}`;
}

export function verifyCompilePanelContract(markup = renderCompileOptionsPanels()) {
  return [
    "role=\"host-project-properties-panel\"",
    "role=\"host-compile-options-panel\"",
    "role=\"host-build-check-panel\"",
    "role=\"host-run-target-panel\"",
    "data-state=\"proven-oxide-only\"",
    "data-state=\"pending-oxvba-hardening\"",
    "data-state=\"oxvba-fixture-evidenced\"",
    "EmbeddedBuildRunHost::build_workspace",
    "data-output-rows=\"0\"",
    "data-real-execution=\"false\""
  ].every((token) => markup.includes(token));
}

export function verifyReferenceComPanelContract(markup = renderReferenceComPanels()) {
  return [
    "role=\"host-references-panel\"",
    "role=\"host-com-candidate-panel\"",
    "role=\"host-reference-repair-panel\"",
    "role=\"host-com-runtime-boundary-panel\"",
    "ComSelectionService direct Rust surface",
    "data-com-runtime-invocation=\"false\"",
    "data-com-runtime=\"false\"",
    "data-candidate-rows=\"0\"",
    "data-preview-rows=\"0\""
  ].every((token) => markup.includes(token));
}

function escapeHtml(value) {
  return String(value)
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;")
    .replaceAll("'", "&#39;");
}
