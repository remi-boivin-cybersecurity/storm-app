-- Your SQL goes here

CREATE TABLE "session_discussions" (
  id UUID PRIMARY KEY,
  session_id UUID references sessions(id) NOT NULL,
  discussion_id UUID references discussions(id) NOT NULL,
  "created_at" timestamp DEFAULT (CURRENT_TIMESTAMP),
  "updated_at" timestamp DEFAULT (CURRENT_TIMESTAMP)
);
