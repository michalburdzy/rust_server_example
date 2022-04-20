use rocket::*;

#[get("/healthcheck")]
pub fn healthcheck() -> &'static str {
  "OK"
}