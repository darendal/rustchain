extern crate iron;
extern crate router;
extern crate rustchain;

use rustchain::MinerNode;
use std::thread;


fn main() {

    let main = MinerNode{port: 3001};

    let thread = main.start();

    thread.join();

}

