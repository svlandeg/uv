use std::fmt::Display;
use std::str::{self, FromStr};

use thiserror::Error;

/// A complete Git SHA, i.e., a 40-character hexadecimal representation of a Git commit.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GitSha(GitOid);

impl GitSha {
    /// Convert the SHA to a truncated representation, i.e., the first 16 characters of the SHA.
    pub fn to_short_string(&self) -> String {
        self.0.to_string()[0..16].to_string()
    }
}

impl From<GitSha> for GitOid {
    fn from(value: GitSha) -> Self {
        value.0
    }
}

impl From<GitOid> for GitSha {
    fn from(value: GitOid) -> Self {
        Self(value)
    }
}

impl Display for GitSha {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for GitSha {
    type Err = OidParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(GitOid::from_str(value)?))
    }
}

impl serde::Serialize for GitSha {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.as_str().serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for GitSha {
    fn deserialize<D>(deserializer: D) -> Result<GitSha, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        GitSha::from_str(&value).map_err(serde::de::Error::custom)
    }
}

/// Unique identity of any Git object (commit, tree, blob, tag).
///
/// Note this type does not validate whether the input is a valid hash.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GitOid {
    len: usize,
    bytes: [u8; 40],
}

impl GitOid {
    /// Return the string representation of an object ID.
    pub(crate) fn as_str(&self) -> &str {
        str::from_utf8(&self.bytes[..self.len]).unwrap()
    }
}

#[derive(Debug, Error, PartialEq)]
pub enum OidParseError {
    #[error("Object ID can be at most 40 hex characters")]
    TooLong,
    #[error("Object ID cannot be parsed from empty string")]
    Empty,
}

impl FromStr for GitOid {
    type Err = OidParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(OidParseError::Empty);
        }

        if s.len() > 40 {
            return Err(OidParseError::TooLong);
        }

        let mut out = [0; 40];
        out[..s.len()].copy_from_slice(s.as_bytes());

        Ok(GitOid {
            len: s.len(),
            bytes: out,
        })
    }
}

impl Display for GitOid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests;
