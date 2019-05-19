use crate::client::Client;
use reqwest::Error;
use serde::Deserialize;

pub fn get_listing(client: &Client) -> Result<Activity, Error> {
    Ok(client.request("/activities")?.json()?)
}

/// Activities ordered by inverse creation date.
#[derive(Deserialize)]
pub struct Activity {
    pub id: u32,
    pub title: String,
    pub content: String,
    pub date: String, // date
    pub links: Vec<String>,
    pub observing_site: String,
    pub telescope: String,
    pub instrument: String,
    pub target_name: String,
    pub profile: String, // enum
    // pub collaboration: Some(String), // enum
    // pub organisation: Some(String), // enum
    pub creation_date: String, // date
    pub coordinates: Coordinates,
    pub programme: Programme,
}

#[derive(Deserialize)]
pub struct Coordinates {
    pub system: String, // enum
    pub right_ascension: f64,
    pub right_ascension_units: String, // enum
    pub declination: f64,
    pub declination_units: String, // enum
    pub epoch: u128,
}

#[derive(Deserialize)]
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
