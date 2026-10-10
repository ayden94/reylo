import mysql from "mysql2/promise";
import mariadb from "mariadb";
import { execute } from "../../src/execute.ts";
const cfg = { host: "127.0.0.1", port: 33306, user: "root", password: "porg", database: "porg" };
const conn = await mysql.createConnection({ ...cfg, namedPlaceholders: true });
await conn.query(`drop table if exists t`);
await conn.query(`create table t (id int primary key auto_increment, name varchar(50) not null, big bigint not null default 9007199254740993, n decimal(10,2) not null default 1.50, at datetime not null default current_timestamp, j json, flag tinyint(1) not null default 1)`);
await conn.query(`insert into t (name, j) values ('ada', '{"a":1}')`);
const q = { dialect: "mysql", sql: "select id, name, big, n, at, j, flag, id + 1 as expr from t where id = ?", params: ["1"] };
const rows = await execute(conn, q, { 1: 1 });
console.log("mysql2 execute:", rows[0], "big type:", typeof rows[0].big, "at:", rows[0].at?.constructor?.name);
// metadata without running: prepared statement
const stmt = await conn.prepare(q.sql);
console.log("mysql2 prepare keys:", Object.keys(stmt), "statement keys:", stmt.statement ? Object.keys(stmt.statement) : null);
const info = stmt.statement ?? stmt;
console.log("mysql2 prepare columns:", info.columns?.map(c => [c.name, c.columnType, c.flags, c.table, c.orgName]));
console.log("mysql2 prepare parameters:", info.parameters?.map(p => [p.name, p.columnType, p.flags]));
await stmt.close();
// named placeholders through mysql2 itself
const [named] = await conn.query("select id from t where id = :id and name = :name", { id: 1, name: "ada" });
console.log("mysql2 namedPlaceholders:", named);
try { const [mixed] = await conn.query("select id from t where id = :id and name = ?", { id: 1 }); console.log("mysql2 mixed:", mixed); } catch (e) { console.log("mysql2 mixed ERR:", e.message); }
// mariadb connector
const pool = mariadb.createPool({ ...cfg, connectionLimit: 1 });
const mrows = await execute(pool, q, { 1: 1 });
console.log("mariadb execute:", mrows[0], "big type:", typeof mrows[0].big, "meta:", mrows.meta?.map(m => [m.name(), m.type, m.columnType]).slice(0, 3));
const mnamed = await pool.query({ namedPlaceholders: true, sql: "select id from t where id = :id" }, { id: 1 });
console.log("mariadb namedPlaceholders:", mnamed);
await conn.query(`drop table t`); await conn.end(); await pool.end();
