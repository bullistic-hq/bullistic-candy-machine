use thiserror::Error;

#[derive(Debug, Error)]
pub enum MerkleAllowlistError {
    #[error("Could not find merkle allowlist config file at path: '{0}'. Please be sure to run the command from the root level of the bullistic-candy-machine repo.")]
    MissingFileError(String),

    #[error("Failed to parse merkle allowlist config file, error: {0}")]
    ParseConfigError(String),
}
