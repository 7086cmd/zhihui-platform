import { copyFile } from "fs/promises";
import { existsSync } from "node:fs";
import { resolve } from "path";

export async function copyBackendFiles() {
  const root = resolve();
  // 1. Copy the `server.exe`
  if (existsSync(resolve(root, "dist/server/server.exe"))) {
    await copyFile(
      resolve(root, "target/release/server.exe"),
      resolve(root, "dist/backend/server.exe")
    );
  } else {
    console.error("Can not find the server build.");
    process.exit(1);
  }

  // 2. Copy the `socket.exe`

  if (existsSync(resolve(root, "dist/socket.exe"))) {
    await copyFile(
      resolve(root, "dist/socket.exe"),
      resolve(root, "dist/backend/socket.exe")
    );
  } else {
    console.error("Can not find the socket build.");
    process.exit(1);
  }

  // 3. Copy the `readme` file

  if (existsSync(resolve(root, "README.md"))) {
    await copyFile(
      resolve(root, "README.md"),
      resolve(root, "dist/backend/README.md")
    );
  } else {
    console.error("Can not find the readme file.");
    process.exit(1);
  }
}
