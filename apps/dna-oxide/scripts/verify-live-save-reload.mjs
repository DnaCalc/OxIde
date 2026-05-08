import { createServer } from "node:http";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { extname, dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { chromium } from "playwright";
import { containsForbiddenClaimToken } from "../src/app-instrumentation.js";
import { stableSourceHash } from "../src/editable-source-boundary.js";

const here = dirname(fileURLToPath(import.meta.url));
const appRoot = resolve(here, "..");
const repoRoot = resolve(appRoot, "..", "..");
const targetDir = resolve(repoRoot, "target");
const tempProjectRoot = resolve(targetDir, "w350-b04-temp-project", "ThinSliceHello");
const tempModulePath = resolve(tempProjectRoot, "Module1.bas");
const tempProjectPath = resolve(tempProjectRoot, "ThinSliceHello.basproj");
const fixtureModulePath = resolve(repoRoot, "examples", "thin-slice", "Module1.bas");
const fixtureProjectPath = resolve(repoRoot, "examples", "thin-slice", "ThinSliceHello.basproj");

const artifacts = {
  before: resolve(targetDir, "w350-b04-before-edit.json"),
  afterEdit: resolve(targetDir, "w350-b04-after-edit.json"),
  afterSave: resolve(targetDir, "w350-b04-after-save.json"),
  afterDiverge: resolve(targetDir, "w350-b04-after-divergent-edit.json"),
  afterReload: resolve(targetDir, "w350-b04-after-reload.json"),
  events: resolve(targetDir, "w350-b04-events.json"),
  commands: resolve(targetDir, "w350-b04-commands.json"),
  html: resolve(targetDir, "w350-b04-live-save-reload.html"),
  screenshot: resolve(targetDir, "w350-b04-live-save-reload.png"),
  savedModule: resolve(targetDir, "w350-b04-saved-Module1.bas"),
  evidence: resolve(targetDir, "w350-b04-live-save-reload.txt")
};

function assert(condition, message) {
  if (!condition) {
    throw new Error(message);
  }
}

function assertNoOverclaim(label, value) {
  assert(!containsForbiddenClaimToken(value), `${label} contains forbidden overclaim token`);
}

function assertNoHostCoupling(relativePath) {
  const text = readFileSync(resolve(appRoot, relativePath), "utf8");
  const forbidden = ["@tauri-apps", "__TAURI__", "window.__TAURI__"];
  for (const token of forbidden) {
    assert(!text.includes(token), `${relativePath} contains forbidden host coupling token: ${token}`);
  }
}

console.log("== W350-B04 live save/reload verifier ==");
console.log(`repoRoot=${repoRoot}`);
console.log(`tempProjectRoot=${tempProjectRoot}`);

mkdirSync(targetDir, { recursive: true });
mkdirSync(tempProjectRoot, { recursive: true });
const initialSource = readFileSync(fixtureModulePath, "utf8");
writeFileSync(tempModulePath, initialSource, "utf8");
writeFileSync(tempProjectPath, readFileSync(fixtureProjectPath, "utf8"), "utf8");

const hostCommandResponses = [];
const server = await startStaticServer(appRoot);
const indexUrl = `http://127.0.0.1:${server.port}/index.html`;
console.log(`indexUrl=${indexUrl}`);

const browser = await chromium.launch({ channel: "msedge", headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 1100 } });
try {
  await page.exposeFunction("__dnaOxideSaveActiveModule", async (packet) => {
    assert(packet.activeModule === "Module1.bas", "save packet activeModule mismatch");
    assert(packet.tempProjectRoot === tempProjectRoot, "save packet tempProjectRoot mismatch");
    writeFileSync(tempModulePath, packet.sourceText, "utf8");
    const response = {
      ok: true,
      commandName: "save-active-module",
      activeModule: packet.activeModule,
      filePath: tempModulePath,
      sourceTextHash: stableSourceHash(packet.sourceText),
      sourceTextLength: packet.sourceText.length,
      provider: "playwright-node-temp-project-copy"
    };
    hostCommandResponses.push(response);
    return response;
  });

  await page.exposeFunction("__dnaOxideReloadActiveModule", async (packet) => {
    assert(packet.activeModule === "Module1.bas", "reload packet activeModule mismatch");
    assert(packet.tempProjectRoot === tempProjectRoot, "reload packet tempProjectRoot mismatch");
    const sourceText = readFileSync(tempModulePath, "utf8");
    const response = {
      ok: true,
      commandName: "reload-active-module",
      activeModule: packet.activeModule,
      filePath: tempModulePath,
      sourceText,
      sourceTextHash: stableSourceHash(sourceText),
      sourceTextLength: sourceText.length,
      provider: "playwright-node-temp-project-copy"
    };
    hostCommandResponses.push({ ...response, sourceText: "<omitted-from-response-log>" });
    return response;
  });

  await page.addInitScript(({ bootstrap }) => {
    window.__DNA_OXIDE_BOOTSTRAP__ = bootstrap;
    window.__DNA_OXIDE_HOST_SERVICES__ = {
      saveActiveModule: (packet) => window.__dnaOxideSaveActiveModule(packet),
      reloadActiveModule: (packet) => window.__dnaOxideReloadActiveModule(packet)
    };
  }, {
    bootstrap: {
      projectName: "ThinSliceHello",
      projectFile: "ThinSliceHello.basproj",
      activeModule: "Module1.bas",
      sourceText: initialSource,
      persistedSourceText: initialSource,
      sourceProvenance: "target/w350-b04-temp-project-copy",
      tempProjectRoot
    }
  });

  await page.goto(indexUrl);
  await page.waitForFunction(() => Boolean(window.__DNA_OXIDE_TEST_DRIVER__));
  await page.locator('[data-testid="source-editor"]').waitFor();

  const before = await page.evaluate(() => window.__DNA_OXIDE_TEST_DRIVER__.snapshot());
  assert(before.dirty === false, "initial temp project source must be clean");
  assert(before.sourceText === initialSource, "browser app did not bootstrap from temp source");
  assert(before.hostCommandBoundary.saveActiveModuleAvailable === true, "save host command boundary unavailable");
  assert(before.hostCommandBoundary.reloadActiveModuleAvailable === true, "reload host command boundary unavailable");

  const savedText = `${initialSource}\n' W350-B04 saved through temp project copy`;
  await page.locator('[data-testid="source-editor"]').fill(savedText);
  await page.waitForFunction(() => window.__DNA_OXIDE_TEST_DRIVER__.snapshot().dirty === true);
  const afterEdit = await page.evaluate(() => window.__DNA_OXIDE_TEST_DRIVER__.snapshot());
  assert(afterEdit.sourceText.includes("W350-B04 saved"), "after-edit snapshot missing save marker");

  await page.locator('[data-testid="save-active-module-command"]').click();
  await page.waitForFunction(() => window.__DNA_OXIDE_TEST_DRIVER__.snapshot().dirty === false);
  const afterSave = await page.evaluate(() => window.__DNA_OXIDE_TEST_DRIVER__.snapshot());
  const savedFileText = readFileSync(tempModulePath, "utf8");
  assert(afterSave.dirty === false, "after-save snapshot must be clean");
  assert(savedFileText === savedText, "temp Module1.bas was not saved with edited text");

  const divergentText = `${savedText}\n' unsaved divergent text before reload`;
  await page.locator('[data-testid="source-editor"]').fill(divergentText);
  await page.waitForFunction(() => window.__DNA_OXIDE_TEST_DRIVER__.snapshot().dirty === true);
  const afterDiverge = await page.evaluate(() => window.__DNA_OXIDE_TEST_DRIVER__.snapshot());
  assert(afterDiverge.sourceText.includes("unsaved divergent"), "after-diverge snapshot missing divergent marker");

  await page.locator('[data-testid="reload-active-module-command"]').click();
  await page.waitForFunction(() => {
    const snapshot = window.__DNA_OXIDE_TEST_DRIVER__.snapshot();
    return snapshot.dirty === false && snapshot.sourceText.includes("W350-B04 saved") && !snapshot.sourceText.includes("unsaved divergent");
  });
  const afterReload = await page.evaluate(() => window.__DNA_OXIDE_TEST_DRIVER__.snapshot());
  const events = await page.evaluate(() => window.__DNA_OXIDE_TEST_DRIVER__.eventLog());
  const commands = await page.evaluate(() => window.__DNA_OXIDE_TEST_DRIVER__.commandLog());
  const html = await page.content();

  assert(afterReload.dirty === false, "after-reload snapshot must be clean");
  assert(afterReload.sourceText === savedText, "reload did not restore saved temp file text");
  assert(commands.some((command) => command.commandName === "save-active-module"), "command log missing save-active-module");
  assert(commands.some((command) => command.commandName === "reload-active-module"), "command log missing reload-active-module");
  assert(events.some((event) => event.kind === "source-saved-to-temp-copy" && event.detail.commandBoundary === "injected-browser-host-service"), "event log missing host-service save");
  assert(events.some((event) => event.kind === "source-reloaded-from-temp-copy" && event.detail.commandBoundary === "injected-browser-host-service"), "event log missing host-service reload");

  for (const [label, value] of [
    ["before", JSON.stringify(before)],
    ["afterEdit", JSON.stringify(afterEdit)],
    ["afterSave", JSON.stringify(afterSave)],
    ["afterDiverge", JSON.stringify(afterDiverge)],
    ["afterReload", JSON.stringify(afterReload)],
    ["events", JSON.stringify(events)],
    ["commands", JSON.stringify(commands)],
    ["html", html]
  ]) {
    assertNoOverclaim(label, value);
  }

  assertNoHostCoupling("src/main.js");
  assertNoHostCoupling("src/app-instrumentation.js");
  assertNoHostCoupling("src/editable-source-boundary.js");

  writeFileSync(artifacts.before, `${JSON.stringify(before, null, 2)}\n`, "utf8");
  writeFileSync(artifacts.afterEdit, `${JSON.stringify(afterEdit, null, 2)}\n`, "utf8");
  writeFileSync(artifacts.afterSave, `${JSON.stringify(afterSave, null, 2)}\n`, "utf8");
  writeFileSync(artifacts.afterDiverge, `${JSON.stringify(afterDiverge, null, 2)}\n`, "utf8");
  writeFileSync(artifacts.afterReload, `${JSON.stringify(afterReload, null, 2)}\n`, "utf8");
  writeFileSync(artifacts.events, `${JSON.stringify(events, null, 2)}\n`, "utf8");
  writeFileSync(artifacts.commands, `${JSON.stringify(commands, null, 2)}\n`, "utf8");
  writeFileSync(artifacts.html, html, "utf8");
  writeFileSync(artifacts.savedModule, savedFileText, "utf8");
  await page.screenshot({ path: artifacts.screenshot, fullPage: true });
  writeFileSync(artifacts.evidence, [
    "W350-B04 save/reload through DnaOxIde command boundary evidence",
    "",
    `repoRoot=${repoRoot}`,
    `indexUrl=${indexUrl}`,
    `tempProjectRoot=${tempProjectRoot}`,
    `tempModulePath=${tempModulePath}`,
    "driver=Playwright Chromium with channel=msedge",
    "hostCommandBoundary=injected-browser-host-service",
    "saveWritesTempProjectCopy=true",
    "reloadReadsTempProjectCopy=true",
    "checkedInFixtureMutated=false",
    `beforeDirty=${before.dirty}`,
    `afterEditDirty=${afterEdit.dirty}`,
    `afterSaveDirty=${afterSave.dirty}`,
    `afterDivergeDirty=${afterDiverge.dirty}`,
    `afterReloadDirty=${afterReload.dirty}`,
    `savedFileHash=${stableSourceHash(savedFileText)}`,
    `afterReloadHash=${afterReload.sourceTextHash}`,
    `hostCommandResponses=${JSON.stringify(hostCommandResponses)}`,
    `eventKinds=${events.map((event) => event.kind).join(",")}`,
    `commandNames=${commands.map((command) => command.commandName).join(",")}`,
    "runtimeExecutionClaimed=false",
    "nativeRuntimeClaimed=false",
    "comRuntimeClaimed=false",
    "fakeResponses=false",
    "fakeDebugData=false",
    "",
    "Artifacts:",
    `- ${artifacts.before}`,
    `- ${artifacts.afterEdit}`,
    `- ${artifacts.afterSave}`,
    `- ${artifacts.afterDiverge}`,
    `- ${artifacts.afterReload}`,
    `- ${artifacts.events}`,
    `- ${artifacts.commands}`,
    `- ${artifacts.html}`,
    `- ${artifacts.screenshot}`,
    `- ${artifacts.savedModule}`,
    "",
    "Status: PASS"
  ].join("\n"), "utf8");

  console.log(`W350-B04 live save/reload verification passed: ${artifacts.evidence}`);
} finally {
  await browser.close();
  await server.close();
}

function startStaticServer(root) {
  const server = createServer((request, response) => {
    const requestUrl = new URL(request.url ?? "/", "http://127.0.0.1");
    const pathname = requestUrl.pathname === "/" ? "/index.html" : requestUrl.pathname;
    const normalized = resolve(root, `.${decodeURIComponent(pathname)}`);
    if (!normalized.startsWith(root)) {
      response.writeHead(403).end("forbidden");
      return;
    }

    try {
      const body = readFileSync(normalized);
      response.writeHead(200, { "content-type": contentType(normalized) });
      response.end(body);
    } catch (error) {
      response.writeHead(404).end(String(error));
    }
  });

  return new Promise((resolvePromise, reject) => {
    server.on("error", reject);
    server.listen(0, "127.0.0.1", () => {
      const address = server.address();
      resolvePromise({
        port: address.port,
        close: () => new Promise((closeResolve, closeReject) => {
          server.close((error) => error ? closeReject(error) : closeResolve());
        })
      });
    });
  });
}

function contentType(path) {
  switch (extname(path)) {
    case ".html":
      return "text/html; charset=utf-8";
    case ".js":
      return "text/javascript; charset=utf-8";
    case ".css":
      return "text/css; charset=utf-8";
    case ".json":
      return "application/json; charset=utf-8";
    default:
      return "application/octet-stream";
  }
}
