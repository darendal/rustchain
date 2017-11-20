extern crate rustchain;

use rustchain::MinerNode;
use rustchain::settings::*;

use std::env;
use std::fs;


fn setup() {
    env::set_var("RUN_MODE", "test");
}

fn clear_chain_data(path: &str) {
    fs::remove_dir_all(path).unwrap();
}



#[test]
fn concurrent_mine() {

    setup();

    let settings = Settings::new().unwrap();

    // Remove all existing chain data (start mining from clean slate)
    clear_chain_data(&settings.block_settings.chain_directory);

    let mut main_node = MinerNode::new(3000);

    // ensure chain data is actually deleted as expected
    // and that a new node bootstraps with an initial block
    assert_eq!(main_node.block_chain.len(), 1);

    // Start by mining 6 blocks
    main_node.mine_to_size(6);

    main_node.start();

    {
        let mut secondary_node = MinerNode::new(3001);

        assert_eq!(secondary_node.block_chain.len(), 1);

        // Query the given port for its chain
        secondary_node.chain_sync(String::from("http://localhost:3000/blockchain")).unwrap();

        assert_eq!(main_node.block_chain, secondary_node.block_chain);

    }

    let secondary_node = MinerNode::new(3001);

    assert_eq!(main_node.block_chain, secondary_node.block_chain);

}