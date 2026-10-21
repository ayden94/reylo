import postgres from "postgres";
import createPost from "../queries/create_post.sql";
import getUser from "../queries/get_user.sql";
import listPosts from "../queries/list_posts.sql";

const sql = postgres(process.env["DATABASE_URL"]!);

const user = await getUser.run(sql, { id: 1 });
//    ^? { id: number; name: string; email: string | null; role: "admin" | "member"; created_at: string } | null
if (user) console.log(`${user.name} (${user.role}) joined ${user.created_at}`);

const post = await createPost.run(sql, { user_id: 1, title: "From main.ts", body: null });
//    ^? { id: number; title: string }
console.log(`created #${post.id} ${post.title}`);

const posts = await listPosts.run(sql, { role: "admin" });
//    ^? { id: number; title: string; body: string | null; author: string }[]
for (const post of posts) console.log(`#${post.id} ${post.title} by ${post.author}`);

await sql.end();
