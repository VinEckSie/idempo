//! Idempotency policies and storage abstractions for Rust services.
//!
//! This crate provides deterministic, testable idempotency behavior
//! for HTTP and gRPC services.
//!
//! The focus is a small, predictable core API with pluggable storage
//! backends and fingerprinting strategies.

mod config;
mod error;
mod policy;

#[cfg(test)]
mod tests_support;

pub use config::IdempoConfig;
pub use error::IdempoError;
pub use policy::{
    DefaultFingerprintStrategy, Fingerprint, FingerprintStrategy, IdempoPolicy, IdempotencyKey,
    IdempotencyStore, MemoryStore, ReplayDecision,
};
