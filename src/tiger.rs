extern crate diesel;
extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;

use self::postgres::Error;
use self::postgres::{Connection, TlsMode};
use std::error;
use std::fmt;

#[derive(Debug)]
pub struct GeocodedAddress {
    pub rating: i32,
    pub lon: f64,
    pub lat: f64,
    pub street_number: i32,
    pub street: String,
    pub street_type: String,
    pub city: String,
    pub state: String,
    pub zip_code: String,
}

#[derive(Debug)]
pub struct GeocodeError;

impl fmt::Display for GeocodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Geocode Error")
    }
}

impl error::Error for GeocodeError {
    fn description(&self) -> &str {
        "A geocoding error has occurred"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

pub fn geocode(
    connection: &Connection,
    address: String,
) -> Result<Vec<GeocodedAddress>, GeocodeError> {
    let sql = format!(
        "SELECT g.rating, ST_X(g.geomout) As lon, ST_Y(g.geomout) As lat,
(addy).address As street_number, (addy).streetname As street,
(addy).streettypeabbrev As street_type, (addy).location As
city, (addy).stateabbrev As st,(addy).zip
FROM geocode('{}') As g;",
        address
    );

    let mut geocoded_addresses: Vec<GeocodedAddress> = vec![];

    for row in connection.query(&sql, &[]).unwrap().iter() {
        let geocoded = GeocodedAddress {
            rating: row.get(0),
            lon: row.get(1),
            lat: row.get(2),
            street_number: row.get(3),
            street: row.get(4),
            street_type: row.get(5),
            city: row.get(6),
            state: row.get(7),
            zip_code: row.get(8),
        };
        geocoded_addresses.push(geocoded);
    }

    match geocoded_addresses.len() {
        0 => Err(GeocodeError),
        _ => Ok(geocoded_addresses),
    }
}

pub fn get_connection(url: String) -> Result<Connection, Error> {
    Connection::connect(url, TlsMode::None)
}
