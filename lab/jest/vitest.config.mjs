// Vitest runs the ESM test unchanged through reylo/vite; `globals` provides Jest's test and expect.
import reylo from "reylo/vite";
export default { plugins: [reylo()], test: { include: ["query.test.mjs"], globals: true } };
