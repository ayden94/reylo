// Creates the database DATABASE_URL names, unless it exists, and the tables of setup.sql, with
// their rows.
import { readFile } from "node:fs/promises";
import mysql from "mysql2/promise";

const url = process.env["DATABASE_URL"];
if (!url) throw new Error("Set DATABASE_URL, in .env or in the environment");

const server = new URL(url);
const database = decodeURIComponent(server.pathname.slice(1));
server.pathname = "/";

const conn = await mysql.createConnection({ uri: server.href, multipleStatements: true });
await conn.query(`create database if not exists \`${database}\``);
await conn.changeUser({ database });
await conn.query(await readFile("setup.sql", "utf8"));
await conn.end();
console.log("created users and posts");
