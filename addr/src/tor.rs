// Set of libraries for privacy-preserving networking apps
//
// SPDX-License-Identifier: Apache-2.0
//
// Written in 2019-2023 by
//     Dr. Maxim Orlovsky <orlovsky@cyphernet.org>
//
// Copyright 2022-2023 Cyphernet Initiative, Switzerland
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

use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

use base32::Alphabet;
use cypher::ed25519::PublicKey;
use cypher::{EcPk, EcPkInvalid};
use sha3::Digest;

const ALPHABET: Alphabet = Alphabet::RFC4648 { padding: false };
pub const ONION_V3_RAW_LEN: usize = 35;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(into = "String", try_from = "String")
)]
pub struct OnionAddrV3 {
    pk: PublicKey,
    checksum: u16,
}

impl From<PublicKey> for OnionAddrV3 {
    fn from(pk: PublicKey) -> Self {
        let mut h = sha3::Sha3_256::default();
        h.update(b".onion checksum");
        h.update(&pk[..]);
        h.update([3u8]);
        let hash = h.finalize();
        let checksum = u16::from_le_bytes([hash[0], hash[1]]);
        Self { pk, checksum }
    }
}

impl From<OnionAddrV3> for PublicKey {
    fn from(onion: OnionAddrV3) -> Self { onion.pk }
}

impl OnionAddrV3 {
    pub fn into_public_key(self) -> PublicKey { self.pk }

    pub fn into_raw_bytes(self) -> [u8; ONION_V3_RAW_LEN] {
        let mut data = [0u8; ONION_V3_RAW_LEN];
        data[..32].copy_from_slice(self.pk.as_slice());
        data[32..34].copy_from_slice(&self.checksum.to_le_bytes());
        data[ONION_V3_RAW_LEN - 1] = 3;
        data
    }

    pub fn checksum(self) -> u16 { self.checksum }
}

#[derive(Clone, Eq, PartialEq, Debug, Display, Error, From)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[display(doc_comments)]
pub enum OnionAddrError {
    /// onion address {0} doesn't end with `.onion` suffix.
    NoSuffix(String),

    /// onion address {0} has an invalid base32 encoding.
    InvalidBase32(String),

    /// onion address {0} has an invalid length.
    InvalidLen(String),

    /// version {1} encoded in address {0} doesn't match V3.
    VersionMismatch(String, u8),

    /// onion address {addr} has an invalid checksum {found} instead of {expected}.
    InvalidChecksum {
        expected: u16,
        found: u16,
        addr: String,
    },

    /// address contains invalid public key
    #[from(EcPkInvalid)]
    InvalidKey,
}

impl FromStr for OnionAddrV3 {
    type Err = OnionAddrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let stripped =
            s.strip_suffix(".onion").ok_or_else(|| OnionAddrError::NoSuffix(s.to_owned()))?;
        let data: Vec<u8> = base32::decode(ALPHABET, stripped)
            .ok_or_else(|| OnionAddrError::InvalidBase32(s.to_owned()))?;
        if data.len() != ONION_V3_RAW_LEN {
            return Err(OnionAddrError::InvalidLen(s.to_owned()));
        }
        let ver = data[ONION_V3_RAW_LEN - 1];
        if ver != 3 {
            return Err(OnionAddrError::VersionMismatch(s.to_owned(), ver));
        }
        let mut key = [0u8; 32];
        key.copy_from_slice(&data[..32]);
        let pk = OnionAddrV3::from(PublicKey::from_pk_compressed(key)?);
        let checksum = u16::from_le_bytes([data[32], data[33]]);
        if pk.checksum != checksum {
            return Err(OnionAddrError::InvalidChecksum {
                expected: pk.checksum,
                found: checksum,
                addr: s.to_owned(),
            });
        }
        Ok(pk)
    }
}

impl Display for OnionAddrV3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let b32 = base32::encode(ALPHABET, &self.into_raw_bytes());
        write!(f, "{}.onion", b32)
    }
}

impl From<OnionAddrV3> for String {
    fn from(other: OnionAddrV3) -> Self { other.to_string() }
}

impl TryFrom<String> for OnionAddrV3 {
    type Error = OnionAddrError;

    fn try_from(value: String) -> Result<Self, Self::Error> { Self::from_str(&value) }
}
