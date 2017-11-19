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

use super::SETTINGS;

#[derive(Serialize, Deserialize, Debug, Eq)]
pub struct Block {
    pub index: u32,
    timestamp: DateTime<Utc>,
    pub prev_hash: String,
    pub hash: String,
    data: String,
}

impl Block {
    /// Creates an empty Block
    pub fn default() -> Block {
        Block {
            index: 0,
            timestamp: Utc::now(),
            prev_hash: String::default(),
            hash: String::default(),
            data: String::default()
        }
    }

    /// Creates a new Block with a valid header
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

    /// Creates the first block in the chain
    /// and ensures its header is correct
    pub fn create_first_block() -> Block {
        Block::new(
            0,
            Utc::now(),
            String::default(),
            String::from("First block data"),
        )
    }

    /// Save the Block to the filesystem
    pub fn save(&self, path: &Path) -> Result<(), Box<Error>> {
        // Add block to path
        let mut path = PathBuf::from(path);

        path.push(self.index.to_string());
        path.set_extension("chain");

        let mut file = File::create(path)?;

        file.write_all(self.serialize().as_bytes())?;

        Ok(())
    }

    /// Returns a JSON representation of the Block
    pub fn serialize(&self) -> String {
        return serde_json::to_string(&self).expect("Error serializing block");
    }

    /// Converts a JSON string into a Block
    pub fn deserialize(contents: &str) -> Block {
        serde_json::from_str(&contents.to_string()).unwrap()
    }

    /// Read the file at the given path and deserialize it into a Block
    pub fn read_from_file(entry: &Path) -> Block {
        let mut file = File::open(entry).unwrap();

        let mut contents = String::new();

        file.read_to_string(&mut contents);

        let b = Block::deserialize(&contents.to_string());

        return b;
    }

    /// Generate the blocks header.
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

    /// Generate this block's hash, using a nonce to check preceding 0's
    fn calculate_hash(&self) -> String {
        let mut nonce = 0;
        let comparable = String::from_utf8(vec![b'0'; SETTINGS.block_settings.number_of_zeroes]).unwrap();

        loop {
            let header_string = self.generate_header(nonce);

            let mut sha = Sha256::new();
            sha.input_str(&header_string);

            let result = sha.result_str();

            if &result[0..SETTINGS.block_settings.number_of_zeroes] == comparable {
                println!("Found header: {}", result);
                return result;
            }
            nonce = nonce + 1;
        }
    }

    /// Use this block's hash to generate a new Block
    pub fn mine_block(&self) -> Block {
        let b = Block::new(
            self.index + 1,
            Utc::now(),
            self.hash.clone(),
            format!("I block {}", self.index),
        );
        assert!(b.is_valid());
        b
    }

    /// Check if this Block's header is valid
    /// Currently only checks that the hash has the correct amount of preceding 0's
    pub fn is_valid(&self) -> bool {
        &self.hash[0..SETTINGS.block_settings.number_of_zeroes] == String::from_utf8(vec![b'0'; SETTINGS.block_settings.number_of_zeroes]).unwrap()
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
        self.hash == other.hash && self.index == other.index
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn block_validity() {
        let valid = Block {
            index: 0,
            prev_hash: String::default(),
            hash: String::from("0000000000"),
            timestamp: Utc::now(),
            data: String::from("Valid")
        };

        assert!(valid.is_valid());

        let invalid = Block {
            index: 0,
            prev_hash: String::default(),
            hash: String::from("these are some words"),
            timestamp: Utc::now(),
            data: String::from("Valid")
        };

        assert!(!invalid.is_valid());
    }

    #[test]
    fn block_equality() {

        // Switch to Test environment to speed up mining
        env::set_var("RUN_MODE", "test");

        let block_0 = Block::new(
            0,
            Utc.timestamp(0,0),
            String::default(),
            String::default()
        );

        let block_0_clone = Block::new(
            0,
            Utc.timestamp(0,0),
            String::default(),
            String::default()
        );

        assert_eq!(block_0, block_0_clone);

        let diff_index = Block::new(
            1,
            Utc.timestamp(0,0),
            String::default(),
            String::default()
        );

        assert_ne!(block_0, diff_index);

        let diff_timestamp = Block::new(
            0,
            Utc.timestamp(100, 100),
            String::default(),
            String::default()
        );

        assert_ne!(block_0, diff_timestamp);

        let diff_prev_hash = Block::new(
            0,
            Utc.timestamp(0,0),
            block_0.hash.clone(),
            String::default()
        );

        assert_ne!(block_0, diff_prev_hash);

        let diff_data = Block::new(
            0,
            Utc.timestamp(0,0),
            String::default(),
            String::from("this is some data")
        );

        assert_ne!(block_0, diff_data);

    }
}
