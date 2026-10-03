// Drives `tsc --lsp --stdio` the way an editor does, with runExternalCode on, to
// prove the content mapper works through the language server: diagnostics for a
// broken .sql file and a hover over a query result.
// usage: node lab/lsp-client.mjs <projectDir> <fileToOpen> <line> <character>
import { spawn } from "node:child_process";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";

const [projectDir, file, line, character] = process.argv.slice(2);
const root = resolve(projectDir);
const target = resolve(root, file);
const tsc = spawn(process.env.DOTSQL_TSC ?? resolve(root, "node_modules/.bin/tsc"), ["--lsp", "--stdio"], { cwd: root, stdio: ["pipe", "pipe", "inherit"] });
let seq = 0;
const pending = new Map();
const send = (method, params, isNotification = false) => {
  const msg = { jsonrpc: "2.0", method, params, ...(isNotification ? {} : { id: ++seq }) };
  const body = Buffer.from(JSON.stringify(msg));
  tsc.stdin.write(`Content-Length: ${body.length}\r\n\r\n`);
  tsc.stdin.write(body);
  return isNotification ? undefined : new Promise((res) => pending.set(seq, res));
};
let buf = Buffer.alloc(0);
const diagnostics = [];
tsc.stdout.on("data", (chunk) => {
  buf = Buffer.concat([buf, chunk]);
  for (;;) {
    let end = buf.indexOf("\r\n\r\n");
    let sep = 4;
    if (end === -1) { end = buf.indexOf("\n\n"); sep = 2; }
    if (end === -1) return;
    const len = Number(/Content-Length: (\d+)/i.exec(buf.subarray(0, end).toString())[1]);
    if (buf.length < end + sep + len) return;
    const msg = JSON.parse(buf.subarray(end + sep, end + sep + len).toString());
    buf = buf.subarray(end + sep + len);
    if (msg.id !== undefined && pending.has(msg.id)) { pending.get(msg.id)(msg); pending.delete(msg.id); }
    else if (msg.id !== undefined && msg.method) {
      // Server → client request (client/registerCapability, workspace/configuration, ...): answer it.
      const result = msg.method === "workspace/configuration" ? (msg.params?.items ?? []).map(() => null) : null;
      const body = Buffer.from(JSON.stringify({ jsonrpc: "2.0", id: msg.id, result }));
      tsc.stdin.write(`Content-Length: ${body.length}\r\n\r\n`); tsc.stdin.write(body);
    }
    else if (msg.method === "textDocument/publishDiagnostics") diagnostics.push(msg.params);
    else if (msg.method === "window/logMessage" && /mapper|dotsql/i.test(msg.params?.message ?? "")) console.log("LOG", msg.params.message.replace(/\n/g, " ").slice(0, 300));
  }
});
console.log("sending initialize");
const init = await send("initialize", { processId: process.pid, rootUri: pathToFileURL(root).href, capabilities: {}, initializationOptions: { runExternalCode: true, logVerbosity: 1 }, workspaceFolders: [{ uri: pathToFileURL(root).href, name: "root" }] });
console.log("initialize ok; content mappers on:", init.result?.capabilities !== undefined);
send("initialized", {}, true);
for (const extra of (process.env.LSP_OPEN ?? "").split(",").filter(Boolean)) {
  const p = resolve(root, extra);
  send("textDocument/didOpen", { textDocument: { uri: pathToFileURL(p).href, languageId: "sql", version: 1, text: readFileSync(p, "utf8") } }, true);
}
send("textDocument/didOpen", { textDocument: { uri: pathToFileURL(target).href, languageId: "typescript", version: 1, text: readFileSync(target, "utf8") } }, true);
await new Promise((r) => setTimeout(r, 2500));
const hover = await send("textDocument/hover", { textDocument: { uri: pathToFileURL(target).href }, position: { line: Number(line), character: Number(character) } });
console.log("hover:", JSON.stringify(hover.result?.contents ?? hover.error).slice(0, 400));
await new Promise((r) => setTimeout(r, 1500));
// tsgo's server may serve diagnostics on request rather than by push.
for (const f of [file, ...(process.env.LSP_OPEN ?? "").split(",").filter(Boolean)]) {
  const uri = pathToFileURL(resolve(root, f)).href;
  const pulled = await send("textDocument/diagnostic", { textDocument: { uri } });
  const items = pulled.result?.items ?? [];
  console.log("pulled diagnostics", f, JSON.stringify(items.map((x) => ({ start: x.range.start, source: x.source, message: x.message.slice(0, 100) }))));
}
for (const d of diagnostics) if (d.diagnostics.length) console.log("diagnostics", d.uri.split("/").slice(-2).join("/"), JSON.stringify(d.diagnostics.map((x) => ({ range: x.range.start, source: x.source, message: x.message.slice(0, 80) }))));
if (!diagnostics.some((d) => d.diagnostics.length)) console.log("no diagnostics published (files clean)");
await send("shutdown", {}); send("exit", {}, true); setTimeout(() => process.exit(0), 500);
