// The query module on the edge runtime. postgres.js needs TCP sockets, which the edge runtime does
// not have, so the client here is a stub of the node-postgres shape that answers with a fixed row.
import getUser from "../../../get_user.sql";

export const runtime = "edge";

const stub = {
  async query(text: string, values?: unknown[]) {
    return { rows: [{ id: values?.[0], name: "stub", email: null, role: "member", tags: [], created_at: new Date(0), big: BigInt(7), text }] };
  },
};

export async function GET(request: Request) {
  const id = Number(new URL(request.url).searchParams.get("id") ?? 1);
  const user = await getUser.run(stub, { id });
  return Response.json({ runtime: "edge", user, cardinality: getUser.cardinality, params: getUser.params });
}
