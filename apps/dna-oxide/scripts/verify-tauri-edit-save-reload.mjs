import { chromium } from "playwright";
import { spawn, execFile } from "node:child_process";
import { mkdir, stat, writeFile, readFile } from "node:fs/promises";
import { existsSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { performance } from "node:perf_hooks";

const appRoot = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const repoRoot = resolve(appRoot, "..", "..");
const targetRoot = resolve(repoRoot, "target");
const exePath = resolve(appRoot, "src-tauri", "target", "release", "dna-oxide-tauri-scaffold.exe");
const cdpPort = Number(process.env.W352_B03_CDP_PORT ?? 9232);
const cdpUrl = `http://127.0.0.1:${cdpPort}`;
const checkedInFixtureModule = resolve(repoRoot, "examples", "thin-slice", "Module1.bas");
const nativeTempModule = resolve(repoRoot, "target", "w352-tauri-native-temp-project-copy", "Module1.bas");

function assert(condition, message) {
  if (!condition) {
    throw new Error(message);
  }
}

function psJson(script) {
  return new Promise((resolvePromise, reject) => {
    execFile("powershell", ["-NoProfile", "-Command", script], { windowsHide: true }, (error, stdout, stderr) => {
      if (error) {
        reject(new Error(`${error.message}\n${stderr}`));
        return;
      }
      try {
        resolvePromise(JSON.parse(stdout.trim() || "null"));
      } catch (parseError) {
        reject(new Error(`Could not parse PowerShell JSON: ${parseError.message}\n${stdout}`));
      }
    });
  });
}

async function processMemoryBytes(rootPid) {
  const script = `
$root = ${rootPid}
$all = Get-CimInstance Win32_Process
$ids = New-Object System.Collections.Generic.HashSet[int]
[void]$ids.Add($root)
$changed = $true
while ($changed) {
  $changed = $false
  foreach ($p in $all) {
    if ($ids.Contains([int]$p.ParentProcessId) -and -not $ids.Contains([int]$p.ProcessId)) {
      [void]$ids.Add([int]$p.ProcessId)
      $changed = $true
    }
  }
}
$sum = 0
foreach ($id in $ids) {
  $proc = Get-Process -Id $id -ErrorAction SilentlyContinue
  if ($proc) { $sum += $proc.WorkingSet64 }
}
[pscustomobject]@{ totalWorkingSet64 = $sum } | ConvertTo-Json -Depth 2
`;
  return psJson(script);
}

async function waitForCdp(timeoutMs = 15000) {
  const started = performance.now();
  let lastError = null;
  while (performance.now() - started < timeoutMs) {
    try {
      const response = await fetch(`${cdpUrl}/json/version`);
      if (response.ok) {
        return response.json();
      }
    } catch (error) {
      lastError = error;
    }
    await new Promise((resolveDelay) => setTimeout(resolveDelay, 150));
  }
  throw new Error(`Timed out waiting for WebView2 CDP at ${cdpUrl}: ${lastError?.message ?? "no response"}`);
}

async function firstDnaOxidePage(browser) {
  const deadline = performance.now() + 15000;
  while (performance.now() < deadline) {
    for (const context of browser.contexts()) {
      for (const page of context.pages()) {
        try {
          if (await page.locator('[data-testid="dnaoxide-w350-app"]').count()) {
            return page;
          }
        } catch {
          // retry while WebView navigates
        }
      }
    }
    await new Promise((resolveDelay) => setTimeout(resolveDelay, 150));
  }
  throw new Error("Timed out locating DnaOxIde WebView page over CDP");
}

async function main() {
  await mkdir(targetRoot, { recursive: true });
  assert(existsSync(exePath), `Release Tauri executable missing: ${exePath}. Run npm run tauri:build first.`);
  const checkedInBefore = await readFile(checkedInFixtureModule, "utf8");
  assert(!checkedInBefore.includes("W352 native save/reload proof"), "checked-in fixture was already mutated before B03");

  const startedAt = performance.now();
  const child = spawn(exePath, [], {
    cwd: repoRoot,
    env: {
      ...process.env,
      WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS: `--remote-debugging-port=${cdpPort} --remote-allow-origins=*`
    },
    detached: false,
    stdio: ["ignore", "pipe", "pipe"]
  });

  const stdout = [];
  const stderr = [];
  child.stdout.on("data", (chunk) => stdout.push(String(chunk)));
  child.stderr.on("data", (chunk) => stderr.push(String(chunk)));

  let browser = null;
  try {
    const cdpVersion = await waitForCdp();
    browser = await chromium.connectOverCDP(cdpUrl);
    const page = await firstDnaOxidePage(browser);
    await page.waitForSelector('[data-testid="source-editor"]', { timeout: 10000 });
    const firstObservableUiMs = Math.round(performance.now() - startedAt);

    const beforeSnapshot = await page.evaluate(() => globalThis.__DNA_OXIDE_TEST_DRIVER__.snapshot());
    const editedSource = `${beforeSnapshot.sourceText.trimEnd()}\n' W352 native save/reload proof ${Date.now()}\n`;

    await page.locator('[data-testid="source-editor"]').fill(editedSource);
    await page.waitForFunction(() => globalThis.__DNA_OXIDE_TEST_DRIVER__.snapshot().dirty === true, null, { timeout: 10000 });
    const afterEditSnapshot = await page.evaluate(() => globalThis.__DNA_OXIDE_TEST_DRIVER__.snapshot());

    const saveStarted = performance.now();
    await page.locator('[data-testid="save-active-module-command"]').click();
    await page.waitForFunction(() => {
      const snap = globalThis.__DNA_OXIDE_TEST_DRIVER__.snapshot();
      return snap.dirty === false && snap.lastCommand === "save-active-module";
    }, null, { timeout: 10000 });
    const nativeSaveRoundTripMs = Math.round(performance.now() - saveStarted);
    const afterSaveSnapshot = await page.evaluate(() => globalThis.__DNA_OXIDE_TEST_DRIVER__.snapshot());

    await page.locator('[data-testid="source-editor"]').fill(`${editedSource}\n' unsaved transient text\n`);
    await page.waitForFunction(() => globalThis.__DNA_OXIDE_TEST_DRIVER__.snapshot().dirty === true, null, { timeout: 10000 });

    const reloadStarted = performance.now();
    await page.locator('[data-testid="reload-active-module-command"]').click();
    await page.waitForFunction((expectedText) => {
      const snap = globalThis.__DNA_OXIDE_TEST_DRIVER__.snapshot();
      return snap.dirty === false
        && snap.lastCommand === "reload-active-module"
        && snap.sourceText === expectedText;
    }, editedSource, { timeout: 10000 });
    const nativeReloadRoundTripMs = Math.round(performance.now() - reloadStarted);
    const afterReloadSnapshot = await page.evaluate(() => globalThis.__DNA_OXIDE_TEST_DRIVER__.snapshot());

    const eventLog = await page.evaluate(() => globalThis.__DNA_OXIDE_TEST_DRIVER__.eventLog());
    const commandLog = await page.evaluate(() => globalThis.__DNA_OXIDE_TEST_DRIVER__.commandLog());
    await page.screenshot({ path: resolve(targetRoot, "w352-b03-tauri-edit-save-reload.png"), fullPage: true });
    await new Promise((resolveDelay) => setTimeout(resolveDelay, 1000));
    const memory = await processMemoryBytes(child.pid);

    const tempModuleSource = await readFile(nativeTempModule, "utf8");
    const checkedInAfter = await readFile(checkedInFixtureModule, "utf8");

    assert(afterEditSnapshot.dirty === true, "edit did not mark app dirty");
    assert(afterSaveSnapshot.hostCommandBoundary.provider === "tauri-linked-native-rust", "save did not use Tauri native provider");
    assert(afterSaveSnapshot.dirty === false, "save did not clear dirty state");
    assert(afterSaveSnapshot.persistedSourceText === editedSource, "save did not persist edited source into app state");
    assert(afterReloadSnapshot.sourceText === editedSource, "reload did not restore native persisted source");
    assert(!afterReloadSnapshot.sourceText.includes("unsaved transient text"), "reload retained unsaved transient edit");
    assert(tempModuleSource === editedSource, "native temp module file does not match saved source");
    assert(checkedInAfter === checkedInBefore, "checked-in fixture changed during native save/reload test");
    assert(eventLog.some((event) => event.kind === "source-saved-to-temp-copy" && event.detail.response?.providerLabel === "native-filesystem"), "native save event evidence missing");
    assert(eventLog.some((event) => event.kind === "source-reloaded-from-temp-copy" && event.detail.response?.providerLabel === "native-filesystem"), "native reload event evidence missing");

    const metrics = {
      firstObservableUiMs,
      nativeSaveRoundTripMs,
      nativeReloadRoundTripMs,
      idleProcessMemoryBytes: memory.totalWorkingSet64,
      cdpBrowser: cdpVersion.Browser ?? cdpVersion["Browser"] ?? null
    };

    await writeFile(resolve(targetRoot, "w352-b03-tauri-edit-save-reload-state.json"), JSON.stringify({
      beforeSnapshot,
      afterEditSnapshot,
      afterSaveSnapshot,
      afterReloadSnapshot,
      eventLog,
      commandLog,
      nativeTempModule,
      checkedInFixtureModule,
      metrics
    }, null, 2));

    await writeFile(resolve(targetRoot, "w352-b03-tauri-edit-save-reload.txt"), [
      "W352-B03 Tauri edit/save/reload through native Rust commands",
      `release_exe=${exePath}`,
      `visual_artifact=target/w352-b03-tauri-edit-save-reload.png`,
      `state_artifact=target/w352-b03-tauri-edit-save-reload-state.json`,
      `native_temp_module=${nativeTempModule}`,
      `checked_in_fixture_unchanged=${checkedInAfter === checkedInBefore}`,
      `save_provider=${afterSaveSnapshot.hostCommandBoundary.provider}`,
      `save_response_provider=${eventLog.find((event) => event.kind === "source-saved-to-temp-copy")?.detail.response?.providerLabel ?? "missing"}`,
      `reload_response_provider=${eventLog.find((event) => event.kind === "source-reloaded-from-temp-copy")?.detail.response?.providerLabel ?? "missing"}`,
      `native_save_round_trip_ms=${nativeSaveRoundTripMs}`,
      `native_reload_round_trip_ms=${nativeReloadRoundTripMs}`,
      `first_observable_ui_ms=${firstObservableUiMs}`,
      `idle_process_tree_working_set_bytes=${memory.totalWorkingSet64}`,
      "no_browser_only_host_service_substitution=true",
      "runtime_debug_immediate_com_claims_remain_false=true",
      ""
    ].join("\n"));

    console.log("W352-B03 Tauri edit/save/reload verification passed");
    console.log(JSON.stringify(metrics, null, 2));
  } finally {
    if (browser) {
      await browser.close().catch(() => {});
    }
    if (!child.killed) {
      child.kill();
    }
    await new Promise((resolveDelay) => setTimeout(resolveDelay, 500));
    if (!child.killed) {
      child.kill("SIGKILL");
    }
    if (stdout.length || stderr.length) {
      await writeFile(resolve(targetRoot, "w352-b03-tauri-process-output.txt"), JSON.stringify({
        stdout: stdout.join(""),
        stderr: stderr.join("")
      }, null, 2));
    }
  }
}

main().catch((error) => {
  console.error(error);
  process.exit(1);
});
