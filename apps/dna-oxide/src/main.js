import {
  createDnaOxIdeHostShellModel,
  renderDnaOxIdeHostShell
} from "./host-shell.js";

import { createBrowserFixtureCommandClient } from "./command-client.js";
import {
  createInstrumentedDnaOxIdeApp,
  installDnaOxIdeTestDriver,
  verifyInstrumentationContract
} from "./app-instrumentation.js";

export const DNA_OXIDE_FRONTEND_SCAFFOLD = Object.freeze({
  productName: "DNA OxIde",
  appName: "DnaOxIde",
  scaffoldKind: "frontend-entry",
  sharedUiOwner: "oxide-ui-leptos shared contract via W345 static host fixture",
  claims: Object.freeze({
    realExecution: false,
    nativeRuntime: false,
    comRuntime: false,
    fakeImmediateResponses: false,
    fakeDebugData: false
  })
});

function renderScaffoldBanner(root) {
  if (!root) {
    return;
  }

  root.dataset.frontendScaffold = "mounted";
  root.dataset.sharedUiImplementedHere = "false";
  root.dataset.realExecution = String(DNA_OXIDE_FRONTEND_SCAFFOLD.claims.realExecution);
  root.dataset.nativeRuntime = String(DNA_OXIDE_FRONTEND_SCAFFOLD.claims.nativeRuntime);
  root.dataset.comRuntime = String(DNA_OXIDE_FRONTEND_SCAFFOLD.claims.comRuntime);
  root.dataset.hostUiProofMode = "static-frontend-host-fixture";

  const model = createDnaOxIdeHostShellModel(createBrowserFixtureCommandClient());
  root.innerHTML = renderDnaOxIdeHostShell(model);
}

function mountInstrumentedApp(root, targetWindow = globalThis.window) {
  if (!root) {
    return null;
  }

  const bootstrap = targetWindow?.__DNA_OXIDE_BOOTSTRAP__ ?? {};
  const app = createInstrumentedDnaOxIdeApp({
    ...bootstrap,
    hostServices: targetWindow?.__DNA_OXIDE_HOST_SERVICES__ ?? null
  });
  root.dataset.frontendScaffold = "mounted";
  root.dataset.sharedUiImplementedHere = "false";
  root.dataset.realExecution = String(DNA_OXIDE_FRONTEND_SCAFFOLD.claims.realExecution);
  root.dataset.nativeRuntime = String(DNA_OXIDE_FRONTEND_SCAFFOLD.claims.nativeRuntime);
  root.dataset.comRuntime = String(DNA_OXIDE_FRONTEND_SCAFFOLD.claims.comRuntime);
  root.dataset.hostUiProofMode = app.instrumentation.proofMode;

  const render = (focusSelector = null) => {
    root.innerHTML = app.renderApp();
    wireInstrumentedAppDom(root, app, render);
    if (focusSelector) {
      root.querySelector(focusSelector)?.focus();
    }
  };

  render();

  if (targetWindow) {
    installDnaOxIdeTestDriver(targetWindow, app);
  }

  return app;
}

function wireInstrumentedAppDom(root, app, render) {
  const editor = root.querySelector('[data-testid="source-editor"]');
  editor?.addEventListener("focus", () => {
    app.injectInteraction({ type: "focusEditor", via: "dom-focus" });
  });
  editor?.addEventListener("input", (event) => {
    app.injectInteraction({
      type: "replaceSource",
      text: event.currentTarget.value,
      via: "dom-input"
    });
    render('[data-testid="source-editor"]');
  });

  root.querySelector('[data-testid="focus-editor-command"]')?.addEventListener("click", () => {
    app.runCommand("focus-editor", { via: "dom-click" });
    render('[data-testid="source-editor"]');
  });

  root.querySelector('[data-testid="save-active-module-command"]')?.addEventListener("click", async () => {
    await app.runHostCommand("save-active-module", { via: "dom-click" });
    render();
  });

  root.querySelector('[data-testid="reload-active-module-command"]')?.addEventListener("click", async () => {
    await app.runHostCommand("reload-active-module", { via: "dom-click" });
    render();
  });
}

export function verifyFrontendScaffold() {
  const { productName, appName, claims } = DNA_OXIDE_FRONTEND_SCAFFOLD;
  return productName === "DNA OxIde"
    && appName === "DnaOxIde"
    && claims.realExecution === false
    && claims.nativeRuntime === false
    && claims.comRuntime === false
    && claims.fakeImmediateResponses === false
    && claims.fakeDebugData === false
    && verifyInstrumentationContract();
}

export { mountInstrumentedApp, renderScaffoldBanner };

if (typeof document !== "undefined") {
  mountInstrumentedApp(document.getElementById("dna-oxide-root"));
}

if (typeof process !== "undefined" && process.argv.includes("--verify-scaffold")) {
  if (!verifyFrontendScaffold()) {
    console.error("DNA OxIde frontend scaffold verification failed");
    process.exit(1);
  }

  console.log("DNA OxIde frontend scaffold verification passed");
}
