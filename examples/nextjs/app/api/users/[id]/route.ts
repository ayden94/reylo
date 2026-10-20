import { sql } from "@/lib/db";
import getUser from "@/queries/get_user.sql";

export async function GET(_request: Request, { params }: RouteContext<"/api/users/[id]">) {
  const id = Number((await params).id);
  const user = Number.isSafeInteger(id) ? await getUser.run(sql, { id }) : null;
  //    ^? { id: number; name: string; email: string | null; role: "admin" | "member"; created_at: Date } | null
  if (!user) return Response.json({ error: "not found" }, { status: 404 });
  return Response.json(user);
}
