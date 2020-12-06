use hostname;
use serde::{Deserialize, Serialize};
use std::ffi::OsString;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct About {
  pub name: String,
  pub version: String,
  pub host: String,
}

impl Default for About {
  fn default() -> About {
    let host = hostname::get()
      .unwrap_or(OsString::from("unknown"))
      .to_str()
      .unwrap()
      .to_owned();
    About {
      version: env!("CARGO_PKG_VERSION").into(),
      name: env!("CARGO_PKG_NAME").into(),
      host,
    }
  }
}
