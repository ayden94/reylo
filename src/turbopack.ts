// The Turbopack loader, `turbopack: { rules: { "*.sql": { loaders: ["reylo/turbopack"], as: "*.js" } } }`
// in a Next.js config: Turbopack runs webpack-style loaders, so it is `reylo/webpack`'s function.
export { default } from "./webpack.ts";
