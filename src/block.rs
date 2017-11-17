use std::io::prelude::*;
use std::path::Path;
use std::path::PathBuf;
use std::fs::File;
use std::error::Error;
use std::cmp::Ordering;
use chrono::prelude::*;
use crypto::sha2::Sha256;
use crypto::digest::Digest;
use serde_json;


const CHAIN_DIR: &str = "chaindata";
const NUM_ZEROS: usize = 4;

#[derive(Serialize, Deserialize, Debug, Eq)]
pub struct Block {
    pub index: u32,
    timestamp: DateTime<Utc>,
    prev_hash: String,
    pub hash: String,
    data: String,
}

impl Block {
    pub fn default() -> Block {
        Block{
            index: 0,
            timestamp: Utc::now(),
            prev_hash: String::default(),
            hash: String::default(),
            data: String::default()
        }
    }
    pub fn new(index: u32, timestamp: DateTime<Utc>, prev_hash: String, data: String) -> Block {
        let mut block = Block {
            index,
            timestamp,
            prev_hash,
            hash: String::default(),
            data,
        };
        block.hash = block.calculate_hash();
        return block;
    }

    pub fn create_first_block() -> Block {
        Block::new(
            0,
            Utc::now(),
            String::default(),
            String::from("First block data"),
        )
    }

    pub fn save(&self, path: &Path) -> Result<(), Box<Error>> {
        // Add block to path
        let mut path = PathBuf::from(path);

        path.push(self.index.to_string());
        path.set_extension("chain");

        let mut file = File::create(path)?;

        file.write_all(self.serialize().as_bytes())?;

        Ok(())
    }

    pub fn serialize(&self) -> String {
        return serde_json::to_string(&self).expect("Error serializing block");
    }

    pub fn deserialize(contents: &str) -> Block {
        serde_json::from_str(&contents.to_string()).unwrap()
    }

    pub fn read_from_file(entry: &Path) -> Block {
        let mut file = File::open(entry).unwrap();

        let mut contents = String::new();

        file.read_to_string(&mut contents);

        Block::deserialize(&contents.to_string())
    }

    fn generate_header(&self, nonce: usize) -> String {
        format!(
            "{}{}{}{}{}",
            self.index,
            self.prev_hash,
            self.data,
            self.timestamp,
            nonce
        )
    }

    fn calculate_hash(&self) -> String {
        let mut nonce = 0;
        let comparable = String::from_utf8(vec![b'0';NUM_ZEROS]).unwrap();

        loop {
            let header_string = self.generate_header(nonce);

            let mut sha = Sha256::new();
            sha.input_str(&header_string);

            let result = sha.result_str();

            if &result[0..NUM_ZEROS] == comparable {
                println!("Found header: {}", result);
                return result;
            } 
            nonce = nonce + 1;
        }
    }

    pub fn mine(&self) -> Block {
        Block::new(
            self.index + 1,
            Utc::now(),
            self.hash.clone(),
            format!("I block {}", self.index),
        )
    }
}

impl Ord for Block {
    fn cmp(&self, other: &Block) -> Ordering {
        self.index.cmp(&other.index)
    }
}

impl PartialOrd for Block {
    fn partial_cmp(&self, other: &Block) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Block {
    fn eq(&self, other: &Block) -> bool {
        self.hash == other.hash
    }
}
