import { resolve } from "node:path";
import { mkdir } from "node:fs/promises";
import { existsSync } from "node:fs";

export async function mkdirBackendFiles() {
  const root = resolve();
  const folder = resolve(root, "dist/backend");
  const parent = resolve(root, "dist");

  if (!existsSync(parent)) {
    await mkdir(parent);
  }
  // 1. Create the `backend` folder
  if (!existsSync(folder)) {
    await mkdir(folder);
  }
}
