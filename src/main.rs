use std::{path::PathBuf};

use std::io::{Read, Write};
use blake2::{ Blake2b512};
use blake2::digest::{ Output};

use tokio::io::{AsyncBufReadExt, AsyncReadExt};

mod file_hasher;

#[tokio::main]
async fn main() {

    //compare_hash(PathBuf::from("watchable.json")).await;
    let mut comparer = file_hasher::HashComparer::new(PathBuf::from("watchable.json")).await;
    let result = comparer.compare().await;
    println!("{result:?}");
}



