``` rust
// Marker node for pbr mesh
struct RoadNode {
	connections: Vec<Entity>,
}

// Holds a reference to the start and end nodes
// Determines if the connection is a straight line or a curve
struct RoadEdge {
	start: Entity,
	end: Entity,
	
	center: Option<Center>,
	lanes: u32,
	length: u32,
}

// Marker component for pbr mesh. Should be child of RoadEdge
struct RoadController;
```

There are building and simulation phases.

Road graph consist of grouped nodes and edges. This means in the graph there is no difference between lanes, but the edge defines this internally. Instead lane switching is kept internally to the edge itself. Cars can steer towards the lane they are in using the closest point on the circle the arc makes. Cars are parented to the edge they are currently driving on.

Maybe implement a lookup/register function to occupy a position on a lane and to see further ahead?
 