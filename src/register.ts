// One preload for every runtime: `node --import porg/register`, `nub --import porg/register`,
// `bun --preload porg/register` (or bunfig.toml). It is the build step for code that runs
// unbundled: each `.sql` file becomes its module (`src/loader.ts`) as it loads, from `.porg/` or
// the database. Node's load hook (module.registerHooks) and Bun's plugin under `require()` must
// answer synchronously, while the analysis waits on the database, so a worker thread analyzes and
// this thread blocks until it answers, as SafeQL's ESLint rule does.
import { readFileSync } from "node:fs";
import { MessageChannel, receiveMessageOnPort, Worker, workerData } from "node:worker_threads";

declare const Bun: undefined | { plugin(p: { name: string; setup(build: { onLoad(o: { filter: RegExp }, cb: (args: { path: string }) => { loader: string; contents: string }): void }): void }): void };

let worker: Worker | undefined;

function sqlModuleSync(path: string): string {
  // The worker is this file's sibling, compiled (`.js`) or not (`.ts`).
  const extension = import.meta.url.slice(import.meta.url.lastIndexOf("."));
  worker ??= new Worker(new URL(`./register-worker${extension}`, import.meta.url), { workerData: { porgWorker: true } });
  worker.unref();
  const done = new Int32Array(new SharedArrayBuffer(4));
  const { port1, port2 } = new MessageChannel();
  worker.postMessage({ path, source: readFileSync(path, "utf8"), port: port2, done }, [port2]);
  // The analysis is bounded by the driver's own timeouts (10 s to connect); this one only ends the
  // wait on a worker that died.
  const waited = Atomics.wait(done, 0, 0, 120_000);
  const reply = receiveMessageOnPort(port1)?.message as { code: string } | { error: string } | undefined;
  port1.close();
  if (!reply) throw new Error(`${path}: Porg's worker ${waited === "timed-out" ? "did not answer in 120 s" : "ended without an answer"}.`);
  if ("error" in reply) throw new Error(reply.error);
  return reply.code;
}

// The worker inherits the preload; it loads no `.sql` file itself.
if (!(workerData as { porgWorker?: boolean } | null)?.porgWorker) {
  if (typeof Bun !== "undefined") {
    Bun.plugin({
      name: "porg-sql",
      setup(build) {
        build.onLoad({ filter: /\.sql$/ }, (args) => ({ loader: "js", contents: sqlModuleSync(args.path) }));
      },
    });
  } else {
    const { registerHooks } = await import("node:module");
    const { fileURLToPath } = await import("node:url");
    registerHooks({
      load(url, context, nextLoad) {
        if (!url.endsWith(".sql")) return nextLoad(url, context);
        return { format: "module", source: sqlModuleSync(fileURLToPath(url)), shortCircuit: true };
      },
    });
  }
}
