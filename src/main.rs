extern crate iron;

use iron::response;
use iron::status;
use std::fs;
use std::io::BufReader;

fn stream(_: &mut iron::Request) -> iron::IronResult<iron::Response> {
    let bufreader = BufReader::new(fs::File::open("/dev/urandom").unwrap());
    Ok(iron::Response::with((status::Ok, response::BodyReader(bufreader))))
}

fn main() {
    let mut iron = iron::Iron::new(stream);
    iron.threads = 1;
    iron.http("localhost:3000").unwrap();
}
