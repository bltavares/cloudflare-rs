extern crate cloudflare;
extern crate hyper;

use hyper::client::Client;
use cloudflare::Authentication;
use std::env;

fn auth_with_domain() -> Authentication {
    let email = env::var_os("CLOUDFLARE_EMAIL")
                    .expect("CLOUDFLARE_EMAIL is not set")
                    .to_str()
                    .unwrap()
                    .to_owned();
    let token = env::var_os("CLOUDFLARE_TOKEN")
                    .expect("CLOUDFLARE_TOKEN is not set")
                    .to_str()
                    .unwrap()
                    .to_owned();
    let domain = env::var_os("CLOUDFLARE_DOMAIN")
                     .expect("CLOUDFLARE_DOMAIN is not set")
                     .to_str()
                     .unwrap()
                     .to_owned();

    Authentication {
        email: email,
        token: token,
        domain: Some(domain),
    }
}

#[test]
fn test_retrive_records() {
    let mut client = Client::new();
    let auth = auth_with_domain();
    let response = cloudflare::list_records(&mut client, &auth).unwrap();
    println!("{:?}", response);
    assert!(true);
}
