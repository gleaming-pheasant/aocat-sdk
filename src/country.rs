use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::Error;

#[derive(Debug, Default)]
pub struct CountryBuilder<'a> {
    name: Option<Cow<'a, str>>,
    qid: Option<u32>,
}

impl<'a> CountryBuilder<'a> {
    pub fn new() -> Self {
        CountryBuilder {
            name: None,
            qid: None,
        }
    }

    pub fn set_name(mut self, name: impl Into<Cow<'a, str>>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn set_qid(mut self, qid: u32) -> Self {
        self.qid = Some(qid);
        self
    }

    pub fn build(self) -> Result<Country<'a>, Error> {
        if self.name.is_none() {
            return Err(Error::MissingField("name"));
        }

        if self.qid.is_none() {
            return Err(Error::MissingField("qid"));
        }

        Ok(Country {
            name: self.name.unwrap(),
            qid: self.qid.unwrap(),
        })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Country<'a> {
    name: Cow<'a, str>,
    qid: u32,
}

impl Country<'static> {
    pub const fn new_const(name: &'static str, qid: u32) -> Self {
        Country {
            name: Cow::Borrowed(name),
            qid
        }
    }
}

impl<'a> Country<'a> {
    pub fn new(name: impl Into<Cow<'a, str>>, qid: u32) -> Self {
        Country {
            name: name.into(),
            qid
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn qid(&self) -> u32 {
        self.qid
    }
}
