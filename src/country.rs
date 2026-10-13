use serde::{Deserialize, Serialize};

use crate::Error;

#[derive(Debug, Default)]
pub struct CountryBuilder {
    name: Option<String>,
    qid: Option<u32>,
}

impl CountryBuilder {
    pub fn new() -> Self {
        CountryBuilder {
            name: None,
            qid: None,
        }
    }

    pub fn set_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn set_qid(mut self, qid: u32) -> Self {
        self.qid = Some(qid);
        self
    }

    pub fn build(self) -> Result<Country, Error> {
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

#[derive(Debug, Deserialize, Serialize)]
pub struct Country {
    name: String,
    qid: u32,
}

impl Country {
    pub fn new(name: String, qid: u32) -> Self {
        Country { name, qid }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn qid(&self) -> u32 {
        self.qid
    }
}
