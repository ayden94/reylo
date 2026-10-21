import assert from "node:assert/strict";
import { after, test } from "node:test";
import postgres from "postgres";
import createPost from "../queries/create_post.sql";
import getUser from "../queries/get_user.sql";
import listPosts from "../queries/list_posts.sql";

const sql = postgres(process.env["DATABASE_URL"]!);
after(() => sql.end());

test("get_user finds ada, and null for an id nobody has", async () => {
  const ada = await getUser.run(sql, { id: 1 });
  assert.equal(ada?.name, "ada");
  assert.equal(ada?.role, "admin");
  // An instant is its ISO 8601 text in UTC, which Temporal.Instant.from reads.
  assert.match(ada?.created_at ?? "", /^\d{4}-\d\d-\d\dT\d\d:\d\d:\d\d(\.\d+)?Z$/);
  assert.equal(await getUser.run(sql, { id: 999 }), null);
});

test("create_post returns the new row, which list_posts then lists", async () => {
  // a transaction rolled back at the end, so the test leaves no row behind
  const rollback = new Error("rollback");
  await sql
    .begin(async (tx) => {
      const post = await createPost.run(tx, { user_id: 1, title: "From the test", body: "draft" });
      assert.equal(post.title, "From the test");

      const posts = await listPosts.run(tx, { role: "admin" });
      assert.deepEqual(posts.map((p) => p.title).slice(0, 2), ["Hello", "Again"]);
      assert.equal(posts[1]?.body, null);
      assert.ok(posts.every((p) => p.author === "ada"));
      assert.equal(posts.at(-1)?.id, post.id);
      throw rollback;
    })
    .catch((error) => {
      if (error !== rollback) throw error;
    });
});
