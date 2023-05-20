use std::{ sync::{Arc, Mutex, mpsc::Receiver}, path::{Path, PathBuf}, collections::HashMap, ops::DerefMut, borrow::BorrowMut, error::Error};

use notify::{RecommendedWatcher, Watcher, RecursiveMode};

pub struct CatNamesReader{
    file_update: Option<Receiver<()>>,
    path: PathBuf,
    catNameAndPhotoUrl: Option<HashMap<String,String>>
}



impl Default for CatNamesReader {
    fn default() -> Self {
        CatNamesReader { 
            file_update: None,
            path: Path::new("./watchable.txt").to_path_buf(),
            catNameAndPhotoUrl: None
        }
    }
}

#[derive(Debug)]
pub enum MonitorStartError {
    PathError(notify::Error),
    FileWatcherCreationError(notify::Error),
    FileNotLoaded(ReadError)
}

#[derive(Debug)]
pub enum ReadError {
    FailedFileInitialLoad(String),
    FailedFileLoad(DeserializationError, HashMap<String,String>)//the hashmap's currently loaded value is given to fallback to
}

#[derive(Debug)]
pub enum DeserializationError {
    FailedToDeserialize(serde_json::Error),
    FileAccessError(std::io::Error)
}


impl CatNamesReader{
    pub fn new() ->Result<CatNamesReader,MonitorStartError>{
        let mut reader = CatNamesReader::default();
        reader.begin_monitor()?;
        reader.load().map_err(MonitorStartError::FileNotLoaded)?;
        
        Ok(reader)
    }


    fn begin_monitor(&mut self) -> Result<(), MonitorStartError>{
        // = CatNamesReader::default();
        let (sender,receiver) = std::sync::mpsc::channel();
        self.file_update = Some(receiver);

        //setup the file watcher
        let mut watcher = RecommendedWatcher::new(
            //Event handler
            move |result: Result<notify::Event, notify::Error>|{
                println!("event occoured!");

                let event = match result {
                    Err(e) => {
                        eprintln!("Error fetching event information! \n{:?}",e);
                        return;
                    },

                    Ok(event) => event
                };
                
                {//update global value
                    // let mut v = match lock.lock(){
                    //     Ok(v) =>{
                    //         println!("updating require_file_read");
                    //         v
                    //     },
                    //     Err(e)=>{
                    //         eprintln!("mutex was poisoned!");
                    //         e.into_inner()
                    //     }
                    // };
                    // *v = true;

                    match sender.send(()){
                        Ok(())=>{},
                        Err(e)=>{
                            println!("failed to send:\n{:?}",e);
                        }
                    }
                }
            },

            //setup config
            notify::Config::default()
        )

        //give the error a nice name and pass it on if it happened
        .map_err(MonitorStartError::FileWatcherCreationError)?;

        watcher.watch(self.path.as_path(), RecursiveMode::NonRecursive)
            .map_err(MonitorStartError::PathError)? ;
        
        Ok(())
    }

    //read the file in memory or if neccesary hot loading it.
    fn load(&mut self) ->  Result<HashMap<String,String>,ReadError> {
        let file_read_required = {
            let result =  self.file_update.as_mut().unwrap().try_recv();
            match result {
                Ok(_) =>{true},
                Err(e)=>{println!("recieved failed:\n{:?}",e) ;false}
            }
        };


        if file_read_required | self.catNameAndPhotoUrl.is_none() {

            let path = self.path.clone();
            let newData = self.deserialize(&path);
            match newData {
                Ok(v) => {
                


                    self.catNameAndPhotoUrl = Some(v);
                    return Ok(self.catNameAndPhotoUrl.clone().unwrap());
                },

                Err(e) =>{
                    if self.catNameAndPhotoUrl.is_none(){
                        return Err(ReadError::FailedFileInitialLoad(
                            format!("failed to read from file on first try. Aborting.\n{:?}",e)
                            )
                        );
                    }
                    else {
                        return Err(ReadError::FailedFileLoad(e,self.catNameAndPhotoUrl.clone().unwrap()));
                    }
                }
            }
        }
        
        Ok(self.catNameAndPhotoUrl.clone().expect("expected catNameAndPhotoUrl to already be initialized!"))

    }

    /// Panics if file failed to load for first time. This should not be possible
    pub fn read(&mut self) -> HashMap<String,String>{
        match  self.load(){
            Ok(v) =>{v},
            Err(e)=>{
                match e {
                    ReadError::FailedFileInitialLoad(e) =>{
                        panic!("failed to load file for first time because{}",e);
                    },
                    ReadError::FailedFileLoad(e2, hm) =>{
                        eprintln!("Failed to hotreload from file! Falling back to last load value\n {:?}",e2);
                        hm
                    }
                }
            }
        }
    }
            
    fn deserialize(&mut self,filePath: &PathBuf) -> Result<HashMap<String,String>, DeserializationError>
    {
        print!("deserializing...");
        let fileContents =  std::fs::read_to_string(filePath)
            .map_err(DeserializationError::FileAccessError)?;
        let fileContents = {
            if fileContents.is_empty(){
                let mut hashmap:HashMap<String, String, _> = HashMap::default();
                hashmap.insert("key_0".into(), "value_0".into());
                
                let output = serde_json::to_string_pretty::<HashMap<String,String>>(&hashmap)
                    .map_err(DeserializationError::FailedToDeserialize)?;

                std::fs::write(filePath, output.clone())
                    .map_err(DeserializationError::FileAccessError)?;

                output

            }
            else {
                fileContents
            }
        };



        let catData: Result<HashMap<String, String>, serde_json::Error> = serde_json::from_str::<HashMap<String,String>>(fileContents.as_str());
        let output = catData.map_err(DeserializationError::FailedToDeserialize)?;



     
        return Ok(output);
    }
    

}

