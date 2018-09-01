extern crate dotenv;

use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    let db_url = env::var("POSTGRES_URL");
    match db_url {
        Ok(url) => println!("{:#?}", url),
        Err(_) => println!("Please provide connection in POSTGRES_URL environment variable"),
    }
}
