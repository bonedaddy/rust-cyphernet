// Set of libraries for privacy-preserving networking apps
//
// SPDX-License-Identifier: Apache-2.0
//
// Written in 2023 by
//     Dr. Maxim Orlovsky <orlovsky@cyphernet.org>
//
// Copyright 2023 Cyphernet Initiative, Switzerland
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use cypher::{EcPkInvalid, EcSkInvalid, EcdhError};

#[derive(Clone, Eq, PartialEq, Debug, Display, Error, From)]
#[display(doc_comments)]
pub enum EncryptionError {
    /// message length {0} exceeds maximum size allowed for the encryption
    /// protocol frame.
    ExceedingMaxLength(usize),

    /// invalid keys for ECDH: {0}
    #[from]
    Ecdh(EcdhError),

    /// invalid remote public key
    #[from]
    InvalidPk(EcPkInvalid),

    /// invalid local secret key
    #[from]
    InvalidSk(EcSkInvalid),

    /// ChaCha20Poly1305 AEAD encryptor error.
    #[from]
    ChaCha(chacha20poly1305::aead::Error),
}

#[derive(Debug, Clone, PartialEq, Eq, Display, Error, From)]
#[display(doc_comments)]
pub enum HandshakeError {
    /// unexpected version of noise protocol {version} in act {act} of handshake.
    UnexpectedVersion { version: u8, act: u8 },

    /// invalid remote ephemeral pubkey provided during noise handshake.
    InvalidEphemeralPubkey,

    /// the initiator has provided an invalid pubkey
    InvalidInitiatorPubkey,

    /// invalid length of handshake act {act}: expected {expected}, provided {found}
    InvalidActLen {
        act: u8,
        expected: usize,
        found: usize,
    },

    #[from]
    #[from(chacha20poly1305::aead::Error)]
    #[display(inner)]
    Encryption(EncryptionError),

    /// noise handshake is complete, nothing to process.
    Complete,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Display, Error)]
#[display("incomplete Noise handshake")]
pub struct IncompleteHandshake;

#[derive(Clone, Eq, PartialEq, Debug, Display, Error, From)]
#[display(doc_comments)]
pub enum NoiseError {
    /// received a non-empty payload from the remote peer when an empty payload
    /// was expected
    PayloadNotEmpty,

    /// handshake is complete, no further advance is possible
    HandshakeComplete,

    #[display(inner)]
    #[from]
    Encryption(EncryptionError),
}
