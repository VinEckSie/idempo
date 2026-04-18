use idempo::{
    DefaultFingerprintStrategy, Fingerprint, FingerprintStrategy, IdempotencyKey, MemoryStore,
    ReplayDecision,
};

#[test]
fn public_types_are_usable_from_crate_root() {
    let _store = MemoryStore::new();
    let _key = IdempotencyKey::new("req-123").unwrap();
    let _fingerprint = Fingerprint::new([1, 2, 3]).unwrap();
    let _decision = ReplayDecision::New;

    let strategy = DefaultFingerprintStrategy;
    let _ = strategy.fingerprint(b"request");
}
