import { chromium } from "playwright";
import { spawn, execFile } from "node:child_process";
import { mkdir, readdir, stat, writeFile } from "node:fs/promises";
import { existsSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { performance } from "node:perf_hooks";

const appRoot = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const repoRoot = resolve(appRoot, "..", "..");
const targetRoot = resolve(repoRoot, "target");
const exePath = resolve(appRoot, "src-tauri", "target", "release", "dna-oxide-tauri-scaffold.exe");
const distRoot = resolve(appRoot, "dist");
const cdpPort = Number(process.env.W352_CDP_PORT ?? 9231);
const cdpUrl = `http://127.0.0.1:${cdpPort}`;

function assert(condition, message) {
  if (!condition) {
    throw new Error(message);
  }
}

async function directorySize(path) {
  const info = await stat(path);
  if (!info.isDirectory()) {
    return info.size;
  }
  let total = 0;
  for (const entry of await readdir(path)) {
    total += await directorySize(resolve(path, entry));
  }
  return total;
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
$items = @()
foreach ($id in $ids) {
  $proc = Get-Process -Id $id -ErrorAction SilentlyContinue
  if ($proc) {
    $sum += $proc.WorkingSet64
    $items += [pscustomobject]@{ id = $proc.Id; name = $proc.ProcessName; workingSet64 = $proc.WorkingSet64 }
  }
}
[pscustomobject]@{ totalWorkingSet64 = $sum; processes = $items } | ConvertTo-Json -Depth 4
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
          const marker = await page.locator('[data-testid="dnaoxide-w350-app"]').count();
          if (marker > 0) {
            return page;
          }
        } catch {
          // page may be navigating; retry
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
  assert(existsSync(distRoot), `Tauri dist missing: ${distRoot}. Run npm run tauri:dist first.`);

  const exeSizeBytes = (await stat(exePath)).size;
  const distSizeBytes = await directorySize(distRoot);
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
    await page.waitForSelector('[data-testid="dnaoxide-w350-app"]', { timeout: 10000 });
    const firstObservableUiMs = Math.round(performance.now() - startedAt);

    const beforeSnapshot = await page.evaluate(() => globalThis.__DNA_OXIDE_TEST_DRIVER__.snapshot());
    const probeStarted = performance.now();
    await page.locator('[data-testid="desktop-host-probe-command"]').click();
    await page.waitForFunction(() => {
      const el = document.querySelector('[data-testid="native-host-probe-result"]');
      return el?.getAttribute("data-linked-native-rust") === "true";
    }, null, { timeout: 10000 });
    const nativeProbeRoundTripMs = Math.round(performance.now() - probeStarted);

    const afterSnapshot = await page.evaluate(() => globalThis.__DNA_OXIDE_TEST_DRIVER__.snapshot());
    const eventLog = await page.evaluate(() => globalThis.__DNA_OXIDE_TEST_DRIVER__.eventLog());
    const commandLog = await page.evaluate(() => globalThis.__DNA_OXIDE_TEST_DRIVER__.commandLog());
    const domState = await page.evaluate(() => ({
      title: document.title,
      appPresent: document.querySelector('[data-testid="dnaoxide-w350-app"]') !== null,
      nativeProbeText: document.querySelector('[data-testid="native-host-probe-result"]')?.textContent ?? null,
      linkedNativeRust: document.querySelector('[data-testid="native-host-probe-result"]')?.getAttribute("data-linked-native-rust") ?? null,
      hostProvider: document.querySelector('[data-testid="host-command-provider"]')?.textContent ?? null,
      lastCommand: document.querySelector('[data-testid="last-command"]')?.textContent ?? null
    }));

    await page.screenshot({ path: resolve(targetRoot, "w352-b02-webview-after-probe.png"), fullPage: true });
    await new Promise((resolveDelay) => setTimeout(resolveDelay, 2000));
    const memory = await processMemoryBytes(child.pid);

    assert(beforeSnapshot.productName === "DNA OxIde", "before snapshot did not come from DnaOxIde test driver");
    assert(afterSnapshot.hostCommandBoundary.provider === "tauri-linked-native-rust", "host provider was not Tauri native Rust");
    assert(afterSnapshot.hostCommandBoundary.lastNativeCommandResult?.linked_native_rust === true, "native probe result did not report linked_native_rust=true");
    assert(eventLog.some((event) => event.kind === "desktop-host-capabilities-probed"), "probe event missing");
    assert(domState.linkedNativeRust === "true", "DOM-like inspected state did not show linked native Rust result");

    const metrics = {
      exePath,
      exeSizeBytes,
      distRoot,
      distSizeBytes,
      firstObservableUiMs,
      nativeProbeRoundTripMs,
      idleProcessMemoryBytes: memory.totalWorkingSet64,
      processMemory: memory,
      cdpPort,
      cdpBrowser: cdpVersion.Browser ?? cdpVersion["Browser"] ?? null
    };

    await writeFile(resolve(targetRoot, "w352-b02-webview-state.json"), JSON.stringify({
      domState,
      beforeSnapshot,
      afterSnapshot,
      eventLog,
      commandLog,
      metrics
    }, null, 2));

    await writeFile(resolve(targetRoot, "w352-b02-performance-size-baseline.txt"), [
      "W352-B02 performance and size baseline",
      `release_exe=${exePath}`,
      `release_exe_size_bytes=${exeSizeBytes}`,
      `dist_path=${distRoot}`,
      `dist_size_bytes=${distSizeBytes}`,
      `cold_start_to_first_observable_ui_ms=${firstObservableUiMs}`,
      `native_host_probe_round_trip_ms=${nativeProbeRoundTripMs}`,
      `idle_process_tree_working_set_bytes=${memory.totalWorkingSet64}`,
      `cdp_port=${cdpPort}`,
      `cdp_browser=${metrics.cdpBrowser}`,
      "baseline_note=svelte strong zippy budget surface starts here; file follow-up beads for sluggishness or bloat regressions",
      ""
    ].join("\n"));

    await writeFile(resolve(targetRoot, "w352-b02-webview-automation.txt"), [
      "W352-B02 WebView automation over real desktop host",
      `release_exe=${exePath}`,
      `cdp_url=${cdpUrl}`,
      "visual_artifact=target/w352-b02-webview-after-probe.png",
      "dom_like_state_artifact=target/w352-b02-webview-state.json",
      "performance_size_artifact=target/w352-b02-performance-size-baseline.txt",
      "interaction=clicked [data-testid=desktop-host-probe-command] inside the real Tauri WebView over WebView2 CDP",
      `native_probe_visible=${domState.linkedNativeRust === "true"}`,
      `host_provider=${domState.hostProvider}`,
      `last_command=${domState.lastCommand}`,
      `event_log_contains_desktop_host_capabilities_probed=${eventLog.some((event) => event.kind === "desktop-host-capabilities-probed")}`,
      "no_browser_only_host_service_substitution=true",
      "runtime_debug_immediate_com_claims_remain_false=true",
      ""
    ].join("\n"));

    console.log("W352-B02 WebView automation verification passed");
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
      await writeFile(resolve(targetRoot, "w352-b02-tauri-process-output.txt"), JSON.stringify({
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
