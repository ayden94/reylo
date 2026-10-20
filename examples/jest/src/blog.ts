import type { Database } from "better-sqlite3";
import createPost from "../queries/create_post.sql";
import getUser from "../queries/get_user.sql";
import listPosts from "../queries/list_posts.sql";

/** Publishes a post by the given user: only an admin may. */
export async function publish(db: Database, userId: number, title: string, body: string | null) {
  const author = await getUser.run(db, { id: userId });
  if (!author) throw new Error(`no user ${userId}`);
  if (author.role !== "admin") throw new Error(`${author.name} is not an admin`);
  return createPost.run(db, { user_id: userId, title, body });
}

/** The posts admins wrote, oldest first, each with its author's name. */
export function adminPosts(db: Database) {
  return listPosts.run(db, { role: "admin" });
}
