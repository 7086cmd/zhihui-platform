import { build, Platform } from "electron-builder";

async function buildApp() {
  const result = await build({
    targets: Platform.WINDOWS.createTarget(),
    config: {
      appId: "com.magnifique.client",
      productName: "Magnifique Client",
      artifactName: "${productName}-${version}.${ext}",
      files: ["./dist/client/**"],
      extraMetadata: {
        main: "dist/client/magnifique.min.js",
      },
      asar: true,
      extends: null,
      win: {
        icon: "./assets/icons/icon.ico",
        target: [
          {
            target: "nsis",
            arch: ["x64", "ia32"],
          },
          {
            target: "portable",
            arch: ["x64", "ia32"],
          },
          {
            target: "zip",
            arch: ["x64", "ia32"],
          },
        ],
        publish: [
          {
            provider: "github",
            owner: "7086cmd",
            repo: "integrant",
          },
        ],
      },
      nsis: {
        oneClick: false,
        allowToChangeInstallationDirectory: true,
        allowElevation: true,
        perMachine: true,
        shortcutName: "Magnifique Client",
        menuCategory: "Magnifique",
      },
    },
    publish: "always",
  });
  return result;
}

function main() {
  buildApp().catch((e) => {
    console.error(e);
    process.exit(1);
  });
}

main();
