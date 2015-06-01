extern crate hyper;
extern crate rustc_serialize;

mod errors;

use hyper::Url;
use hyper::client::Client;
use hyper::client::response::Response;
use hyper::error::Error;
use std::collections::HashMap;
use std::io::Read;
use rustc_serialize::json;
use rustc_serialize::{Decoder, Decodable};
use errors::CloudFlareErrors;

const DO_URL : &'static str = "https://www.cloudflare.com/api_json.html";

#[derive(Debug,Eq,PartialEq)]
pub enum Actions {
    AllRecords
}

#[derive(Debug,Eq,PartialEq,Clone)]
pub struct Authentication {
    pub email: String,
    pub token: String,
    pub domain: Option<String>
}

#[derive(Debug,PartialEq,Eq)]
pub struct Record {
    rec_id: String,
    zone_name: String,
    display_name: String,
    name: String,
    content: String,
    record_type: String,
    prio: Option<String>,
    ttl: String,
}

#[derive(Debug,PartialEq,Eq,RustcDecodable)]
struct CloudFlareResponse {
    response: CloudFlareResponseRecs
}

#[derive(Debug,PartialEq,Eq,RustcDecodable)]
struct CloudFlareResponseRecs {
    recs: CloudFlareResponseRecords
}

#[derive(Debug,PartialEq,Eq,RustcDecodable)]
struct CloudFlareResponseRecords {
    objs: Vec<Record>
}

impl Decodable for Record {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        d.read_struct("root", 0, |d| {
            Ok(Record {
                rec_id: try!(d.read_struct_field("rec_id", 0, Decodable::decode)),
                zone_name: try!(d.read_struct_field("zone_name", 0, Decodable::decode)),
                display_name: try!(d.read_struct_field("display_name", 0, Decodable::decode)),
                name: try!(d.read_struct_field("name", 0, Decodable::decode)),
                content: try!(d.read_struct_field("content", 0, Decodable::decode)),
                record_type: try!(d.read_struct_field("type", 0, Decodable::decode)),
                prio: try!(d.read_struct_field("prio", 0, Decodable::decode)),
                ttl: try!(d.read_struct_field("ttl", 0, Decodable::decode)),
            })
        })
    }
}

impl<'a> From<&'a Authentication> for HashMap<&'a str, &'a str> {
    fn from(auth : &'a Authentication) -> HashMap<&'a str, &'a str> {
        let mut hash : HashMap<&'a str, &'a str> = HashMap::new();
        hash.insert("email", &auth.email);
        hash.insert("tkn", &auth.token);

        if auth.domain.is_some() {
            let domain = auth.domain.as_ref().unwrap();
            hash.insert("z", domain);
        }

        hash
    }
}

fn payload_for_action<'a>(auth : &'a Authentication, action: Actions) -> HashMap<&'a str, &'a str> {
    let mut auth_info = HashMap::<&str,&str>::from(auth);
    let action = match action {
        Actions::AllRecords => "rec_load_all"
    };

    auth_info.insert("a", action);
    auth_info
}

pub fn list_records(client: &mut Client, auth : &Authentication) -> Result<Vec<Record>, CloudFlareErrors> {
    let payload = payload_for_action(auth, Actions::AllRecords);
    let mut url = Url::parse(DO_URL).unwrap();
    url.set_query_from_pairs(payload.into_iter());

    let read_body = |mut response : Response| -> Result<String, Error> {
        let mut body = String::new();
        response.read_to_string(&mut body).map(|_| body).map_err(Error::from)
    };

    let parse_body = |body: String| -> Result<Vec<Record>, CloudFlareErrors> {
        let parsed = try!(json::decode::<CloudFlareResponse>(&body));
        Ok(parsed.response.recs.objs)
    };


    let body = try!(client.get(url).send().map(read_body));
    try!(body.map(parse_body))
}

#[test]
fn it_converts_authentication_into_request_params() {
    let auth  = Authentication {
        email: "email@example.com".to_owned(),
        token: "token".to_owned(),
        domain: None
    };

    let auth_info = HashMap::<_,_>::from(&auth);
    let expected_info : HashMap<_,_> = vec![("tkn", "token"), ("email", "email@example.com")].into_iter().collect();
    assert_eq!(auth_info, expected_info);
}

#[test]
fn it_converts_authentication_into_request_params_including_a_domain() {
    let auth  = Authentication {
        email: "email@example.com".to_owned(),
        token: "token".to_owned(),
        domain: Some("example.com".to_owned())
    };

    let auth_info = HashMap::<_,_>::from(&auth);
    let expected_info : HashMap<_,_> = vec![("tkn", "token"), ("email", "email@example.com"), ("z", "example.com")].into_iter().collect();
    assert_eq!(auth_info, expected_info);
}


#[test]
fn it_parses_records_from_json() {
    let expected = Record {
        rec_id: "1234".to_owned(),
        zone_name: "example.com".to_owned(),
        display_name: "www".to_owned(),
        name: "www".to_owned(),
        content: "example.com".to_owned(),
        record_type: "CNAME".to_owned(),
        prio: None,
        ttl: "1".to_owned(),
    };

    let response = r#"{
            "rec_id": "1234",
            "zone_name": "example.com",
            "display_name": "www",
            "name": "www",
            "content": "example.com",
            "type": "CNAME",
            "prio": null,
            "ttl": "1"
        }"#;

    let result = json::decode::<Record>(response).unwrap();
    assert_eq!(result, expected);
}
