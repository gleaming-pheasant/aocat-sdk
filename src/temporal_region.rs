use serde::Deserialize;
use serde::de::{self, IgnoredAny, MapAccess, Visitor};
use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::Error;

const FIELDS: &[&str] = &["from", "until", "period_name"];

#[derive(Debug, Default)]
pub struct TemporalRegionBuilder {
    from: Option<i32>,
    until: Option<i32>,
    period_name: Option<String>,
}

impl TemporalRegionBuilder {
    pub fn new() -> Self {
        TemporalRegionBuilder {
            from: None,
            until: None,
            period_name: None,
        }
    }

    pub fn set_from(mut self, from: i32) -> Self {
        self.from = Some(from);
        self
    }

    pub fn set_until(mut self, until: i32) -> Self {
        self.until = Some(until);
        self
    }

    pub fn set_period_name(mut self, period_name: String) -> Self {
        self.period_name = Some(period_name);
        self
    }

    pub fn build(self) -> Result<TemporalRegion, Error> {
        match (self.from, self.until, self.period_name) {
            (Some(from), Some(until), Some(period_name)) => Ok(TemporalRegion::Both {
                from,
                until: Some(until),
                period_name,
            }),
            (Some(from), None, Some(period_name)) => Ok(TemporalRegion::Both {
                from,
                until: None,
                period_name,
            }),
            (Some(from), Some(until), None) => Ok(TemporalRegion::Period {
                from,
                until: Some(until),
            }),
            (Some(from), None, None) => Ok(TemporalRegion::Period { from, until: None }),
            _ => Err(Error::MissingField(
                "\"from\" or \"period_name\" must be provided",
            )),
        }
    }
}

#[derive(Debug)]
pub enum TemporalRegion {
    Period {
        from: i32,
        until: Option<i32>,
    },
    Name(String),
    Both {
        from: i32,
        until: Option<i32>,
        period_name: String,
    },
}

impl Serialize for TemporalRegion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            TemporalRegion::Period { from, until } => {
                let mut state = serializer.serialize_struct("TemporalRegion", 2)?;
                state.serialize_field("from", from)?;
                state.serialize_field("until", until)?;
                state.end()
            }
            TemporalRegion::Name(period_name) => {
                let mut state = serializer.serialize_struct("TemporalRegion", 1)?;
                state.serialize_field("period_name", period_name)?;
                state.end()
            }
            TemporalRegion::Both {
                from,
                until,
                period_name,
            } => {
                let mut state = serializer.serialize_struct("TemporalRegion", 3)?;
                state.serialize_field("from", from)?;
                state.serialize_field("until", until)?;
                state.serialize_field("period_name", period_name)?;
                state.end()
            }
        }
    }
}

impl<'de> Deserialize<'de> for TemporalRegion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct TemporalRegionVisitor;

        impl<'de> Visitor<'de> for TemporalRegionVisitor {
            type Value = TemporalRegion;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("an object with `from`, `until`, and/or `period_name` fields")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut from = None;
                let mut until = None;
                let mut period_name = None;

                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "from" => {
                            if from.is_some() {
                                return Err(de::Error::duplicate_field("from"));
                            }
                            from = Some(map.next_value::<i32>()?);
                        }
                        "until" => {
                            if until.is_some() {
                                return Err(de::Error::duplicate_field("until"));
                            }
                            until = Some(map.next_value::<Option<i32>>()?);
                        }
                        "period_name" => {
                            if period_name.is_some() {
                                return Err(de::Error::duplicate_field("period_name"));
                            }
                            period_name = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }

                match (from, until, period_name) {
                    (Some(from), until, Some(period_name)) => Ok(TemporalRegion::Both {
                        from,
                        until: until.unwrap_or(None),
                        period_name,
                    }),
                    (Some(from), until, None) => Ok(TemporalRegion::Period {
                        from,
                        until: until.unwrap_or(None),
                    }),
                    (None, None, Some(period_name)) => Ok(TemporalRegion::Name(period_name)),
                    (None, Some(_), _) => Err(de::Error::custom("\"until\" requires \"from\"")),
                    (None, None, None) => Err(de::Error::custom(
                        "\"from\" or \"period_name\" must be provided",
                    )),
                }
            }
        }

        deserializer.deserialize_struct("TemporalRegion", FIELDS, TemporalRegionVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::TemporalRegion;

    #[test]
    fn deserializes_period_region() {
        let region: TemporalRegion = serde_json::from_str(r#"{"from":2020,"until":null}"#).unwrap();

        assert!(matches!(
            region,
            TemporalRegion::Period {
                from: 2020,
                until: None
            }
        ));
    }

    #[test]
    fn deserializes_name_region() {
        let region: TemporalRegion = serde_json::from_str(r#"{"period_name":"modern"}"#).unwrap();

        assert!(matches!(region, TemporalRegion::Name(name) if name == "modern"));
    }

    #[test]
    fn deserializes_region_with_both_fields() {
        let region: TemporalRegion =
            serde_json::from_str(r#"{"from":2020,"until":2024,"period_name":"modern"}"#).unwrap();

        assert!(matches!(
            region,
            TemporalRegion::Both {
                from: 2020,
                until: Some(2024),
                period_name
            } if period_name == "modern"
        ));
    }

    #[test]
    fn rejects_region_without_required_fields() {
        assert!(serde_json::from_str::<TemporalRegion>(r#"{"until":2024}"#).is_err());
    }
}
