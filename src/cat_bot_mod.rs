//load file gradually into Blake2 hashing algorithm


use std::io::Write;
use std::path::PathBuf;
use blake2::{Blake2b, Blake2b512};
use blake2::digest::consts::U64;
use blake2::digest::{FixedOutput, Output};
use tokio::io::AsyncReadExt;

#[derive(Debug)]
pub enum HashComparisonResult {
    Equal,
    NotEqual,
}


pub struct HashCompare {
    last_hash: Output<Blake2b<U64>>,
    file_path: PathBuf
}



impl HashCompare {
    pub async fn new(file_path: PathBuf) -> HashCompare {
        return HashCompare {
            last_hash: compute_hash(&file_path).await,
            file_path
        }
    }

    pub async fn compare(&mut self) -> HashComparisonResult {
        let new_hash = compute_hash(&self.file_path).await;
        let result = match new_hash == self.last_hash {
            true => {
                HashComparisonResult::Equal
            }
            false => {
                HashComparisonResult::NotEqual
            }
        };

        self.last_hash = new_hash;
        result
    }
}

async fn compute_hash(path: &PathBuf) -> Output<Blake2b<U64>> {
    let hash = {
        let mut hasher = Blake2b512::default();
        //read and hash bytes in chunks
        let file = tokio::fs::File::open(path).await.unwrap();
        let mut buffer = tokio::io::BufReader::new(file); //makes a buffer that reads from file in 8kb increments


        //push the changes to the hash 100 bytes at a time, to limit allocations
        loop {
            //make somewhere to store the chunk. It won't let us use Vec<u8> for some reason :/
            let mut byte_buffer = bytes::BytesMut::with_capacity(10);

            //read some bytes into the buffer
            let bytes_count_read = buffer.read_buf(&mut byte_buffer).await.unwrap();

            if bytes_count_read == 0 {
                break;
            }

            //convert the bytes to a data structure write will accept
            let bytes:Vec<u8> = byte_buffer.to_vec();
            hasher.write(bytes.as_slice()).unwrap();
        }

        hasher.finalize_fixed()
    };

    return hash;
}