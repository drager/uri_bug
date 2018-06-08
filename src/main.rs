extern crate hyper;

fn main() {
    let uri = "https://graph.facebook.com/v3.0/?fields=page{id,name}";
    let parsed_uri = uri.parse::<hyper::Uri>();
    assert!(parsed_uri.is_ok());
}
