use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Response {
  status: String,
  message: String,
}

impl Response {
  pub fn ok(msg: &str) -> Self {
    Response {
      status: "Success".to_string(),
      message: msg.to_string()
    }
  }
  pub fn err(msg: &str) -> Self {
    Response {
      status: "Error".to_string(),
      message: msg.to_string(),
    }
  }
}