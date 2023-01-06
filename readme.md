# Rust - ETH DB

## Abstract
The purpose of this project is to be able to sync a full node, populate/seed the database and run queries rather than using WebSocket or HttpProvider/JsonRpcProvider, allowing data manipulation and analysis.


## Prerequisites 
- Install geth & sync a node using snap mode (requires approx. 800 GBs)
- Install Rust
- Install PostgreSQL (or use docker)

## Application
The app will populate/seed your database with blocks & transactions, if you stop the app, it will start from the last synced block.

## Run
```
diesel run migration
cargo run
```


## Screenshots
![Screenshot](https://i.imgur.com/3OVM3EE.png)

