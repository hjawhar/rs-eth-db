-- This file should undo anything in `up.sql`
DROP INDEX IF EXISTS sender_index;
DROP INDEX IF EXISTS receiver_index;
DROP INDEX IF EXISTS hash_index;