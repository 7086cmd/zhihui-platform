import { build } from "vite";
import { resolve } from "path";

async function buildPages() {
  const result = await build({
    publicDir: resolve("public/"),
    outDir: resolve("dist/client/pages"),
    root: resolve("packages/front/pages"),
    logLevel: "silent",
  });
  return result;
}

function main() {
  buildPages().catch((err) => {
    console.error(err);
    process.exit(1);
  });
}

main();
