import { createServer, type ServerResponse } from "node:http";
import { json } from "node:stream/consumers";
import mysql from "mysql2/promise";
// Each .sql import is a typed query: the types come from the database (tsconfig.json's content
// mapper), and webpack turns the file into a module (webpack.config.mjs).
import getUser from "../queries/get_user.sql";
import listPosts from "../queries/list_posts.sql";
import createPost from "../queries/create_post.sql";
import lastInsertId from "../queries/last_insert_id.sql";

const db = mysql.createPool(process.env.DATABASE_URL!);

const server = createServer(async (req, res) => {
  const { pathname } = new URL(req.url!, "http://localhost");
  try {
    if (req.method === "GET" && pathname === "/posts") {
      return send(res, 200, await listPosts.run(db));
    }
    const user = /^\/users\/(\d+)$/.exec(pathname);
    if (req.method === "GET" && user) {
      const row = await getUser.run(db, { id: Number(user[1]) });
      return row ? send(res, 200, row) : send(res, 404, { error: "no such user" });
    }
    if (req.method === "POST" && pathname === "/posts") {
      const input = (await json(req)) as { user_id: number; title: string; body?: string | null };
      // MySQL has no `returning`: the insert resolves to void, and last_insert_id() reads the new
      // id on the same connection.
      const conn = await db.getConnection();
      try {
        await createPost.run(conn, { user_id: input.user_id, title: input.title, body: input.body ?? null });
        const { id } = await lastInsertId.run(conn);
        return send(res, 201, { id });
      } finally {
        conn.release();
      }
    }
    send(res, 404, { error: "not found" });
  } catch (error) {
    send(res, 500, { error: String(error) });
  }
});

function send(res: ServerResponse, status: number, body: unknown) {
  res.writeHead(status, { "content-type": "application/json" }).end(JSON.stringify(body));
}

const port = Number(process.env.PORT ?? 3000);
server.listen(port, () => console.log(`listening on http://localhost:${port}`));

// A signal stops new connections, and the pool closes after the last request is answered.
// `once`, because a signal can arrive more than once and each server.close() emits "close".
server.once("close", () => db.end());
for (const signal of ["SIGINT", "SIGTERM"]) {
  process.on(signal, () => server.close());
}
