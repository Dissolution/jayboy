use thiserror::Error;

#[derive(Debug, Error)]
pub enum JayBoyError {
    #[error("Misc error: {0}")]
    Misc(#[from] anyhow::Error),
}