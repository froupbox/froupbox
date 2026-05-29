import type { RsbuildConfig, EnvironmentConfig } from "@rsbuild/core";
import { env } from "node:process";

const isOfflineBuild = process.env.OFFLINE_BUILD;

export default {
  mode: env.NODE_ENV as any,
  environments: {
    editor: {
      source: { entry: { index: "./editor/main.ts" } },
      html: { template: "website/index.html", title: "froupbox" },
      output: { distPath: "dist/", assetPrefix: "." },
      tools: { rspack: { output: { filename: "beepbox_editor.min.js" } } },
    },
    player: {
      source: { entry: { index: "./player/main.ts" } },
      html: {
        template: "website/player/index.html",
        title: "froupbox Song Player",
      },
      output: { distPath: "dist/player/", assetPrefix: "." },
      tools: { rspack: { output: { filename: "beepbox_player.min.js" } } },
    },
    synth: {
      source: { entry: { index: "./synth/synth.ts" } },
      output: { distPath: "dist/", assetPrefix: "." },
      tools: {
        htmlPlugin: false,
        rspack: { output: { filename: "beepbox_synth.min.js" } },
      },
    },
  },
  source: {
    define: {
      OFFLINE: isOfflineBuild,
    },
  },
  splitChunks: false,
  dev: {
    hmr: false,
  },
  tools: {
    swc: { jsc: { loose: true } },
    rspack: { output: { library: "beepbox" } },
  },
  server: {
    port: 5559,
    publicDir: { name: "website" },
    htmlFallback: false,
  },
} satisfies RsbuildConfig;
