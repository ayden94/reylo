// Runtime surface. Placeholder while the shape settles: a typed query handle.
export type Query<Params, Row> = {
  readonly sql: string;
  readonly __params?: Params;
  readonly __row?: Row;
};
