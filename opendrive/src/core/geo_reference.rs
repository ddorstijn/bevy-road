use serde::{Serialize, Deserialize};

/// Spatial reference systems are standardized by the European Petroleum Survey Group Geodesy (EPSG)
/// and are defined by parameters describing the geodetic datum. A geodetic datum is a coordinate
/// reference system for a collection of positions that are relative to an ellipsoid model of the
/// earth.
/// A geodetic datum is described by a projection string according to PROJ, that is, a format for
/// the exchange of data between two coordinate systems. This data shall be marked as CDATA, because
/// it may contain characters that interfere with the XML syntax of an element’s attribute.
/// In ASAM OpenDRIVE, the information about the geographic reference of an ASAM OpenDRIVE dataset
/// is represented by the `<geoReference>` element within the `<header>` element.
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct GeoReference {
    /// https://proj.org/usage/projections.html
    pub proj: Option<String>,
}