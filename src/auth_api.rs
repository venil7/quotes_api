use crate::error::ApiError;
use jwt_simple::algorithms::HS256Key;
use jwt_simple::prelude::*;
use serde::{Deserialize, Serialize};

pub struct AuthApi {
  key: HS256Key,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiClaims {
  pub email: String,
  pub write: bool,
}

impl AuthApi {
  pub fn validate_token(&self, token: &str) -> Result<ApiClaims, ApiError> {
    let claims = self
      .key
      .verify_token::<ApiClaims>(&token, None)
      .map_err(|e| format!("verify_token {}", e))?;

    Ok(claims.custom)
  }

  pub fn create_token(&self, claims: &ApiClaims) -> Result<String, ApiError> {
    let jwt_claims = Claims::with_custom_claims(claims.clone(), Duration::from_hours(2));
    let token = self
      .key
      .authenticate(jwt_claims)
      .map_err(|e| format!("authenticate {}", e))?;

    Ok(token)
  }
}

impl AuthApi {
  pub fn from(str: &str) -> Self {
    let key = HS256Key::from_bytes(&str.as_bytes());
    AuthApi { key }
  }
}
