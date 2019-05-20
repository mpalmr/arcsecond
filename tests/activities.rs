use arcsecond::Client;

#[test]
#[allow(clippy::assertions_on_constants)]
fn test_get_listing() {
    match Client::default().get_activity_listing() {
        Ok(response) => assert_ne!(response.len(), 0),
        _ => assert!(false, "did not get response"),
    };
}

#[test]
#[allow(clippy::assertions_on_constants)]
fn get_by_id() {
    match Client::default().get_activity_by_id(5589) {
        Ok(activity) => {
            assert_eq!(activity.id, 5589);
            assert_eq!(activity.title, "New ESO Observation Set");
        },
        _ => assert!(false, "did not get response"),
    };
}
