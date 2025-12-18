# LIT Rust Crypto

[![Crates.io](https://img.shields.io/crates/v/lit-rust-crypto.svg)](https://crates.io/crates/lit-rust-crypto)
[![Documentation](https://docs.rs/lit-rust-crypto/badge.svg)](https://docs.rs/lit-rust-crypto)
[![License](https://img.shields.io/crates/l/lit-rust-crypto.svg)](LICENSE)

A convenience wrapper around cryptography libraries for [LIT Protocol](https://litprotocol.com). This crate consolidates all elliptic curve libraries used by LIT projects to keep versioning consistent across the ecosystem.

## Supported Curves

| Curve | Crate | Description |
|-------|-------|-------------|
| **BLS12-381** | `bls12_381_plus`, `blsful`, `blstrs_plus` | Pairing-friendly curve for BLS signatures |
| **Curve25519** | `curve25519-dalek-ml` | Fast curve for Ed25519/X25519 |
| **Decaf377** | `decaf377_plus` | Ristretto group over BLS12-377 |
| **Ed448-Goldilocks** | `ed448-goldilocks-plus` | High-security curve (448-bit) |
| **JubJub** | `jubjub-plus` | Embedded curve over BLS12-381 |
| **secp256k1** | `k256` | Bitcoin/Ethereum curve |
| **P-256** | `p256` | NIST P-256 curve |
| **P-384** | `p384` | NIST P-384 curve |
| **Pasta** | `pasta_curves_plus` | Pallas and Vesta curves for recursive SNARKs |

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
lit-rust-crypto = "0.6"
```

Or with specific features:

```toml
[dependencies]
lit-rust-crypto = { version = "0.6", default-features = false, features = ["k256", "p256"] }
```

## Usage

The crate re-exports all curve libraries at the top level:

```rust
use lit_rust_crypto::{k256, p256, elliptic_curve};

// Use secp256k1
use lit_rust_crypto::k256::ecdsa::{SigningKey, Signature};

// Use P-256
use lit_rust_crypto::p256::SecretKey;

// Access common traits
use lit_rust_crypto::group::Group;
use lit_rust_crypto::ff::Field;
```

### BLS Signatures

```rust
use lit_rust_crypto::blsful;
// BLS signature operations via blsful
```

### Verifiable Secret Sharing

```rust
use lit_rust_crypto::vsss_rs;
// Shamir secret sharing and more
```

## Features

### Curve Selection Features

Enable only the curves you need:

| Feature | Description |
|---------|-------------|
| `blst` | BLS12-381 via blst backend |
| `rust` | Pure Rust BLS12-381 implementation |
| `curve25519-dalek` | Curve25519 support |
| `decaf377` | Decaf377 curve |
| `ed448-goldilocks` | Ed448-Goldilocks curve |
| `jubjub` | JubJub curve |
| `k256` | secp256k1 curve |
| `p256` | NIST P-256 curve |
| `p384` | NIST P-384 curve |
| `pasta` | Pallas and Vesta curves |
| `vsss-rs` | Verifiable secret sharing |

### Functionality Features

| Feature | Description |
|---------|-------------|
| `alloc` | Enable alloc support (no_std compatible) |
| `std` | Enable std support |
| `arithmetic` | Field/group arithmetic operations |
| `bits` | Bit manipulation support |
| `digest` | Hash digest integration |
| `ecdh` | Elliptic curve Diffie-Hellman |
| `ecdsa` | ECDSA signatures |
| `hash2curve` | Hash-to-curve support |
| `jwk` | JSON Web Key support |
| `pkcs8` | PKCS#8 key encoding |
| `pem` | PEM encoding support |
| `schnorr` | Schnorr signatures (k256 only) |
| `serde` | Serialization support |
| `zeroize` | Secure memory zeroing |

### Default Features

The default feature set includes common curves and functionality:

- `blsful/default`
- `blst`
- `curve25519-dalek/default`
- `decaf377/default`
- `ed448-goldilocks/default`
- `elliptic-curve/default`
- `jubjub/default`
- `k256/default`
- `p256/default`
- `p384/default`
- `pasta/default`
- `vsss-rs/default`

## no_std Support

This crate supports `no_std` environments. Disable default features and enable only what you need:

```toml
[dependencies]
lit-rust-crypto = { version = "0.6", default-features = false, features = ["k256", "alloc"] }
```

## License

Licensed under either of:

- [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
- [MIT License](http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please see the [repository](https://github.com/LIT-Protocol/lit-rust-crypto) for more information.
