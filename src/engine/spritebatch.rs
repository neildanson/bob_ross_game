use engine::SpriteSheet;
use engine::Vertex;

pub struct SpriteBatch {
    pub quads: Vec<Vertex>,
    pub indices: Vec<u32>, /* TODO make internals private
                            * Render via trait */
}

impl SpriteBatch {
    pub fn new() -> SpriteBatch {

        SpriteBatch {
            quads: Vec::new(),
            indices: Vec::new(),
        }
    }

    //TODO - add a reference to the spritesheet!
    pub fn add(&mut self, x: f32, y: f32, sprite_index : usize, spritesheet:&SpriteSheet) {
        let i = self.quads.len() as u32;
        let sprite = spritesheet.coords(sprite_index);
        for v in SpriteBatch::quad(x, y, sprite.width, sprite.height, sprite.coords).iter() {
            self.quads.push(v.clone());
        }

        self.indices.push(i);
        self.indices.push(i + 1);
        self.indices.push(i + 2);
        self.indices.push(i);
        self.indices.push(i + 3);
        self.indices.push(i + 2);
    }

    pub fn clear(&mut self) {
        self.indices.clear();
        self.quads.clear();
    }


    fn quad(x: f32, y: f32, width: f32, height: f32, tex_coords : [[f32;2];4]) -> [Vertex; 4] {
        [Vertex {
             position: [x, y],
             tex_coord: tex_coords[0],
         },
         Vertex {
             position: [x, y + height],
             tex_coord: tex_coords[1],
         },
         Vertex {
             position: [x + width, y + height],
             tex_coord: tex_coords[2],
         },
         Vertex {
             position: [x + width, y],
             tex_coord: tex_coords[3],
         }]
    }
}
