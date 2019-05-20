use crate::client::Client;
use reqwest::Error;
use serde::Deserialize;

pub fn get_listing(client: &Client) -> Result<Vec<Activity>, Error> {
    println!("{}", client.request("/activities/")?.text()?);
    Ok(client.request("/activities/")?.json()?)
}

pub fn get_listing_by_id(client: &Client, id: u32) -> Result<Activity, Error> {
    Ok(client.request(&format!("/activities/{}", id))?.json()?)
}

/// Activities ordered by inverse creation date.
#[derive(Deserialize, Debug)]
pub struct Activity {
    pub id: u32,
    pub title: String,
    pub content: String,
    pub date: String,
    pub links: Vec<String>,
    pub observing_site: String,
    pub telescope: String,
    pub instrument: String,
    pub target_name: String,
    pub profile: String,
    pub collaboration: Option<String>,
    pub organisation: Option<String>,
    pub creation_date: String,
    pub coordinates: Coordinates,
    pub programme: Programme,
}

#[derive(Deserialize, Debug)]
pub enum CoordinatesSystem {
    ICRS,
    FK5,
    FK4,
    FK4NoETerms,
    Galactic,
    AltAz,
}

#[derive(Deserialize, Debug)]
pub struct Coordinates {
    pub system: CoordinatesSystem,
    pub right_ascension: f64,
    pub right_ascension_units: String,
    pub declination: f64,
    pub declination_units: String,
    pub epoch: u128,
}

#[derive(Deserialize, Debug)]
pub struct Programme {
    pub id: u32,
    pub programme_id: String,
    pub period: String,
    pub observing_mode: String, // enum
    pub programme_type: String, // enum
    pub allocated_time: String,
    pub telescope_name: String,
    pub instrument_name: String,
    pub investigators_list: String,
    pub programme_title: String,
    pub remarks: String,
    pub abstract_url: String,
    pub r#abstract: String,
    pub observer_name: String,
    pub raw_files_url: String,
    pub publications_url: String,
}
