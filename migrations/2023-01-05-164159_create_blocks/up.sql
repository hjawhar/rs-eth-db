-- Your SQL goes here-- Your SQL goes here 
DROP TABLE IF EXISTS blocks;
CREATE TABLE blocks ( 
  number BigInt PRIMARY KEY,
  timestamp BigInt NOT NULL,
  hash VARCHAR NOT NULL
)