import {
  COMMAND_CLIENT_BUCKETS,
  DNA_OXIDE_COMMANDS,
  BROWSER_WASM_COMPILE_PROFILE,
  browserWasmCompileCheckUnavailableResponse,
  createBrowserFixtureCommandClient,
  createDnaOxIdeCommandClient,
  unavailableFixtureResponse,
  verifyCommandClientContract
} from "../src/command-client.js";

const invoked = [];
const injected = createDnaOxIdeCommandClient((commandName, payload) => {
  invoked.push({ commandName, payload });
  return Promise.resolve({ commandName, payload, via: "injected" });
});

if (!verifyCommandClientContract()) {
  console.error("DNA OxIde command client contract verification failed");
  process.exit(1);
}

if (injected.tauriImportedHere !== false || injected.sharedUiCoupledToTauri !== false) {
  console.error("DNA OxIde injected client claims Tauri/shared UI coupling");
  process.exit(1);
}

await injected.invoke(DNA_OXIDE_COMMANDS.openProjectPath, { path: "examples/thin-slice/ThinSliceHello.basproj" });
if (invoked.length !== 1 || invoked[0].commandName !== DNA_OXIDE_COMMANDS.openProjectPath) {
  console.error("DNA OxIde injected client did not delegate command invocation");
  process.exit(1);
}

const browserClient = createBrowserFixtureCommandClient();
const runtime = await browserClient.invoke(DNA_OXIDE_COMMANDS.runProject, { entrypoint: "Module1.Main" });
if (runtime.bucket !== COMMAND_CLIENT_BUCKETS.oxvbaFixtureEvidenced || runtime.enabled !== false) {
  console.error("DNA OxIde browser fixture did not label runtime as fixture-evidenced disabled");
  process.exit(1);
}

const compile = unavailableFixtureResponse(DNA_OXIDE_COMMANDS.getCompileOptions);
if (compile.bucket !== COMMAND_CLIENT_BUCKETS.pendingOxvbaHardening || compile.claims.fakeDebugData !== false) {
  console.error("DNA OxIde browser fixture did not preserve pending/no-claim compile state");
  process.exit(1);
}

const browserCompileCheck = await browserClient.invoke(DNA_OXIDE_COMMANDS.buildCheck, { sourceText: "Sub Main()\nEnd Sub" });
if (browserCompileCheck.hostBridgeCommand !== "compile.check"
  || browserCompileCheck.profileId !== BROWSER_WASM_COMPILE_PROFILE.profileId
  || browserCompileCheck.status !== "unavailable"
  || browserCompileCheck.wasmProbe.adapterBacked !== false
  || browserCompileCheck.wasmProbe.nativeFilesystemRequired !== false
  || browserCompileCheck.wasmProbe.nativeProcessRequired !== false
  || browserCompileCheck.wasmProbe.comRuntimeRequired !== false
  || !browserCompileCheck.disabledReason.includes("wasm32-unknown-unknown")
  || !browserCompileCheck.disabledReason.includes("oxvba-com")
  || browserCompileCheck.claims.fakeResponses !== false) {
  console.error("DNA OxIde browser fixture did not return the typed WASM compile/check unavailable packet");
  process.exit(1);
}

const directBrowserCompileCheck = browserWasmCompileCheckUnavailableResponse({ projectPath: "browser-memory" });
if (!directBrowserCompileCheck.unavailableOutputs.includes("native-process-build")) {
  console.error("DNA OxIde browser WASM compile/check packet did not mark native-only outputs unavailable");
  process.exit(1);
}

console.log("DNA OxIde command client contract verification passed");
