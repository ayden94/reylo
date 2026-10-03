// Vitest runs the ESM test unchanged through dotsql/vite; `globals` provides Jest's test and expect.
import dotsql from "dotsql/vite";
export default { plugins: [dotsql()], test: { include: ["query.test.mjs"], globals: true } };
