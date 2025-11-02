use std::path::PathBuf;

use ohkami::{
    IntoResponse, Response,
    openapi::{self, Schema},
    serde::Serialize,
};
use reqwest::header::ToStrError;
use thiserror::Error;
use tokio::{io, task::JoinError};
use tracing::{error, instrument};

#[derive(Serialize, Schema)]
#[openapi(component)]
struct ErrorResponse {
    code: u16,
    message: String,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("I/O error")]
    Io(#[from] io::Error),
    #[error("Json convertation error")]
    Json(#[from] serde_json::Error),
    #[error("Fail to join task")]
    TaskJoin(#[from] JoinError),
    #[error("Tee error")]
    Tee(#[from] tee_morphosis::error::TeeError),
    #[error("Query expected name, but got none")]
    QueryNameNotFound,
    #[error("Reqwest error")]
    Reqwest(#[from] reqwest::Error),
    #[error("Reqwest header convertation error")]
    ToStrError(#[from] ToStrError),
    #[error("Fail to save: {name}, path {path:#?}, {error}")]
    SaveFailed {
        path: PathBuf,
        name: String,
        error: String,
    },
    #[error("Fail to download: {name}, {error}")]
    DownloadFailed { name: String, error: String },
}

impl IntoResponse for Error {
    #[instrument]
    fn into_response(self) -> Response {
        match self {
            Error::QueryNameNotFound => Response::BadRequest().with_json(ErrorResponse {
                code: 400,
                message: "Query expected name, but got none".to_string(),
            }),
            Error::Io(e) => {
                tracing::error!("I/O error: {}", e);
                Response::InternalServerError().with_json(ErrorResponse {
                    code: 500,
                    message: "Skin not found".to_string(),
                })
            }
            Error::Tee(e) => {
                tracing::error!("Tee error: {}", e);
                Response::InternalServerError().with_json(ErrorResponse {
                    code: 500,
                    message: "Failed to render UV".to_string(),
                })
            }
            Error::Reqwest(e) => {
                tracing::error!("Reqwest error: {}", e);
                Response::InternalServerError().with_json(ErrorResponse {
                    code: 500,
                    message: "External request failed".to_string(),
                })
            }
            Error::ToStrError(e) => {
                tracing::error!("Header conversion error: {}", e);
                Response::InternalServerError().with_json(ErrorResponse {
                    code: 500,
                    message: "Invalid header value".to_string(),
                })
            }
            Error::TaskJoin(e) => {
                tracing::error!("Task join error: {}", e);
                Response::InternalServerError().with_json(ErrorResponse {
                    code: 500,
                    message: "Background task failed".to_string(),
                })
            }
            Error::Json(e) => {
                tracing::error!("JSON error: {}", e);
                Response::InternalServerError().with_json(ErrorResponse {
                    code: 500,
                    message: "JSON processing failed".to_string(),
                })
            }
            Error::SaveFailed {
                path,
                name,
                error,
            } => {
                tracing::error!("Save failed: {} at {:?}: {}", name, path, error);
                Response::InternalServerError().with_json(ErrorResponse {
                    code: 500,
                    message: format!("Failed to save {}", name),
                })
            }
            Error::DownloadFailed {
                name,
                error,
            } => {
                tracing::error!("Download failed: {}: {}", name, error);
                Response::InternalServerError().with_json(ErrorResponse {
                    code: 500,
                    message: format!("Failed to download {}", name),
                })
            }
        }
    }

    fn openapi_responses() -> openapi::Responses {
        openapi::Responses::new([
            (
                400,
                openapi::Response::when("Bad request - invalid query parameters")
                    .content("application/json", <ErrorResponse as Schema>::schema()),
            ),
            (
                500,
                openapi::Response::when("Internal server error")
                    .content("application/json", <ErrorResponse as Schema>::schema()),
            ),
        ])
    }
}
