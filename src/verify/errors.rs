use thiserror::Error;

#[derive(Error, Debug)]
pub enum VerifyError {
    #[error("Failed to get candy machine account data from Miraland for address: {0}.")]
    FailedToGetAccountData(String),
    #[error("{0} mismatch (expected='{1}', found='{2}')")]
    Mismatch(String, String, String),
}
