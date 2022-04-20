use rocket::*;
use crate::routes::response::Response;
use rocket_contrib::json::Json;

#[get("/")]
pub fn users() -> Json<Response> {
  Json(Response::ok("Array of users"))
}

#[post("/new")]
pub fn create_user() -> Json<Response> {
  Json(Response::ok("New user"))
}

#[get("/<id>")]
pub fn user(id: String) -> Json<Response> {
  Json(Response::ok("Single user"))
}

#[put("/<id>")]
pub fn update_user(id: String) -> Json<Response> {
  Json(Response::ok("Updated user"))
}

#[delete("/<id>")]
pub fn delete_user(id: String) -> Json<Response> {
  Json(Response::ok("Deleted user"))
}