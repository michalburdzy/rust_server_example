use rocket::http::Status;

mod common; 

#[test]
fn healthcheck_test(){
  let client = common::setup();
  let mut response = client.get("/healthcheck").dispatch();

  assert_eq!(response.status(), Status::Ok);
  assert_eq!(response.body_string(), Some("OK".into()));
}