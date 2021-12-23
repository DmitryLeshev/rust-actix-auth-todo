DROP TABLE IF EXISTS "role" cascade;
DROP TABLE IF EXISTS "account" cascade;
DROP TABLE IF EXISTS "account_role" cascade;
DROP TABLE IF EXISTS "todolist" cascade;
DROP TABLE IF EXISTS "todoitem" cascade;

-- CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
-- uuid DEFAULT uuid_generate_v4()


CREATE TABLE "role" (
    "role_id" bigint generated always as identity,
    "name" varchar NOT NULL
);

ALTER TABLE "role" ADD CONSTRAINT "pk_role" PRIMARY KEY ("role_id");

CREATE TABLE "account" (
    "account_id" bigint generated always as identity,
    "email" varchar(64) NOT NULL UNIQUE,
    "first_name" varchar(64) NOT NULL,
    "last_name" varchar(64) NOT NULL,
    "hash_password" varchar NOT NULL,
    "created_at" timestamp NOT NULL DEFAULT current_timestamp,
    "updated_at" timestamp NOT NULL DEFAULT current_timestamp
);

ALTER TABLE "account" ADD CONSTRAINT "pk_account" PRIMARY KEY ("account_id");

CREATE TABLE "account_role" (
    "account_id" bigint NOT NULL,
    "role_id" bigint NOT NULL,
    "created_at" timestamp NOT NULL DEFAULT current_timestamp,
    "updated_at" timestamp NOT NULL DEFAULT current_timestamp
);

ALTER TABLE "account_role" ADD CONSTRAINT "pk_account_role" PRIMARY KEY ("account_id", "role_id");
ALTER TABLE "account_role" ADD CONSTRAINT "fk_account_role_account" FOREIGN KEY ("account_id") REFERENCES "account" ("account_id") ON DELETE CASCADE;
ALTER TABLE "account_role" ADD CONSTRAINT "fk_account_role_role" FOREIGN KEY ("role_id") REFERENCES "role" ("role_id") ON DELETE CASCADE;


CREATE TABLE "todolist" (
    "todolist_id" bigint generated always as identity,
    "account_id" bigint NOT NULL,
    "name" varchar(64) NOT NULL,
    "created_at" timestamp NOT NULL DEFAULT current_timestamp,
    "updated_at" timestamp NOT NULL DEFAULT current_timestamp
);

ALTER TABLE "todolist" ADD CONSTRAINT "pk_todolist" PRIMARY KEY ("todolist_id");
ALTER TABLE "todolist" ADD CONSTRAINT "fk_todolist_account" FOREIGN KEY ("account_id") REFERENCES "account" ("account_id") ON DELETE CASCADE;

CREATE TABLE "todoitem" (
    "todoitem_id" bigint generated always as identity,
    "todolist_id" bigint NOT NULL,
    "name" varchar(64) NOT NULL,
    "description" text,
    "active" boolean DEFAULT true,
    "completed" boolean DEFAULT false,
    "deleted" boolean DEFAULT false,
    "created_at" timestamp NOT NULL DEFAULT current_timestamp,
    "updated_at" timestamp NOT NULL DEFAULT current_timestamp
);

ALTER TABLE "todoitem" ADD CONSTRAINT "pk_todoitem" PRIMARY KEY ("todoitem_id");
ALTER TABLE "todoitem" ADD CONSTRAINT "fk_todoitem_todolist" FOREIGN KEY ("todolist_id") REFERENCES "todolist" ("todolist_id") ON DELETE CASCADE;

