import mysql from "mysql2/promise";
import query from "./query.sql";
const conn = await mysql.createConnection(process.env.DATABASE_URL ?? "mysql://root:lando@127.0.0.1:33306/lando");
console.log(JSON.stringify({ dialect: query.dialect, ada: await query.execute(conn, { id: 1, skip: "" }), nobody: await query.execute(conn, { id: 99, skip: "" }) }));
await conn.end();
