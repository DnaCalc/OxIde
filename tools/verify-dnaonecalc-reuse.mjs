import { execFileSync } from "node:child_process";
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";

const repoRoot = process.cwd();
const targetDir = resolve(repoRoot, "target");
const profilePath = resolve(repoRoot, "docs", "fixtures", "dnaonecalc-consumer-profile.json");
const profile = JSON.parse(readFileSync(profilePath, "utf8"));
mkdirSync(targetDir, { recursive: true });

const dnaOneCalcRender = runGuilab("gui-dnaonecalc-web-shell-host-contract");
const sharedUiRender = runGuilab("gui-shared-ui-shell-component");
writeIfAbsentOrSame(resolve(targetDir, "w348-dnaonecalc-web-shell-host-contract.html"), dnaOneCalcRender);
writeIfAbsentOrSame(resolve(targetDir, "w348-shared-ui-shell-component.html"), sharedUiRender);

for (const token of [
  "data-host=\"DnaOneCalc\"",
  "data-sibling-repo-writes=\"false\"",
  "data-host-mount-claimed=\"false\"",
  "data-native-runtime=\"false\"",
  "data-com-runtime=\"false\"",
  "data-dom-audited=\"false\"",
  "DnaOneCalc",
  "OxIde",
  "OxVba",
  "ThinSliceHello",
  "Module1.bas"
]) {
  assert(dnaOneCalcRender.includes(token), `DnaOneCalc render missing ${token}`);
}

for (const token of [
  "data-source=\"oxide-ui-leptos\"",
  "data-component-crate=\"oxide-ui-leptos\"",
  "role=\"shared-ide-surface\"",
  "GuiShellPacket+RuntimeServicePacket+ImmediateServicePacket+DebugServicePacket",
  "ThinSliceHello",
  "Module1.bas",
  "data-native-runtime=\"false\"",
  "data-com-runtime=\"false\"",
  "data-fake-responses=\"false\"",
  "data-fake-debug-data=\"false\"",
  "data-dom-audited=\"false\""
]) {
  assert(sharedUiRender.includes(token), `Shared UI render missing ${token}`);
}

for (const forbidden of [
  "data-sibling-repo-writes=\"true\"",
  "data-host-mount-claimed=\"true\"",
  "data-native-runtime=\"true\"",
  "data-com-runtime=\"true\"",
  "data-fake-responses=\"true\"",
  "data-fake-debug-data=\"true\"",
  "data-dom-audited=\"true\""
]) {
  assert(!dnaOneCalcRender.includes(forbidden), `DnaOneCalc render contains forbidden ${forbidden}`);
  assert(!sharedUiRender.includes(forbidden), `Shared UI render contains forbidden ${forbidden}`);
}

assert(!sharedUiRender.includes("apps/dna-oxide"), "shared UI render should not depend on DnaOxIde app path");
assert(!sharedUiRender.includes("DnaOxIde static host proof"), "shared UI render should not be DnaOxIde host shell");
assert(profile.reusedCrates.includes("oxide-ui-leptos"), "profile missing oxide-ui-leptos");
assert(profile.reusedCrates.includes("oxide-host-bridge"), "profile missing oxide-host-bridge");
assert(profile.siblingRepoWrites === false, "profile claims sibling writes");
assert(profile.realDnaOneCalcMountClaimed === false, "profile claims real DnaOneCalc mount");

console.log("DnaOneCalc shared UI reuse verification passed");

function runGuilab(scenario) {
  return execFileSync("cargo", [
    "run",
    "--manifest-path",
    "crates/Cargo.toml",
    "-p",
    "oxide-guilab",
    "--",
    "render",
    scenario
  ], { cwd: repoRoot, encoding: "utf8" });
}

function writeIfAbsentOrSame(path, content) {
  if (existsSync(path)) {
    const existing = readFileSync(path, "utf8");
    if (existing !== content) {
      console.log(`Reuse render artifact already exists and differs after renderer evolution; verified current render in memory without overwriting: ${path}`);
    }
    return;
  }
  writeFileSync(path, content, "utf8");
}

function assert(condition, message) {
  if (!condition) {
    console.error(message);
    process.exit(1);
  }
}
