#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;

extern crate serde_json;
extern crate chrono;
extern crate crypto;
extern crate serde;
extern crate config;
extern crate iron;
extern crate router;
extern crate reqwest;

use std::thread;
use std::error::Error;
use settings::Settings;
use iron::prelude::*;
use iron::status;
use router::Router;
use iron::mime::Mime;
use block_chain::BlockChain;
use block::Block;


mod block;
pub mod settings;
mod block_chain;

lazy_static! {
    static ref SETTINGS: Settings = {
        Settings::new().unwrap()
    };

}

pub struct MinerNode {
    port: u16,
    pub block_chain: BlockChain
}

impl MinerNode {

    pub fn new(port: u16) -> MinerNode {
        let path = format!("{}/{}",&SETTINGS.block_settings.chain_directory, port);
        MinerNode{
            port,
            block_chain: BlockChain::new(path)
        }

    }

    pub fn start(&self) -> thread::JoinHandle<()> {
        let mut router = Router::new();
        let port = self.port.clone();
        router.get("/blockchain", view_chain, "Index");

        thread::spawn(move || {
            println!("Miner Node started on port: {}", port);
            Iron::new(router).http(format!("localhost:{}", port)).unwrap();
        })
    }
    
    pub fn mine_to_size(&mut self, size: usize) {
        while self.block_chain.len() < size {
            println!("Current chain size: {}, required is {}. Mining new node", self.block_chain.len(), size);
            self.block_chain.mine();
        }
    }

    pub fn chain_sync(&mut self, url: String) -> Result<(), Box<Error>> {

        let client = reqwest::Client::new();
        let mut response = client.get(&url).send()?;

        if let Ok(json) = response.json::<Vec<Block>>() {
            self.block_chain.node_blocks = json;
            self.block_chain.save_chain();
            return Ok(());
        }

        panic!();
    }
}

fn view_chain(r: &mut Request) -> IronResult<Response> {
    let content_type = "application/json".parse::<Mime>().unwrap();

    let path = format!("{}/{}",&SETTINGS.block_settings.chain_directory, r.url.port());
    let chain = BlockChain::new(path);

    Ok(Response::with(
        (content_type, status::Ok, chain.to_string()),
    ))
}
