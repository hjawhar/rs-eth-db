-- Your SQL goes here
CREATE TABLE transactions( 
  hash VARCHAR NOT NULL,
  value numeric NOT NULL,
  position int NOT NULL,
  sender VARCHAR NOT NULL,
  receiver VARCHAR NOT NULL,
  input VARCHAR NOT NULL,
  block_number BigInt NOT NULL,
  PRIMARY KEY(hash, block_number)
);

create index sender_index on transactions(sender);
create index receiver_index on transactions(receiver);
create index hash_index on transactions(hash);