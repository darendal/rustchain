use block::Block;
use std::fs;
use std::path::Path;
use std::fmt;
use std::cmp::Ordering;

use serde_json;

#[derive(Debug, Eq)]
pub struct BlockChain {
    pub node_blocks: Vec<Block>,
    chain_dir: String
}

impl fmt::Display for BlockChain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self.node_blocks).unwrap())
    }
}

impl BlockChain {
    /// Creates a new chain, generating blocks from existing filesystem blocks
    pub fn new(chain_dir: String) -> BlockChain {

        let mut chain = BlockChain {
            node_blocks: Vec::default(),
            chain_dir: chain_dir.clone()
        };
        chain.bootstrap_chaindata();
        chain.sync();
        return chain;
    }

    /// Synchronizes this chain with blocks in the filesystem
    pub fn sync(&mut self) {
        let path = Path::new(&self.chain_dir);
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
                    }
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
        let new_block : Block;

        // Open new scope so we can immutably borrow last_block
        {
            let last_block = self.node_blocks.iter().max().unwrap();

            new_block = last_block.mine_block();

            assert_eq!(last_block.hash, new_block.prev_hash);
            assert_eq!(last_block.index, new_block.index - 1);
        }

        // ...and then give up the borrow to add the new block to the chain
        new_block.save(Path::new(&self.chain_dir));
        self.node_blocks.push(new_block);
    }

    /// Creates the chaindata folder and an initial block if not already present
    fn bootstrap_chaindata(&self) {
        let path = Path::new(&self.chain_dir);

        // Create chaindata directory if doesn't exist
        fs::create_dir_all(path).unwrap();

        let any_files = fs::read_dir(path).unwrap().count() > 0;

        if !any_files {
            let first_block = Block::create_first_block();

            first_block.save(path);
        }
    }

    pub fn len(&self) -> usize {
        self.node_blocks.len()
    }

    pub fn save_chain(&self) {
        let path = Path::new(&self.chain_dir);
        self.node_blocks.iter().for_each(|x|{
            x.save(&path);
        })
    }

}


impl Ord for BlockChain {
    fn cmp(&self, other: &BlockChain) -> Ordering {
        self.node_blocks.len().cmp(&other.node_blocks.len())
    }
}

impl PartialOrd for BlockChain {
    fn partial_cmp(&self, other: &BlockChain) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for BlockChain {
    fn eq(&self, other: &BlockChain) -> bool {
        self.node_blocks.len() == other.node_blocks.len() && self.node_blocks == other.node_blocks
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn chain_equality() {

        let blocks1 = vec![Block::default(), Block::default()];
        let blocks2 = vec![Block::default(), Block::default()];

        let mut chain1 = BlockChain { node_blocks: blocks1, chain_dir: String::default() };
        let mut chain2 = BlockChain { node_blocks: blocks2, chain_dir: String::default() };

        assert_eq!(chain1, chain2);

        chain2.node_blocks.push(Block::default());

        assert_ne!(chain1,chain2);

        chain1.node_blocks.push(Block::create_first_block());

        assert_ne!(chain1, chain2);
    }
}