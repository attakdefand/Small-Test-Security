# ASVS v5 Checklist (starter)

- V2 AuthN: MFA, lockout, session rotation
- V3 Access Control: deny-by-default, context-aware checks
- V4 Input Validation: strict schemas, canonicalization
- V5 Crypto: TLS 1.2+, AEAD, key rotation
- V6 Errors/Logging: no secrets, tamper-evident logs
- V7 Data Protection: at-rest encryption, field-level controls
- V8 Communications: secure headers, CORS allowlist
- V9 HTTP: CSRF where relevant, cache policy
- V11 Business Logic: abuse stories + invariants

Map each item to concrete tests under `top10/` and `api_top10/`.
