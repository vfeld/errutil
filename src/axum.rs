//Copyright (c) 2023 Volker Kleinfeld

use axum::{response::{Response, IntoResponse}, http::StatusCode, Json};
use crate::ErrorInfo;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorBody {
    pub status_code: u16,
    pub status_description: String,
    pub error_id: String,
}

pub fn error_body(status: StatusCode,error_info: &ErrorInfo) -> ErrorBody {
    ErrorBody {
        status_code: s.as_u16(),
        status_description: status.canonical_reason().unwrap_or(status.as_str()).to_string(),
        error_id: error_info.errorid.clone(),
    }
}

#[tracing::instrument(skip_all)]
///Generates an axum response, given an http status code, error, and error information
pub fn err_resp(s: StatusCode, error: &impl std::error::Error, error_info: &ErrorInfo) -> Response {
    tracing::error!("{:?}", error);
    let mut response = (s, Json(error_body(s,error_info))).into_response();
    response.extensions_mut().insert(error_info.clone());
    response
}