// Vitest runs the ESM test unchanged through motherload/vite; `globals` provides Jest's test and expect.
import motherload from "motherload/vite";
export default { plugins: [motherload()], test: { include: ["query.test.mjs"], globals: true } };
