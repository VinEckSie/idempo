use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdempoError {
    MissingKey,
    InvalidKey,
    EmptyFingerprint,
    FingerprintMismatch,
    StoreError,
    InvalidConfig,
}

impl fmt::Display for IdempoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IdempoError::MissingKey => write!(f, "missing idempotency key"),
            IdempoError::InvalidKey => write!(f, "invalid idempotency key"),
            IdempoError::EmptyFingerprint => write!(f, "fingerprint cannot be empty"),
            IdempoError::FingerprintMismatch => write!(f, "fingerprint mismatch"),
            IdempoError::StoreError => write!(f, "store error"),
            IdempoError::InvalidConfig => write!(f, "invalid configuration"),
        }
    }
}

impl std::error::Error for IdempoError {}
