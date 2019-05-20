use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Catalogue {
    pub name: String,
    pub secondary_name: String,
    pub pk: u32,
    pub url: String,
    pub source_url: String,
    pub description: String,
    pub rows: String,
}

pub mod activity {
    use super::Deserialize;

    /// Activities are the records the observing activities around the world.
    /// They intend to gather in a single object an observing activity in a given
    /// observing site, with a given telescope, a given instrument by a given
    /// observer, or collaboration or organisation.
    #[derive(Deserialize, Debug)]
    pub struct Activity {
        pub id: u32,
        pub title: String,
        pub content: String,
        pub date: String,
        pub links: Vec<String>,
        pub observing_site: Option<String>,
        pub telescope: Option<String>,
        pub instrument: String,
        pub target_name: String,
        pub profile: String,
        pub collaboration: Option<String>,
        pub organisation: Option<String>,
        pub creation_date: String,
        pub coordinates: Option<Coordinates>,
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
        pub epoch: f64,
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
}
