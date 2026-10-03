import { join } from "node:path";
import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  agentRules: false,
  // next build runs `tsc` without --runExternalCode, so the content mapper cannot run there and
  // every .sql import is TS2307; the type check is the separate `tsc --runExternalCode` step.
  typescript: { ignoreBuildErrors: true },
  turbopack: {
    // The lab's node_modules/motherload is a symlink to the repository root, outside this directory;
    // Turbopack resolves nothing outside its root, so the root is the repository here. An app that
    // installs motherload from the registry keeps the default root.
    root: join(import.meta.dirname, "../.."),
    rules: {
      "*.sql": { condition: { not: "browser" }, loaders: ["motherload/webpack-loader"], as: "*.js" },
    },
  },
  webpack(config, { isServer }) {
    if (isServer) config.module.rules.push({ test: /\.sql$/, use: "motherload/webpack-loader" });
    return config;
  },
};

export default nextConfig;
