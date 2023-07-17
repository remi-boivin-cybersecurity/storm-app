-- Your SQL goes here
CREATE TABLE "discussions" (
  "id" UUID PRIMARY KEY,
  "body" text NOT NULL,
  "created_at" timestamp DEFAULT (CURRENT_TIMESTAMP),
  "updated_at" timestamp DEFAULT (CURRENT_TIMESTAMP)
);
