#[derive(Copy, Clone)]
pub struct Vertex {
	position: (f32, f32, f32)
}

implement_vertex!(Vertex, position);

pub const vertices: [Vertex; 4] = [
	Vertex {position: (1.0, 1.0, 0.0)},
	Vertex {position: (-1.0, 1.0, 0.0)},
	Vertex {position: (-1.0, -1.0, 0.0)},
	Vertex {position: (1.0, -1.0, 0.0)}
];

pub const indices: [u16;6] = [
	0, 1, 3, 1, 2, 3
];
