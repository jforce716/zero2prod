-- Add up migration script here
-- Create subscriptions table
CREATE TABLE IF NOT EXISTS subscription(
    id uuid PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    subscribed_at timestamptz NOT NULL 
);