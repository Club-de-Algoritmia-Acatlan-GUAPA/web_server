-- Add migration script here
ALTER TABLE problem
ALTER COLUMN testcases TYPE text[];
