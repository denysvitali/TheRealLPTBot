extern crate curl;

use std::io::{stdout, Write};
use curl::easy::Easy;

fn main() {
    let url = "https://google.com/";
    let mut easy = Easy::new();
    easy.url(url).unwrap();
    easy.write_function(|data| {
        Ok(stdout().write(data).unwrap())
    }).unwrap();
    easy.perform().unwrap();
}
