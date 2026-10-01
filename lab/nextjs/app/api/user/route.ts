import getUser from "../../../get_user.sql";
import { sql } from "../../../db";

export async function GET(request: Request) {
  const id = Number(new URL(request.url).searchParams.get("id") ?? 1);
  const user = await getUser.run(sql, { id });
  // `text` is the statement as the engine rewrote it for the database, which the loader puts in
  // the module; it is not part of the typed surface, hence the cast.
  const rewritten = (getUser as unknown as { text?: string }).text ?? null;
  return Response.json({ user, cardinality: getUser.cardinality, params: getUser.params, rewritten });
}
