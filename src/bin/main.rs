extern crate iron;
extern crate router;
extern crate rustchain;

use rustchain::MinerNode;


fn main() {

    let mut main = MinerNode::new(3000);

    let thread = main.start();

    main.mine_to_size(6);

    thread.join();

}

