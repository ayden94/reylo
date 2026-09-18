import { Database } from "bun:sqlite";
const db = new Database(":memory:");
db.exec(`create table t (id integer primary key, name text not null, big integer); insert into t (name) values ('ada')`);
const st = db.prepare("select id, name, big, id + 1 as expr from t where id = :id");
console.log("bun:sqlite all:", JSON.stringify(st.all({ ":id": 1 })), "columnNames:", JSON.stringify(st.columnNames), "declaredTypes:", JSON.stringify((st as any).declaredTypes ?? null), "columnTypes:", JSON.stringify((st as any).columnTypes ?? null));
try { console.log("bun:sqlite mixed:", JSON.stringify(db.prepare("select :a as a, ? as b, $c as c").all({ ":a": 1, "$c": 3 }, 2))); } catch (e) { console.log("bun:sqlite mixed ERR", (e as Error).message); }
