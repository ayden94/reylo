declare const l: any;

// get single user
const getUserById = l.User.byId();
await getUserById({ id: "abc123" });

// get single user by UNIQUE field
const getUserByUnique = l.User.byUnique("externalId");
await getUserById({ id: "abc123" });

// fetch multiple users
const getUsers = l.User((user) => ({
	where: user.name.eq("Colin"),
	limit: 10,
	skip: 10,
	order: user.createdAt,
	dir: "asc",
}));
await getUsers({ where: { name: "John" } });


// l.with({name: l.string}).select_user((user, params) => ({

// }));

l.with({
	id: l.string,
	data: l.User.$update
}).User.updateOne((_, params) => ({
	whereId: params.id,
	set: params.$data
}))
const getUsersWithParams = l.User({ name: l.string }, (user, params) => ({
	with: { name: l.string },
	where: user.name.eq(params.name),
	limit: 10,
	skip: 10,
	order: user.createdAt,
	dir: "asc",
	
}));
await getUsers({ name: "John" });

// l.with({name: l.string}).User((user, params) => ({
// 	$where: user.name.eq(params.name),
// 	$limit: 10,
// 	id: true,
// 	name: true,
// 	postCount: l.count(user.posts),
// }).run(db);

// update multiple users
l.with({name: l.string()}).select$user((user, params) => ({
	where: user.name.eq(params.name),
	select: {
		id: true,
		count: l.count(user.posts),
	}
})

// l.User.select((user, params) => ({
// 	where: user.name.eq("$name"),
// 	select: {
// 		id: true,
// 		count: l.count(user.posts),
// 	}
// })

// })
const updateUserPoints = getUsers.update({
	points: true,
});
await updateUserPoints({
	where: { name: "John" },
	update: { points: 25 },
});

// delete multiple users
const deleteUsers = getUsers.delete();
await deleteUsers({ where: { name: "John" } });

// create a new user
const createUser = l.User.create();
await createUser({
	create: {
		name: "John",
		email: "john@example.com",
		password: "password",
	},
});

// selection sets
const getUserWithFields = getUser.select({
	id: true,
	name: true,
	points: true,
});
await getUserWithFields({ where: { id: "abc123" } });
// => { id: true, name: true, points: true }

// computed fields
const getUserWithComputedFields = getUser.select((user) => ({
	id: true,
	name: true,
	points: true,
	computed: l.upper(user.name),
	postCount: l.count(user.posts),
}));
await getUserWithComputedFields({ where: { id: "abc123" } });
// => { id: true, name: true, points: true, computed: true, postCount: true }

// nested selection sets
const getUserWithNestedFields = getUser.select((user) => ({
	id: true,
	name: true,
	// get all fields
	posts: true,
	// specify fields
	posts2: user.posts.select({
		id: true,
		title: true,
	}),
}));

// chaining
const updateSinglePostByIdFetchIdOnly = l.Post.byId()
	.update({
		title: true,
	})
	.select({
		id: true,
		title: true,
	});

await updateSinglePostByIdFetchIdOnly({
	id: "abc123",
	update: { title: "New Title" },
});

// without chaining
const updatePostsNoChaining = l.Post.update(
	{ prefix: l.string, title: l.string },
	(post, params) => ({
		where: post.title.gte(params.prefix),
		limit: 10,
		skip: 10,
		orderBy: post.createdAt,
		dir: "asc",
		set: {
			title: params.title,
		},
		select: { id: true, title: true },
	}),
);

const updatePostsNoChaining2 = l.params(
	{ prefix: l.string, title: l.string },
	(params) =>
		l.user.update((post) => ({
			where: post.title.gte(params.prefix),
			orderBy: post.createdAt,
			dir: "asc",
			set: {
				title: params.title,
			},
			select: { id: true, title: true },
		})),
);

const updatePostNoChaining3 = l.post.update(
	{ prefix: l.string, title: l.string },
	(post, params) =>
		post
			.where(post.title.gte(params.prefix))
			.orderBy(post.createdAt, "asc")
			.set({
				title: params.title,
			})
			.select({ id: true, title: true }),
);

// const updatePostsNoChaining4 = l.input({
// 	prefix: l.string,
// 	title: l.string,
// });
const updatePostsNoChaining4 = l.post.update((post) => ({
	where: post.title.gte("A"),
	set: { title: "New Title" },
}));

const updatePostsNoChaining3 = l.user.insert((user) =>
	user
		.values({
			name: true,
			title: true,
		})
		.select({ id: true, title: true }),
);
await updatePostsNoChaining({
	prefix: "A",
	update: { title: "New Title" },
});

function test(arg: any, { arg2, ...rest }: { arg2: true }) {
	// arg2;//
}
