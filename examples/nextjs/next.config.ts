import { join } from "node:path";
import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  cacheComponents: true,
  partialPrefetching: true,
  // Reylo: next build's own type check cannot run content mappers; the build script runs the real
  // one (tsc --noEmit --runExternalCode) first.
  typescript: { ignoreBuildErrors: true },
  turbopack: {
    // Only because Reylo is linked from this repository's dist/, outside this directory, and
    // Turbopack resolves nothing outside its root. An app that installs reylo drops this line.
    root: join(import.meta.dirname, "../.."),
    // Reylo: each .sql file becomes a module that runs its statement, on the server only.
    rules: {
      "*.sql": {
        condition: { not: "browser" },
        loaders: ["reylo/turbopack"],
        as: "*.js",
      },
    },
  },
  // Reylo: the same loader for `next dev --webpack` and `next build --webpack`, on the server only.
  webpack(config, { isServer }) {
    if (isServer) {
      config.module.rules.push({ test: /\.sql$/, use: "reylo/webpack" });
    }
    return config;
  },
};

export default nextConfig;
