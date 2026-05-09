import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { containsForbiddenClaimToken, createInstrumentedDnaOxIdeApp } from "../src/app-instrumentation.js";

const here = dirname(fileURLToPath(import.meta.url));
const appRoot = resolve(here, "..");
const repoRoot = resolve(appRoot, "..", "..");
const targetDir = resolve(repoRoot, "target");

function assert(condition, message) {
  if (!condition) {
    throw new Error(message);
  }
}

const hostServices = Object.freeze({
  provider: "tauri-linked-native-rust",
  async saveActiveModule(payload = {}) {
    return {
      commandName: "dna_oxide_save_active_module",
      hostBridgeCommand: "document.save",
      providerLabel: "native-filesystem",
      projectName: payload.projectName,
      activeModule: payload.activeModule,
      sourceText: payload.sourceText,
      noClaims: {
        real_execution_claimed: false,
        native_runtime_claimed: false,
        com_runtime_claimed: false,
        fake_responses: false,
        fake_debug_data: false
      }
    };
  },
  async getCompileOptions() {
    return {
      commandName: "dna_oxide_get_compile_options",
      hostBridgeCommand: "compile.options",
      bucketLabel: "OxVbaAvailableSubset",
      enabled: true,
      profileId: "dnaoxide-desktop-tauri-native",
      providerLabel: "oxvba-current-api",
      projectPath: "target/w355-ui-temp/ThinSliceHello.basproj",
      projectName: "ThinSliceHello",
      outputType: "Exe",
      buildTarget: "Bundle",
      runtimeFlavor: "Lite",
      moduleCount: 2,
      unavailableOptions: ["native-wrapper-output-path", "com-runtime-invocation"],
      noClaims: {
        real_execution_claimed: false,
        native_runtime_claimed: false,
        com_runtime_claimed: false,
        fake_responses: false,
        fake_debug_data: false
      }
    };
  },
  async buildCheck() {
    return {
      commandName: "dna_oxide_build_check",
      hostBridgeCommand: "compile.check",
      bucketLabel: "OxVbaAvailableSubset",
      enabled: true,
      profileId: "dnaoxide-desktop-tauri-native",
      providerLabel: "oxvba-current-api",
      status: "succeeded",
      projectPath: "target/w355-ui-temp/ThinSliceHello.basproj",
      projectName: "ThinSliceHello",
      diagnostics: [],
      compiledSummary: {
        instruction_count: 12,
        procedure_count: 2,
        slot_count: 3,
        user_slot_count: 1
      },
      unavailableOutputs: ["native-wrapper-exe", "runtime-run", "debug-session", "immediate-window", "com-server"],
      noClaims: {
        real_execution_claimed: false,
        native_runtime_claimed: false,
        com_runtime_claimed: false,
        fake_responses: false,
        fake_debug_data: false
      }
    };
  }
});

const app = createInstrumentedDnaOxIdeApp({ hostServices });
const before = app.snapshot();
app.injectInteraction({ type: "appendSource", text: "\n' W355 UI compile adoption proof", via: "verify-compile-build-ui" });
await app.runHostCommand("save-active-module", { via: "verify-compile-build-ui" });
await app.runHostCommand("compile-options", { via: "verify-compile-build-ui" });
await app.runHostCommand("build-check", { via: "verify-compile-build-ui" });
const after = app.snapshot();
const markup = app.renderApp();
const events = app.eventLog();
const commands = app.commandLog();

assert(before.hostCommandBoundary.buildCheckAvailable === true, "build check host service unavailable");
assert(after.dirty === false, "saved source should be clean before compile result");
assert(after.hostCommandBoundary.lastCompileOptionsResult.profileId === "dnaoxide-desktop-tauri-native", "compile options profile not adopted");
assert(after.hostCommandBoundary.lastBuildCheckResult.status === "succeeded", "build check status not adopted");
assert(markup.includes('data-testid="compile-build-panel"'), "compile/build panel missing");
assert(markup.includes('data-profile-id="dnaoxide-desktop-tauri-native"'), "compile/build panel missing profile id");
assert(markup.includes('data-provider="oxvba-current-api"'), "compile/build panel missing provider");
assert(markup.includes('data-build-status="succeeded"'), "compile/build panel missing status");
assert(markup.includes("procedures=2"), "compile/build summary missing procedure count");
assert(markup.includes("runtime-run"), "compile/build unavailable output missing runtime-run");
assert(events.some((event) => event.kind === "compile-options-loaded"), "compile options event missing");
assert(events.some((event) => event.kind === "build-check-completed"), "build check event missing");
assert(commands.some((command) => command.commandName === "build-check"), "build check command missing");
assert(!containsForbiddenClaimToken(JSON.stringify(after)), "snapshot contains forbidden overclaim token");
assert(!containsForbiddenClaimToken(markup), "markup contains forbidden overclaim token");

mkdirSync(targetDir, { recursive: true });
writeFileSync(resolve(targetDir, "w355-b03-compile-build-ui-state.json"), `${JSON.stringify({ before, after, events, commands }, null, 2)}\n`, "utf8");
writeFileSync(resolve(targetDir, "w355-b03-compile-build-ui.html"), markup, "utf8");
writeFileSync(resolve(targetDir, "w355-b03-compile-build-ui.txt"), [
  "W355-B03 compile/build UI adoption proof",
  `repoRoot=${repoRoot}`,
  "profile=dnaoxide-desktop-tauri-native",
  "provider=oxvba-current-api",
  "compile_status=succeeded",
  "compile_build_panel=true",
  "no_fake_build_output=true",
  "runtime_debug_immediate_com_claims_remain_false=true",
  ""
].join("\n"), "utf8");

console.log("W355-B03 compile/build UI adoption verification passed");
