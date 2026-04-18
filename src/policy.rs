use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::Mutex;

use crate::error::IdempoError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IdempotencyKey(String);

impl IdempotencyKey {
    pub fn new(value: impl Into<String>) -> Result<Self, IdempoError> {
        let v = value.into();

        if v.trim().is_empty() {
            return Err(IdempoError::InvalidKey);
        }

        Ok(Self(v))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fingerprint(Vec<u8>);

impl Fingerprint {
    pub fn new(bytes: impl Into<Vec<u8>>) -> Result<Self, IdempoError> {
        let bytes = bytes.into();

        if bytes.is_empty() {
            return Err(IdempoError::EmptyFingerprint);
        }

        Ok(Self(bytes))
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

pub trait FingerprintStrategy {
    fn fingerprint(&self, input: &[u8]) -> Fingerprint;
}

#[derive(Debug, Clone, Default)]

pub struct DefaultFingerprintStrategy;

impl FingerprintStrategy for DefaultFingerprintStrategy {
    fn fingerprint(&self, input: &[u8]) -> Fingerprint {
        let mut hasher = DefaultHasher::new();
        input.hash(&mut hasher);
        let hash = hasher.finish().to_be_bytes().to_vec();

        Fingerprint::new(hash).expect("default fingerprint strategy must produce non-empty output")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReplayDecision {
    New,
    Replay,
    Conflict,
}

pub trait IdempotencyStore: Send + Sync {
    fn check(
        &self,
        key: &IdempotencyKey,
        fingerprint: &Fingerprint,
    ) -> Result<ReplayDecision, IdempoError>;

    fn save(&self, key: IdempotencyKey, fingerprint: Fingerprint) -> Result<(), IdempoError>;
}

#[derive(Debug, Default)]
pub struct MemoryStore {
    entries: Mutex<HashMap<IdempotencyKey, Fingerprint>>,
}

impl MemoryStore {
    pub fn new() -> Self {
        Self::default()
    }
}

impl IdempotencyStore for MemoryStore {
    fn check(
        &self,
        key: &IdempotencyKey,
        fingerprint: &Fingerprint,
    ) -> Result<ReplayDecision, IdempoError> {
        let map = self.entries.lock().map_err(|_| IdempoError::StoreError)?;

        match map.get(key) {
            None => Ok(ReplayDecision::New),
            Some(existing) if existing == fingerprint => Ok(ReplayDecision::Replay),
            Some(_) => Ok(ReplayDecision::Conflict),
        }
    }

    fn save(&self, key: IdempotencyKey, fingerprint: Fingerprint) -> Result<(), IdempoError> {
        let mut map = self.entries.lock().map_err(|_| IdempoError::StoreError)?;

        match map.get(&key) {
            None => {
                map.insert(key, fingerprint);
                Ok(())
            }
            Some(existing) if existing == &fingerprint => Ok(()),
            Some(_) => Err(IdempoError::FingerprintMismatch),
        }
    }
}

pub struct IdempoPolicy<S, F> {
    store: S,
    fingerprint_strategy: F,
}

impl<S, F> IdempoPolicy<S, F> {
    pub fn new(store: S, fingerprint_strategy: F) -> Self {
        Self {
            store,
            fingerprint_strategy,
        }
    }

    pub fn store(&self) -> &S {
        &self.store
    }

    pub fn fingerprint_strategy(&self) -> &F {
        &self.fingerprint_strategy
    }
}

impl<S, F> IdempoPolicy<S, F>
where
    S: IdempotencyStore,
    F: FingerprintStrategy,
{
    pub fn evaluate(
        &self,
        key: &IdempotencyKey,
        fingerprint: &[u8],
    ) -> Result<ReplayDecision, IdempoError> {
        let fingerprint = self.fingerprint_strategy.fingerprint(fingerprint);
        self.store.check(key, &fingerprint)
    }

    pub fn record(&self, key: IdempotencyKey, fingerprint: &[u8]) -> Result<(), IdempoError> {
        let fingerprint = self.fingerprint_strategy.fingerprint(fingerprint);
        self.store.save(key, fingerprint)
    }
}
