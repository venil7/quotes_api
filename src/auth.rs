use crate::auth_api::AuthApi;
use crate::error::ApiError;
use actix_web::dev::ServiceRequest;
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
    match auth_api.validate_token(credentials.token()) {
      true => Ok(req),
      _ => Err(Error::from(ApiError::new())),
    }
  } else {
    Err(Error::from(ApiError::new()))
  }
}
