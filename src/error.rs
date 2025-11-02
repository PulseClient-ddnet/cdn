use std::path::PathBuf;

use ohkami::{IntoResponse, Response};
use reqwest::header::ToStrError;
use thiserror::Error;
use tokio::{io, task::JoinError};
use tracing::{error, instrument};

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
            Error::Io(_) => Response::InternalServerError().with_text("Skin not found"),
            Error::Tee(_) => Response::InternalServerError().with_text("Fail to render uv"),
            Error::QueryNameNotFound => {
                Response::BadRequest().with_text("Query expected name, but got none")
            }
            Error::Reqwest(_) => Response::InternalServerError(),
            Error::ToStrError(_) => Response::InternalServerError(),
            Error::DownloadFailed {
                name: _,
                error: _,
            } => todo!(),
            Error::TaskJoin(_join_error) => Response::InternalServerError(),
            Error::Json(_error) => Response::InternalServerError(),
            Error::SaveFailed {
                path: _,
                name: _,
                error: _,
            } => Response::InternalServerError(),
        }
    }
}
