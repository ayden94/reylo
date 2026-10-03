// The MySQL connection the analysis uses when the project has no mysql2: the client/server
// protocol's handshake, the mysql_native_password, caching_sha2_password and
// mysql_clear_password logins, TLS, a server-side prepare's metadata and a text query. It is not
// a driver: it runs the analysis's statements one at a time and decodes the values its catalog
// queries return (numbers, strings, null), nothing else.
import { constants, createHash, publicEncrypt } from "node:crypto";
import { connect as netConnect, isIP, type Socket } from "node:net";
import { connect as tlsConnect } from "node:tls";

export interface MysqlField {
  name: string;
  type: number;
  flags: number;
  charset: number;
}

export interface MysqlWire {
  /** A server-side prepare's parameters and result columns; the statement is closed again. */
  prepare(sql: string): Promise<{ params: MysqlField[]; columns: MysqlField[] }>;
  /** Rows as arrays of values, in column order; each `?` outside a quoted string takes the next value. */
  query(sql: string, values?: unknown[]): Promise<unknown[][]>;
  close(): Promise<void>;
}

const CLIENT_LONG_PASSWORD = 0x1;
const CLIENT_LONG_FLAG = 0x4;
const CLIENT_CONNECT_WITH_DB = 0x8;
const CLIENT_PROTOCOL_41 = 0x200;
const CLIENT_SSL = 0x800;
const CLIENT_TRANSACTIONS = 0x2000;
const CLIENT_SECURE_CONNECTION = 0x8000;
const CLIENT_MULTI_RESULTS = 0x20000;
const CLIENT_PLUGIN_AUTH = 0x80000;
const CLIENT_PLUGIN_AUTH_LENENC_CLIENT_DATA = 0x200000;
const SERVER_MORE_RESULTS_EXISTS = 0x8;
/** utf8mb4_unicode_ci, mysql2's default, so the server reports the same character sets either way. */
const CHARSET = 224;

type TlsMode = "disabled" | "preferred" | "required" | "verify-ca" | "verify-full";

/**
 * The TLS a URL asks for, in the spellings MySQL URLs use: `ssl-mode` (the mysql client's),
 * `sslmode`, Prisma's `sslaccept`, and mysql2's `ssl` (a boolean, a JSON object or a profile
 * name). Without one, TLS is used when the server offers it, as the mysql client's PREFERRED does.
 */
export function tlsMode(params: URLSearchParams): TlsMode {
  const mode = (params.get("ssl-mode") ?? params.get("ssl_mode") ?? params.get("sslmode"))?.toLowerCase().replace(/_/g, "-");
  if (mode === "disabled" || mode === "disable") return "disabled";
  if (mode === "preferred" || mode === "prefer") return "preferred";
  if (mode === "required" || mode === "require") return "required";
  if (mode === "verify-ca") return "verify-ca";
  if (mode === "verify-identity" || mode === "verify-full") return "verify-full";
  const accept = params.get("sslaccept");
  if (accept === "strict") return "verify-full";
  if (accept === "accept_invalid_certs") return "required";
  const ssl = params.get("ssl");
  if (ssl === null) return "preferred";
  if (ssl === "false" || ssl === "0") return "disabled";
  if (ssl === "true" || ssl === "1") return "required";
  try {
    return (JSON.parse(ssl) as { rejectUnauthorized?: boolean }).rejectUnauthorized === false ? "required" : "verify-full";
  } catch {
    return "verify-full";
  }
}

/** An ERR packet: the server's error number, SQLSTATE and message. */
function serverError(p: Buffer, fatal: boolean): Error {
  const errno = p.readUInt16LE(1);
  let o = 3;
  let sqlState = "HY000";
  if (p[o] === 0x23) {
    sqlState = p.toString("latin1", o + 1, o + 6);
    o += 6;
  }
  return Object.assign(new Error(p.toString("utf8", o)), { errno, sqlState, fatal });
}

/** Packets in and out of one socket, which TLS may replace once. */
class Packets {
  #socket!: Socket;
  #buf: Buffer = Buffer.alloc(0);
  #parts: Buffer[] = [];
  #ready: Buffer[] = [];
  #waiting: { resolve: (p: Buffer) => void; reject: (e: Error) => void }[] = [];
  failure: Error | undefined;
  /** The sequence number of the next packet written. */
  seq = 0;

  constructor(socket: Socket) {
    this.#attach(socket);
  }

  #attach(socket: Socket) {
    this.#socket = socket;
    socket.on("data", this.#data);
    socket.on("error", this.fail);
    socket.on("close", this.#closed);
  }

  #data = (chunk: Buffer) => {
    this.#buf = this.#buf.length ? Buffer.concat([this.#buf, chunk]) : chunk;
    while (this.#buf.length >= 4) {
      const length = this.#buf.readUIntLE(0, 3);
      if (this.#buf.length < 4 + length) break;
      this.seq = (this.#buf[3]! + 1) & 0xff;
      const part = this.#buf.subarray(4, 4 + length);
      this.#buf = this.#buf.subarray(4 + length);
      // A payload of 16 MB or more arrives in pieces; a shorter piece ends it.
      if (length === 0xffffff) {
        this.#parts.push(part);
        continue;
      }
      const payload = this.#parts.length ? Buffer.concat([...this.#parts.splice(0), part]) : part;
      const waiter = this.#waiting.shift();
      if (waiter) waiter.resolve(payload);
      else this.#ready.push(payload);
    }
  };

  #closed = () => this.fail(Object.assign(new Error("Connection lost: the server closed the connection."), { code: "PROTOCOL_CONNECTION_LOST" }));

  fail = (e: Error) => {
    this.failure ??= e;
    for (const waiter of this.#waiting.splice(0)) waiter.reject(this.failure);
    this.#socket.destroy();
  };

  read(): Promise<Buffer> {
    const ready = this.#ready.shift();
    if (ready) return Promise.resolve(ready);
    if (this.failure) return Promise.reject(this.failure);
    return new Promise((resolve, reject) => this.#waiting.push({ resolve, reject }));
  }

  write(payload: Buffer, seq = this.seq) {
    if (this.failure) throw this.failure;
    const header = Buffer.alloc(4);
    header.writeUIntLE(payload.length, 0, 3);
    header[3] = seq;
    this.#socket.write(Buffer.concat([header, payload]));
    this.seq = (seq + 1) & 0xff;
  }

  /** Continues on TLS over the same socket, after the SSLRequest packet. */
  secure(options: { servername: string | undefined; rejectUnauthorized: boolean; hostname: boolean }) {
    const raw = this.#socket;
    raw.off("data", this.#data);
    raw.off("error", this.fail);
    raw.off("close", this.#closed);
    this.#attach(tlsConnect({ socket: raw, servername: options.servername, rejectUnauthorized: options.rejectUnauthorized, ...(!options.hostname && { checkServerIdentity: () => undefined }) }));
  }

  close(): Promise<void> {
    const socket = this.#socket;
    if (socket.destroyed) return Promise.resolve();
    return new Promise((resolve) => {
      socket.once("close", () => resolve());
      socket.end();
      setTimeout(() => socket.destroy(), 1000).unref();
    });
  }
}

const sha1 = (...parts: Buffer[]) => createHash("sha1").update(Buffer.concat(parts)).digest();
const sha256 = (...parts: Buffer[]) => createHash("sha256").update(Buffer.concat(parts)).digest();
const xor = (a: Buffer, b: Buffer) => Buffer.from(a.map((x, i) => x ^ b[i % b.length]!));
const nul = (s: string) => Buffer.from(`${s}\0`, "utf8");

/** The first answer a login plugin sends for a password and the server's 20-byte nonce. */
function scramble(plugin: string, password: string, nonce: Buffer, secure: boolean): Buffer {
  const pw = Buffer.from(password, "utf8");
  switch (plugin) {
    case "mysql_native_password":
      return pw.length ? xor(sha1(pw), sha1(nonce, sha1(sha1(pw)))) : Buffer.alloc(0);
    case "caching_sha2_password":
      return pw.length ? xor(sha256(pw), sha256(sha256(sha256(pw)), nonce)) : Buffer.alloc(0);
    case "mysql_clear_password":
      if (!secure) throw new Error("The server asks for the password in clear text (mysql_clear_password), which is only sent over TLS.");
      return nul(password);
    default:
      throw new Error(`The server asks for the ${plugin} login, which Porg's MySQL connection does not support; install mysql2 in the project.`);
  }
}

function lenenc(p: Buffer, o: number): [value: number, next: number] {
  const first = p[o]!;
  if (first < 0xfb) return [first, o + 1];
  if (first === 0xfc) return [p.readUInt16LE(o + 1), o + 3];
  if (first === 0xfd) return [p.readUIntLE(o + 1, 3), o + 4];
  return [Number(p.readBigUInt64LE(o + 1)), o + 9];
}

function lenencBytes(n: number): Buffer {
  if (n < 0xfb) return Buffer.from([n]);
  if (n < 0x10000) return Buffer.from([0xfc, n & 0xff, n >> 8]);
  const b = Buffer.alloc(4);
  b[0] = 0xfd;
  b.writeUIntLE(n, 1, 3);
  return b;
}

/** A Column Definition packet (protocol 4.1). */
function field(p: Buffer): MysqlField {
  let o = 0;
  let name = "";
  for (let i = 0; i < 6; i++) {
    const [length, next] = lenenc(p, o);
    if (i === 4) name = p.toString("utf8", next, next + length);
    o = next + length;
  }
  o = lenenc(p, o)[1];
  return { name, charset: p.readUInt16LE(o), type: p[o + 6]!, flags: p.readUInt16LE(o + 7) };
}

const isEof = (p: Buffer) => p[0] === 0xfe && p.length < 9;

// mysql2's text-protocol conversions for the types the catalog queries return.
const NUMBERS = new Set([1, 2, 3, 4, 5, 8, 9, 13]); // tiny, short, long, float, double, longlong, int24, year
const JSON_TYPE = 245;
const BINARY = 63;
const STRINGS = new Set([15, 249, 250, 251, 252, 253, 254]); // varchar, the blobs, var_string, string

function row(p: Buffer, fields: MysqlField[]): unknown[] {
  let o = 0;
  return fields.map((f) => {
    if (p[o] === 0xfb) {
      o += 1;
      return null;
    }
    const [length, next] = lenenc(p, o);
    o = next + length;
    const bytes = p.subarray(next, o);
    if (NUMBERS.has(f.type)) return Number(bytes.toString("latin1"));
    if (f.type === JSON_TYPE) return JSON.parse(bytes.toString("utf8"));
    if (f.charset === BINARY && STRINGS.has(f.type)) return Buffer.from(bytes);
    return bytes.toString("utf8");
  });
}

function escape(v: unknown): string {
  if (v === null || v === undefined) return "NULL";
  if (typeof v === "number" && Number.isFinite(v)) return String(v);
  if (typeof v === "bigint") return String(v);
  if (typeof v === "boolean") return v ? "true" : "false";
  if (typeof v === "string") return `'${v.replace(/[\0\b\t\n\r\x1a"'\\]/g, (c) => ESCAPES[c]!)}'`;
  throw new TypeError(`Porg's MySQL connection cannot bind ${typeof v} ${String(v)}`);
}
const ESCAPES: Record<string, string> = { "\0": "\\0", "\b": "\\b", "\t": "\\t", "\n": "\\n", "\r": "\\r", "\x1a": "\\Z", '"': '\\"', "'": "\\'", "\\": "\\\\" };

/** The statement with each `?` outside a quoted string replaced by the next value, as mysql2's `query` binds. */
export function format(sql: string, values: unknown[]): string {
  if (!values.length) return sql;
  let out = "";
  let quote = "";
  let next = 0;
  for (let i = 0; i < sql.length; i++) {
    const ch = sql[i]!;
    if (quote) {
      out += ch;
      if (ch === "\\" && quote !== "`") out += sql[++i] ?? "";
      else if (ch === quote) quote = "";
    } else if (ch === "'" || ch === '"' || ch === "`") {
      quote = ch;
      out += ch;
    } else if (ch === "?") {
      if (next >= values.length) throw new Error(`The statement has more placeholders than the ${values.length} values given.`);
      out += escape(values[next++]);
    } else out += ch;
  }
  if (next !== values.length) throw new Error(`The statement has ${next} placeholders for ${values.length} values.`);
  return out;
}

/** Opens a connection to a `mysql://user:password@host:port/database` URL. */
export async function connectMysql(url: string, timeoutMs: number): Promise<MysqlWire> {
  const u = new URL(url);
  const host = u.hostname.replace(/^\[(.*)\]$/, "$1") || "localhost";
  const user = decodeURIComponent(u.username);
  const password = decodeURIComponent(u.password);
  const database = decodeURIComponent(u.pathname.replace(/^\//, ""));
  const socketPath = u.searchParams.get("socketPath") ?? u.searchParams.get("socket");
  const mode = tlsMode(u.searchParams);

  const socket = socketPath ? netConnect({ path: socketPath }) : netConnect({ host, port: Number(u.port || 3306) });
  socket.setNoDelay(true);
  const wire = new Packets(socket);
  const timer = setTimeout(() => wire.fail(Object.assign(new Error(`connect ETIMEDOUT: no answer from ${host} within ${timeoutMs / 1000} s`), { code: "ETIMEDOUT" })), timeoutMs);
  try {
    // The server's Initial Handshake (protocol 10).
    const hello = await wire.read();
    if (hello[0] === 0xff) throw serverError(hello, true);
    if (hello[0] !== 10) throw new Error(`The server speaks MySQL protocol ${hello[0]}, not 10.`);
    let o = hello.indexOf(0, 1) + 1 + 4;
    const nonce1 = hello.subarray(o, o + 8);
    o += 9;
    let caps = hello.readUInt16LE(o);
    o += 2;
    let nonce = nonce1;
    let plugin = "mysql_native_password";
    if (o < hello.length) {
      caps = (caps | (hello.readUInt16LE(o + 3) << 16)) >>> 0;
      const authLength = hello[o + 5]!;
      o += 16;
      if (caps & CLIENT_SECURE_CONNECTION) {
        const n = Math.max(13, authLength - 8);
        nonce = Buffer.concat([nonce1, hello.subarray(o, o + n - 1)]);
        o += n;
      }
      if (caps & CLIENT_PLUGIN_AUTH) {
        const end = hello.indexOf(0, o);
        plugin = hello.toString("utf8", o, end < 0 ? hello.length : end);
      }
    }
    if (!(caps & CLIENT_PROTOCOL_41)) throw new Error("The server does not speak MySQL's protocol 4.1.");

    const offered = (caps & CLIENT_SSL) !== 0;
    const tls = mode === "disabled" ? false : mode === "preferred" ? offered && !socketPath : true;
    if (tls && !offered) throw new Error(`The URL asks for TLS (${mode}), and the server does not offer it.`);
    const flags =
      (CLIENT_LONG_PASSWORD | CLIENT_LONG_FLAG | CLIENT_PROTOCOL_41 | CLIENT_TRANSACTIONS | CLIENT_SECURE_CONNECTION | CLIENT_MULTI_RESULTS | (caps & (CLIENT_PLUGIN_AUTH | CLIENT_PLUGIN_AUTH_LENENC_CLIENT_DATA)) | (database ? CLIENT_CONNECT_WITH_DB : 0) | (tls ? CLIENT_SSL : 0)) >>> 0;
    const head = Buffer.alloc(32);
    head.writeUInt32LE(flags, 0);
    head.writeUInt32LE(0x1000000, 4);
    head[8] = CHARSET;
    if (tls) {
      wire.write(head);
      wire.secure({ servername: isIP(host) || socketPath ? undefined : host, rejectUnauthorized: mode === "verify-ca" || mode === "verify-full", hostname: mode === "verify-full" });
    }
    const secure = tls || Boolean(socketPath);
    const auth = scramble(plugin, password, nonce, secure);
    wire.write(Buffer.concat([head, nul(user), flags & CLIENT_PLUGIN_AUTH_LENENC_CLIENT_DATA ? lenencBytes(auth.length) : Buffer.from([auth.length]), auth, database ? nul(database) : Buffer.alloc(0), flags & CLIENT_PLUGIN_AUTH ? nul(plugin) : Buffer.alloc(0)]));

    // The login: OK, an error, a switch to another plugin, or caching_sha2_password's next step.
    for (;;) {
      const p = await wire.read();
      if (p[0] === 0x00) break;
      if (p[0] === 0xff) throw serverError(p, true);
      if (p[0] === 0xfe) {
        const end = p.indexOf(0, 1);
        plugin = p.toString("utf8", 1, end);
        nonce = p.subarray(end + 1, end + 21);
        wire.write(scramble(plugin, password, nonce, secure));
      } else if (p[0] === 0x01 && plugin === "caching_sha2_password") {
        const data = p.subarray(1);
        if (data.length === 1 && data[0] === 3) continue; // the server's cache knew the password: OK follows
        if (data.length === 1 && data[0] === 4) wire.write(secure ? nul(password) : Buffer.from([2])); // full authentication; without TLS, ask for the server's RSA key
        else wire.write(publicEncrypt({ key: data.toString("latin1"), padding: constants.RSA_PKCS1_OAEP_PADDING, oaepHash: "sha1" }, xor(nul(password), nonce)));
      } else throw new Error(`Unexpected packet 0x${p[0]!.toString(16)} during the ${plugin} login.`);
    }
  } catch (e) {
    clearTimeout(timer);
    wire.fail(e as Error);
    throw e;
  }
  clearTimeout(timer);

  // One statement at a time, as the protocol requires.
  let tail: Promise<unknown> = Promise.resolve();
  const serial = <T>(run: () => Promise<T>): Promise<T> => {
    const result = tail.then(run, run);
    tail = result.catch(() => {});
    return result;
  };
  const fields = async (count: number) => {
    const out: MysqlField[] = [];
    for (let i = 0; i < count; i++) out.push(field(await wire.read()));
    if (count && !isEof(await wire.read())) throw new Error("Expected the end of the column definitions.");
    return out;
  };

  return {
    prepare: (sql) =>
      serial(async () => {
        wire.write(Buffer.concat([Buffer.from([0x16]), Buffer.from(sql, "utf8")]), 0);
        const ok = await wire.read();
        if (ok[0] === 0xff) throw serverError(ok, false);
        const id = ok.readUInt32LE(1);
        const params = await fields(ok.readUInt16LE(7));
        const columns = await fields(ok.readUInt16LE(5));
        const close = Buffer.alloc(5);
        close[0] = 0x19;
        close.writeUInt32LE(id, 1);
        wire.write(close, 0); // COM_STMT_CLOSE has no answer
        return { params, columns };
      }),
    query: (sql, values = []) =>
      serial(async () => {
        wire.write(Buffer.concat([Buffer.from([0x03]), Buffer.from(format(sql, values), "utf8")]), 0);
        let rows: unknown[][] | undefined;
        for (;;) {
          const first = await wire.read();
          if (first[0] === 0xff) throw serverError(first, false);
          let status: number;
          if (first[0] === 0x00) {
            const o = lenenc(first, lenenc(first, 1)[1])[1];
            status = first.readUInt16LE(o);
          } else {
            const columns = await fields(lenenc(first, 0)[0]);
            const set: unknown[][] = [];
            for (;;) {
              const p = await wire.read();
              if (isEof(p)) {
                status = p.readUInt16LE(3);
                break;
              }
              if (p[0] === 0xff) throw serverError(p, false);
              set.push(row(p, columns));
            }
            rows ??= set;
          }
          if (!(status & SERVER_MORE_RESULTS_EXISTS)) return rows ?? [];
        }
      }),
    close: () =>
      serial(async () => {
        try {
          wire.write(Buffer.from([0x01]), 0); // COM_QUIT
        } catch {}
        await wire.close();
      }),
  };
}
