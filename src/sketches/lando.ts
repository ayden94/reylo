type ObjectExpr<T> = {
	[k in keyof T]: T[k] extends string
		? StringExpr
		: T[k] extends number
			? NumberExpr
			: T[k] extends boolean
				? BooleanExpr
				: never;
};
type BooleanExpr = {
	eq(val: boolean | BooleanExpr): BooleanExpr;
};
type NumberExpr = {
	eq(val: number | NumberExpr): BooleanExpr;
	gt(val: number | NumberExpr): BooleanExpr;
	lt(val: number | NumberExpr): BooleanExpr;
	gte(val: number | NumberExpr): BooleanExpr;
	lte(val: number | NumberExpr): BooleanExpr;
};

type StringExpr = {
	eq(val: string | StringExpr): BooleanExpr;
	gt(val: string | StringExpr): BooleanExpr;
	lt(val: string | StringExpr): BooleanExpr;
	gte(val: string | StringExpr): BooleanExpr;
	lte(val: string | StringExpr): BooleanExpr;
};
type User = { name: string; id: string };
declare const l: {
	upper(arg: unknown): unknown;
	User: {
		select: (
			getter: (u: ObjectExpr<User>) => Partial<{
				id: true;
				name: true;
				limit: number;
				filter: unknown;
				[k: string]: unknown;
			}>,
		) => ObjectExpr<User>;
	};
};

l.User.select((u) => ({
	id: true,
	name: true,
	computed: l.upper(u.name),
	filter: u.name.eq("asdf"),
	limit: 1324,
}));

l.User.select.one((u) => ({
	id: true,
	name: true,
}));

export {};
