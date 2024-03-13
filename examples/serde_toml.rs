#![allow(unused_imports)]
use serde::{
    Serialize,
    Deserialize,
};

use dotenvy;
use env2toml::env2toml;

#[allow(dead_code)]
#[derive(Debug,Deserialize)]
struct Config {
   title: String,
   owner: Option<Owner>,
   database: Database,
   servers: Servers,
}

#[allow(dead_code)]
#[derive(Debug,Deserialize)]
struct Owner {
   name: String,
}

#[allow(dead_code)]
#[derive(Debug,Deserialize)]
struct Database {
    enabled: bool,
    ports: Vec<u16>,
}

#[allow(dead_code)]
#[derive(Debug,Deserialize)]
struct Servers {
   alpha: Server,
   beta: Server,
}

#[allow(dead_code)]
#[derive(Debug,Deserialize)]
struct Server {
    role: String,
    ip: String,
}


fn main() {
    dotenvy::dotenv().ok();
    let result = env2toml("APP_").unwrap();
    println!("{}\n", result);
    
    let config: Config = toml::from_str(&result).unwrap();
    println!("{:?}", config);
}