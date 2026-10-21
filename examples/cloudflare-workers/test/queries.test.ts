import { env, exports } from "cloudflare:workers";
import { expect, test } from "vitest";
import createPost from "../queries/create_post.sql";
import getUser from "../queries/get_user.sql";
import listPosts from "../queries/list_posts.sql";

test("get_user resolves to a row, or null", async () => {
  const ada = await getUser.run(env.DB, { id: 1 });
  expect(ada).toMatchObject({ id: 1, name: "ada", email: "ada@example.com", role: "admin" });
  expect(await getUser.run(env.DB, { id: 99 })).toBeNull();
});

test("create_post resolves to the row it wrote, and list_posts lists it", async () => {
  const post = await createPost.run(env.DB, { user_id: 1, title: "From a test", body: null });
  expect(post).toEqual({ id: 3, title: "From a test" });
  const posts = await listPosts.run(env.DB, { role: "admin" });
  expect(posts.map((p) => p.title)).toEqual(["Hello", "Again", "From a test"]);
});

test("the Worker answers through its fetch handler", async () => {
  const res = await exports.default.fetch("http://example.com/users/2");
  expect(res.status).toBe(200);
  expect(await res.json()).toMatchObject({ id: 2, name: "bob", email: null });
  expect((await exports.default.fetch("http://example.com/users/99")).status).toBe(404);
});
