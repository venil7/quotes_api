pub struct AuthApi;

impl AuthApi {
  pub fn new() -> AuthApi {
    AuthApi {}
  }
  pub fn validate_token(&self, token: &str) -> bool {
    token.contains("aaa")
  }
}
