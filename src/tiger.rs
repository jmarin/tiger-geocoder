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
    pub srid: i32,
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
        ST_SRID(g.geomout) as srid, (addy).address As street_number, (addy).streetname As street,
(addy).streettypeabbrev As street_type, (addy).location As
city, (addy).stateabbrev As st,(addy).zip
FROM geocode('{}') As g;",
        address
    );

    let mut geocoded_addresses: Vec<GeocodedAddress> = vec![];

    for row in connection.query(&sql, &[]).unwrap().iter() {
        let rating: Option<i32> = row.get(0);
        let longitude: Option<f64> = row.get(1);
        let latitude: Option<f64> = row.get(2);
        let srid: Option<i32> = row.get(3);
        let street_number: Option<i32> = row.get(4);
        let street: Option<String> = row.get(5);
        let street_type: Option<String> = row.get(6);
        let city: Option<String> = row.get(7);
        let state: Option<String> = row.get(8);
        let zip_code: Option<String> = row.get(9);

        let geocoded = GeocodedAddress {
            rating: rating.unwrap_or_default(),
            lon: longitude.unwrap_or_default(),
            lat: latitude.unwrap_or_default(),
            srid: srid.unwrap_or_default(),
            street_number: street_number.unwrap_or_default(),
            street: street.unwrap_or_default(),
            street_type: street_type.unwrap_or_default(),
            city: city.unwrap_or_default(),
            state: state.unwrap_or_default(),
            zip_code: zip_code.unwrap_or_default(),
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
