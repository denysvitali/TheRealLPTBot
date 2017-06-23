extern crate curl;
extern crate yaml_rust;
extern crate base64;
extern crate serde_json;

use std::fs::File;
use std::io::prelude::*;

use std::path::Path;

use std::io::{stdout, Write, Result};
use std::process;
use std::str::FromStr;
use curl::easy::{Easy, Form, List};
use yaml_rust::{YamlLoader, YamlEmitter};
use serde_json::Value;

const VERSION: &'static str = "0.1";


fn main() {

    let path = Path::new("credentials.yml");
    let display = path.display();
    if !path.exists() {
        match File::create(path) {
            Err(why) => panic!("Unable to create file {}: {}", display, why),
            _ => (),
        };
    }

    let mut file = match File::open("credentials.yml") {
        Err(why) => panic!("Cannot open credentials.yml: {}", why),
        Ok(file) => file,
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Err(why) => panic!("Cannot read credentials.yml: {}", why),
        Ok(_) => (),
    };

    let yaml = YamlLoader::load_from_str(&contents).unwrap();
    if yaml.len() == 0 {
        println!("Please, add your credentials to {}", display);
        process::exit(1);
    }

    let doc = &yaml[0];
    let username = doc["username"].as_str().expect("Username is not defined in credentials");
    let password = doc["password"].as_str().expect("Password is not defined in credentials");
    let appid = doc["app_id"].as_str().expect("app_id is not defined in credentials");
    let secret = doc["secret"].as_str().expect("Secret is not defined in credentials");

    println!();
    println!("Username:\t {}", username);
    println!("Password:\t {}", password);
    println!("Secret:\t\t {}", secret);
    println!();

    login(&username, &password, &appid, &secret);


    //let url = "https://google.com/";
    //let mut easy = Easy::new();
    //easy.url(url).unwrap();
    //easy.write_function(|data| {
    //    Ok(stdout().write(data).unwrap())
    //}).unwrap();
    //easy.perform().unwrap();
}

fn login(username : &str, password : &str, appid: &str, secret: &str){

    println!("Performing authentication...");
    let data = format!("grant_type=password&username={}&password={}", username, password);
    let mut data = data.as_bytes();
    let auth = format!("Authorization: Basic {}", base64::encode(&format!("{}:{}", appid, secret)));
    let mut resp = Vec::new();


    println!("Length: {}", data.len());

    let mut list = List::new();
    list.append(&auth).unwrap();
    list.append("Content-Type: application/x-www-form-urlencoded").unwrap();

    let mut easy = Easy::new();
    easy.url("https://www.reddit.com/api/v1/access_token").unwrap();
    easy.post(true).unwrap();
    easy.post_field_size(data.len() as u64).unwrap();
    easy.http_headers(list).unwrap();
    {
        let mut transfer = easy.transfer();
        transfer.read_function(|buf| {
            Ok(data.read(buf).unwrap_or(0))
        }).unwrap();
        transfer.write_function(|data| {
            Ok(resp.write(data).unwrap())
        }).unwrap();
        transfer.perform().unwrap();
    }

    let response = String::from_utf8(resp).unwrap();

    println!("Response Code: {:?}", easy.response_code().unwrap());
    println!("Response Body: {}", response);

    let v: Value = serde_json::from_str(&response).expect("Invalid JSON");
    let access_token = v["access_token"].as_str().expect("Access Token not returned. Request failed.");
    println!("Access Token: {}", access_token);

    get_me(access_token);
}

fn get_me(access_token : &str){
    let user_agent: String = format!("TheRealLPTBot ({})", VERSION);
    let mut easy = Easy::new();
    easy.url("https://oauth.reddit.com/api/v1/me").unwrap();

    let mut data : Vec<u8> = Vec::new();
    let mut resp = Vec::new();

    let mut list = List::new();
    list.append(&format!("Authorization: bearer {}", access_token)).unwrap();
    list.append(&format!("User-Agent: {}", user_agent)).unwrap();
    easy.http_headers(list).unwrap();

    {
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {
            Ok(resp.write(data).unwrap())
        }).unwrap();
        transfer.perform().unwrap();
    }

    let response = String::from_utf8(resp).unwrap();
    println!("/v1/me: {}", response);

    let v: Value = serde_json::from_str(&response).expect("Invalid JSON");
    println!("{}", v);
}
