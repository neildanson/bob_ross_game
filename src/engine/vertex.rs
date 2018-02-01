#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coord: [f32; 2],
}

//TODO Implement in renderer
implement_vertex!(Vertex, position, tex_coord);
