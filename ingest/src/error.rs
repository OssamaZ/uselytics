use axum::{
  http::StatusCode,
  response::{IntoResponse, Response},
  Json,
};
use serde_json::{json, Value};
use thiserror::Error;

use crate::response::AppResponse;

pub type AppResult = Result<AppResponse<Value>, AppError>;

#[derive(Debug, Error)]
pub enum AppError {
  #[error("Resource not found")]
  NotFound,
  #[error("Request unauthorized")]
  Unauthorized,
  #[error("{0}")]
  BadRequest(String),
  #[error("Unexpected error occurred")]
  InternalServerError,
  #[error("Unexpected internal error occurred")]
  Anyhow(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
  fn into_response(self) -> Response {
    let (status, error_msg) = match self {
      Self::NotFound => (StatusCode::NOT_FOUND, Self::NotFound.to_string()),
      Self::Unauthorized => (StatusCode::UNAUTHORIZED, Self::Unauthorized.to_string()),
      Self::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
      Self::InternalServerError => (
        StatusCode::INTERNAL_SERVER_ERROR,
        Self::InternalServerError.to_string(),
      ),
      Self::Anyhow(msg) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{msg}")),
      // _ => (
      //   StatusCode::INTERNAL_SERVER_ERROR,
      //   String::from("Unexpected error occurred"),
      // ),
    };

    (status, Json(json!(error_msg))).into_response()
  }
}
