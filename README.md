# CarMaintenace

Application for monitoring and scheduling car maintenace.

It is my first project in Rust. Already plunged myself to deep water.

### Current state: in hold

<hr>

## Dependencies
* chrono = "0.4.30" #simple funcionality for dates
* argon2rs = "0.2.5" #very safe hashing algorithm crate
* hex = "0.4.3" #for converting hash table to String
* rand = "0.8.5" #generate random numbers
* rusqlite = "0.29.0" #sqlite database for rust
* mysql* #not used yet - prod purpose

<hr>

## Catalog structure
-> documentation - documentation folder, containing design assumptions.  
-> maintenance, users, test, vehicles - folders and files containing main project structures and theirs methods.  
-> test - simple automatic tests, ran on crutial functions.  
-> target - Cargo folder.  
-> Cargo.toml - Cargo file, containing dependencies.  
-> car_maintenance.sql - project database, made using MySQL.  


<hr>

## Documentation
Please check the documentation folder

<hr>

## Info and help
* [https://crates.io/]
* [https://doc.rust-lang.org/std/]
* [https://doc.rust-lang.org/book/]

* lib.rs files are made for routing main to individual structures

<hr>

## Special Thanks
Yet to come


