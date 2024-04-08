import { fileURLToPath, URL } from "node:url";

import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import { quasar, transformAssetUrls } from "@quasar/vite-plugin";


// https://vitejs.dev/config/
export default defineConfig({
  optimizeDeps: {
    exclude: ["@mapbox/node-pre-gyp", "@mapbox/node-pre-gyp", "mock-aws-s3", "aws-sdk", "nock"],
  },
  plugins: [
    vue({
      template: { transformAssetUrls },
    }),
    quasar({
      sassVariables: "src/quasar-variables.sass",
    }),
  ],
  resolve: {
    alias: {
      "@": fileURLToPath(new URL("./src", import.meta.url)),
    },
  },
});
