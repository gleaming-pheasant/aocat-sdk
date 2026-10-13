use serde::{Deserialize, Serialize, Serializer};
use wkt::ToWkt;

/// Wrapper around `geojson::Geometry` to implement `Serialize` (as WKT) and `Deserialize` (from
/// GeoJSON).
#[derive(Clone, Debug, Deserialize)]
pub struct Geometry(geojson::Geometry);

impl Geometry {
    /// Create a new `Geometry` from a `geojson::Geometry`.
    pub fn new(geometry: geojson::Geometry) -> Self {
        Geometry(geometry)
    }
}

impl Serialize for Geometry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let wkt_string = geo_types::Geometry::<f64>::try_from(&self.0)
            .map_err(serde::ser::Error::custom)?
            .to_wkt()
            .to_string();

        serializer.serialize_str(&wkt_string)
    }
}
