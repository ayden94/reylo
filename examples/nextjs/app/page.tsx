import { Suspense } from "react";
import { revalidatePath } from "next/cache";
import { sql } from "@/lib/db";
import createPost from "@/queries/create_post.sql";
import listPosts from "@/queries/list_posts.sql";

async function addPost(form: FormData) {
  "use server";
  await createPost.run(sql, {
    user_id: Number(form.get("user_id")),
    title: String(form.get("title")),
    body: String(form.get("body") ?? "") || null,
  });
  revalidatePath("/");
}

async function Posts() {
  const posts = await listPosts.run(sql);
  //    ^? { id: number; title: string; body: string | null; author: string }[]
  return (
    <ul>
      {posts.map((post) => (
        <li key={post.id}>
          <b>{post.title}</b> by {post.author}
          {post.body && <p>{post.body}</p>}
        </li>
      ))}
    </ul>
  );
}

export default function Home() {
  return (
    <main>
      <h1>Posts</h1>
      {/* Cache Components: a query runs at request time, inside <Suspense>. */}
      <Suspense fallback={<p>Loading…</p>}>
        <Posts />
      </Suspense>
      <form action={addPost}>
        <input name="user_id" type="number" defaultValue={1} required />
        <input name="title" placeholder="Title" required />
        <textarea name="body" placeholder="Body" />
        <button>Post</button>
      </form>
    </main>
  );
}
