#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [i32; 2],
    pub tex_coord: [f32; 2],
}

//TODO Implement in renderer
implement_vertex!(Vertex, position, tex_coord);
