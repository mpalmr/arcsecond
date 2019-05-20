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

#[derive(Deserialize, Debug)]
pub struct Exoplanet {
    pub name: String,
    pub coordinates: Option<Coordinates>,
    pub detection_method: String,
    pub mass_detection_method: String,
    pub radius_detection_method: String,
    pub parent_star: String,
    pub mass: AstronomicalData,
    pub inclination: AstronomicalData,
    pub semi_major_axis: AstronomicalData,
    pub orbital_period: AstronomicalData,
    pub eccentricity: UnitlessAstronomicalData,
    pub omega_angle: AstronomicalData,
    pub anomaly_angle: AstronomicalData,
    pub lambda_angle: AstronomicalData,
    pub time_periastron: UnitlessAstronomicalData,
    pub time_conjonction: UnitlessAstronomicalData,
    pub angular_distance: AstronomicalData,
    pub tzero_primary_transit: UnitlessAstronomicalData,
    pub tzero_secondary_transit: UnitlessAstronomicalData,
    pub impact_parameter: AstronomicalData,
    pub tzero_radial_velocity: UnitlessAstronomicalData,
    pub velocity_semiamplitude: AstronomicalData,
    pub calculated_temperature: AstronomicalData,
    pub measured_temperature: AstronomicalData,
    pub hottest_point_longitude: AstronomicalData,
    pub geometric_albedo: UnitlessAstronomicalData,
    pub surface_gravity: UnitlessAstronomicalData,
}

pub mod activity {
    use super::{Deserialize, Coordinates};

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
pub enum CoordinatesSystem {
    ICRS,
    FK5,
    FK4,
    FK4NoETerms,
    Galactic,
    AltAz,
}

#[derive(Deserialize, Debug)]
pub struct AstronomicalData {
    value: f64,
    unit: String,
    error_max: String,
    error_min: String,
    bibcode: String,
}

#[derive(Deserialize, Debug)]
pub struct UnitlessAstronomicalData {
    value: f64,
    error_max: String,
    error_min: String,
    bibcode: String,
}
