use std::{ sync::{Arc, Mutex}, path::{Path, PathBuf}, collections::HashMap};

use notify::{RecommendedWatcher, Watcher, RecursiveMode};

pub struct CatNamesReader{
    require_file_read: Arc<Mutex<bool>>, 
    path: PathBuf,
    catNameAndPhotoUrl: Option<HashMap<String,String>>
}



impl Default for CatNamesReader {
    fn default() -> Self {
        CatNamesReader { 
            require_file_read: Arc::new(Mutex::new(true)),
            path: Path::new("./watchable.txt").to_path_buf(),
            catNameAndPhotoUrl: None
        }
    }
}

#[derive(Debug)]
pub enum MonitorStartError {
    PathError(notify::Error),
    FileWatcherCreationError(notify::Error)
}

#[derive(Debug)]
pub enum ReadError {
    FailedFileInitialLoad(String),
    FailedFileLoad(String)
}

#[derive(Debug)]
pub enum DeserializationError {
    FailedToDeserialize(serde_json::Error)
}


impl CatNamesReader{
    pub fn new() -> CatNamesReader{
        let mut reader = CatNamesReader::default();
        reader.begin_monitor();
        return  reader;

    }


    fn begin_monitor(&mut self) -> Result<(), MonitorStartError>{
        // = CatNamesReader::default();

        //setup the file watcher
        let mut watcher = RecommendedWatcher::new(
            //Event handler
            move |result: Result<notify::Event, notify::Error>|{
                let event = match result {
                    Err(e) => {
                        eprintln!("Error fetching event information! \n{:?}",e);
                        return;
                    },

                    Ok(event) => event
                };
                
                {//update global value
                    let mut v = match  self.require_file_read.lock(){
                        Ok(v) =>{
                            v
                        },
                        Err(e)=>{
                            eprintln!("mutex was poisoned!");
                            e.into_inner()
                        }
                    };
                    *v = true;
                }
            },

            //setup config
            notify::Config::default()
        )

        //give the error a nice name and pass it on if it happened
        .map_err(MonitorStartError::FileWatcherCreationError)?;

        watcher.watch(&self.path, RecursiveMode::NonRecursive)
            .map_err(MonitorStartError::PathError)? ;
        
        Ok(())
    }

    //read the file in memory or if neccesary hot loading it.
    pub fn read(&mut self) ->  Result<HashMap<String,String>,ReadError> {
        let file_read_required = {
            let v = match  self.require_file_read.lock(){
                Ok(v) =>{
                    v
                },
                Err(e)=>{
                    eprintln!("Mutex was poisoned! Ignoring >~>");
                    e.into_inner()
                }
            };
            *v
        };


        if file_read_required | self.catNameAndPhotoUrl.is_none() {

            let newData = CatNamesReader::deserialize(&self.path);
            match newData {
                Ok(v) => {
                


                    self.catNameAndPhotoUrl = Some(v);
                    return Ok(self.catNameAndPhotoUrl.clone().unwrap());
                },

                Err(e) =>{
                    if self.catNameAndPhotoUrl.is_none(){
                        return Err(ReadError::FailedFileInitialLoad(
                            format!("failed to read from file on first try. Aborting.\n{:?}",e)
                            .to_string()));
                    }
                    else {
                        return Err(ReadError::FailedFileLoad("".to_string()));
                    }
                }
            }
        }
        
        Ok(self.catNameAndPhotoUrl.clone().expect("expected catNameAndPhotoUrl to already be initialized!"))

    }
            
    fn deserialize(filePath: &PathBuf) -> Result<HashMap<String,String>, DeserializationError>
    {
        let fileContents =  std::fs::read_to_string(filePath).unwrap();
        let catData: Result<HashMap<String, String>, serde_json::Error> = serde_json::from_str::<HashMap<String,String>>(fileContents.as_str());
        let output = catData.map_err(DeserializationError::FailedToDeserialize)?;
        
        return Ok(output);
    }
    

}

