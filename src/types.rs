use std::borrow::Cow;
use std::ops::Deref;

use fluent_uri::UriRef;
use serde::de::{self, Deserializer};
use serde::{Deserialize, Serialize};

use crate::Error;
use crate::agent::Agent;
use crate::enums::{AriadneSubject, DataType};

pub type Agents<'a> = MinLengthVec<1, Agent<'a>>;
pub type AriadneSubjects = MinLengthVec<1, AriadneSubject>;
pub type DataTypes = MinLengthVec<1, DataType>;
pub type NativeSubjects<'a> = MinLengthVec<1, Cow<'a, str>>;
pub type NativeTypes<'a> = MinLengthVec<1, Cow<'a, str>>;

#[derive(Clone, Debug, Serialize)]
pub struct MinLengthVec<const LEN: usize, T>(Vec<T>);

impl<const LEN: usize, T> MinLengthVec<LEN, T> {
    /// # Errors
    /// If the provided vector has fewer than `LEN` elements, returns an `Error::EmptyVec`.
    pub fn new(vec: Vec<T>) -> Result<Self, Error> {
        if vec.len() < LEN {
            return Err(Error::EmptyVec {
                field: "MinLengthVec",
                min: LEN,
            });
        }
        Ok(Self(vec))
    }

    #[must_use]
    pub fn into_inner(self) -> Vec<T> {
        self.0
    }

    pub fn push(&mut self, value: T) {
        self.0.push(value);
    }

    pub fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.0.extend(iter);
    }

    pub fn pop(&mut self) -> Option<T> {
        (self.0.len() > LEN).then(|| self.0.pop()).flatten()
    }

    pub fn truncate(&mut self, len: usize) {
        self.0.truncate(len.max(LEN));
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.0
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        (self.0.len() > LEN && index < self.0.len())
            .then(|| Some(self.0.remove(index)))
            .flatten()
    }
}

impl<const LEN: usize, T> Deref for MinLengthVec<LEN, T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'de, const LEN: usize, T> Deserialize<'de> for MinLengthVec<LEN, T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let vec = Vec::<T>::deserialize(deserializer)?;
        Self::new(vec).map_err(de::Error::custom)
    }
}

impl<const LEN: usize, T> IntoIterator for MinLengthVec<LEN, T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<const LEN: usize, T> TryFrom<Vec<T>> for MinLengthVec<LEN, T> {
    type Error = Error;

    fn try_from(v: Vec<T>) -> Result<Self, Self::Error> {
        Self::new(v)
    }
}

/// Wrapper around a string for ensuring that a Uri is valid (using the `fluent-uri` crate). Derefs
/// to the inner string after construction.
#[derive(Clone, Debug, Serialize)]
pub struct Uri<'a>(Cow<'a, str>);

impl Deref for Uri<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> TryFrom<Cow<'a, str>> for Uri<'a> {
    type Error = Error;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        match value {
            Cow::Borrowed(s) => Uri::try_from(s),
            Cow::Owned(s) => Uri::try_from(s),
        }
    }
}

impl<'a, 's> TryFrom<&'s str> for Uri<'a>
where
    's: 'a,
{
    type Error = Error;

    /// This implementation does not catch empty strings, on the basis that individual structs can
    /// deserialize these values as `None` if desired.
    fn try_from(value: &'s str) -> Result<Self, Self::Error> {
        let (start_bytes, trimmed_byte_len) = {
            let trimmed = value.trim();

            if trimmed.is_empty() {
                return Err(Error::InvalidUri(value.to_string()));
            }

            let trimmed_byte_len = trimmed.len();

            /* Early return if can't be parsed as a Uri. Returns human-readable error.
             * Comes after byte_len check, as this is a more expensive validation check.
             * Maps a Error::InvalidUri error directly, to ensure format match on
             * `TryFrom<String>` implementation used by the other `visit_...` functions. */
            if UriRef::parse(trimmed).is_err() {
                return Err(Error::InvalidUri(value.to_string()));
            }

            /* trimmed will only ever start at the same ptr as s or later, so tells us how
             * far along to push the trimmed buffer. */
            let start = trimmed.as_ptr() as usize - value.as_ptr() as usize;
            (start, trimmed_byte_len)
        };

        if trimmed_byte_len == value.len() {
            // wasn't shorter when trimmed (i.e. no whitespace)
            Ok(Uri(Cow::Borrowed(value)))
        } else {
            // was shorter when trimmed, return just the slice for the inner content
            Ok(Uri(Cow::Borrowed(
                &value[start_bytes..start_bytes + trimmed_byte_len],
            )))
        }
    }
}

impl TryFrom<String> for Uri<'_> {
    type Error = Error;

    /// This implementation does not catch empty strings, on the basis that individual structs can
    /// deserialize these values as `None` if desired.
    fn try_from(mut value: String) -> Result<Self, Self::Error> {
        let (start_bytes, byte_len) = {
            let trimmed = value.trim();

            if trimmed.is_empty() {
                return Err(Error::InvalidUri(value.clone()));
            }

            let byte_len = trimmed.len();

            // Early return if can't be parsed as a Uri. Returns human-readable error.
            // Comes after byte_len check, as this is a more expensive validation check.
            if UriRef::parse(trimmed).is_err() {
                return Err(Error::InvalidUri(value.clone()));
            }

            // trimmed will only ever start at the same ptr as s or later, so tells us how far along
            // to push the trimmed buffer.
            let start = trimmed.as_ptr() as usize - value.as_ptr() as usize;
            (start, byte_len)
        };

        if byte_len == value.len() {
            // wasn't shorted when trimmed
            return Ok(Uri(Cow::Owned(value)));
        }

        value.truncate(start_bytes + byte_len); // truncate in place.
        value.drain(..start_bytes);
        Ok(Uri(Cow::Owned(value)))
    }
}

impl<'de, 'a> Deserialize<'de> for Uri<'a>
where
    'de: 'a,
{
    // Visitor implementations are relatively costly here (references are cloned before any
    // validation). This has been done to reduce boilerplate and make the code more readable.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct UriVisitor;

        impl<'de> serde::de::Visitor<'de> for UriVisitor {
            type Value = Uri<'de>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "A valid URI")
            }

            // definitely borrowed. contiguous bytes with no escapes to skip in the buffer.
            fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Uri::try_from(v).map_err(|e| serde::de::Error::custom(e))
            }

            /* temporary, holding buffer that will be dropped quickly (e.g. if serde_json has
             * replaced \r\n with actual bytes, therefore making the buffer non-contiguous).
             * Our only real choice here is to cast it straight to a `String`. */
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let v = v.to_string();

                // Pass the String straight to TryFrom. Error messages return the same user-facing
                // message and type through mapping.
                Uri::try_from(v).map_err(|e| serde::de::Error::custom(e))
            }

            // definitely owned.
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                // Pass the String straight to TryFrom. Error messages return the same user-facing
                // message and type through mapping.
                Uri::try_from(v).map_err(|e| serde::de::Error::custom(e))
            }
        }

        deserializer.deserialize_str(UriVisitor)
    }
}
