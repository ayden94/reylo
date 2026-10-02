// CommonJS run with no --experimental-vm-modules: Jest cannot require an ES module then, and
// Porg's runtime is one, so babel-jest compiles Porg's own files to CommonJS (an installed copy
// also needs transformIgnorePatterns: ["/node_modules/(?!porg/)"]).
module.exports = {
  watchman: false,
  testMatch: ["<rootDir>/query.test.cjs"],
  transform: {
    "\\.sql$": "porg/jest",
    "\\.[jt]s$": ["babel-jest", { presets: ["@babel/preset-typescript"], plugins: ["@babel/plugin-transform-modules-commonjs"] }],
  },
};
