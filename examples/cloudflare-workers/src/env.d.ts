// The Worker's bindings, from wrangler.jsonc (the type of `env`), and its main module (the type of
// `exports` in the tests).
declare namespace Cloudflare {
  interface Env {
    DB: D1Database;
  }
  interface GlobalProps {
    mainModule: typeof import("./worker");
  }
}
