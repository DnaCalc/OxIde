import { cp, mkdir } from "node:fs/promises";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const appRoot = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const distRoot = resolve(appRoot, "dist");

await mkdir(resolve(distRoot, "src"), { recursive: true });
await cp(resolve(appRoot, "index.html"), resolve(distRoot, "index.html"));
await cp(resolve(appRoot, "src"), resolve(distRoot, "src"), { recursive: true });

console.log(`DNA OxIde Tauri dist prepared at ${distRoot}`);
