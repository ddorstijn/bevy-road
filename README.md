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

Every road consists of a bunch of a bunc of nodes connected by one or more connection. 

1 connection gives a dead end.
2 connections gives a joint of a road.
3 or more connections gives a junction.

Connections do not necessarily end at an intersection. Instead the node creates a mesh connecting all the connections together.
This allows for fine-grained control of the placement of roads and junctions.
