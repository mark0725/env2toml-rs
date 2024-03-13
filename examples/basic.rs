use dotenvy;
use env2toml::env2toml;


fn main() {
    dotenvy::dotenv().ok();
    let result = env2toml("APP_").unwrap();
    println!("{}", result);
}