import { DatabaseSync } from "node:sqlite";
import { unlinkSync } from "node:fs";
try { unlinkSync(new URL("./lab.db", import.meta.url)); } catch {}
const db = new DatabaseSync(new URL("./lab.db", import.meta.url).pathname);
db.exec(`create table users (id integer primary key, name text not null, email text, score real, avatar blob, created_at text not null default current_timestamp);
insert into users (name, email, score) values ('ada', 'ada@example.com', 1.5), ('bob', null, null);`);
db.close();
