-- Your SQL goes here
CREATE TABLE transactions( 
  hash VARCHAR PRIMARY KEY,
  value BigInt NOT NULL,
  position int NOT NULL,
  sender VARCHAR NOT NULL,
  receiver VARCHAR,
  input VARCHAR
)