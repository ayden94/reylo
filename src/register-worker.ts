// The worker behind `src/bridge.ts` (`reylo/register`, `reylo/jest`): it builds a `.sql` file's
// module, which can take a database round trip, while the loading thread blocks on `done`. It
// imports the loader only when asked, so that a failure to load it is an answer and not a silent
// worker.
import { parentPort } from "node:worker_threads";
import type { MessagePort } from "node:worker_threads";

type Request = { path: string; source: string; port: MessagePort; done: Int32Array };

parentPort!.on("message", async ({ path, source, port, done }: Request) => {
  let reply: { code: string } | { error: string };
  try {
    const { sqlModule } = await import("./loader.ts");
    reply = { code: (await sqlModule(path, source)).code };
  } catch (e) {
    reply = { error: (e as Error).message };
  }
  port.postMessage(reply);
  port.close();
  Atomics.store(done, 0, 1);
  Atomics.notify(done, 0);
});
