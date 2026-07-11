# Security Policy

## Supported versions

The following versions currently receive security updates:

| Version | Supported |
| ------- | --------- |
| 0.1.x   | Yes       |
| < 0.1   | No        |

## Reporting a vulnerability

Please do not report security vulnerabilities through public GitHub issues.

Instead, use GitHub's private vulnerability reporting feature for this
repository, if available, or contact the maintainer privately.

When reporting a vulnerability, include:

- a clear description of the issue;
- affected versions;
- reproduction steps or a minimal proof of concept;
- the expected and actual behavior;
- any known impact;
- suggested remediation, if available.

Please avoid including real secrets, private keys, access tokens, or sensitive
user data in the report.

## Response expectations

The maintainer will aim to:

- acknowledge the report within 7 days;
- investigate and assess the impact;
- coordinate a fix and release when appropriate;
- credit the reporter, unless anonymity is requested.

Response times may vary because this is an early-stage open-source project.

## Security status

`agentic-receipts` is experimental and has not undergone an independent
security audit.

Do not use it for high-stakes production, financial, or compliance-sensitive
workloads without appropriate review, testing, and key-management controls.

## Scope

Security reports are especially relevant for issues involving:

- signature verification bypasses;
- receipt tampering not being detected;
- private key exposure;
- unsafe serialization or parsing;
- replay or receipt-chain weaknesses;
- dependency-related vulnerabilities.
