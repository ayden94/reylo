import query from "./query.sql";
const r: { id: number; name: string } | null = null! as Awaited<ReturnType<typeof query.run>>;
