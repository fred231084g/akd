// Copyright (c) Meta Platforms, Inc. and affiliates.
//
// This source code is licensed under both the MIT license found in the
// LICENSE-MIT file in the root directory of this source tree and the Apache
// License, Version 2.0 found in the LICENSE-APACHE file in the root directory
// of this source tree.

//! This module contains verification calls for different proofs contained in the AKD crate

pub mod base;
pub mod history;
pub mod lookup;

#[cfg(feature = "nostd")]
use alloc::format;
#[cfg(feature = "nostd")]
use alloc::string::String;
#[cfg(feature = "nostd")]
use alloc::string::ToString;

/// Proof verification error types
#[derive(Debug, Eq, PartialEq)]
pub enum VerificationError {
    /// Error verifying a membership proof
    MembershipProof(String),
    /// Error verifying a non-membership proof
    NonMembershipProof(String),
    /// Error verifying a lookup proof
    LookupProof(String),
    /// Error verifying a history proof
    HistoryProof(String),
    /// Error hashing during verification
    Hash(crate::hash::HashError),
    /// Error verifying a VRF proof
    #[cfg(feature = "vrf")]
    Vrf(crate::ecvrf::VrfError),
    /// Error converting protobuf types during verification
    #[cfg(feature = "protobuf")]
    Serialization(crate::proto::ConversionError),
}

impl core::fmt::Display for VerificationError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let code = match &self {
            VerificationError::MembershipProof(err) => format!("(Membership proof) - {}", err),
            VerificationError::NonMembershipProof(err) => {
                format!("(Non-membership proof) - {}", err)
            }
            VerificationError::LookupProof(err) => format!("(Lookup proof) - {}", err),
            VerificationError::HistoryProof(err) => format!("(History proof) - {}", err),
            VerificationError::Hash(hash) => hash.to_string(),
            #[cfg(feature = "vrf")]
            VerificationError::Vrf(vrf) => vrf.to_string(),
            #[cfg(feature = "protobuf")]
            VerificationError::Serialization(proto) => proto.to_string(),
        };
        write!(f, "Verification error {}", code)
    }
}

#[cfg(feature = "vrf")]
impl From<crate::ecvrf::VrfError> for VerificationError {
    fn from(input: crate::ecvrf::VrfError) -> Self {
        VerificationError::Vrf(input)
    }
}

impl From<crate::hash::HashError> for VerificationError {
    fn from(input: crate::hash::HashError) -> Self {
        VerificationError::Hash(input)
    }
}

#[cfg(feature = "protobuf")]
impl From<crate::proto::ConversionError> for VerificationError {
    fn from(input: crate::proto::ConversionError) -> Self {
        VerificationError::Serialization(input)
    }
}

#[cfg(feature = "protobuf")]
impl From<protobuf::Error> for VerificationError {
    fn from(input: protobuf::Error) -> Self {
        let conv: crate::proto::ConversionError = input.into();
        conv.into()
    }
}

// Re-export the necessary verification functions
pub use base::{verify_membership, verify_nonmembership};
pub use history::{key_history_verify, HistoryVerificationParams};
pub use lookup::lookup_verify;
