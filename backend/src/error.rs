use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug)]
pub enum AppError {
    HttpError(StatusCode, anyhow::Error),
    Internal(anyhow::Error),
    Unauthorized,
    Forbidden,
    NotFound,
    BadRequest(String),
    InternalServerError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::HttpError(status_code, err) => {
                (status_code, format!("Error: {}", err)).into_response()
            }
            AppError::Internal(err) => {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Internal Server Error: {}", err),
                )
                    .into_response()
            }
            AppError::Unauthorized => {
                (StatusCode::UNAUTHORIZED, "Unauthorized").into_response()
            }
            AppError::Forbidden => {
                (StatusCode::FORBIDDEN, "Forbidden").into_response()
            }
            AppError::NotFound => {
                (StatusCode::NOT_FOUND, "Not Found").into_response()
            }
            AppError::BadRequest(msg) => {
                (StatusCode::BAD_REQUEST, msg).into_response()
            }
            AppError::InternalServerError(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response()
            }
        }
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        AppError::Internal(err.into())
    }
}
