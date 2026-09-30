import getUser from "../get_user.sql";
import { sql } from "../db";

export const dynamic = "force-dynamic";

export default async function Page() {
  const user = await getUser.run(sql, { id: 1 });
  return <pre id="row">{JSON.stringify(user)}</pre>;
}
