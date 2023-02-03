``` rust
// Holds a Vec3 of the coordinate
struct Waypoint {
	position: Vec3,
}

// Holds references to a group of waypoints that form a group of lanes
struct ConnectionGroup {
	position: Vec3,
	rotation: f32,
	waypoints: &vec<Waypoint>,
}

// Holds
struct Intersection {
	incoming: vec<&WaypointGroup>,
	outgoing: vec<&WaypointGroup>,
}

// Holds a reference to the start and end waypoints
// Determines if the connection is a straight line or a curve
struct Connection {
	start: &Waypoint,
	end: &Wapoint
}
```


Generate a waypoint group
Use waypoints to generate road segments
Determine type of roadsegment
	- *Straight*
		No additional info needed. Traversal calculation is a lerp
	- *Curved*
		Determine center point. Traversal calculation is calculated along arc between waypoints.
	- *Intersection*
		Determine all incoming points. And all outgoing points. Traversal function is calculated
		by giving a start and end node. And calculating the arc or straight traversal function 
		between them.

Generate mesh
	Traverse connection