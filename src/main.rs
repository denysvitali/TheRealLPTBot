extern crate curl;
extern crate yaml_rust;

use std::fs::File;
use std::io::prelude::*;

use std::path::Path;

use std::io::{stdout, Write, Result};
use std::process;
use curl::easy::Easy;
use yaml_rust::{YamlLoader, YamlEmitter};

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

    println!("Username: {}", username);
    println!("Password: {}", password);


    //let url = "https://google.com/";
    //let mut easy = Easy::new();
    //easy.url(url).unwrap();
    //easy.write_function(|data| {
    //    Ok(stdout().write(data).unwrap())
    //}).unwrap();
    //easy.perform().unwrap();
}
