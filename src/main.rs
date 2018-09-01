#[macro_use]
extern crate structopt;
extern crate dotenv;

use dotenv::dotenv;
use std::env;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "address")]
struct Opt {
    address: String,
}

fn main() {
    dotenv().ok();
    let db_url = env::var("POSTGRES_URL");
    match db_url {
        Ok(url) => {
            let opt = Opt::from_args();
            let address = opt.address;
            println!("{:#?}", address);
        }
        Err(_) => println!("Please provide connection in POSTGRES_URL environment variable"),
    }
}
