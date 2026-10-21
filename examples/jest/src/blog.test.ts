import { afterAll, afterEach, beforeEach, expect, test } from "@jest/globals";
import getUser from "../queries/get_user.sql";
import { adminPosts, publish } from "./blog.ts";
import { openDatabase } from "./db.ts";

const db = openDatabase();

// Each test runs inside a transaction that is rolled back, so the database stays as `setup` left it.
beforeEach(() => {
  db.exec("begin");
});
afterEach(() => {
  db.exec("rollback");
});
afterAll(() => {
  db.close();
});

test("get_user.sql finds a user by id, or null", async () => {
  expect(await getUser.run(db, { id: 1 })).toMatchObject({ id: 1, name: "ada", email: "ada@example.com", role: "admin" });
  expect(await getUser.run(db, { id: 99 })).toBeNull();
});

test("an admin publishes a post", async () => {
  expect(await publish(db, 1, "From Jest", null)).toEqual({ id: 3, title: "From Jest" });
  const posts = await adminPosts(db);
  expect(posts.map((post) => `${post.title} by ${post.author}`)).toEqual(["Hello by ada", "Again by ada", "From Jest by ada"]);
});

test("a member cannot publish", async () => {
  await expect(publish(db, 2, "Nope", null)).rejects.toThrow("bob is not an admin");
});
