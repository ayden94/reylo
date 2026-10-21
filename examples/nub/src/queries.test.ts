import assert from "node:assert/strict";
import { DatabaseSync } from "node:sqlite";
import { after, test } from "node:test";
import createPost from "../queries/create_post.sql";
import getUser from "../queries/get_user.sql";
import listPosts from "../queries/list_posts.sql";

const db = new DatabaseSync("app.db");
after(() => db.close());

test("get_user finds ada, and null for an id nobody has", async () => {
  const ada = await getUser.run(db, { id: 1 });
  assert.equal(ada?.name, "ada");
  assert.equal(ada?.role, "admin");
  assert.equal(await getUser.run(db, { id: 999 }), null);
});

test("create_post returns the new row, which list_posts then lists", async () => {
  // a transaction rolled back at the end, so the test leaves no row behind
  db.exec("begin");
  try {
    const post = createPost.runSync(db, { user_id: 1, title: "From the test", body: "draft" });
    assert.equal(post.title, "From the test");

    const posts = await listPosts.run(db, { role: "admin" });
    assert.deepEqual(posts.map((p) => p.title).slice(0, 2), ["Hello", "Again"]);
    assert.equal(posts[1]?.body, null);
    assert.ok(posts.every((p) => p.author === "ada"));
    assert.equal(posts.at(-1)?.id, post.id);
  } finally {
    db.exec("rollback");
  }
});
