extern crate hyper;
extern crate rustc_serialize;
extern crate chrono;
extern crate url;

use hyper::Client;
use hyper::header::ContentType;

use chrono::UTC;

use std::io::Read;
use std::fs::File;

use rustc_serialize::json::{self, Json};

use url::percent_encoding::{utf8_percent_encode, PATH_SEGMENT_ENCODE_SET};

#[derive(RustcEncodable, RustcDecodable, Debug)]
pub struct ConfigStruct {
    alldocs_url: String,
    view_url: String,
    update_url: String,
    date_prop: String
}

fn get_config() -> ConfigStruct {
    let mut configfile = File::open("config.json").unwrap();
    let mut data = String::new();
    configfile.read_to_string(&mut data).unwrap();
    let result: ConfigStruct = json::decode(&data).unwrap();
    return result;
}

fn get_latest_post(url: String) -> json::Object {
    let client = Client::new();
    let mut res = client.get(&url).send().unwrap();

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    let decoded = Json::from_str(&body).unwrap();

    let result = decoded
        .as_object().unwrap()
        .get("rows").unwrap()
        [0].as_object().unwrap()
        .get("doc").unwrap()
        .as_object().unwrap();

    result.clone()
}

fn get_latest_view_id(url: String) -> String {
    let client = Client::new();
    let mut res = client.get(&url).send().unwrap();

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    let decoded = Json::from_str(&body).unwrap();

    let result = decoded
        .as_object().unwrap()
        .get("rows").unwrap()
        [0].as_object().unwrap()
        .get("id").unwrap()
        .as_string().unwrap();

    result.to_string()
}

fn bump_post(update_url: String, id: &str, post: &json::Object) -> json::Object {
    let url = format!("{}{}", update_url, utf8_percent_encode(id, PATH_SEGMENT_ENCODE_SET));

    let client = Client::new();

    let doc = json::encode(&post).unwrap();
    let mut res = client.put(&url)
        .header(ContentType::json())
        .body(&doc)
        .send().unwrap();

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    let decoded = Json::from_str(&body).unwrap();

    decoded.as_object().unwrap().clone()
}

fn main() {
    let config = get_config();

    println!("Checking for update...");
    let post = get_latest_post(config.alldocs_url);

    let id = post.get("_id").unwrap().as_string().unwrap();
    let viewid = get_latest_view_id(config.view_url);

    let created = post.get(&config.date_prop).unwrap().as_string().unwrap();
    let now = UTC::now().to_rfc3339();

    println!("Latest: {}, in view: {}, in past: {}", id, viewid == id, created < &now);
    if viewid != id && created < &now {
        println!("Bumping...");
        let result = bump_post(config.update_url, id, &post);
        println!("Bump finished! new rev {}", result.get("rev").unwrap());
    }

}