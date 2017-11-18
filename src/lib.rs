extern crate chrono;
extern crate crypto;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use block::Block;
use std::fs;
use std::path::Path;
use std::fmt;

mod block;

const CHAIN_DIR: &str = "chaindata";

#[derive(Debug)]
pub struct Chain {
    pub node_blocks: Vec<Block>,
}

impl fmt::Display for Chain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self.node_blocks).unwrap())
    }
}

impl Chain {
    /// Creates a new chain, generating blocks from existing filesystem blocks
    pub fn new() -> Chain {
        bootstrap_chaindata();
        let mut chain = Chain {
            node_blocks: Vec::default(),
        };
        chain.sync();
        return chain;
    }

    /// Synchronizes this chain with blocks in the filesystem
    pub fn sync(&mut self) {
        let path = Path::new(CHAIN_DIR);
        let mut node_blocks: Vec<Block> = Vec::default();

        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap().path();
            if entry.is_file() {
                node_blocks.push(Block::read_from_file(&entry));
            }
        }
        node_blocks.sort();

        {
            let mut last: Option<&Block> = None;
            for x in node_blocks.iter() {
                match last {
                    Some(prev) => {
                        assert_eq!(prev.hash, x.prev_hash);
                        last = Some(x);
                    },
                    None => last = Some(x)
                }
            }
        }


        self.node_blocks = node_blocks;
    }

    /// Uses the latest block in the chain to generate a new Block,
    /// which is then added to the chain
    pub fn mine(&mut self) {
        // start with empty block
        let mut new_block = Block::default();

        // Open new scope so we can immutably borrow last_block
        {
            let last_block = self.node_blocks.iter().max().unwrap();

            new_block = last_block.mine_block();

            assert_eq!(last_block.hash, new_block.prev_hash);
        }

        // ...and then give up the borrow to add the new block to the chain
        new_block.save(Path::new(CHAIN_DIR));
        self.node_blocks.push(new_block);
    }
}

/// Creates the chaindata folder and an initial block if not already present
fn bootstrap_chaindata() {
    let path = Path::new(CHAIN_DIR);

    // Create chaindata directory if doesn't exist
    fs::create_dir_all(path).unwrap();

    let any_files = fs::read_dir(path).unwrap().count() > 0;

    if !any_files {
        let first_block = Block::create_first_block();

        first_block.save(path);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
