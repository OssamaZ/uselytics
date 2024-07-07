use axum::{
  http::StatusCode,
  response::{IntoResponse, Response},
  Json,
};
use serde::Serialize;
use serde_json::json;

pub struct AppResponse<T>(pub T);

impl<T> IntoResponse for AppResponse<T>
where
  T: Serialize,
{
  fn into_response(self) -> Response {
    (StatusCode::OK, Json(json!(self.0))).into_response()
  }
}
