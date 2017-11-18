extern crate rustchain;

use rustchain::Chain;
use std::env;


fn setup() {
    env::set_var("RUN_MODE", "test");
}


#[test]
fn it_adds_two() {

    setup();
    let chain = Chain::new();
}