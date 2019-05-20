use arcsecond::client::Client;
use arcsecond::endpoint::activities;

#[test]
fn test_get_listing() {
    let activity_listing = activities::get_listing(&Client::default());
    println!("\n{:?}", activity_listing);
    assert!(false);
}
