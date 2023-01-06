-- Your SQL goes here
DROP TABLE IF EXISTS transactions; 
CREATE TABLE transactions( 
  hash VARCHAR NOT NULL,
  value numeric NOT NULL,
  position int NOT NULL,
  sender VARCHAR NOT NULL,
  receiver VARCHAR NOT NULL,
  input VARCHAR NOT NULL,
  block_number BigInt NOT NULL,
  PRIMARY KEY(hash, block_number)
)