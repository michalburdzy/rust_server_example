use rocket::local::Client;
use rocket_tut::build_rocket;

pub fn setup() -> Client {
  Client::new(build_rocket()).expect("Valid Rocket instance")
}