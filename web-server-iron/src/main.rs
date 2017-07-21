extern crate iron;
extern crate router;

use iron::prelude::*;
use iron::status;
use router::Router;

fn main() {
  let mut router = Router::new();

  router.get("/", hello_world);
  router.post("/data", randomfriend);

  fn hello_world(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Hello World!")))
  }

  fn data(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Got some data")))
  }

  Iron::new(router).http("localhost:3000").unwrap();
  println!("On 3000");
}