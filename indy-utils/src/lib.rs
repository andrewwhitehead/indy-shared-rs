pub extern crate aead;

pub extern crate hex;

#[cfg(any(feature = "serde_support", test))]
#[macro_use]
pub extern crate serde;

#[macro_use]
pub extern crate zeroize;

/// Common macros
#[macro_use]
mod macros;

mod error;
pub use error::{ConversionError, EncryptionError, UnexpectedError, ValidationError};

/// Trait for qualifiable identifier types, having an optional prefix and method
#[macro_use]
pub mod qualifiable;
pub use qualifiable::Qualifiable;

/// Trait definition for validatable data types
#[macro_use]
mod validation;
pub use validation::Validatable;

/// base58 encoding and decoding
pub mod base58;

/// Indy DID representation and derivation
pub mod did;

/// Indy signing keys and verification keys
pub mod keys;

/// Random number generation
pub mod random;

/// Base64 encoding and decoding
#[cfg(feature = "base64")]
pub mod base64;

/// Hash algorithms
#[cfg(feature = "hash")]
pub mod hash;

/// Message packing and unpacking
#[cfg(feature = "pack")]
pub mod pack;

/// Generation of normalized ledger transaction for signing
#[cfg(feature = "txn_signature")]
pub mod txn_signature;

/// Wallet query language
#[cfg(feature = "wql")]
pub mod wql;

/// Re-export ursa to avoid version conflicts
#[cfg(any(
    feature = "cl",
    feature = "cl_native",
    feature = "ed25519",
    feature = "hash",
    feature = "pack",
    feature = "wallet_key"
))]
pub extern crate ursa;

pub extern crate once_cell;
