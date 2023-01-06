-- Your SQL goes here
create index sender_index on transactions(sender);
create index receiver_index on transactions(receiver);
create index hash_index on transactions(hash);