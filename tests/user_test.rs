use rocket::http::{ContentType, Status};

mod common;

#[test]
fn users(){
  let client = common::setup();
  let mut response = client.get("/api/users").dispatch();

  assert_eq!(response.status(), Status::Ok);
  assert_eq!(response.content_type(), Some(ContentType::JSON));
  assert_eq!(response.body_string(), Some("{\"status\":\"Success\",\"message\":\"Array of users\"}".into()));
}

#[test]
fn user(){
  let client = common::setup();
  let mut response = client.get("/api/users/123").dispatch();

  assert_eq!(response.status(), Status::Ok);
  assert_eq!(response.content_type(), Some(ContentType::JSON));
  assert_eq!(response.body_string(), Some("{\"status\":\"Success\",\"message\":\"Single user\"}".into()));
}

#[test]
fn create_user(){
  let client = common::setup();
  let mut response = client.post("/api/users/new").dispatch();

  assert_eq!(response.status(), Status::Ok);
  assert_eq!(response.content_type(), Some(ContentType::JSON));
  assert_eq!(response.body_string(), Some("{\"status\":\"Success\",\"message\":\"New user\"}".into()));
}

#[test]
fn update_user(){
  let client = common::setup();
  let mut response = client.put("/api/users/123").dispatch();

  assert_eq!(response.status(), Status::Ok);
  assert_eq!(response.content_type(), Some(ContentType::JSON));
  assert_eq!(response.body_string(), Some("{\"status\":\"Success\",\"message\":\"Updated user\"}".into()));
}

#[test]
fn delete_user(){
  let client = common::setup();
  let mut response = client.delete("/api/users/123").dispatch();

  assert_eq!(response.status(), Status::Ok);
  assert_eq!(response.content_type(), Some(ContentType::JSON));
  assert_eq!(response.body_string(), Some("{\"status\":\"Success\",\"message\":\"Deleted user\"}".into()));
}

