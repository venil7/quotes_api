use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Quote {
  pub name: String,
  pub time: String,
  pub price: f64,
}
