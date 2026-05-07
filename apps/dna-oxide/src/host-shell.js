import {
  COMMAND_CLIENT_BUCKETS,
  DNA_OXIDE_COMMANDS,
  NO_CLAIM_FLAGS,
  createBrowserFixtureCommandClient,
  unavailableFixtureResponse
} from "./command-client.js";

export const DNA_OXIDE_HOST_UI_PROOF = Object.freeze({
  productName: "DNA OxIde",
  appName: "DnaOxIde",
  proofMode: "static-frontend-host-fixture",
  hostPath: "apps/dna-oxide/src/main.js",
  sharedUiBoundary: "W342/W343/W344 contracts",
  sharedUiCrate: "oxide-ui-leptos",
  hostBridgeCrate: "oxide-host-bridge",
  liveTauriWebViewIpcDriven: false,
  browserClickKeyAutomationDriven: false,
  fullDomAccessibilityAuditClaimed: false,
  claims: NO_CLAIM_FLAGS
});

const DEFAULT_SOURCE_PREVIEW = `Attribute VB_Name = "Module1"\nOption Explicit\n\nPublic Sub Main()\n    Dim answer As Integer\n    answer = 40 + 2\n    Debug.Print answer\nEnd Sub\n`;

const SERVICE_COMMANDS = Object.freeze({
  runtime: DNA_OXIDE_COMMANDS.runProject,
  immediate: DNA_OXIDE_COMMANDS.evaluateImmediate,
  debug: DNA_OXIDE_COMMANDS.debugAttach,
  com: DNA_OXIDE_COMMANDS.findComCandidates
});

export function createDnaOxIdeHostShellModel(
  commandClient = createBrowserFixtureCommandClient(),
  overrides = {}
) {
  const commandRows = commandClient.commandNames().map((commandName) => {
    const bucket = commandClient.bucketForCommand(commandName);
    return Object.freeze({
      commandName,
      bucket,
      enabled: bucket === COMMAND_CLIENT_BUCKETS.provenOxideOnly,
      disabledReason: bucket === COMMAND_CLIENT_BUCKETS.provenOxideOnly
        ? null
        : `${commandName} remains ${bucket} in the W345 static host proof`
    });
  });

  const runtime = unavailableFixtureResponse(SERVICE_COMMANDS.runtime, { project: "ThinSliceHello" });
  const immediate = unavailableFixtureResponse(SERVICE_COMMANDS.immediate, { expression: "?answer" });
  const debug = unavailableFixtureResponse(SERVICE_COMMANDS.debug, { module: "Module1.bas" });
  const com = unavailableFixtureResponse(SERVICE_COMMANDS.com, { project: "ThinSliceHello" });

  return Object.freeze({
    proof: DNA_OXIDE_HOST_UI_PROOF,
    project: Object.freeze({
      name: overrides.projectName ?? "ThinSliceHello",
      projectFile: overrides.projectFile ?? "ThinSliceHello.basproj",
      moduleName: overrides.moduleName ?? "Module1.bas",
      sourceText: overrides.sourceText ?? DEFAULT_SOURCE_PREVIEW,
      sourceProvenance: overrides.sourceProvenance ?? "static-preview-of-checked-in-thin-slice-fixture"
    }),
    diagnostics: Object.freeze({
      label: "Diagnostics",
      state: "available-subset-or-static-not-run",
      provider: "OxVba language service adapter boundary",
      disabledReason: "Static host proof does not execute diagnostics; W220/W344 cover adapter boundaries."
    }),
    lifecycle: Object.freeze({
      dirty: overrides.lifecycle?.dirty ?? false,
      provider: overrides.lifecycle?.provider ?? "proven-oxide-only",
      proofPath: overrides.lifecycle?.proofPath ?? "static-host-fixture",
      events: Object.freeze(overrides.lifecycle?.events ?? [
        "opened-static-host-fixture",
        "commands-visible"
      ]),
      commands: Object.freeze(overrides.lifecycle?.commands ?? [
        DNA_OXIDE_COMMANDS.openProjectPath,
        DNA_OXIDE_COMMANDS.loadActiveModule,
        DNA_OXIDE_COMMANDS.saveActiveModule,
        DNA_OXIDE_COMMANDS.reloadActiveModule,
        DNA_OXIDE_COMMANDS.revertActiveModule,
        DNA_OXIDE_COMMANDS.saveSessionSnapshot,
        DNA_OXIDE_COMMANDS.loadSessionSnapshot
      ])
    }),
    commandRows: Object.freeze(commandRows),
    services: Object.freeze({ runtime, immediate, debug, com }),
    claims: Object.freeze(commandClient.noClaimFlags())
  });
}

export function renderDnaOxIdeHostShell(model = createDnaOxIdeHostShellModel()) {
  const commandMarkup = model.commandRows.map(renderCommandRow).join("\n");
  const lifecycleMarkup = model.lifecycle.commands.map((commandName) => {
    const bucket = bucketForRenderedCommand(model, commandName);
    return `<li role="host-lifecycle-command" data-command="${escapeHtml(commandName)}" data-state="${escapeHtml(bucket)}">${escapeHtml(commandName)}</li>`;
  }).join("\n");
  const lifecycleEventMarkup = model.lifecycle.events.map((event) => (
    `<li role="host-lifecycle-event" data-event="${escapeHtml(event)}">${escapeHtml(event)}</li>`
  )).join("\n");

  return `<section role="dnaoxide-host-ui-proof" class="host-proof" data-proof-mode="${escapeHtml(model.proof.proofMode)}" data-product="${escapeHtml(model.proof.productName)}" data-app="${escapeHtml(model.proof.appName)}" data-shared-ui-boundary="${escapeHtml(model.proof.sharedUiBoundary)}" data-shared-ui-crate="${escapeHtml(model.proof.sharedUiCrate)}" data-host-bridge-crate="${escapeHtml(model.proof.hostBridgeCrate)}" data-live-tauri-webview-ipc="false" data-browser-click-key-automation="false" data-full-dom-accessibility-audit="false" data-real-execution="${String(model.claims.realExecutionClaimed)}" data-native-runtime="${String(model.claims.nativeRuntimeClaimed)}" data-com-runtime="${String(model.claims.comRuntimeClaimed)}" data-fake-responses="${String(model.claims.fakeResponses)}" data-fake-debug-data="${String(model.claims.fakeDebugData)}">
  <header role="host-branding" class="hero" data-surface="host-shell-mounted">
    <p class="eyebrow">DnaOxIde static host proof</p>
    <h1>${escapeHtml(model.proof.productName)}</h1>
    <p class="summary">Reviewable host shell mounted through the DnaOxIde frontend path. Live Tauri/WebView IPC is not claimed.</p>
  </header>

  <section role="host-project-spine" class="panel" data-project="${escapeHtml(model.project.name)}" data-module="${escapeHtml(model.project.moduleName)}" data-provenance="${escapeHtml(model.project.sourceProvenance)}">
    <h2>Project</h2>
    <p><strong>${escapeHtml(model.project.name)}</strong> / ${escapeHtml(model.project.projectFile)}</p>
    <p>${escapeHtml(model.project.moduleName)}</p>
  </section>

  <section role="host-editor-boundary" class="panel editor-panel" data-module="${escapeHtml(model.project.moduleName)}" data-editable-projection="true">
    <h2>Editor</h2>
    <pre role="host-source-preview"><code>${escapeHtml(model.project.sourceText)}</code></pre>
  </section>

  <section role="host-diagnostics-panel" class="panel" data-state="${escapeHtml(model.diagnostics.state)}" data-provider="${escapeHtml(model.diagnostics.provider)}">
    <h2>${escapeHtml(model.diagnostics.label)}</h2>
    <p role="host-diagnostics-disabled-reason">${escapeHtml(model.diagnostics.disabledReason)}</p>
  </section>

  <section role="host-lifecycle-panel" class="panel" data-provider="${escapeHtml(model.lifecycle.provider)}" data-dirty="${String(model.lifecycle.dirty)}" data-proof-path="${escapeHtml(model.lifecycle.proofPath)}">
    <h2>Lifecycle</h2>
    <ul>${lifecycleMarkup}</ul>
    <ol>${lifecycleEventMarkup}</ol>
  </section>

  <section role="host-command-palette" class="panel command-grid" data-command-count="${model.commandRows.length}">
    <h2>Command palette availability</h2>
${commandMarkup}
  </section>

  ${renderServicePanel("Runtime", "host-runtime-panel", model.services.runtime)}
  ${renderServicePanel("Immediate", "host-immediate-panel", model.services.immediate)}
  ${renderServicePanel("Debug", "host-debug-panel", model.services.debug)}
  ${renderServicePanel("COM references", "host-com-panel", model.services.com)}

  <section role="host-claim-boundaries" class="claim-grid" aria-label="Capability boundaries">
    ${renderClaim("Real execution", "real-execution", model.claims.realExecutionClaimed)}
    ${renderClaim("Native runtime", "native-runtime", model.claims.nativeRuntimeClaimed)}
    ${renderClaim("COM runtime", "com-runtime", model.claims.comRuntimeClaimed)}
    ${renderClaim("Fake Immediate responses", "fake-immediate-responses", model.claims.fakeResponses)}
    ${renderClaim("Fake debug data", "fake-debug-data", model.claims.fakeDebugData)}
  </section>
</section>`;
}

export function verifyHostShellContract(markup = renderDnaOxIdeHostShell()) {
  const requiredTokens = [
    "role=\"dnaoxide-host-ui-proof\"",
    "data-proof-mode=\"static-frontend-host-fixture\"",
    "DNA OxIde",
    "ThinSliceHello",
    "Module1.bas",
    "role=\"host-command-palette\"",
    "dna_oxide_open_project_path",
    "dna_oxide_run_project",
    "dna_oxide_evaluate_immediate",
    "dna_oxide_debug_attach",
    "dna_oxide_find_com_candidates",
    "data-state=\"proven-oxide-only\"",
    "data-state=\"oxvba-available-subset\"",
    "data-state=\"oxvba-fixture-evidenced\"",
    "data-state=\"pending-oxvba-hardening\"",
    "role=\"host-runtime-panel\"",
    "role=\"host-immediate-panel\"",
    "role=\"host-debug-panel\"",
    "role=\"host-com-panel\"",
    "data-real-execution=\"false\"",
    "data-native-runtime=\"false\"",
    "data-com-runtime=\"false\"",
    "data-fake-responses=\"false\"",
    "data-fake-debug-data=\"false\""
  ];

  return requiredTokens.every((token) => markup.includes(token))
    && !markup.includes("data-real-execution=\"true\"")
    && !markup.includes("data-native-runtime=\"true\"")
    && !markup.includes("data-com-runtime=\"true\"")
    && !markup.includes("data-fake-responses=\"true\"")
    && !markup.includes("data-fake-debug-data=\"true\"");
}

function renderCommandRow(row) {
  return `    <div role="host-command" data-command="${escapeHtml(row.commandName)}" data-state="${escapeHtml(row.bucket)}" data-enabled="${String(row.enabled)}">
      <span>${escapeHtml(row.commandName)}</span>
      ${row.disabledReason ? `<span role="host-command-disabled-reason">${escapeHtml(row.disabledReason)}</span>` : ""}
    </div>`;
}

function renderServicePanel(title, role, response) {
  const service = serviceMetadata(role);
  return `<section role="${role}" class="panel" data-command="${escapeHtml(response.commandName)}" data-state="${escapeHtml(response.bucket)}" data-provider="native-service-missing" data-enabled="${String(response.enabled)}" data-real-execution="${String(response.claims.realExecutionClaimed)}" data-native-runtime="${String(response.claims.nativeRuntimeClaimed)}" data-com-runtime="${String(response.claims.comRuntimeClaimed)}" data-fake-responses="${String(response.claims.fakeResponses)}" data-fake-debug-data="${String(response.claims.fakeDebugData)}" ${service.attributes}>
    <h2>${escapeHtml(title)}</h2>
    <p role="host-service-disabled-reason">${escapeHtml(response.disabledReason ?? "available")}</p>
    <p role="host-service-empty-state">${escapeHtml(service.emptyState)}</p>
  </section>`;
}

function serviceMetadata(role) {
  switch (role) {
    case "host-runtime-panel":
      return {
        attributes: 'data-output-events="0" data-runtime-id=""',
        emptyState: "No runtime events or runtime ID are synthesized by the static host proof."
      };
    case "host-immediate-panel":
      return {
        attributes: 'data-immediate-responses="0" data-immediate-session-id=""',
        emptyState: "No Immediate responses are synthesized by the static host proof."
      };
    case "host-debug-panel":
      return {
        attributes: 'data-callstack-frames="0" data-locals="0" data-watches="0" data-breakpoints="0" data-debug-session-id=""',
        emptyState: "No callstack, locals, watches, breakpoints, or debug session ID are synthesized."
      };
    case "host-com-panel":
      return {
        attributes: 'data-com-candidates="0" data-com-runtime-invocation="false"',
        emptyState: "COM capability may be fixture-evidenced, but COM runtime invocation is not claimed."
      };
    default:
      return {
        attributes: 'data-service-rows="0"',
        emptyState: "No service rows are synthesized."
      };
  }
}

function renderClaim(label, id, value) {
  return `<article data-claim="${escapeHtml(id)}" data-value="${String(value)}"><span>${escapeHtml(label)}</span><strong>${String(value)}</strong></article>`;
}

function bucketForRenderedCommand(model, commandName) {
  return model.commandRows.find((row) => row.commandName === commandName)?.bucket
    ?? COMMAND_CLIENT_BUCKETS.unavailableNoClaim;
}

function escapeHtml(value) {
  return String(value)
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;")
    .replaceAll("'", "&#39;");
}
