// The shapes of a tagged-template client (postgres.js, Bun.sql): the sql object itself, a
// sql.begin transaction, a savepoint inside it, and a reserved connection.
import { callbackTx, ignoreSentinel, manualTx, sentinel, type Shape, type TxOps } from "./harness.ts";

export function taggedShapes(sql: any, opts: { reserve: boolean; nestedSavepoint?: boolean }): Shape[] {
  const begin = (fn: (tx: any) => Promise<unknown>) => sql.begin(fn);
  const savepointOps: TxOps = {
    commit: (body) => begin((tx) => tx.savepoint((sp: any) => body(sp))),
    rollback: (body) =>
      begin((tx) =>
        tx
          .savepoint(async (sp: any) => {
            await body(sp);
            throw sentinel();
          })
          .catch(ignoreSentinel),
      ),
    failing: (body) => begin((tx) => tx.savepoint((sp: any) => body(sp))),
  };
  if (opts.nestedSavepoint !== false) {
    savepointOps.savepoint = (outer, inner) =>
      begin((tx) =>
        tx.savepoint(async (sp: any) => {
          await outer(sp);
          await sp
            .savepoint(async (sp2: any) => {
              await inner(sp2);
              throw sentinel();
            })
            .catch(ignoreSentinel);
        }),
      );
  }
  const shapes: Shape[] = [
    { name: "sql", kind: "async", scope: (fn) => fn(sql) },
    {
      name: "sql.begin(tx)",
      kind: "async",
      inTransaction: true,
      scope: (fn) => begin((tx) => fn(tx)),
      tx: callbackTx(begin, (tx, fn) => tx.savepoint(fn)),
    },
    {
      name: "tx.savepoint(sp)",
      kind: "async",
      inTransaction: true,
      scope: (fn) => begin((tx) => tx.savepoint((sp: any) => fn(sp))),
      tx: savepointOps,
    },
  ];
  if (opts.reserve) {
    const open = async () => {
      const r = await sql.reserve();
      return { client: r, release: () => r.release() };
    };
    shapes.push({
      name: "sql.reserve()",
      kind: "async",
      scope: async (fn) => {
        const { client, release } = await open();
        try {
          await fn(client);
        } finally {
          release();
        }
      },
      tx: manualTx({
        open,
        begin: (r) => r.unsafe("BEGIN"),
        commit: (r) => r.unsafe("COMMIT"),
        rollback: (r) => r.unsafe("ROLLBACK"),
        exec: (r, text) => r.unsafe(text),
      }),
    });
  }
  return shapes;
}
