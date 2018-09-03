extern crate diesel;
extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;

use self::postgres::Error;
use self::postgres::{Connection, TlsMode};
use std::error;
use std::fmt;

pub struct GeocodedAddress {
    rating: u8,
    lon: f64,
    lat: f64,
    street_number: i32,
    street: String,
    street_type: String,
    city: String,
    state: String,
    zip_code: String,
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

pub fn geocode(address: &str) -> Result<GeocodedAddress, GeocodeError> {
    Ok(GeocodedAddress {
        rating: 1,
        lon: -77.0590732421937,
        lat: 38.9072140041614,
        street_number: 1311,
        street: String::from("30th"),
        street_type: String::from("St"),
        city: String::from("Washington"),
        state: String::from("DC"),
        zip_code: String::from("20007"),
    })
}

pub fn get_connection(url: String) -> Result<Connection, Error> {
    Connection::connect(url, TlsMode::None)
}
