use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Login {
  pub email: String,
  pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginToken {
  pub token: String,
}
