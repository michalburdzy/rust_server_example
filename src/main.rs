#![feature(proc_macro_hygiene, decl_macro)]

use std::path::{Path,PathBuf};
use rocket::*;
use rocket::response::NamedFile;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::helmet::SpaceHelmet;

const PUBLIC_FILES_PATH: &str = "public";

#[get("/<file..>")]
fn serve_file_manually(file: PathBuf) -> Option<NamedFile> {
    let path = Path::new(PUBLIC_FILES_PATH).join(file);
    NamedFile::open(path).ok()
}

#[get("/echo/<echo>")]
fn echo_handler(echo: String) -> String {
    format!("{}", echo)
}

fn build_rocket() -> rocket::Rocket {
    rocket::ignite()
    .attach(SpaceHelmet::default())
    .mount("/", routes![echo_handler])
    .mount(&format!("/{}", PUBLIC_FILES_PATH), routes![serve_file_manually])
    .mount("/static", StaticFiles::from(PUBLIC_FILES_PATH))
}

fn main(){
    build_rocket()
    .launch();
}

#[cfg(test)]
mod test {
    use super::build_rocket;
    use rocket::local::Client;
    use rocket::http::Status;

    #[test]
    fn echo_test(){
        let client = Client::new(build_rocket()).expect("Valid Rocket instance");
        let mut response = client.get("/echo/test_echo").dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("test_echo".into()));
    }
}