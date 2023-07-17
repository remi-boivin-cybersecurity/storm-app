-- Your SQL goes here
CREATE TABLE "sessions" (
  "id" UUID PRIMARY KEY,
  "created_at" timestamp DEFAULT (CURRENT_TIMESTAMP),
  "updated_at" timestamp DEFAULT (CURRENT_TIMESTAMP)
);
-- Your SQL goes here
