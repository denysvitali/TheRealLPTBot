extern crate curl;
extern crate yaml_rust;
extern crate base64;
extern crate serde_json;
extern crate regex;

use std::fs::File;
use std::io::prelude::*;

use std::path::Path;

use std::io::{stdout, Write, Result};
use std::process;
use std::str::FromStr;
use curl::easy::{Easy, Form, List};
use yaml_rust::{YamlLoader, YamlEmitter};
use serde_json::Value;

use regex::Regex;

const VERSION: &'static str = "0.1";
const DEBUG: bool = false;
const SUBREDDIT_SOURCE: &'static str = "denvit";
const SUBREDDIT_DEST: &'static str = "LifeProTips";


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

    if DEBUG {
        println!();
        println!("Username:\t {}", username);
        println!("Password:\t {}", password);
        println!("Secret:\t\t {}", secret);
        println!();
    }

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

    if DEBUG { println!("Performing authentication..."); }
    let data = format!("grant_type=password&username={}&password={}", username, password);
    let mut data = data.as_bytes();
    let auth = format!("Authorization: Basic {}", base64::encode(&format!("{}:{}", appid, secret)));
    let mut resp = Vec::new();

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

    if DEBUG {
        println!("Response Code: {:?}", easy.response_code().unwrap());
        println!("Response Body: {}", response);
    }

    let v: Value = serde_json::from_str(&response).expect("Invalid JSON");
    let access_token = v["access_token"].as_str().expect("Access Token not returned. Request failed.");

    if DEBUG {
        println!("Access Token: {}", access_token);
    }

    //get_me(access_token);
    //get_lpt();
    get_comments("6ixc7g");
}

fn get_me(access_token : &str){
    let mut easy = Easy::new();
    easy.url("https://oauth.reddit.com/api/v1/me").unwrap();

    let mut resp = Vec::new();

    let mut list = List::new();
    list.append(&format!("Authorization: bearer {}", access_token)).unwrap();
    list.append(&format!("User-Agent: {}", get_ua())).unwrap();
    easy.http_headers(list).unwrap();

    {
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {
            Ok(resp.write(data).unwrap())
        }).unwrap();
        transfer.perform().unwrap();
    }

    let response = String::from_utf8(resp).unwrap();
    //println!("/v1/me: {}", response);

    let v: Value = serde_json::from_str(&response).expect("Invalid JSON");
    println!("Hello {}, comment karma: {}, link karma: {}", v["name"].as_str().unwrap(), v["comment_karma"], v["link_karma"]);
}

fn get_lpt(){
    let mut easy = Easy::new();
    easy.url(&format!("https://www.reddit.com/r/{}/.json", SUBREDDIT_SOURCE)).unwrap();

    let mut resp = Vec::new();

    let mut list = List::new();
    //list.append(&format!("Authorization: bearer {}", access_token)).unwrap();
    list.append(&format!("User-Agent: {}", get_ua())).unwrap();
    easy.http_headers(list).unwrap();

    {
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {
            Ok(resp.write(data).unwrap())
        }).unwrap();
        transfer.perform().unwrap();
    }

    let response = String::from_utf8(resp).unwrap();
    //println!("/v1/me: {}", response);

    let v: Value = serde_json::from_str(&response).expect("Invalid JSON");
    let children = v["data"]["children"].as_array().unwrap();
    for i in children {
        let title = i["data"]["title"].as_str().unwrap();
        let id = i["data"]["id"].as_str().unwrap();
        let re = Regex::new(r"^LPT: ").unwrap();
        if re.is_match(title) {
            // It is a LPT, load comments
            println!("\n====\nID: {}\nTitle: {}\n====\n", id, title);
            let comments = get_comments(id);
        }
    }

    //println!("{:?}", v["data"]);
}

fn get_comments(lpt_id: &str) -> Vec<Value> {
    let rv = Vec::new();

    let mut easy = Easy::new();
    let url = &format!("https://www.reddit.com/r/{}/{}.json", SUBREDDIT_SOURCE, lpt_id);
    easy.url(url).unwrap();
    easy.follow_location(true);
    println!("URL: {}", url);
    let mut resp = Vec::new();

    let mut list = List::new();
    //list.append(&format!("Authorization: bearer {}", access_token)).unwrap();
    list.append(&format!("User-Agent: {}", get_ua())).unwrap();
    easy.http_headers(list).unwrap();

    {
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {
            Ok(resp.write(data).unwrap())
        }).unwrap();
        transfer.perform().unwrap();
    }

    let response = String::from_utf8(resp).unwrap();
    let v: Value = serde_json::from_str(&response).expect("Invalid JSON");
    let children = v[1]["data"]["children"].as_array().unwrap();

    for i in children {
        parse_child(i);
        //println!("=============\n\n");
    }

    rv
}

fn parse_child(parent: &serde_json::Value){
    if !parent.is_object() {
        return;
    }
    if parent["kind"].as_str().unwrap() != "t1" {
        return;
    }
    
    let bodyText = parent["data"]["body"].as_str().unwrap();

    let comment_children = parent["data"]["replies"]["data"]["children"].as_array();
    let re = Regex::new(r"(?i)real lpt is always in the comments").unwrap();
    match comment_children {
        Some(some) => {
            for j in some {
                if j["kind"].as_str().unwrap() != "t1" {
                    continue;
                }
                let childText = j["data"]["body"].as_str().unwrap();
                if re.is_match(childText) {
                    println!("The Real LPT: {}", bodyText);
                }
                else{
                    parse_child(j);
                }
            }
        },
        _ => {},
    };
}

fn get_ua() -> String {
    String::from(format!("TheRealLPTBot ({})", VERSION))
}
