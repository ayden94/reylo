// Each .sql import is a typed query: the type check types it from the database, and reylo/vite
// builds it into a module that runs it on the D1 binding.
import createPost from "../queries/create_post.sql";
import getUser from "../queries/get_user.sql";
import listPosts from "../queries/list_posts.sql";

export default {
  async fetch(request, env): Promise<Response> {
    const url = new URL(request.url);
    const userId = /^\/users\/(\d+)$/.exec(url.pathname)?.[1];

    if (request.method === "GET" && url.pathname === "/posts") {
      const role = url.searchParams.get("role") ?? "admin";
      return Response.json(await listPosts.run(env.DB, { role }));
    }
    if (request.method === "GET" && userId) {
      const user = await getUser.run(env.DB, { id: Number(userId) });
      return user ? Response.json(user) : Response.json({ error: "not found" }, { status: 404 });
    }
    if (request.method === "POST" && url.pathname === "/posts") {
      const { user_id, title, body = null } = await request.json<{ user_id: number; title: string; body?: string | null }>();
      return Response.json(await createPost.run(env.DB, { user_id, title, body }), { status: 201 });
    }
    return Response.json({ error: "not found" }, { status: 404 });
  },
} satisfies ExportedHandler<Cloudflare.Env>;
