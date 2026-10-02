// Vitest runs the ESM test unchanged through porg/vite; `globals` provides Jest's test and expect.
import porg from "porg/vite";
export default { plugins: [porg()], test: { include: ["query.test.mjs"], globals: true } };
