use engine::{BoundingBox, Camera, SpriteSheet,Vertex};

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
    pub fn add(&mut self, x: f32, y: f32, sprite_index : usize, spritesheet:&SpriteSheet, camera : &Camera) {
        let sprite = spritesheet.coords(sprite_index);
        let sprite_boundingbox = BoundingBox::new(x, y, sprite.width, sprite.height);
        if camera.boundingbox.intersects(&sprite_boundingbox) {
            let i = self.quads.len() as u32;
            self.quads.push(
                Vertex {
                    position: [x, y],
                    tex_coord: sprite.tex_coords[0],
                });
            self.quads.push(
                Vertex {
                    position: [x, y + sprite.height],
                    tex_coord: sprite.tex_coords[1],
                });
            self.quads.push(
                Vertex {
                    position: [x + sprite.width, y + sprite.height],
                    tex_coord: sprite.tex_coords[2],
                });
            self.quads.push(
                Vertex {
                    position: [x + sprite.width, y],
                    tex_coord: sprite.tex_coords[3],
                });

            self.indices.push(i);
            self.indices.push(i + 1);
            self.indices.push(i + 2);
            self.indices.push(i);
            self.indices.push(i + 3);
            self.indices.push(i + 2);
        }
    }

    pub fn clear(&mut self) {
        self.indices.clear();
        self.quads.clear();
    }
}
