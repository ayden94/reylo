import { afterAll, expect, test } from "vitest";
import { db } from "../app/.server/db";
import getUser from "../app/.server/queries/get_user.sql";
import listPosts from "../app/.server/queries/list_posts.sql";
import createPost from "../app/.server/queries/create_post.sql";

afterAll(() => db.end());

test("get_user reads one user, or null", async () => {
  expect(await getUser.run(db, { id: 1 })).toMatchObject({ name: "ada", email: "ada@example.com", role: "admin" });
  expect(await getUser.run(db, { id: 999 })).toBeNull();
});

test("list_posts reads the posts with their authors", async () => {
  expect((await listPosts.run(db)).slice(0, 2)).toEqual([
    { id: 1, title: "Hello", body: "first post", user_id: 1, author: "ada" },
    { id: 2, title: "Again", body: null, user_id: 1, author: "ada" },
  ]);
});

test("create_post writes a post that list_posts reads", async () => {
  // In a transaction that is rolled back, so the database stays as it was.
  const client = await db.connect();
  try {
    await client.query("begin");
    const before = await listPosts.run(client);
    const { id } = await createPost.run(client, { user_id: 2, title: "From Vitest", body: null });
    expect(await listPosts.run(client)).toEqual([...before, { id, title: "From Vitest", body: null, user_id: 2, author: "bob" }]);
  } finally {
    await client.query("rollback");
    client.release();
  }
});
