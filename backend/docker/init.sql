CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY,
    attempts INTEGER NOT NULL DEFAULT 0,
    word TEXT NOT NULL
);
