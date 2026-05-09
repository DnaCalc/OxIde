import { chromium } from "playwright";
import { spawn, execFile } from "node:child_process";
import { mkdir, writeFile, readFile } from "node:fs/promises";
import { existsSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { performance } from "node:perf_hooks";

const appRoot = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const repoRoot = resolve(appRoot, "..", "..");
const targetRoot = resolve(repoRoot, "target");
const exePath = resolve(appRoot, "src-tauri", "target", "release", "dna-oxide-tauri-scaffold.exe");
const cdpPort = Number(process.env.W355_B03_CDP_PORT ?? 9233);
const cdpUrl = `http://127.0.0.1:${cdpPort}`;
const checkedInFixtureModule = resolve(repoRoot, "examples", "thin-slice", "Module1.bas");

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
    const editedSource = `${beforeSnapshot.sourceText.trimEnd()}\n' W355 native edit-save-compile proof ${Date.now()}\n`;

    await page.locator('[data-testid="source-editor"]').fill(editedSource);
    await page.waitForFunction(() => globalThis.__DNA_OXIDE_TEST_DRIVER__.snapshot().dirty === true, null, { timeout: 10000 });

    const saveStarted = performance.now();
    await page.locator('[data-testid="save-active-module-command"]').click();
    await page.waitForFunction(() => {
      const snap = globalThis.__DNA_OXIDE_TEST_DRIVER__.snapshot();
      return snap.dirty === false && snap.lastCommand === "save-active-module";
    }, null, { timeout: 10000 });
    const nativeSaveRoundTripMs = Math.round(performance.now() - saveStarted);

    const optionsStarted = performance.now();
    await page.locator('[data-testid="compile-options-command"]').click();
    await page.waitForFunction(() => {
      const result = globalThis.__DNA_OXIDE_TEST_DRIVER__.snapshot().hostCommandBoundary.lastCompileOptionsResult;
      return result?.profileId === "dnaoxide-desktop-tauri-native" && result?.providerLabel === "oxvba-current-api";
    }, null, { timeout: 10000 });
    const compileOptionsRoundTripMs = Math.round(performance.now() - optionsStarted);

    const buildStarted = performance.now();
    await page.locator('[data-testid="build-check-command"]').click();
    await page.waitForFunction(() => {
      const result = globalThis.__DNA_OXIDE_TEST_DRIVER__.snapshot().hostCommandBoundary.lastBuildCheckResult;
      return result?.profileId === "dnaoxide-desktop-tauri-native"
        && result?.providerLabel === "oxvba-current-api"
        && result?.status === "succeeded"
        && Boolean(result?.compiledSummary);
    }, null, { timeout: 10000 });
    const buildCheckRoundTripMs = Math.round(performance.now() - buildStarted);

    const afterBuildSnapshot = await page.evaluate(() => globalThis.__DNA_OXIDE_TEST_DRIVER__.snapshot());
    const eventLog = await page.evaluate(() => globalThis.__DNA_OXIDE_TEST_DRIVER__.eventLog());
    const commandLog = await page.evaluate(() => globalThis.__DNA_OXIDE_TEST_DRIVER__.commandLog());
    const markup = await page.locator('[data-testid="dnaoxide-w350-app"]').evaluate((node) => node.outerHTML);
    await page.screenshot({ path: resolve(targetRoot, "w355-b03-tauri-edit-save-compile.png"), fullPage: true });
    await new Promise((resolveDelay) => setTimeout(resolveDelay, 1000));
    const memory = await processMemoryBytes(child.pid);
    const checkedInAfter = await readFile(checkedInFixtureModule, "utf8");

    const options = afterBuildSnapshot.hostCommandBoundary.lastCompileOptionsResult;
    const build = afterBuildSnapshot.hostCommandBoundary.lastBuildCheckResult;

    assert(afterBuildSnapshot.dirty === false, "save did not leave source clean before compile");
    assert(options.outputType === "Exe", "compile options did not surface OxVba output type");
    assert(options.buildTarget === "Bundle", "compile options did not surface OxVba build target");
    assert(build.status === "succeeded", "build check did not succeed");
    assert(build.compiledSummary.procedure_count > 0, "compiled summary has no procedures");
    assert(build.unavailableOutputs.includes("runtime-run"), "runtime output was not explicitly unavailable");
    assert(eventLog.some((event) => event.kind === "source-saved-to-temp-copy"), "save event missing");
    assert(eventLog.some((event) => event.kind === "compile-options-loaded"), "compile options event missing");
    assert(eventLog.some((event) => event.kind === "build-check-completed"), "build check event missing");
    assert(commandLog.some((command) => command.commandName === "build-check"), "build command log missing");
    assert(markup.includes('data-testid="compile-build-panel"'), "compile/build panel missing");
    assert(markup.includes('data-build-status="succeeded"'), "compile/build panel did not show succeeded status");
    assert(!JSON.stringify(afterBuildSnapshot).includes('fake_responses":true'), "fake response claim appeared");
    assert(checkedInAfter === checkedInBefore, "checked-in fixture changed during native edit/save/compile test");

    const metrics = {
      firstObservableUiMs,
      nativeSaveRoundTripMs,
      compileOptionsRoundTripMs,
      buildCheckRoundTripMs,
      idleProcessMemoryBytes: memory.totalWorkingSet64,
      cdpBrowser: cdpVersion.Browser ?? cdpVersion["Browser"] ?? null
    };

    await writeFile(resolve(targetRoot, "w355-b03-tauri-edit-save-compile-state.json"), JSON.stringify({
      beforeSnapshot,
      afterBuildSnapshot,
      eventLog,
      commandLog,
      metrics
    }, null, 2));

    await writeFile(resolve(targetRoot, "w355-b03-tauri-edit-save-compile.txt"), [
      "W355-B03 Tauri edit/save/compile through native Rust commands",
      `release_exe=${exePath}`,
      `visual_artifact=target/w355-b03-tauri-edit-save-compile.png`,
      `state_artifact=target/w355-b03-tauri-edit-save-compile-state.json`,
      `compile_profile=${build.profileId}`,
      `compile_provider=${build.providerLabel}`,
      `compile_status=${build.status}`,
      `compiled_procedure_count=${build.compiledSummary.procedure_count}`,
      `compiled_instruction_count=${build.compiledSummary.instruction_count}`,
      `compile_options=${options.outputType}/${options.buildTarget}/${options.runtimeFlavor}`,
      `native_save_round_trip_ms=${nativeSaveRoundTripMs}`,
      `compile_options_round_trip_ms=${compileOptionsRoundTripMs}`,
      `build_check_round_trip_ms=${buildCheckRoundTripMs}`,
      `first_observable_ui_ms=${firstObservableUiMs}`,
      `idle_process_tree_working_set_bytes=${memory.totalWorkingSet64}`,
      `checked_in_fixture_unchanged=${checkedInAfter === checkedInBefore}`,
      "no_fake_build_output=true",
      "runtime_debug_immediate_com_claims_remain_false=true",
      ""
    ].join("\n"));

    console.log("W355-B03 Tauri edit/save/compile verification passed");
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
      await writeFile(resolve(targetRoot, "w355-b03-tauri-process-output.txt"), JSON.stringify({
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
