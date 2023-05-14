use std::{path::Path, time::Duration, error};

use notify::{Watcher, RecommendedWatcher};
use serenity::futures::future::ok;
mod cat_names_reader;
fn main() {
    //println!("Hello, world!");

    let mut name_reader = cat_names_reader::CatNamesReader::new();

    let data = name_reader.read().unwrap();
    

    loop {
        std::thread::sleep(Duration::from_millis(1000));
    }
    
}

