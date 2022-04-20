#![feature(proc_macro_hygiene, decl_macro)]
#![allow(unused_attributes)]

#[macro_use] use rocket::*;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::helmet::SpaceHelmet;

mod routes;

pub fn build_rocket() -> rocket::Rocket {
  rocket::ignite().attach(SpaceHelmet::default())
  .mount("/", routes![routes::healthcheck::healthcheck])
  .mount("/api/users", routes![
    routes::user::users,
    routes::user::user,
    routes::user::create_user,
    routes::user::update_user,
    routes::user::delete_user,
  ])
  .mount("/files", StaticFiles::from("static/"))
}