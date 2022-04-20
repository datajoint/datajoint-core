use std::io::BufReader;
use std::io::Read;
use std::fs::File;
use md5::{Md5, Digest};

pub struct Hash {
}

impl Hash{
    pub fn uuid_from_stream<R: Read>(mut stream: BufReader<R>) -> String {
        let mut hasher = Md5::new();
        const CHUNK_SIZE: usize = 1 << 14;

        loop {
            let mut chunk = Vec::with_capacity(CHUNK_SIZE);
            let n = stream.by_ref().take(CHUNK_SIZE as u64).read_to_end(&mut chunk);
            
            if n.unwrap() == 0 { break;}

            hasher.update(chunk);
        }   
        
        let result = hasher.finalize();
        hex::encode(&result)
    }

    pub fn uuid_from_buffer(bytes: &[u8]) -> String {        
        Hash::uuid_from_stream(BufReader::new(bytes))
    }

    pub fn uuid_from_file(filepath: String) -> String {
        Hash::uuid_from_stream(BufReader::new(File::open(filepath).unwrap()))
    }
}