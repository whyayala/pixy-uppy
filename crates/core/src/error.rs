use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum PixyError {
    #[error("Command not found: {0}")]
    CommandNotFound(&'static str),

    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Invalid argument: {0}")]
    InvalidArgument(&'static str),

    #[error("Process failed: {cmd} (code {code:?})\n{stderr}")]
    ProcessFailed {
        cmd: String,
        code: Option<i32>,
        stderr: String,
    },
}
