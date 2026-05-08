import { createInstrumentedDnaOxIdeApp, containsForbiddenClaimToken } from "../src/app-instrumentation.js";
import { createTauriDesktopHostServices } from "../src/main.js";

function assert(condition, message) {
  if (!condition) {
    throw new Error(message);
  }
}

const calls = [];
const tauriWindow = {
  ["__" + "TAURI__"]: {
    core: {
      invoke(commandName, payload) {
        calls.push({ commandName, payload });
        return Promise.resolve({
          command_name: "dna_oxide_desktop_host_capabilities_probe",
          command_spine: "WebView UI -> Tauri invoke -> #[tauri::command] -> linked Rust command module -> typed packet",
          app_name: "DnaOxIde",
          product_name: "DNA OxIde",
          linked_native_rust: true,
          project_path: payload.projectPath ?? "examples/thin-slice/ThinSliceHello.basproj",
          availability_count: 26,
          enabled_count: 10,
          disabled_count: 16,
          sample_enabled_command: "project.open",
          sample_disabled_command: "runtime.run",
          real_execution_claimed: false,
          native_runtime_claimed: false,
          com_runtime_claimed: false,
          fake_responses: false,
          fake_debug_data: false
        });
      }
    }
  }
};

const services = createTauriDesktopHostServices(tauriWindow);
assert(services?.provider === "tauri-linked-native-rust", "expected Tauri linked-native host provider");

const app = createInstrumentedDnaOxIdeApp({ hostServices: services });
const before = app.snapshot();
assert(before.hostCommandBoundary.desktopHostCapabilitiesProbeAvailable === true, "native probe unavailable");

await app.runHostCommand("desktop-host-capabilities-probe", { via: "w352-b01-check" });
const after = app.snapshot();
const eventKinds = app.eventLog().map((event) => event.kind);
const markup = app.renderApp();

assert(calls.length === 1, "expected one Tauri invoke call");
assert(calls[0].commandName === "dna_oxide_desktop_host_capabilities_probe", "wrong Tauri command invoked");
assert(after.hostCommandBoundary.provider === "tauri-linked-native-rust", "wrong host command provider");
assert(after.hostCommandBoundary.lastNativeCommandResult.linked_native_rust === true, "linked native Rust flag not surfaced");
assert(after.hostCommandBoundary.lastNativeCommandResult.availability_count === 26, "availability count not surfaced");
assert(eventKinds.includes("desktop-host-capabilities-probed"), "probe event not recorded");
assert(markup.includes('data-testid="desktop-host-probe-command"'), "native probe button missing");
assert(markup.includes('data-linked-native-rust="true"'), "linked native Rust render marker missing");
assert(!containsForbiddenClaimToken(JSON.stringify(after)), "forbidden claim token appeared in snapshot");
assert(!containsForbiddenClaimToken(markup), "forbidden claim token appeared in markup");

console.log("W352-B01 Tauri command spine verification passed");
