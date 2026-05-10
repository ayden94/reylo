// Hand-built schema fixtures. Small enough to read top-to-bottom; each
// covers one shape we want validation / serialization to handle.
//
// All fixtures share the same `metadata` so snapshots stay focused on
// the parts we're actually exercising.

import type { LandoSchema } from '#schema';
import { FORMAT_VERSION } from '#schema';

const META = {
  generatedAt: '2026-01-01T00:00:00.000Z',
  generatedBy: 'lando@0.0.0-fixture',
  source: { kind: 'introspection', fingerprint: 'fixture' },
  checksum: 'fixture',
} as const;

/** User + Post with a one-to-many relation. */
export const simple: LandoSchema = {
  formatVersion: FORMAT_VERSION,
  models: {
    User: {
      name: 'User',
      tableName: 'users',
      primaryKey: { fields: ['id'] },
      fields: [
        {
          name: 'id',
          columnName: 'id',
          type: { kind: 'scalar', scalar: 'uuid' },
          nullable: false,
          attributes: {},
        },
        {
          name: 'email',
          columnName: 'email',
          type: { kind: 'scalar', scalar: 'string' },
          nullable: false,
          attributes: { maxLength: 255 },
        },
        {
          name: 'posts',
          columnName: '',
          type: { kind: 'relation', relation: 'UserPosts' },
          nullable: false,
          attributes: {},
        },
      ],
      uniques: [{ fields: ['email'] }],
      indexes: [],
    },
    Post: {
      name: 'Post',
      tableName: 'posts',
      primaryKey: { fields: ['id'] },
      fields: [
        {
          name: 'id',
          columnName: 'id',
          type: { kind: 'scalar', scalar: 'uuid' },
          nullable: false,
          attributes: {},
        },
        {
          name: 'title',
          columnName: 'title',
          type: { kind: 'scalar', scalar: 'string' },
          nullable: false,
          attributes: {},
        },
        {
          name: 'authorId',
          columnName: 'author_id',
          type: { kind: 'scalar', scalar: 'uuid' },
          nullable: false,
          attributes: {},
        },
        {
          name: 'author',
          columnName: '',
          type: { kind: 'relation', relation: 'UserPosts' },
          nullable: false,
          attributes: {},
        },
      ],
      uniques: [],
      indexes: [{ fields: ['authorId'], unique: false }],
    },
  },
  enums: {},
  relations: [
    {
      name: 'UserPosts',
      kind: 'one-to-many',
      from: 'Post',
      to: 'User',
      fkColumns: ['author_id'],
      references: ['id'],
      forwardName: 'author',
      reverseName: 'posts',
      onDelete: 'cascade',
    },
  ],
  metadata: META,
};

/** A schema with a Postgres enum. */
export const withEnum: LandoSchema = {
  formatVersion: FORMAT_VERSION,
  models: {
    Account: {
      name: 'Account',
      tableName: 'accounts',
      primaryKey: { fields: ['id'] },
      fields: [
        {
          name: 'id',
          columnName: 'id',
          type: { kind: 'scalar', scalar: 'uuid' },
          nullable: false,
          attributes: {},
        },
        {
          name: 'role',
          columnName: 'role',
          type: { kind: 'enum', enum: 'Role' },
          nullable: false,
          attributes: {},
        },
      ],
      uniques: [],
      indexes: [],
    },
  },
  enums: {
    Role: { name: 'Role', values: ['user', 'admin'] },
  },
  relations: [],
  metadata: META,
};

/** Single-table variant inheritance. */
export const withInheritance: LandoSchema = {
  formatVersion: FORMAT_VERSION,
  models: {
    Account: {
      name: 'Account',
      tableName: 'accounts',
      primaryKey: { fields: ['id'] },
      fields: [
        {
          name: 'id',
          columnName: 'id',
          type: { kind: 'scalar', scalar: 'uuid' },
          nullable: false,
          attributes: {},
        },
        {
          name: 'kind',
          columnName: 'kind',
          type: { kind: 'scalar', scalar: 'string' },
          nullable: false,
          attributes: {},
        },
        {
          name: 'displayName',
          columnName: 'display_name',
          type: { kind: 'scalar', scalar: 'string' },
          nullable: true,
          attributes: {},
        },
        {
          name: 'permissions',
          columnName: 'permissions',
          type: { kind: 'scalar', scalar: 'json' },
          nullable: true,
          attributes: {},
        },
      ],
      uniques: [],
      indexes: [],
      polymorphism: {
        kind: 'inheritance',
        discriminator: 'kind',
        strategy: 'single-table',
        variants: {
          user: { include: ['displayName'] },
          admin: { include: ['permissions'] },
        },
      },
    },
  },
  enums: {},
  relations: [],
  metadata: META,
};

/** Polymorphic association — exclusive arc. */
export const withAssociation: LandoSchema = {
  formatVersion: FORMAT_VERSION,
  models: {
    Comment: {
      name: 'Comment',
      tableName: 'comments',
      primaryKey: { fields: ['id'] },
      fields: [
        {
          name: 'id',
          columnName: 'id',
          type: { kind: 'scalar', scalar: 'uuid' },
          nullable: false,
          attributes: {},
        },
        {
          name: 'body',
          columnName: 'body',
          type: { kind: 'scalar', scalar: 'string' },
          nullable: false,
          attributes: {},
        },
        {
          name: 'postId',
          columnName: 'post_id',
          type: { kind: 'scalar', scalar: 'uuid' },
          nullable: true,
          attributes: {},
        },
        {
          name: 'photoId',
          columnName: 'photo_id',
          type: { kind: 'scalar', scalar: 'uuid' },
          nullable: true,
          attributes: {},
        },
      ],
      uniques: [],
      indexes: [],
      polymorphism: {
        kind: 'association',
        target: 'target',
        variants: {
          post: { column: 'post_id', references: 'Post' },
          photo: { column: 'photo_id', references: 'Photo' },
        },
      },
    },
    Post: {
      name: 'Post',
      tableName: 'posts',
      primaryKey: { fields: ['id'] },
      fields: [
        {
          name: 'id',
          columnName: 'id',
          type: { kind: 'scalar', scalar: 'uuid' },
          nullable: false,
          attributes: {},
        },
      ],
      uniques: [],
      indexes: [],
    },
    Photo: {
      name: 'Photo',
      tableName: 'photos',
      primaryKey: { fields: ['id'] },
      fields: [
        {
          name: 'id',
          columnName: 'id',
          type: { kind: 'scalar', scalar: 'uuid' },
          nullable: false,
          attributes: {},
        },
      ],
      uniques: [],
      indexes: [],
    },
  },
  enums: {},
  relations: [],
  metadata: META,
};

export const all = { simple, withEnum, withInheritance, withAssociation } as const;
