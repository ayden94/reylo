import { Form, Link } from "react-router";
import type { Route } from "./+types/home";
import { db } from "../.server/db";
import listPosts from "../.server/queries/list_posts.sql";
import createPost from "../.server/queries/create_post.sql";

// `loader` and `action` run on the server only: React Router removes them, and the imports only
// they use, from the browser bundle.
export async function loader() {
  return { posts: await listPosts.run(db) };
}

export async function action({ request }: Route.ActionArgs) {
  const form = await request.formData();
  return createPost.run(db, {
    user_id: Number(form.get("user_id")),
    title: String(form.get("title")),
    body: String(form.get("body")) || null,
  });
}

export default function Home({ loaderData }: Route.ComponentProps) {
  return (
    <main>
      <h1>Posts</h1>
      <ul>
        {loaderData.posts.map((post) => (
          <li key={post.id}>
            {post.title} by <Link to={`/users/${post.user_id}`}>{post.author}</Link>
            {post.body && <p>{post.body}</p>}
          </li>
        ))}
      </ul>
      <Form method="post">
        <input name="user_id" type="number" defaultValue={1} />
        <input name="title" placeholder="Title" required />
        <input name="body" placeholder="Body" />
        <button type="submit">Post</button>
      </Form>
    </main>
  );
}
