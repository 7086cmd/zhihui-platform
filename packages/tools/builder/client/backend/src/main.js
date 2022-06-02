import { build } from "esbuild";
import { resolve } from "path/win32";
import chalk from "chalk";

/**
 * @function buildClient
 * @returns {Promise<{js: string, map: string}>}
 * @description Builds the client
 * @async
 */
async function buildClient() {
  const { js, map } = await build({
    entryPoints: [
      resolve("packages/front/client/src/main.ts").replaceAll("\\", "/"),
    ],
    outfile: "dist/client/magnifique.min.js",
    sourcemap: true,
    minify: true,
    bundle: true,
    target: "es2020",
    platform: "node",
    format: "cjs",
    external: ["electron"],
    define: {
      "process.env.NODE_ENV": '"production"',
    },
    loader: {
      ".md": "text",
    },
    logLevel: "silent",
  });
  return { js, map };
}

/**
 * @function buildPreload
 * @returns {Promise<{js: string, map: string}>}
 * @description Builds the preload
 * @async
 */
async function buildPreload() {
  const { js, map } = await build({
    entryPoints: [
      resolve("packages/front/client/src/preload.ts").replaceAll("\\", "/"),
    ],
    outfile: "dist/client/preload.min.js",
    sourcemap: true,
    minify: true,
    bundle: true,
    target: "es2020",
    platform: "node",
    format: "cjs",
    external: ["electron"],
    define: {
      "process.env.NODE_ENV": '"production"',
    },
    loader: {
      ".md": "text",
    },
    logLevel: "silent",
  });
  return { js, map };
}
/**
 * @function main
 * @description Builds the client and preload
 */
function main() {
  buildClient().then(() => {
    return buildPreload();
  });
}

main();
