pub struct SpriteBatch {
    pub quads:Vec<[Vertex;4]>,
    pub indices:Vec<u16>,
    //TODO make internals private
    //Render via trait
}

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
    tex_coord: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coord);

impl SpriteBatch {
    pub fn new() -> SpriteBatch {

        SpriteBatch { quads : Vec::new(), indices : Vec::new() }
    }

    pub fn add(&mut self, width: f32, height:f32) {
        self.quads.push(SpriteBatch::quad(width, height));
        let i = self.indices.len() as u16;
        self.indices.push(i);
        self.indices.push(i+1);
        self.indices.push(i+2);
        self.indices.push(i);
        self.indices.push(i+3);
        self.indices.push(i+2);
    }


    fn quad(width: f32, height:f32) -> [Vertex; 4] {
        [
            Vertex { position: [ 0.0, 0.0 ], tex_coord: [ 0.0, 0.0 ] },
            Vertex { position: [ 0.0, height ], tex_coord: [ 0.0, 1.0 ] },
            Vertex { position: [ width, height ], tex_coord: [ 1.0, 1.0 ] },
            Vertex { position: [ width, 0.0 ], tex_coord: [ 1.0, 0.0 ] },
        ]    
    }


}