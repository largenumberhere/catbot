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


pub struct HashComparer{
    lastHash: Output<Blake2b<U64>>,
    filePath: PathBuf
}



impl HashComparer {
    pub async fn new(filePath: PathBuf) -> HashComparer{
        return HashComparer {
            lastHash: compute_hash(&filePath).await,
            filePath
        }
    }

    pub async fn compare(&mut self) -> HashComparisonResult {
        let new_hash = compute_hash(&self.filePath).await;
        let result = match new_hash == self.lastHash {
            true => {
                HashComparisonResult::Equal
            }
            false => {
                HashComparisonResult::NotEqual
            }
        };

        self.lastHash = new_hash;
        result
    }
}

async fn compute_hash(path: &PathBuf) -> Output<Blake2b<U64>> {
    let hash = {
        let mut hasher = Blake2b512::default();
        //read and hash bytes in chunks
        let file = tokio::fs::File::open(path).await.unwrap();
        let mut buffer = tokio::io::BufReader::new(file); //makes a buffer that reads from file in 8kb increments

        //push the changes to the hash 10 bytes at a time, to limit allocations
        loop {
            let mut byte_buffer = bytes::BytesMut::with_capacity(10);
            let bytes_count_read = buffer.read_buf(&mut byte_buffer).await.unwrap();

            println!("{bytes_count_read} bytes read.");

            if bytes_count_read == 0 {
                break;
            }
            let bytes:Vec<u8> = byte_buffer.into();
            hasher.write(bytes.as_slice()).unwrap();
        }

        hasher.finalize_fixed()
    };

    return hash;
}