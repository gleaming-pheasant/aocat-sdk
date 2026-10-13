use std::ops::Deref;

use fluent_uri::UriRef;
use serde::de::{self, Deserializer, Visitor};
use serde::{Deserialize, Serialize};

use crate::Error;
use crate::agent::Agent;
use crate::enums::{AriadneSubject, DataType};

pub type Agents = MinLengthVec<1, Agent>;
pub type AriadneSubjects = MinLengthVec<1, AriadneSubject>;
pub type DataTypes = MinLengthVec<1, DataType>;
pub type NativeSubjects = MinLengthVec<1, String>;
pub type NativeTypes = MinLengthVec<1, String>;

#[derive(Debug, Serialize)]
pub struct MinLengthVec<const LEN: usize, T>(Vec<T>);

impl<const LEN: usize, T> MinLengthVec<LEN, T> {
    pub fn new(vec: Vec<T>) -> Result<Self, Error> {
        if vec.len() < LEN {
            return Err(Error::EmptyVec {
                field: "MinLengthVec",
                min: LEN,
            });
        }
        Ok(MinLengthVec(vec))
    }

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
        MinLengthVec::new(vec).map_err(de::Error::custom)
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
        MinLengthVec::new(v)
    }
}

/// Wrapper around a string for ensuring that a Uri is valid (using the `fluent-uri` crate). Derefs
/// to the inner string after construction.
#[derive(Debug, Serialize)]
pub struct Uri(String);

impl Deref for Uri {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<String> for Uri {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if UriRef::parse(value.trim()).is_err() {
            return Err(Error::InvalidUri(value));
        }
        Ok(Uri(value))
    }
}

impl<'de> Deserialize<'de> for Uri {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        pub struct UriVisitor;

        impl<'de> Visitor<'de> for UriVisitor {
            type Value = Uri;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a valid URI string")
            }

            fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Uri::try_from(v.to_string()).map_err(de::Error::custom)
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Uri::try_from(value.to_string()).map_err(de::Error::custom)
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Uri::try_from(v).map_err(de::Error::custom)
            }
        }

        deserializer.deserialize_string(UriVisitor)
    }
}
