use crate::auth_api::AuthApi;
use actix_web::dev::{HttpResponseBuilder, ServiceRequest};
use actix_web::http::StatusCode;
use actix_web::Error;
use actix_web_httpauth::extractors::basic::BasicAuth;
use actix_web_httpauth::extractors::bearer::BearerAuth;

pub async fn basic_validator(
  req: ServiceRequest,
  _credentials: BasicAuth,
) -> Result<ServiceRequest, Error> {
  Ok(req)
}

pub async fn bearer_validator(
  req: ServiceRequest,
  credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
  if let Some(auth_api) = req.app_data::<AuthApi>() {
    if let Ok(_claims) = auth_api.validate_token(credentials.token()) {
      // _claims are available here,
      // and can be attached to req, to be passed further down
      return Ok(req);
    }
  }
  let response = HttpResponseBuilder::new(StatusCode::UNAUTHORIZED);
  Err(Error::from(response))
}
