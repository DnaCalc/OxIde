import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import {
  createDnaOxIdeHostShellModel,
  renderDnaOxIdeHostShell,
  verifyHostShellContract
} from "../src/host-shell.js";
import { createBrowserFixtureCommandClient } from "../src/command-client.js";

const here = dirname(fileURLToPath(import.meta.url));
const appRoot = resolve(here, "..");
const repoRoot = resolve(appRoot, "..", "..");
const fixturePath = resolve(repoRoot, "examples", "thin-slice", "Module1.bas");
const proofDir = resolve(repoRoot, "target", "w345-host-lifecycle-proof-files");
const sourceCopyPath = resolve(proofDir, "Module1.source-copy.bas");
const editedCopyPath = resolve(proofDir, "Module1.edited-copy.bas");
const renderPath = resolve(repoRoot, "target", "w345-host-lifecycle-proof.html");

const fixtureSource = readFileSync(fixturePath, "utf8");
const editedSource = fixtureSource.includes("answer = 40 + 2")
  ? fixtureSource.replace("answer = 40 + 2", "answer = 41 + 1")
  : `${fixtureSource}\n' W345 lifecycle proof edit\n`;

mkdirSync(proofDir, { recursive: true });
writeIfAbsentOrSame(sourceCopyPath, fixtureSource);
writeIfAbsentOrSame(editedCopyPath, editedSource);

const model = createDnaOxIdeHostShellModel(createBrowserFixtureCommandClient(), {
  sourceText: editedSource,
  sourceProvenance: "target/w345-host-lifecycle-proof-files/Module1.edited-copy.bas",
  lifecycle: {
    dirty: true,
    provider: "proven-oxide-only-temp-copy",
    proofPath: "target/w345-host-lifecycle-proof-files",
    events: [
      "opened-temp-project-copy",
      "saved-working-source-to-temp-copy",
      "reloaded-module-from-temp-copy",
      "session-snapshot-restored-from-temp-copy",
      "checked-in-fixture-unchanged"
    ]
  }
});
const markup = renderDnaOxIdeHostShell(model);

if (!verifyHostShellContract(markup)) {
  console.error("DNA OxIde lifecycle host shell contract failed base verification");
  process.exit(1);
}

const requiredTokens = [
  "role=\"host-lifecycle-panel\"",
  "data-provider=\"proven-oxide-only-temp-copy\"",
  "data-dirty=\"true\"",
  "data-proof-path=\"target/w345-host-lifecycle-proof-files\"",
  "opened-temp-project-copy",
  "saved-working-source-to-temp-copy",
  "reloaded-module-from-temp-copy",
  "session-snapshot-restored-from-temp-copy",
  "checked-in-fixture-unchanged",
  "answer = 41 + 1",
  "data-real-execution=\"false\"",
  "data-native-runtime=\"false\"",
  "data-com-runtime=\"false\""
];

for (const token of requiredTokens) {
  if (!markup.includes(token)) {
    console.error(`DNA OxIde lifecycle proof markup missing token: ${token}`);
    process.exit(1);
  }
}

if (readFileSync(fixturePath, "utf8") !== fixtureSource) {
  console.error("Checked-in examples/thin-slice/Module1.bas changed during lifecycle proof");
  process.exit(1);
}

writeIfAbsentOrSame(renderPath, markup);
console.log(`DNA OxIde host lifecycle proof passed: ${renderPath}`);

function writeIfAbsentOrSame(path, content) {
  if (existsSync(path)) {
    const existing = readFileSync(path, "utf8");
    if (existing !== content) {
      console.error(`Refusing to overwrite differing lifecycle proof file: ${path}`);
      process.exit(1);
    }
    return;
  }

  writeFileSync(path, content, "utf8");
}
