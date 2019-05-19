use arcsecond::client::Client;
use arcsecond::endpoint::activities;

#[test]
fn test_get_listing() {
    println!("{:?}", activities::get_listing(&Client::new()));
    assert!(true);
}
