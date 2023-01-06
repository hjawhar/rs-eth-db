-- Your SQL goes here
create index IF NOT EXISTS sender_index on transactions(sender);
create index IF NOT EXISTS receiver_index on transactions(receiver);
create index IF NOT EXISTS hash_index on transactions(hash);