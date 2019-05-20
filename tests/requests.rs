#![allow(clippy::assertions_on_constants)]

use arcsecond::Client;

#[test]
fn test_get_listing() {
    match Client::default().get_activity_listing() {
        Ok(_) => assert!(true),
        _ => assert!(false, "did not get response"),
    };
}

#[test]
fn get_by_id() {
    match Client::default().get_activity_by_id(5589) {
        Ok(activity) => {
            assert_eq!(activity.id, 5589);
            assert_eq!(activity.title, "New ESO Observation Set");
        }
        _ => assert!(false, "did not get response"),
    };
}

#[test]
fn get_catalogue_listing() {
    match Client::default().get_catalogue_listing() {
        Ok(_) => assert!(true),
        _ => assert!(false, "did not get response"),
    };
}
