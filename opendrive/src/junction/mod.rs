// use crate::junction::connection::Connection;
// use crate::junction::controller::Controller;
// use crate::junction::priority::Priority;
// use crate::junction::surface::Surface;
// use crate::object::orientation::Orientation;
// use junction_type::JunctionType;
// use uom::si::f64::Length;
// use vec1::Vec1;

// pub mod connection;
// pub mod connection_type;
pub mod contact_point;
// pub mod controller;
pub mod element_dir;
// pub mod junction_group;
// pub mod junction_group_type;
// pub mod junction_reference;
// pub mod junction_type;
// pub mod lane_link;
// pub mod predecessor_successor;
// pub mod priority;
// pub mod surface;

// #[derive(Debug, Clone, PartialEq)]
// pub struct Junction {
//     pub connection: Vec1<Connection>,
//     pub priority: Vec<Priority>,
//     pub controller: Vec<Controller>,
//     /// Unique ID within database
//     #[serde(rename = "@id")]
//     pub id: String,
//     /// The main road from which the connecting roads of the virtual junction branch off. This
//     /// attribute is mandatory for virtual junctions and shall not be specified for other junction
//     /// types.
//     #[serde(rename = "@mainRoad")]
//     pub main_road: Option<String>,
//     /// Name of the junction. May be chosen freely.
//     #[serde(rename = "@name")]
//     pub name: Option<String>,
//     /// Defines the relevance of the virtual junction according to the driving direction. This
//     /// attribute is mandatory for virtual junctions and shall not be specified for other junction
//     /// types. The enumerator "none" specifies that the virtual junction is valid in both
//     /// directions.
//     #[serde(rename = "@orientation")]
//     pub orientation: Option<Orientation>,
//     /// End position of the virtual junction in the reference line coordinate system. This attribute
//     /// is mandatory for virtual junctions and shall not be specified for other junction types.
//     #[serde(rename = "@sEnd")]
//     pub s_end: Option<Length>,
//     /// Start position of the virtual junction in the reference line coordinate system. This
//     /// attribute is mandatory for virtual junctions and shall not be specified for other junction
//     /// types.
//     #[serde(rename = "@sStart")]
//     pub s_start: Option<Length>,
//     /// Type of the junction. Common junctions are of type "default". This attribute is mandatory
//     /// for virtual junctions and direct junctions. If the attribute is not specified, the junction
//     /// type is "default".
//     #[serde(rename = "@type")]
//     pub r#type: Option<JunctionType>,
// }
