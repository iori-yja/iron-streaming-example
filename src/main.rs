extern crate zhelezo;

use zhelezo::response;
use zhelezo::status;
use std::fs;
use std::io::BufReader;

fn stream(_: &mut zhelezo::Request) -> zhelezo::IronResult<zhelezo::Response> {
    let bufreader = BufReader::new(fs::File::open("/dev/urandom").unwrap());
    Ok(zhelezo::Response::with((status::Ok, response::BodyReader(bufreader))))
}

fn main() {
    let zhelezo = zhelezo::Iron::new(stream);
    zhelezo.http("localhost:3000");
}
