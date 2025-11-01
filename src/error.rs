use std::path::PathBuf;

use ohkami::{IntoResponse, Response};
use reqwest::header::ToStrError;
use thiserror::Error;
use tokio::{io, task::JoinError};
use tracing::error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("I/O error")]
    Io(#[from] io::Error),
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
    fn into_response(self) -> Response {
        match self {
            Error::Io(e) => {
                error!(error=%e,"I/O error");
                Response::InternalServerError().with_text("Skin not found")
            }
            Error::Tee(e) => {
                error!(error=%e,"Tee error");
                Response::InternalServerError().with_text("Fail to render uv")
            }
            Error::QueryNameNotFound => {
                error!("Query expected name, but got none");
                Response::BadRequest().with_text("Query expected name, but got none")
            }
            Error::Reqwest(e) => {
                error!(error=%e,"Reqwest error");
                Response::InternalServerError()
            }
            Error::ToStrError(e) => {
                error!(error=%e,"Reqwest ToStr error");
                Response::InternalServerError()
            }
            Error::SaveFailed {
                path,
                name,
                error,
            } => todo!(),
            Error::DownloadFailed {
                name,
                error,
            } => todo!(),
            Error::TaskJoin(join_error) => todo!(),
        }
    }
}
