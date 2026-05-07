import { readFileSync } from "node:fs";
import { resolve } from "node:path";

const repoRoot = process.cwd();
const profilePath = resolve(repoRoot, "docs", "fixtures", "dnaonecalc-consumer-profile.json");
const profile = JSON.parse(readFileSync(profilePath, "utf8"));

const requiredInputs = [
  "GuiShellPacket",
  "RuntimeServicePacket",
  "ImmediateServicePacket",
  "DebugServicePacket",
  "HostBridgeCommandAvailability",
  "DnaOneCalcWebShellHostPacket"
];
const requiredCrates = [
  "oxide-ui-leptos",
  "oxide-host-bridge",
  "oxide-core",
  "oxide-webshell",
  "oxide-guilab"
];

function assert(condition, message) {
  if (!condition) {
    console.error(message);
    process.exit(1);
  }
}

assert(profile.profileId === "dnaonecalc-consumer", "profileId mismatch");
assert(profile.siblingRepoWrites === false, "profile claims sibling writes");
assert(profile.realDnaOneCalcMountClaimed === false, "profile claims real DnaOneCalc mount");
assert(profile.productShellOwner === "DnaOneCalc", "DnaOneCalc owner missing");
assert(profile.ideSurfaceOwner === "OxIde", "OxIde owner missing");
assert(profile.vbaTruthOwner === "OxVba", "OxVba owner missing");
for (const input of requiredInputs) {
  assert(profile.sharedInputs.includes(input), `missing shared input ${input}`);
}
for (const crateName of requiredCrates) {
  assert(profile.reusedCrates.includes(crateName), `missing reused crate ${crateName}`);
}
for (const [flag, value] of Object.entries(profile.claimFlags)) {
  assert(value === false, `claim flag ${flag} is not false`);
}
for (const token of profile.visibleTokens) {
  assert(token.includes("false") || token.includes("DnaOneCalc"), `visible token should preserve no-claim/product boundary: ${token}`);
}
console.log("DnaOneCalc consumer profile verification passed");
