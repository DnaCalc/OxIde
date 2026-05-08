import { createServer } from "node:http";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { extname, dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { chromium } from "playwright";
import { containsForbiddenClaimToken } from "../src/app-instrumentation.js";

const here = dirname(fileURLToPath(import.meta.url));
const appRoot = resolve(here, "..");
const repoRoot = resolve(appRoot, "..", "..");
const targetDir = resolve(repoRoot, "target");

const artifacts = {
  before: resolve(targetDir, "w350-b03-live-host-before.json"),
  after: resolve(targetDir, "w350-b03-live-host-after-input.json"),
  events: resolve(targetDir, "w350-b03-live-host-events.json"),
  commands: resolve(targetDir, "w350-b03-live-host-commands.json"),
  html: resolve(targetDir, "w350-b03-live-editable-host.html"),
  screenshot: resolve(targetDir, "w350-b03-live-editable-host.png"),
  evidence: resolve(targetDir, "w350-b03-live-editable-host.txt")
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

console.log("== W350-B03 live editable host mount verifier ==");
console.log(`repoRoot=${repoRoot}`);

mkdirSync(targetDir, { recursive: true });

const server = await startStaticServer(appRoot);
const indexUrl = `http://127.0.0.1:${server.port}/index.html`;
console.log(`indexUrl=${indexUrl}`);

const browser = await chromium.launch({ channel: "msedge", headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 1100 } });
try {
  await page.goto(indexUrl);
  await page.waitForFunction(() => Boolean(window.__DNA_OXIDE_TEST_DRIVER__));
  await page.locator('[data-testid="dnaoxide-w350-app"]').waitFor();
  await page.locator('[data-testid="source-editor"]').waitFor();

  const before = await page.evaluate(() => window.__DNA_OXIDE_TEST_DRIVER__.snapshot());
  assert(before.productName === "DNA OxIde", "wrong product name");
  assert(before.projectName === "ThinSliceHello", "wrong project name");
  assert(before.activeModule === "Module1.bas", "wrong active module");
  assert(before.dirty === false, "initial browser app must be clean");
  assert(before.editableSourceBoundary.hostNeutral === true, "source boundary missing from browser snapshot");

  const editedText = `${before.sourceText}\n' W350-B03 browser DOM input`;
  await page.locator('[data-testid="source-editor"]').fill(editedText);
  await page.waitForFunction(() => window.__DNA_OXIDE_TEST_DRIVER__.snapshot().dirty === true);

  const after = await page.evaluate(() => window.__DNA_OXIDE_TEST_DRIVER__.snapshot());
  const events = await page.evaluate(() => window.__DNA_OXIDE_TEST_DRIVER__.eventLog());
  const commands = await page.evaluate(() => window.__DNA_OXIDE_TEST_DRIVER__.commandLog());
  const dirtyIndicator = await page.locator('[data-testid="dirty-indicator"]').getAttribute("data-dirty");
  const editorValue = await page.locator('[data-testid="source-editor"]').inputValue();
  const html = await page.content();

  assert(after.dirty === true, "browser DOM input must make model dirty");
  assert(after.sourceText.includes("W350-B03 browser DOM input"), "browser model missing input marker");
  assert(editorValue.includes("W350-B03 browser DOM input"), "rendered textarea missing input marker");
  assert(dirtyIndicator === "true", "dirty indicator did not update");
  assert(events.some((event) => event.kind === "source-replaced" && event.detail.via === "dom-input"), "event log missing dom-input source replacement");
  assert(html.includes('data-testid="source-editor"'), "page html missing source editor stable id");
  assert(html.includes('data-dirty="true"'), "page html missing dirty state");

  for (const [label, value] of [
    ["before", JSON.stringify(before)],
    ["after", JSON.stringify(after)],
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
  writeFileSync(artifacts.after, `${JSON.stringify(after, null, 2)}\n`, "utf8");
  writeFileSync(artifacts.events, `${JSON.stringify(events, null, 2)}\n`, "utf8");
  writeFileSync(artifacts.commands, `${JSON.stringify(commands, null, 2)}\n`, "utf8");
  writeFileSync(artifacts.html, html, "utf8");
  await page.screenshot({ path: artifacts.screenshot, fullPage: true });
  writeFileSync(artifacts.evidence, [
    "W350-B03 DnaOxIde live editable host mount evidence",
    "",
    `repoRoot=${repoRoot}`,
    `indexUrl=${indexUrl}`,
    "driver=Playwright Chromium with channel=msedge",
    "browserDomInputDriven=true",
    "window.__DNA_OXIDE_TEST_DRIVER__=present",
    "sourceEditorTestId=data-testid=source-editor",
    "dirtyIndicatorTestId=data-testid=dirty-indicator",
    `beforeDirty=${before.dirty}`,
    `afterDirty=${after.dirty}`,
    `afterSourceTextHash=${after.sourceTextHash}`,
    `eventKinds=${events.map((event) => event.kind).join(",")}`,
    `commandNames=${commands.map((command) => command.commandName).join(",")}`,
    "fixtureMutationGuard=checked separately by wrapper",
    "runtimeExecutionClaimed=false",
    "nativeRuntimeClaimed=false",
    "comRuntimeClaimed=false",
    "fakeResponses=false",
    "fakeDebugData=false",
    "liveTauriWebViewIpcDriven=false",
    "",
    "Artifacts:",
    `- ${artifacts.before}`,
    `- ${artifacts.after}`,
    `- ${artifacts.events}`,
    `- ${artifacts.commands}`,
    `- ${artifacts.html}`,
    `- ${artifacts.screenshot}`,
    "",
    "Status: PASS"
  ].join("\n"), "utf8");

  console.log(`W350-B03 live host mount verification passed: ${artifacts.evidence}`);
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
