-- Your SQL goes here
-- NOTE :

-- When preparing your app for use in production, you may want to run your migrations during the application’s initialization phase. You may also want to include the migration scripts as a part of your code, to avoid having to copy them to your deployment location/image etc.
-- The diesel_migrations crate provides the embed_migrations! macro, allowing you to embed migration scripts in the final binary. Once your code uses it, you can simply include connection.run_pending_migrations(MIGRATIONS) at the start of your main function to run migrations every time the application starts.

CREATE TABLE Users (
  id uuid PRIMARY KEY,
  email VARCHAR NOT NULL UNIQUE,
  password VARCHAR NOT NULL,
  is_validated BOOLEAN NOT NULL DEFAULT FALSE,
  creation_date timestamptz NOT NULL,
  username VARCHAR NOT NULL,
  user_role INTEGER NOT NULL
)