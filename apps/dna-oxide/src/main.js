import {
  createDnaOxIdeHostShellModel,
  renderDnaOxIdeHostShell
} from "./host-shell.js";

import { createBrowserFixtureCommandClient } from "./command-client.js";

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

export function verifyFrontendScaffold() {
  const { productName, appName, claims } = DNA_OXIDE_FRONTEND_SCAFFOLD;
  return productName === "DNA OxIde"
    && appName === "DnaOxIde"
    && claims.realExecution === false
    && claims.nativeRuntime === false
    && claims.comRuntime === false
    && claims.fakeImmediateResponses === false
    && claims.fakeDebugData === false;
}

if (typeof document !== "undefined") {
  renderScaffoldBanner(document.getElementById("dna-oxide-root"));
}

if (typeof process !== "undefined" && process.argv.includes("--verify-scaffold")) {
  if (!verifyFrontendScaffold()) {
    console.error("DNA OxIde frontend scaffold verification failed");
    process.exit(1);
  }

  console.log("DNA OxIde frontend scaffold verification passed");
}
