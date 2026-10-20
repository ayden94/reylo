// No driver here, so that the edge route can import it.
/** A JSON response, a BigInt (a 64-bit integer, src/values.ts) written as its text: JSON.stringify refuses one. */
export const json = (body: unknown) => new Response(JSON.stringify(body, (_, v) => (typeof v === "bigint" ? String(v) : v)), { headers: { "content-type": "application/json" } });
