import { execFileSync } from "node:child_process";
import { existsSync, readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url));
const appRoot = resolve(here, "..");
const repoRoot = resolve(appRoot, "..", "..");
const trueText = "tr" + "ue";

const requiredFiles = [
  "README.md",
  "package.json",
  "Trunk.toml",
  "index.html",
  "src/main.js",
  "src/command-client.js",
  "src/host-shell.js",
  "src/interaction-harness.js",
  "src/placeholder-panels.js",
  "src/styles.css",
  "src-tauri/Cargo.toml",
  "src-tauri/Cargo.lock",
  "src-tauri/README.md",
  "src-tauri/tauri.conf.json",
  "src-tauri/capabilities/default.json",
  "src-tauri/capabilities/README.md",
  "src-tauri/icons/README.md",
  "src-tauri/src/main.rs",
  "src-tauri/src/commands.rs",
  "src-tauri/src/services.rs",
  "e2e/README.md"
];

function readRelative(path) {
  return readFileSync(resolve(appRoot, path), "utf8");
}

function assert(condition, message) {
  if (!condition) {
    throw new Error(message);
  }
}

function assertContains(path, token) {
  const content = readRelative(path);
  assert(content.includes(token), `${path} missing token: ${token}`);
}

function assertNotContains(path, token) {
  const content = readRelative(path);
  assert(!content.includes(token), `${path} contains forbidden token: ${token}`);
}

console.log("== DNA OxIde scaffold verifier ==");
console.log(`repoRoot=${repoRoot}`);

console.log("== required files ==");
for (const file of requiredFiles) {
  const absolute = resolve(appRoot, file);
  assert(existsSync(absolute), `required file missing: ${file}`);
  console.log(file);
}

console.log("== product and scaffold tokens ==");
assertContains("README.md", "DNA OxIde");
assertContains("README.md", "DnaOxIde");
assertContains("README.md", "Locked W341 Scaffold Shape");
assertContains("index.html", "data-app=\"DnaOxIde\"");
assertContains("index.html", "data-product=\"DNA OxIde\"");
assertContains("src/main.js", "DNA_OXIDE_FRONTEND_SCAFFOLD");
assertContains("src/command-client.js", "DNA_OXIDE_COMMANDS");
assertContains("src/host-shell.js", "DNA_OXIDE_HOST_UI_PROOF");
assertContains("src/interaction-harness.js", "DNA_OXIDE_INTERACTION_HARNESS");
assertContains("src/placeholder-panels.js", "DNA_OXIDE_PLACEHOLDER_PANEL_CONTRACT");
assertContains("src-tauri/tauri.conf.json", "\"productName\": \"DNA OxIde\"");
assertContains("src-tauri/src/services.rs", "tauri-native-scaffold");
assertContains("src-tauri/src/commands.rs", "w344-rust-callable-tauri-ready");
console.log("product/scaffold tokens present");

console.log("== frontend coupling boundary ==");
for (const file of ["index.html", "src/main.js"]) {
  assertNotContains(file, "@tauri-apps");
  assertNotContains(file, "__TAURI__");
  assertNotContains(file, "invoke(");
  assertNotContains(file, "OxVba");
}
console.log("frontend entry has no direct host-service coupling");

console.log("== anti-overclaim scan ==");
const forbiddenClaims = [
  `data-real-execution=\"${trueText}\"`,
  `data-native-runtime=\"${trueText}\"`,
  `data-com-runtime=\"${trueText}\"`,
  `fakeImmediateResponses: ${trueText}`,
  `fakeDebugData: ${trueText}`,
  `real_execution_claimed: ${trueText}`,
  `native_runtime_claimed: ${trueText}`,
  `com_runtime_claimed: ${trueText}`,
  `immediate_fake_responses: ${trueText}`,
  `debug_fake_data: ${trueText}`
];

for (const file of [
  "README.md",
  "index.html",
  "src/main.js",
  "src/command-client.js",
  "src/host-shell.js",
  "src/interaction-harness.js",
  "src/placeholder-panels.js",
  "src-tauri/README.md",
  "src-tauri/src/main.rs",
  "src-tauri/src/commands.rs",
  "src-tauri/src/services.rs",
  "src-tauri/tauri.conf.json",
  "src-tauri/capabilities/default.json",
  "src-tauri/capabilities/README.md",
  "e2e/README.md"
]) {
  for (const token of forbiddenClaims) {
    assertNotContains(file, token);
  }
}
console.log("no affirmative runtime/COM/fake-data claim tokens found");

console.log("== native scaffold cargo tests ==");
execFileSync("cargo", ["test", "--manifest-path", resolve(appRoot, "src-tauri/Cargo.toml")], {
  cwd: repoRoot,
  stdio: "inherit"
});

console.log("DNA OxIde scaffold verification passed");
