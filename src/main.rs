#[macro_use]
extern crate structopt;
extern crate dotenv;

use dotenv::dotenv;
use std::env;
use structopt::StructOpt;

mod tiger;
use tiger::get_connection;
use tiger::GeocodeError;

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
            let connection = get_connection(url);
            match connection {
                Ok(conn) => println!("Connection successful!"),
                Err(e) => println!("{:#?}", e),
            }
            println!("{:#?}", address);
        }
        Err(_) => println!("{:#?}", GeocodeError),
    }
}
