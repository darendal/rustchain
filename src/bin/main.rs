extern crate iron;
extern crate router;
extern crate rustchain;
extern crate serde_json;

use rustchain::Chain;
use iron::prelude::*;
use iron::status;
use router::Router;
use iron::mime::Mime;



fn main() {

    let mut chain = Chain::new();

    chain.mine();

    let mut router = Router::new();
    router.get("/", hello_world, "Index");

    Iron::new(router).http("localhost:3000").unwrap();
}

fn hello_world(_: &mut Request) -> IronResult<Response> {
    let content_type = "application/json".parse::<Mime>().unwrap();

    let chain = Chain::new();

    Ok(Response::with(
        (content_type, status::Ok, chain.to_string()),
    ))
}
