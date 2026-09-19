// Drives `tsc --lsp --stdio` for a completion and its resolve, the way an editor does, so the
// completion list and the completion detail can be compared with what the editor shows.
// usage: node lab/lsp-complete.mjs <projectDir> <fileToOpen> <line> <character> [insertText]
// DOC=<path> opens that text under the file's uri instead of the file's own; INSERT_AT=<line> inserts the text there.
import { spawn } from "node:child_process";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";

const [projectDir, file, line, character, insert] = process.argv.slice(2);
const root = resolve(projectDir);
const target = resolve(root, file);
const tsc = spawn(process.env.LANDO_TSC ?? resolve(root, "node_modules/.bin/tsc"), ["--lsp", "--stdio"], { cwd: root, stdio: ["pipe", "pipe", "inherit"] });
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
      const result = msg.method === "workspace/configuration" ? (msg.params?.items ?? []).map(() => null) : null;
      const body = Buffer.from(JSON.stringify({ jsonrpc: "2.0", id: msg.id, result }));
      tsc.stdin.write(`Content-Length: ${body.length}\r\n\r\n`); tsc.stdin.write(body);
    }
  }
});
const init = await send("initialize", { processId: process.pid, rootUri: pathToFileURL(root).href, capabilities: { textDocument: { completion: { completionItem: { resolveSupport: { properties: ["documentation", "detail", "additionalTextEdits"] } } } } }, initializationOptions: { runExternalCode: true }, workspaceFolders: [{ uri: pathToFileURL(root).href, name: "root" }] });
console.log("completionProvider:", JSON.stringify(init.result?.capabilities?.completionProvider));
send("initialized", {}, true);
const uri = pathToFileURL(target).href;
const lines = readFileSync(process.env.DOC ?? target, "utf8").split("\n");
if (insert !== undefined) lines.splice(Number(process.env.INSERT_AT ?? line), 0, insert);
send("textDocument/didOpen", { textDocument: { uri, languageId: "typescript", version: 1, text: lines.join("\n") } }, true);
await new Promise((r) => setTimeout(r, 2500));
const hover = await send("textDocument/hover", { textDocument: { uri }, position: { line: Number(line), character: 0 } });
console.log("hover:", JSON.stringify(hover.result?.contents ?? hover.error ?? null).slice(0, 300));
const completion = await send("textDocument/completion", { textDocument: { uri }, position: { line: Number(line), character: Number(character) }, context: { triggerKind: 2, triggerCharacter: "." } });
const items = completion.result?.items ?? completion.result ?? [];
console.log("completion items:", items.length, JSON.stringify(items.slice(0, 3).map((i) => ({ label: i.label, kind: i.kind, data: i.data }))));
for (const item of items.slice(0, 2)) {
  const resolved = await send("completionItem/resolve", item);
  console.log("resolved:", JSON.stringify({ label: resolved.result?.label, detail: resolved.result?.detail, documentation: resolved.result?.documentation, error: resolved.error }).slice(0, 400));
}
await send("shutdown", {}); send("exit", {}, true); setTimeout(() => process.exit(0), 500);
