// The CommonJS run: `nub run test:cjs`. Without --experimental-vm-modules, Jest cannot load an
// ES module, so Babel compiles the tests to CommonJS, and Reylo's own runtime too.
/** @type {import("jest").Config} */
export default {
  transform: {
    // Reylo: the same transformer writes a CommonJS module when Jest asks for one.
    "\\.sql$": "reylo/jest",
    // preset-env compiles ES modules only when Jest asks for CommonJS, so the run also passes
    // with --experimental-vm-modules, where Jest loads Reylo's runtime as the ES module it is.
    "\\.[jt]s$": [
      "babel-jest",
      { presets: [["@babel/preset-env", { targets: { node: "current" } }], "@babel/preset-typescript"] },
    ],
  },
  // Reylo: Jest compiles nothing under node_modules by default; this lets Babel compile reylo,
  // at node_modules/reylo/ (npm) or in a store such as node_modules/.pnpm/reylo@0.1.0/ (pnpm, nub, Bun).
  transformIgnorePatterns: ["/node_modules/(?!(\\.[^/]+/)?reylo[@/])"],
};
