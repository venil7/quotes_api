pub struct AuthApi;

impl AuthApi {
  pub fn validate_token(&self, token: &str) -> bool {
    token.contains("aaa")
  }
}

impl Default for AuthApi {
  fn default() -> Self {
    AuthApi {}
  }}
