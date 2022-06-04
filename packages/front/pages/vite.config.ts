import { fileURLToPath, URL } from "url";

import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import vueJsx from "@vitejs/plugin-vue-jsx";
import legacy from "@vitejs/plugin-legacy";

import prismjs from "vite-plugin-prismjs";
import monacoEditor from "vite-plugin-monaco-editor";
import { VitePWA as pwa } from "vite-plugin-pwa";

import unplugin_auto_import from "unplugin-auto-import/vite";
import unplugin_vue_components from "unplugin-vue-components/vite";
import unplugin_icons from "unplugin-icons/vite";

import {
  ElementPlusResolver,
  VantResolver,
} from "unplugin-vue-components/resolvers";
import IconResolver from "unplugin-icons/resolver";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    vue(),
    vueJsx(),
    legacy({
      polyfills: ["fetch", "Promise"],
      targets: ["defaults"],
      additionalLegacyPolyfills: ["regenerator-runtime/runtime"],
    }),

    prismjs({}),
    monacoEditor(),
    pwa(),

    unplugin_auto_import({
      resolvers: [ElementPlusResolver(), VantResolver(), IconResolver()],
    }),
    unplugin_vue_components({
      resolvers: [ElementPlusResolver(), VantResolver(), IconResolver()],
    }),
    unplugin_icons(),
  ],

  resolve: {
    alias: {
      "@": fileURLToPath(new URL("./src", import.meta.url)),
    },
  },
  css: {
    postcss: {
      plugins: [require("autoprefixer"), require("tailwindcss")],
    },
  },
});
