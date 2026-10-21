import getUser from "../get_user.sql";
import { sql } from "../db";

export const dynamic = "force-dynamic";

export default async function Page() {
  const user = await getUser.run(sql, { id: 1 });
  // A BigInt, which JSON.stringify refuses, as its text.
  return <pre id="row">{JSON.stringify(user, (_, v) => (typeof v === "bigint" ? String(v) : v))}</pre>;
}
