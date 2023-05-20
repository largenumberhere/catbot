use std::{fs, path::PathBuf, time::Duration};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::io::{Read, Write};
use std::path::Path;
use std::time::UNIX_EPOCH;
use blake2::Blake2b512;
use blake2::digest::FixedOutput;
use tokio::io::AsyncReadExt;


mod cat_reader3;
#[tokio::main]
async fn main() {
    // cat_reader3::cat_reader();
    // //println!("Hello, world!");
    // // let reader = cat_names_reader::CatNamesReader::new();
    // // let mut reader = reader.unwrap();
    //
    // // loop {
    // //     let read_result = reader.read();
    // //    println!("file contains: {:?}",read_result);
    //
    // //     thread::sleep(Duration::from_millis(1000));
    // // }
    // let receiver = cat_reader3::bind_monitor(&PathBuf::from("watchable.json"), |r|{
    //     match r {
    //         Ok(v)=>{
    //             match v.kind {
    //                 notify::EventKind::Modify(e) =>{return Some(v);},
    //                 _ =>{None}
    //
    //             }
    //
    //         }
    //         Err(e) =>{return None;}
    //     }
    // });
    //
    // loop {
    //     let update_result = cat_reader3::check_for_update(receiver.clone()).await;
    //     if update_result {
    //         println!("file updated!")
    //     }
    //     tokio::time::sleep(Duration::from_millis(100)).await;
    // }

    //watch a file
        //compute a file's hash
        //every time requested, compute again and compare
        //return if different or equal



    get_hash(PathBuf::from("watchable.json"));
    
    
}

//compute the hash of a file

static HASH:u64 = 0;
async fn get_hash(path: PathBuf){

    //load file into hash
    let hash_result ={
        let mut hasher = Blake2b512::default();
        let file_data = tokio::fs::read(path.as_path()).await.unwrap();
        hasher.write(file_data.as_slice()).unwrap();
        hasher.finalize_fixed()
    };

    println!("r: {hash_result:?}");
    todo!();
}

