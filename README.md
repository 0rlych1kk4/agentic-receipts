# agentic-receipts

Cryptographically signed, tamper-evident execution receipts for AI agents,
tool calls, GPU workloads, and distributed jobs.

`agentic-receipts` creates verifiable records of agent and workload execution.
Each receipt contains hashes of the input and output, execution metadata, an
Ed25519 public key, and a digital signature.

## Features

- Ed25519 receipt signing and verification
- SHA-256 input and output hashing
- Tamper detection
- Wrong-public-key rejection
- JSON-serializable receipts
- Unique receipt IDs and nonces
- Optional receipt chaining
- Execution timing and latency metadata
- Support for agent, tool-call, and distributed workload records

## Why execution receipts?

AI agents and distributed workloads often execute actions across services,
tools, models, and infrastructure providers.

Traditional logs record what a system claims happened, but they may be edited
afterward. A signed execution receipt provides cryptographic evidence that the
record has not been modified since it was signed.

Possible use cases include:

- AI-agent tool-call auditing
- Multi-agent workflow verification
- GPU workload records
- Distributed task execution
- Usage and billing evidence
- Security and compliance audit trails
- Verifiable automation pipelines

## Receipt structure

A signed receipt can contain:

```json
{
  "version": "0.1.0",
  "receipt_id": "663548a6-8b55-488d-b163-c6144767f83d",
  "agent_id": "risk-agent-001",
  "task_id": "task-001",
  "action_type": "ToolCall",
  "tool_name": "risk_summarizer",
  "model_id": "local-llm-v1",
  "input_hash": "sha256:b520c4a7fb8f7b300810ea9deabff01cef99aaa996b647dc4392a37ca6bdb746",
  "output_hash": "sha256:96984f8fea5d2657dd26aba03464099613d519c597bbf7a740b6e46cccf28d6f",
  "started_at": "2026-07-08T13:44:20.444984Z",
  "completed_at": "2026-07-08T13:44:20.444986Z",
  "latency_ms": 120,
  "nonce": "e908d810-3171-45e2-88d9-94aca83b6829",
  "previous_receipt_hash": null,
  "signature_algorithm": "ed25519",
  "public_key_hex": "556cc2a4c59e91179f71a908869b3ef8e6d4897843ff3ea933e9250b06d91b86",
  "signature_hex": "f06c92b2bc0716dc23cb75ae073152ba523672e4c8ed2fdfb42b20152559718ceb8af73cb0e6dab681074227808a4101beeed279569e2d135fbf5f5fe50a150c"
}
```

The values above are an example. Receipt IDs, nonces, keys, timestamps, hashes,
and signatures are generated at runtime.

## Installation

Until the crate is published to crates.io:

```toml
[dependencies]
agentic-receipts = { git = "https://github.com/0rlych1kk4/agentic-receipts" }
```

## Example

Run the included example:

```bash
cargo run --example create_and_verify
```

Expected output:

```bash
Receipt verified successfully.
Receipt hash: sha256:...
```

The complete example is available in
[`examples/create_and_verify.rs`](./examples/create_and_verify.rs).

## Verification guarantees

Successful verification confirms that:

- the receipt was signed by the private key corresponding to the included or
- trusted Ed25519 public key;
- the signed receipt content has not been modified;
- the input and output hashes remain part of the signed record;
- verification fails when receipt fields are changed;
- verification fails when an unrelated public key is supplied.

## Important limitation

A valid receipt proves the authenticity and integrity of the signed statement.
It does not independently prove that an external action, physical event, model
execution, or GPU computation actually occurred exactly as described. The
signer and the process generating the receipt must still be trusted to observe
and record the execution correctly.

## Receipt chaining

The optional previous_receipt_hash field can connect one receipt to an earlier
receipt.

This can be used to create a tamper-evident sequence of:

- agent actions;
- workflow stages;
- distributed jobs;
- tool calls;
- compute tasks.

Changing or removing an earlier receipt can then break the expected chain.

## Testing

Run the full test suite:

```bash
cargo test
```

The integration tests currently cover:

- successful receipt signing and verification;
- rejection of a tampered receipt;
- rejection when verifying with the wrong public key.

For additional checks:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
```

## Security considerations

Applications using this crate remain responsible for:

- protecting signing keys;
- establishing trust in public keys;
- securely distributing and rotating keys;
- preventing receipt replay where required;
- checking timestamps and receipt freshness;
- enforcing unique receipt IDs and nonces;
- defining authorization and retention policies;
- validating the truthfulness of the recorded execution metadata.

Private signing keys should never be serialized into receipts, logs, source
code, or public repositories.

See [SECURITY.md](./SECURITY.md) for vulnerability reporting instructions.

## Project status

`agentic-receipts` is currently an early-stage project.

Review the API, serialization format, key-management model, and threat model
before using it for production-critical, financial, or compliance-sensitive
workloads.

## Roadmap

Potential future improvements include:

- stable canonical receipt serialization
- key identifiers and key rotation
- receipt expiration
- replay-protection helpers
- receipt-chain verification
- batch verification
- Merkle-tree receipt aggregation
- optional JSON and CBOR formats
- external key-management integrations
- agent-framework integration examples
- GPU attestation and verifiable-compute integrations

## Contributing

Contributions are welcome.

Please keep pull requests focused and include tests for security-sensitive
behavior.

Before submitting a pull request:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
```

## License

Licensed under either of:

- Apache License, Version 2.0
- MIT License
