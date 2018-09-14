#[macro_use]
extern crate structopt;
extern crate dotenv;

#[macro_use]
extern crate prettytable;

use dotenv::dotenv;
use std::env;
use structopt::StructOpt;

use prettytable::Table;

mod tiger;
use tiger::get_connection;
use tiger::GeocodeError;
use tiger::GeocodedAddress;

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
                Ok(conn) => {
                    let geocoded = tiger::geocode(&conn, address).unwrap();
                    display_geocoded(&geocoded);
                }
                Err(e) => println!("{:#?}", e),
            }
        }
        Err(_) => println!("{:#?}", GeocodeError),
    }
}

fn display_geocoded(geocoded: &Vec<GeocodedAddress>) {
    let mut table = Table::new();
    table.add_row(row![
        "rating",
        "lon",
        "lat",
        "srid",
        "street_number",
        "street",
        "street_type",
        "city",
        "state",
        "zip_code"
    ]);
    for g in geocoded {
        table.add_row(row![
            g.rating,
            g.lon,
            g.lat,
            g.srid,
            g.street_number,
            g.street,
            g.street_type,
            g.city,
            g.state,
            g.zip_code
        ]);
    }
    table.printstd();
}
