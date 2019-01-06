//extern crate actix_web;
//extern crate actix_web_httpauth;
//use actix_web::server;

//extern crate serde;
//#[macro_use] extern crate serde_json;
//#[macro_use] extern crate serde_derive;
//use serde_json::Value;


//use shiplift::rep::Version;
//use shiplift::builder::ContainerOptions;
//use shiplift::errors::Error;

//extern crate http;

//mod api;
//mod utils;
/* Terminal and UX */
#[macro_use] extern crate emojicons;
extern crate termion;

/* Docker connection */
extern crate shiplift;
use shiplift::Docker;

/* For shiplift use */
extern crate tokio;
use tokio::prelude::{Future, Stream};

mod c;

fn main() {
	c::head();

	print!("Checking `dockerd`... ");
	let docker = Docker::new();
	let fut = docker.version()
		.map(|r| println!("{}({})", c::ok(), r.version))
		.map_err(|e| {
			println!("{}({})", c::ko(), e);
			::std::process::exit(1)
		});
	tokio::run(fut);
    /*
    let images = docker.images();
    let containers = docker.containers();

    /* Launch container and connect to Redis */
    utils::pullimg(&images, "redis:latest", Some("  :: Getting Redis server image: "));
    utils::checkcontainer(&containers,
        &ContainerOptions::builder("redis:latest")
        .name("stakerd-redis")
        .expose(6379, "tcp", 6379).build()
    , Some("  :: Getting Redis server container up and running: "));
    containers.get("stakerd-redis").start().unwrap();

    /* Launch RESTful API */
    println!("\n:i: {}Ready{} :: Launching the RESTful API to {}http://127.0.0.1:8080{} !", color::Fg(color::Green), color::Fg(color::Reset), color::Fg(color::Cyan), color::Fg(color::Reset));
    server::new(api::root)
        .bind("127.0.0.1:8080")
        .unwrap().run();

    /* Gracefully stopping */
    println!("\n:i: {}Graceful Stop{}", color::Fg(color::Yellow), color::Fg(color::Reset));
*/
}
