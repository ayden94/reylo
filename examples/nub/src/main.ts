import { DatabaseSync } from "node:sqlite";
import createPost from "../queries/create_post.sql";
import getUser from "../queries/get_user.sql";
import listPosts from "../queries/list_posts.sql";

const db = new DatabaseSync("app.db");

const user = await getUser.run(db, { id: 1 });
//    ^? { id: number; name: string; email: string | null; role: string; created_at: string } | null
if (user) console.log(`${user.name} (${user.role}) joined ${user.created_at}`);

// node:sqlite runs synchronously, so runSync returns the row itself
const post = createPost.runSync(db, { user_id: 1, title: "From main.ts", body: null });
//    ^? { id: number; title: string }
console.log(`created #${post.id} ${post.title}`);

const posts = await listPosts.run(db, { role: "admin" });
//    ^? { id: number; title: string; body: string | null; author: string }[]
for (const post of posts) console.log(`#${post.id} ${post.title} by ${post.author}`);

db.close();
