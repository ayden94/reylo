import { data } from "react-router";
import type { Route } from "./+types/user";
import { db } from "../.server/db";
import getUser from "../.server/queries/get_user.sql";

export async function loader({ params }: Route.LoaderArgs) {
  // `get_user.sql` matches the primary key, so `run` resolves to the row or null.
  const user = await getUser.run(db, { id: Number(params.id) });
  if (!user) throw data("No such user", { status: 404 });
  return { user };
}

export default function User({ loaderData: { user } }: Route.ComponentProps) {
  return (
    <main>
      <h1>{user.name}</h1>
      <p>{user.email ?? "no email"}, {user.role}, joined {user.created_at}</p>
    </main>
  );
}
