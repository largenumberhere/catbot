use std::{collections::HashMap, path::PathBuf, thread::sleep, time::Duration, sync::Arc};
use serde::{Deserialize};
use notify::{Watcher, Config, RecommendedWatcher, RecursiveMode, Event, Error};
use tokio::sync::{mpsc::{channel, Receiver}, Mutex};


#[derive(Debug)]
#[derive(Deserialize)]
struct CatData{
    data: HashMap<String,String>,
}



pub fn cat_reader(){
    // let file_root = ".";
    // let file_root = PathBuf::from(file_root);
    // let file_root = file_root.canonicalize().unwrap();

    // let (send, receive ) = channel();
    // let mut watcher:RecommendedWatcher = RecommendedWatcher::new(move|res|{
    //     send.send(res).unwrap();
    // },Config::default()).unwrap();
    // watcher.watch(PathBuf::from(".\\watchable.json").as_path(),RecursiveMode::Recursive).unwrap();

    // // while let Ok(v) = receive.recv(){
    // //      match v {
    // //          Ok(v)=>{println!("{:?}",v)} ,
    // //          Err(e)=>{
    // //             println!("{:?}",e);
    // //             break;
    // //         }
    // //      }
    // // }


    // loop {
        
    //     sleep(Duration::from_millis(1000));
    //     let poll_availible: Result<Result<notify::Event, notify::Error>, std::sync::mpsc::TryRecvError>   = receive.try_recv();
    //     if let Ok(v) = poll_availible{
    //         //clear queue and check if file has changed
    //         let mut is_applicable = false;
    //         while let Ok(v) = receive.try_recv() {
    //             match v {
    //                 Ok(v) =>{
    //                     if let notify::EventKind::Modify(_v2) = v.kind{
    //                         is_applicable = true;
    //                     }
    //                 }
    //                 Err(_e)=>{break;}
    //             }
    //         }

    //         //if nothing notable has hapened ignore it
    //         println!("update required: {}",is_applicable);
    //         if !is_applicable {continue;}

    //         //update cache
    //         println!("reload required!");
    //     }

    //     println!("hello!")

    // }

}

pub fn bind_monitor(file: &PathBuf, result_filter: fn(Result<Event,Error>)-> Option<Event> ) -> Arc<Mutex<Receiver<notify::Event>>>{
    let (send, receive) = channel(1);
    let mut watcter = RecommendedWatcher::new(move |event_result|{
        if let Some(useful_event) = result_filter(event_result) {
            send.blocking_send(useful_event).unwrap();
        }
    },
    Config::default());

    watcter.unwrap().watch(file, RecursiveMode::Recursive).unwrap();

    Arc::new(Mutex::new(receive))
}

pub async fn check_for_update(mut reciever: Arc<Mutex<Receiver<notify::Event>>>) -> bool{
    let messages = reciever.lock().await.recv().await.iter().count(); //count how many have passed
    messages > 0
}

