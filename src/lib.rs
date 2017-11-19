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

use std::thread;
use settings::Settings;
use iron::prelude::*;
use iron::status;
use router::Router;
use iron::mime::Mime;
use block_chain::BlockChain;

mod block;
mod settings;
pub mod block_chain;

lazy_static! {
    static ref SETTINGS: Settings = {
        Settings::new().unwrap()
    };

}

pub struct MinerNode {
    pub port: u16
}

impl MinerNode {

    pub fn start(&self) -> thread::JoinHandle<()> {
        let mut router = Router::new();
        let port = self.port.clone();
        router.get("/", hello_world, "Index");

        thread::spawn(move || {
            Iron::new(router).http(format!("localhost:{}", port)).unwrap();
        })

    }

}

fn hello_world(_: &mut Request) -> IronResult<Response> {
    let content_type = "application/json".parse::<Mime>().unwrap();

    let chain = BlockChain::new();

    Ok(Response::with(
        (content_type, status::Ok, chain.to_string()),
    ))
}
