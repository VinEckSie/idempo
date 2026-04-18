use idempo::{
    DefaultFingerprintStrategy, Fingerprint, FingerprintStrategy, IdempoError, IdempoPolicy,
    IdempotencyKey, IdempotencyStore, MemoryStore, ReplayDecision,
};

#[test]
fn returns_new_for_unknown_key() {
    let store = MemoryStore::new();
    let key = IdempotencyKey::new("req-123").unwrap();
    let fingerprint = Fingerprint::new([1, 2, 3]).unwrap();

    let decision = store.check(&key, &fingerprint).unwrap();

    assert_eq!(decision, ReplayDecision::New);
}

#[test]
fn returns_replay_for_same_key_and_same_fingerprint() {
    let store = MemoryStore::new();
    let key = IdempotencyKey::new("req-123").unwrap();
    let fingerprint = Fingerprint::new([1, 2, 3]).unwrap();

    store.save(key.clone(), fingerprint.clone()).unwrap();

    let decision = store.check(&key, &fingerprint).unwrap();

    assert_eq!(decision, ReplayDecision::Replay);
}

#[test]
fn returns_conflict_for_same_key_and_different_fingerprint() {
    let store = MemoryStore::new();
    let key = IdempotencyKey::new("req-123").unwrap();

    store
        .save(key.clone(), Fingerprint::new([1, 2, 3]).unwrap())
        .unwrap();

    let decision = store
        .check(&key, &Fingerprint::new([9, 9, 9]).unwrap())
        .unwrap();

    assert_eq!(decision, ReplayDecision::Conflict);
}

#[test]
fn rejects_empty_idempotency_key() {
    let result = IdempotencyKey::new("");

    assert_eq!(result, Err(idempo::IdempoError::InvalidKey));
}

#[test]
fn rejects_empty_fingerprint() {
    let result = Fingerprint::new(Vec::<u8>::new());

    assert_eq!(result, Err(idempo::IdempoError::EmptyFingerprint));
}

#[test]
fn rejects_blank_idempotency_key() {
    let result = IdempotencyKey::new("   ");

    assert_eq!(result, Err(idempo::IdempoError::InvalidKey));
}

#[test]
fn default_fingerprint_strategy_is_deterministic_for_same_input() {
    let strategy = DefaultFingerprintStrategy;
    let left = strategy.fingerprint(b"same-request");
    let right = strategy.fingerprint(b"same-request");

    assert_eq!(left, right);
}

#[test]
fn default_fingerprint_strategy_differs_for_different_input() {
    let strategy = DefaultFingerprintStrategy;
    let left = strategy.fingerprint(b"request-a");
    let right = strategy.fingerprint(b"request-b");

    assert_ne!(left, right);
}

#[test]
fn return_idempolicy_new() {
    let store = MemoryStore::new();
    let fingerprint_strategy = DefaultFingerprintStrategy;
    let policy = IdempoPolicy::new(store, fingerprint_strategy);
    let key = IdempotencyKey::new("req123").unwrap();

    let evaluation = policy.evaluate(&key, b"req1").unwrap();

    assert_eq!(ReplayDecision::New, evaluation);
}

#[test]
fn return_idempolicy_replay() {
    let store = MemoryStore::new();
    let fingerprint_strategy = DefaultFingerprintStrategy;
    let policy = IdempoPolicy::new(store, fingerprint_strategy);
    let key = IdempotencyKey::new("req123").unwrap();

    let _ = policy.record(key.clone(), b"req0");
    let evaluation = policy.evaluate(&key, b"req0").unwrap();

    assert_eq!(ReplayDecision::Replay, evaluation);
}

#[test]
fn return_idempolicy_conflict() {
    let store = MemoryStore::new();
    let fingerprint_strategy = DefaultFingerprintStrategy;
    let policy = IdempoPolicy::new(store, fingerprint_strategy);
    let key = IdempotencyKey::new("req1213").unwrap();

    let _ = policy.record(key.clone(), b"finger1");
    let evaluation = policy.evaluate(&key, b"finger2").unwrap();

    assert_eq!(ReplayDecision::Conflict, evaluation);
}

#[test]
fn return_save_fingerprint_conflict() {
    let store = MemoryStore::new();
    let key = IdempotencyKey::new("req123").unwrap();

    let _ = store
        .save(key.clone(), Fingerprint::new([1, 2, 3]).unwrap())
        .unwrap();

    let result = store.save(key.clone(), Fingerprint::new([4, 5, 6]).unwrap());

    assert_eq!(Err(IdempoError::FingerprintMismatch), result);
}
