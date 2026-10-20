import "server-only";
import postgres from "postgres";

// `next dev` evaluates this module again after an edit (with webpack, after any edit), and each
// postgres() opens a pool of its own: keep one client on globalThis across reloads.
const globalForDb = globalThis as typeof globalThis & { sql?: postgres.Sql };

export const sql = (globalForDb.sql ??= postgres(process.env.DATABASE_URL!));
