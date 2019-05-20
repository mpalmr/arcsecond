use arcsecond::client::Client;
use arcsecond::endpoint::activities;

#[test]
#[allow(clippy::assertions_on_constants)]
fn test_get_listing() {
    match activities::get_listing(&Client::default()) {
        Ok(response) => assert_ne!(response.len(), 0),
        _ => assert!(false, "did not get response"),
    };
}

#[test]
#[allow(clippy::assertions_on_constants)]
fn get_by_id() {
    match activities::get_by_id(&Client::default(), 5589) {
        Ok(activity) => {
            assert_eq!(activity.id, 5589);
            assert_eq!(activity.title, "New ESO Observation Set");
        },
        _ => assert!(false, "did not get response"),
    };
}
