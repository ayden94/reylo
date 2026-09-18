import postgres from "postgres";
import getUser from "../queries/get_user.sql";
import listPosts from "../queries/list_posts.sql";

const sql = postgres(process.env["DATABASE_URL"] ?? "postgres://localhost:5432/postgres");

const user = await getUser.execute(sql, { id: 1 });
//    ^? { id: number; name: string; email: string | null; role: "admin" | "member"; created_at: Date } | null
if (user) console.log(`${user.name} (${user.role}) joined ${user.created_at.toISOString()}`);

const posts = await listPosts.execute(sql, { role: "admin" });
//    ^? { id: number; title: string; body: string | null; author: string }[]
for (const post of posts) console.log(`#${post.id} ${post.title} by ${post.author}`);

await sql.end();
