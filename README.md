# Crate Idempo

Idempotency primitives for Rust — safely ensure operations execute only once.

Useful when building APIs, background jobs, or distributed systems where retries may happen and duplicate side effects must be avoided.

## Why idempotency?

In real systems, the same operation may be triggered multiple times:

- network retries
- client resubmissions
- worker crashes and restarts
- concurrent requests
- at-least-once message delivery

Idempotency ensures that repeating the same operation does not produce unintended side effects.

Typical examples:
- charging a payment only once
- creating a resource only once
- processing a job exactly once
- ensuring safe retries in distributed systems

## Features

- simple idempotency primitives
- designed for concurrent environments
- usable in APIs, services, or background workers
- minimal dependencies
- production-oriented design

## Example

```rust
use idempo::IdempotencyStore;

let store = IdempotencyStore::new();

let key = "payment:123";

let result = store.execute_once(key, || {
    // side effect here
    42
});

assert_eq!(result, 42);
```

## Use cases

- REST APIs with retry-safe endpoints
- background job processors
- distributed systems
- message queue consumers
- payment workflows
- webhook processing

## Goals of this crate

This crate aims to provide simple and reliable idempotency primitives that can be composed into real-world Rust services.

Focus:
- correctness
- clarity
- production readiness
- minimal API surface


## Status
Early stage. API may evolve.

## License

MIT
