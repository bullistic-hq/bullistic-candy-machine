use solana_program_test::BanksClientError;
use solana_sdk::transport::TransportError;

#[derive(Debug)]
pub enum SolanaProgramTestError {
    TransportError(TransportError),
    BanksClientError(BanksClientError),
}

impl From<BanksClientError> for SolanaProgramTestError {
    fn from(e: BanksClientError) -> Self {
        SolanaProgramTestError::BanksClientError(e)
    }
}

impl From<TransportError> for SolanaProgramTestError {
    fn from(e: TransportError) -> Self {
        SolanaProgramTestError::TransportError(e)
    }
}

pub type SolanaProgramTestResult<T = ()> = Result<T, SolanaProgramTestError>;
