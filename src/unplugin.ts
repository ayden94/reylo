// One bundler plugin for Vite, Rollup, Rolldown, webpack 5, Rspack, Rsbuild,
// esbuild, Farm and Bun.build(), via unplugin (an optional peer dependency).
import { readFileSync } from "node:fs";
import { createUnplugin } from "unplugin";
import { sqlModuleSource } from "./loader.ts";

export const lando = createUnplugin(() => ({
  name: "lando-sql",
  loadInclude: (id) => id.endsWith(".sql"),
  load: (id) => sqlModuleSource(readFileSync(id, "utf8"), id),
}));

export const vite = lando.vite;
export const rollup = lando.rollup;
export const rolldown = lando.rolldown;
export const webpack = lando.webpack;
export const rspack = lando.rspack;
export const esbuild = lando.esbuild;
export const farm = lando.farm;
export const bun = lando.bun;
