import {
  createDnaOxIdeHostShellModel,
  renderDnaOxIdeHostShell
} from "./host-shell.js";
import { createBrowserFixtureCommandClient, NO_CLAIM_FLAGS } from "./command-client.js";
import {
  createEditableSourceBoundary,
  stableSourceHash,
  verifyEditableSourceBoundaryContract
} from "./editable-source-boundary.js";

export const DNA_OXIDE_APP_INSTRUMENTATION = Object.freeze({
  version: "w350-v1",
  proofMode: "browser-dom-playwright-primary",
  fallbackDriver: "bounded-dom-like-driver-if-playwright-library-unavailable",
  visualArtifacts: true,
  domLikeSnapshots: true,
  commandLog: true,
  eventLog: true,
  interactionInjection: true,
  tempProjectCopiesOnly: true,
  liveTauriWebViewIpcDriven: false,
  fullDomAccessibilityAuditClaimed: false,
  claims: NO_CLAIM_FLAGS
});

export const W350_TEST_DRIVER_GLOBAL = "__DNA_OXIDE_TEST_DRIVER__";

const DEFAULT_SOURCE_TEXT = `Attribute VB_Name = "Module1"\nOption Explicit\n\nPublic Sub Main()\n    Dim answer As Integer\n    answer = 40 + 2\n    Debug.Print answer\nEnd Sub\n`;

export function createInstrumentedDnaOxIdeApp(options = {}) {
  const state = {
    productName: "DNA OxIde",
    appName: "DnaOxIde",
    proofMode: DNA_OXIDE_APP_INSTRUMENTATION.proofMode,
    projectName: options.projectName ?? "ThinSliceHello",
    projectFile: options.projectFile ?? "ThinSliceHello.basproj",
    activeModule: options.activeModule ?? "Module1.bas",
    sourceProvenance: options.sourceProvenance ?? "w350-browser-dom-live-editable-source",
    editorFocused: false,
    focusedSurface: "none",
    lastCommand: null,
    lastNativeCommandResult: null,
    lastCompileOptionsResult: null,
    lastBuildCheckResult: null,
    lifecycleStatus: "clean",
    tempProjectRoot: options.tempProjectRoot ?? "target/w350-temp-project-copy",
    hostServices: options.hostServices ?? null,
    eventLog: [],
    commandLog: [],
    sequence: 0,
    claims: { ...NO_CLAIM_FLAGS }
  };

  const sourceBoundary = createEditableSourceBoundary({
    projectName: state.projectName,
    projectFile: state.projectFile,
    activeModule: state.activeModule,
    sourceText: options.sourceText ?? DEFAULT_SOURCE_TEXT,
    persistedSourceText: options.persistedSourceText ?? options.sourceText ?? DEFAULT_SOURCE_TEXT,
    sourceProvenance: state.sourceProvenance,
    tempProjectRoot: state.tempProjectRoot
  });

  function sourceSnapshot() {
    return sourceBoundary.snapshot();
  }

  function dirty() {
    return sourceSnapshot().dirty;
  }

  function refreshLifecycleStatus() {
    state.lifecycleStatus = dirty() ? "dirty" : "clean";
    return state.lifecycleStatus;
  }

  function pushEvent(kind, detail = {}) {
    const event = Object.freeze({
      sequence: ++state.sequence,
      kind,
      detail: { ...detail },
      dirty: dirty(),
      sourceLength: sourceSnapshot().sourceTextLength
    });
    state.eventLog.push(event);
    return event;
  }

  function pushCommand(commandName, detail = {}) {
    const command = Object.freeze({
      sequence: ++state.sequence,
      commandName,
      detail: { ...detail },
      bucket: "proven-oxide-only-browser-dom-harness",
      enabled: true,
      dirtyBefore: dirty(),
      claims: { ...NO_CLAIM_FLAGS }
    });
    state.commandLog.push(command);
    state.lastCommand = commandName;
    return command;
  }

  function snapshot() {
    refreshLifecycleStatus();
    const source = sourceSnapshot();
    return Object.freeze({
      version: DNA_OXIDE_APP_INSTRUMENTATION.version,
      productName: state.productName,
      appName: state.appName,
      proofMode: state.proofMode,
      projectName: source.projectName,
      projectFile: source.projectFile,
      activeModule: source.activeModule,
      sourceText: source.sourceText,
      sourceTextLength: source.sourceTextLength,
      sourceTextHash: source.sourceTextHash,
      persistedSourceText: source.persistedSourceText,
      persistedSourceTextLength: source.persistedSourceTextLength,
      persistedSourceTextHash: source.persistedSourceTextHash,
      lastReloadedSourceText: source.lastReloadedSourceText,
      lastReloadedSourceTextHash: source.lastReloadedSourceTextHash,
      editorFocused: state.editorFocused,
      focusedSurface: state.focusedSurface,
      dirty: source.dirty,
      lifecycleStatus: state.lifecycleStatus,
      tempProjectRoot: source.tempProjectRoot,
      lastCommand: state.lastCommand,
      commandLogLength: state.commandLog.length,
      eventLogLength: state.eventLog.length,
      lifecycleCommandStates: source.commandStates,
      editableSourceBoundary: source.boundary,
      hostCommandBoundary: Object.freeze({
        saveActiveModuleAvailable: typeof state.hostServices?.saveActiveModule === "function",
        reloadActiveModuleAvailable: typeof state.hostServices?.reloadActiveModule === "function",
        desktopHostCapabilitiesProbeAvailable: typeof state.hostServices?.desktopHostCapabilitiesProbe === "function",
        compileOptionsAvailable: typeof state.hostServices?.getCompileOptions === "function",
        buildCheckAvailable: typeof state.hostServices?.buildCheck === "function",
        provider: state.hostServices?.provider ?? (state.hostServices ? "injected-browser-host-service" : "in-memory-browser-harness"),
        lastNativeCommandResult: state.lastNativeCommandResult,
        lastCompileOptionsResult: state.lastCompileOptionsResult,
        lastBuildCheckResult: state.lastBuildCheckResult
      }),
      noClaimFlags: { ...state.claims },
      instrumentation: { ...DNA_OXIDE_APP_INSTRUMENTATION }
    });
  }

  function eventLog() {
    return state.eventLog.map((event) => ({ ...event, detail: { ...event.detail } }));
  }

  function commandLog() {
    return state.commandLog.map((command) => ({
      ...command,
      detail: { ...command.detail },
      claims: { ...command.claims }
    }));
  }

  function visualSnapshot() {
    const snap = snapshot();
    return `<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>DNA OxIde W350 visual snapshot</title>
  <style>
    body { font-family: system-ui, sans-serif; margin: 1.5rem; background: #10141f; color: #edf2ff; }
    .frame { border: 1px solid #4f6bff; border-radius: 12px; padding: 1rem; }
    textarea { width: 100%; min-height: 14rem; background: #070a12; color: #e4ecff; border: 1px solid #31406b; }
    .status { color: ${snap.dirty ? "#ffd166" : "#95d5b2"}; }
    code { color: #9bf6ff; }
  </style>
</head>
<body data-proof-mode="${escapeHtml(snap.proofMode)}" data-dirty="${String(snap.dirty)}" data-native-runtime="false" data-com-runtime="false" data-fake-responses="false" data-fake-debug-data="false">
  <main class="frame" role="dnaoxide-w350-visual-snapshot">
    <h1>${escapeHtml(snap.productName)}</h1>
    <p><code>${escapeHtml(snap.projectName)}</code> / <code>${escapeHtml(snap.activeModule)}</code></p>
    <p class="status">Lifecycle: ${escapeHtml(snap.lifecycleStatus)}</p>
    <textarea readonly data-testid="source-editor-visual">${escapeHtml(snap.sourceText)}</textarea>
    <p>Events: ${snap.eventLogLength}; Commands: ${snap.commandLogLength}; Last command: ${escapeHtml(snap.lastCommand ?? "none")}</p>
  </main>
</body>
</html>`;
  }

  function renderHostMarkup() {
    const model = createDnaOxIdeHostShellModel(createBrowserFixtureCommandClient(), {
      projectName: sourceSnapshot().projectName,
      projectFile: sourceSnapshot().projectFile,
      moduleName: sourceSnapshot().activeModule,
      sourceText: sourceSnapshot().sourceText,
      sourceProvenance: sourceSnapshot().sourceProvenance,
      lifecycle: {
        dirty: dirty(),
        provider: "w350-browser-dom-instrumented",
        proofPath: state.proofMode,
        events: state.eventLog.map((event) => `${event.sequence}:${event.kind}`)
      }
    });

    return renderDnaOxIdeHostShell(model);
  }

  function renderApp() {
    const snap = snapshot();
    return `<section role="dnaoxide-w350-app" data-testid="dnaoxide-w350-app" data-driver-version="${escapeHtml(snap.version)}" data-proof-mode="${escapeHtml(snap.proofMode)}" data-product="${escapeHtml(snap.productName)}" data-app="${escapeHtml(snap.appName)}" data-project="${escapeHtml(snap.projectName)}" data-module="${escapeHtml(snap.activeModule)}" data-dirty="${String(snap.dirty)}" data-editor-focused="${String(snap.editorFocused)}" data-last-command="${escapeHtml(snap.lastCommand ?? "")}" data-event-log-length="${snap.eventLogLength}" data-command-log-length="${snap.commandLogLength}" data-real-execution="false" data-native-runtime="false" data-com-runtime="false" data-fake-responses="false" data-fake-debug-data="false" data-live-tauri-webview-ipc="false" data-full-dom-accessibility-audit="false">
  <header class="dnaoxide-app-header" data-testid="app-header">
    <p class="eyebrow">DnaOxIde live editable browser proof</p>
    <h1>${escapeHtml(snap.productName)}</h1>
    <p data-testid="project-title">${escapeHtml(snap.projectName)} / ${escapeHtml(snap.activeModule)}</p>
  </header>
  <nav class="dnaoxide-toolbar" aria-label="DnaOxIde commands">
    <button type="button" data-testid="focus-editor-command" data-command="focus-editor">Focus editor</button>
    <button type="button" data-testid="save-active-module-command" data-command="save-active-module">Save</button>
    <button type="button" data-testid="reload-active-module-command" data-command="reload-active-module">Reload</button>
    <button type="button" data-testid="compile-options-command" data-command="compile-options">Compile options</button>
    <button type="button" data-testid="build-check-command" data-command="build-check">Build check</button>
    <button type="button" data-testid="desktop-host-probe-command" data-command="desktop-host-capabilities-probe">Native host probe</button>
  </nav>
  <section class="dnaoxide-layout" data-testid="app-layout">
    <aside class="panel" data-testid="project-panel" data-project-file="${escapeHtml(snap.projectFile)}">
      <h2>Project</h2>
      <p>${escapeHtml(snap.projectFile)}</p>
      <p>Temp project copy only: ${escapeHtml(snap.tempProjectRoot)}</p>
    </aside>
    <main class="panel editor-panel" data-testid="editor-panel" data-module="${escapeHtml(snap.activeModule)}">
      <h2>Source</h2>
      <textarea data-testid="source-editor" data-role="source-editor" spellcheck="false" aria-label="Source editor for ${escapeHtml(snap.activeModule)}">${escapeHtml(snap.sourceText)}</textarea>
      <p data-testid="dirty-indicator" data-dirty="${String(snap.dirty)}">${snap.dirty ? "Dirty" : "Clean"}</p>
    </main>
    <aside class="panel" data-testid="instrumentation-panel">
      <h2>Instrumentation</h2>
      <dl>
        <dt>Events</dt><dd data-testid="event-count">${snap.eventLogLength}</dd>
        <dt>Commands</dt><dd data-testid="command-count">${snap.commandLogLength}</dd>
        <dt>Focused surface</dt><dd data-testid="focused-surface">${escapeHtml(snap.focusedSurface)}</dd>
        <dt>Last command</dt><dd data-testid="last-command">${escapeHtml(snap.lastCommand ?? "none")}</dd>
        <dt>Host provider</dt><dd data-testid="host-command-provider">${escapeHtml(snap.hostCommandBoundary.provider)}</dd>
        <dt>Native probe</dt><dd data-testid="native-host-probe-result" data-linked-native-rust="${String(snap.hostCommandBoundary.lastNativeCommandResult?.linked_native_rust === true)}">${escapeHtml(snap.hostCommandBoundary.lastNativeCommandResult?.command_name ?? "not-run")}</dd>
      </dl>
    </aside>
    <aside class="panel" data-testid="compile-build-panel" data-profile-id="${escapeHtml(snap.hostCommandBoundary.lastBuildCheckResult?.profileId ?? snap.hostCommandBoundary.lastCompileOptionsResult?.profileId ?? "unavailable")}" data-provider="${escapeHtml(snap.hostCommandBoundary.lastBuildCheckResult?.providerLabel ?? snap.hostCommandBoundary.lastCompileOptionsResult?.providerLabel ?? "none")}" data-build-status="${escapeHtml(snap.hostCommandBoundary.lastBuildCheckResult?.status ?? "not-run")}" data-build-enabled="${String(snap.hostCommandBoundary.lastBuildCheckResult?.enabled === true)}" data-fake-responses="false" data-real-execution="false" data-native-runtime="false" data-com-runtime="false">
      <h2>Compile/build</h2>
      <p data-testid="compile-profile">${escapeHtml(snap.hostCommandBoundary.lastBuildCheckResult?.profileId ?? snap.hostCommandBoundary.lastCompileOptionsResult?.profileId ?? "not-run")}</p>
      <p data-testid="compile-options-summary">${escapeHtml(snap.hostCommandBoundary.lastCompileOptionsResult ? `${snap.hostCommandBoundary.lastCompileOptionsResult.outputType}/${snap.hostCommandBoundary.lastCompileOptionsResult.buildTarget}/${snap.hostCommandBoundary.lastCompileOptionsResult.runtimeFlavor}` : "not-run")}</p>
      <p data-testid="build-check-status">${escapeHtml(snap.hostCommandBoundary.lastBuildCheckResult?.status ?? "not-run")}</p>
      <p data-testid="build-check-summary">${escapeHtml(snap.hostCommandBoundary.lastBuildCheckResult?.compiledSummary ? `procedures=${snap.hostCommandBoundary.lastBuildCheckResult.compiledSummary.procedure_count}; instructions=${snap.hostCommandBoundary.lastBuildCheckResult.compiledSummary.instruction_count}` : "no compiled summary")}</p>
      <p data-testid="compile-unavailable-outputs">${escapeHtml((snap.hostCommandBoundary.lastBuildCheckResult?.unavailableOutputs ?? snap.hostCommandBoundary.lastCompileOptionsResult?.unavailableOptions ?? []).join(", "))}</p>
    </aside>
  </section>
  <details class="panel" data-testid="host-shell-details">
    <summary>Host shell compatibility markup</summary>
    ${renderHostMarkup()}
  </details>
</section>`;
  }

  function injectInteraction(interaction) {
    if (!interaction || typeof interaction !== "object") {
      throw new TypeError("injectInteraction requires an interaction object");
    }

    switch (interaction.type) {
      case "focusEditor":
        state.editorFocused = true;
        state.focusedSurface = "source-editor";
        pushEvent("editor-focused", { via: interaction.via ?? "test-driver" });
        break;
      case "replaceSource":
        if (typeof interaction.text !== "string") {
          throw new TypeError("replaceSource interaction requires text");
        }
        sourceBoundary.replaceSource(interaction.text, { via: interaction.via ?? "test-driver" });
        refreshLifecycleStatus();
        pushEvent("source-replaced", {
          via: interaction.via ?? "test-driver",
          textLength: interaction.text.length,
          textHash: stableSourceHash(interaction.text)
        });
        break;
      case "appendSource":
        if (typeof interaction.text !== "string") {
          throw new TypeError("appendSource interaction requires text");
        }
        sourceBoundary.appendSource(interaction.text, { via: interaction.via ?? "test-driver" });
        refreshLifecycleStatus();
        pushEvent("source-appended", {
          via: interaction.via ?? "test-driver",
          textLength: interaction.text.length,
          textHash: stableSourceHash(interaction.text)
        });
        break;
      case "command":
        return runCommand(interaction.commandName, interaction.payload ?? {});
      default:
        throw new Error(`Unsupported W350 interaction type: ${interaction.type}`);
    }

    return snapshot();
  }

  async function runHostCommand(commandName, payload = {}) {
    const command = pushCommand(commandName, { ...payload, commandBoundary: "host-service" });
    switch (commandName) {
      case "save-active-module":
      case "dna_oxide_save_active_module": {
        let response = null;
        if (typeof state.hostServices?.saveActiveModule === "function") {
          response = await state.hostServices.saveActiveModule({
            projectName: sourceSnapshot().projectName,
            projectFile: sourceSnapshot().projectFile,
            activeModule: sourceSnapshot().activeModule,
            tempProjectRoot: sourceSnapshot().tempProjectRoot,
            sourceText: sourceSnapshot().sourceText,
            sourceTextHash: sourceSnapshot().sourceTextHash,
            requestKind: "save-active-module"
          });
        }
        sourceBoundary.saveToPersisted({ commandName, response });
        refreshLifecycleStatus();
        pushEvent("source-saved-to-temp-copy", {
          commandName,
          commandBoundary: response ? "injected-browser-host-service" : "in-memory-browser-harness",
          module: sourceSnapshot().activeModule,
          tempProjectRoot: sourceSnapshot().tempProjectRoot,
          sourceTextHash: sourceSnapshot().sourceTextHash,
          response
        });
        break;
      }
      case "reload-active-module":
      case "dna_oxide_reload_active_module": {
        let response = null;
        if (typeof state.hostServices?.reloadActiveModule === "function") {
          response = await state.hostServices.reloadActiveModule({
            projectName: sourceSnapshot().projectName,
            projectFile: sourceSnapshot().projectFile,
            activeModule: sourceSnapshot().activeModule,
            tempProjectRoot: sourceSnapshot().tempProjectRoot,
            requestKind: "reload-active-module"
          });
        }
        if (response && typeof response.sourceText === "string") {
          sourceBoundary.loadPersistedSource(response.sourceText, { commandName, response });
        } else {
          sourceBoundary.reloadFromPersisted({ commandName, response });
        }
        refreshLifecycleStatus();
        pushEvent("source-reloaded-from-temp-copy", {
          commandName,
          commandBoundary: response ? "injected-browser-host-service" : "in-memory-browser-harness",
          module: sourceSnapshot().activeModule,
          tempProjectRoot: sourceSnapshot().tempProjectRoot,
          sourceTextHash: sourceSnapshot().sourceTextHash,
          response
        });
        break;
      }
      case "desktop-host-capabilities-probe":
      case "dna_oxide_desktop_host_capabilities_probe": {
        if (typeof state.hostServices?.desktopHostCapabilitiesProbe !== "function") {
          return runCommand(commandName, payload);
        }
        const response = await state.hostServices.desktopHostCapabilitiesProbe(payload);
        state.lastNativeCommandResult = response;
        pushEvent("desktop-host-capabilities-probed", {
          commandName,
          commandBoundary: "tauri-linked-native-rust",
          linkedNativeRust: response?.linked_native_rust === true,
          availabilityCount: response?.availability_count ?? null,
          noClaimFlagsFalse: response
            ? response.real_execution_claimed === false
              && response.native_runtime_claimed === false
              && response.com_runtime_claimed === false
              && response.fake_responses === false
              && response.fake_debug_data === false
            : false
        });
        break;
      }
      case "compile-options":
      case "dna_oxide_get_compile_options": {
        if (typeof state.hostServices?.getCompileOptions !== "function") {
          return runCommand(commandName, payload);
        }
        const response = await state.hostServices.getCompileOptions({
          ...payload,
          projectName: sourceSnapshot().projectName,
          projectFile: sourceSnapshot().projectFile,
          activeModule: sourceSnapshot().activeModule,
          tempProjectRoot: sourceSnapshot().tempProjectRoot,
          requestKind: "compile-options"
        });
        state.lastCompileOptionsResult = response;
        pushEvent("compile-options-loaded", {
          commandName,
          commandBoundary: "tauri-linked-native-rust",
          profileId: response?.profileId ?? response?.profile_id ?? null,
          providerLabel: response?.providerLabel ?? response?.provider_label ?? null,
          outputType: response?.outputType ?? response?.output_type ?? null,
          buildTarget: response?.buildTarget ?? response?.build_target ?? null,
          noClaimFlagsFalse: compileNoClaimFlagsFalse(response)
        });
        break;
      }
      case "build-check":
      case "dna_oxide_build_check": {
        if (typeof state.hostServices?.buildCheck !== "function") {
          return runCommand(commandName, payload);
        }
        const response = await state.hostServices.buildCheck({
          ...payload,
          projectName: sourceSnapshot().projectName,
          projectFile: sourceSnapshot().projectFile,
          activeModule: sourceSnapshot().activeModule,
          tempProjectRoot: sourceSnapshot().tempProjectRoot,
          sourceTextHash: sourceSnapshot().sourceTextHash,
          requestKind: "build-check"
        });
        state.lastBuildCheckResult = response;
        pushEvent("build-check-completed", {
          commandName,
          commandBoundary: "tauri-linked-native-rust",
          profileId: response?.profileId ?? response?.profile_id ?? null,
          providerLabel: response?.providerLabel ?? response?.provider_label ?? null,
          status: response?.status ?? null,
          diagnosticCount: response?.diagnostics?.length ?? 0,
          hasCompiledSummary: Boolean(response?.compiledSummary ?? response?.compiled_summary),
          noClaimFlagsFalse: compileNoClaimFlagsFalse(response)
        });
        break;
      }
      default:
        return runCommand(commandName, payload);
    }

    return Object.freeze({ command, snapshot: snapshot() });
  }

  function runCommand(commandName, payload = {}) {
    const command = pushCommand(commandName, payload);
    switch (commandName) {
      case "focus-editor":
      case "dna_oxide_focus_editor":
        state.editorFocused = true;
        state.focusedSurface = "source-editor";
        pushEvent("command-focused-editor", { commandName });
        break;
      case "save-active-module":
      case "dna_oxide_save_active_module":
        sourceBoundary.saveToPersisted({ commandName });
        refreshLifecycleStatus();
        pushEvent("source-saved-to-temp-copy", {
          commandName,
          module: sourceSnapshot().activeModule,
          tempProjectRoot: sourceSnapshot().tempProjectRoot,
          sourceTextHash: sourceSnapshot().sourceTextHash
        });
        break;
      case "reload-active-module":
      case "dna_oxide_reload_active_module":
        sourceBoundary.reloadFromPersisted({ commandName });
        refreshLifecycleStatus();
        pushEvent("source-reloaded-from-temp-copy", {
          commandName,
          module: sourceSnapshot().activeModule,
          tempProjectRoot: sourceSnapshot().tempProjectRoot,
          sourceTextHash: sourceSnapshot().sourceTextHash
        });
        break;
      default:
        pushEvent("command-recorded-no-native-dispatch", { commandName });
        break;
    }

    return Object.freeze({ command, snapshot: snapshot() });
  }

  pushEvent("instrumented-app-created", { proofMode: state.proofMode });

  return Object.freeze({
    instrumentation: DNA_OXIDE_APP_INSTRUMENTATION,
    snapshot,
    eventLog,
    commandLog,
    visualSnapshot,
    renderApp,
    renderHostMarkup,
    injectInteraction,
    runCommand,
    runHostCommand
  });
}

export function installDnaOxIdeTestDriver(targetWindow, app = createInstrumentedDnaOxIdeApp()) {
  if (!targetWindow || typeof targetWindow !== "object") {
    throw new TypeError("installDnaOxIdeTestDriver requires a window-like object");
  }
  Object.defineProperty(targetWindow, W350_TEST_DRIVER_GLOBAL, {
    value: app,
    enumerable: false,
    configurable: true,
    writable: false
  });
  return app;
}

export function verifyInstrumentationContract(app = createInstrumentedDnaOxIdeApp()) {
  if (!verifyEditableSourceBoundaryContract()) {
    return false;
  }

  const before = app.snapshot();
  app.injectInteraction({ type: "focusEditor" });
  app.injectInteraction({ type: "appendSource", text: "\n' W350 instrumentation smoke" });
  const afterEdit = app.snapshot();
  app.runCommand("save-active-module");
  const afterSave = app.snapshot();
  const markup = app.renderApp();
  const visual = app.visualSnapshot();
  const eventKinds = app.eventLog().map((event) => event.kind);
  const commandNames = app.commandLog().map((command) => command.commandName);

  return before.dirty === false
    && before.proofMode === "browser-dom-playwright-primary"
    && afterEdit.dirty === true
    && afterEdit.editorFocused === true
    && afterSave.dirty === false
    && afterSave.persistedSourceText.includes("W350 instrumentation smoke")
    && eventKinds.includes("editor-focused")
    && eventKinds.includes("source-appended")
    && eventKinds.includes("source-saved-to-temp-copy")
    && commandNames.includes("save-active-module")
    && markup.includes('data-testid="source-editor"')
    && markup.includes('data-testid="dirty-indicator"')
    && markup.includes('data-real-execution="false"')
    && markup.includes('data-native-runtime="false"')
    && markup.includes('data-com-runtime="false"')
    && markup.includes('data-fake-responses="false"')
    && markup.includes('data-fake-debug-data="false"')
    && visual.includes('role="dnaoxide-w350-visual-snapshot"')
    && !containsForbiddenClaimToken(markup);
}

function compileNoClaimFlagsFalse(response) {
  const flags = response?.noClaims ?? response?.no_claims;
  if (!flags) {
    return false;
  }
  return flags.real_execution_claimed === false
    && flags.native_runtime_claimed === false
    && flags.com_runtime_claimed === false
    && flags.fake_responses === false
    && flags.fake_debug_data === false;
}

export function forbiddenClaimTokens() {
  const trueText = "tr" + "ue";
  return Object.freeze([
    `data-real-execution="${trueText}"`,
    `data-native-runtime="${trueText}"`,
    `data-com-runtime="${trueText}"`,
    `data-fake-responses="${trueText}"`,
    `data-fake-debug-data="${trueText}"`,
    `realExecutionClaimed":${trueText}`,
    `nativeRuntimeClaimed":${trueText}`,
    `comRuntimeClaimed":${trueText}`,
    `fakeResponses":${trueText}`,
    `fakeDebugData":${trueText}`
  ]);
}

export function containsForbiddenClaimToken(text) {
  return forbiddenClaimTokens().some((token) => String(text).includes(token));
}

function escapeHtml(value) {
  return String(value)
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;")
    .replaceAll("'", "&#39;");
}
