use engine::{BoundingBox, Camera, SpriteSheet, Vertex};
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone)]
pub struct DrawCall {
    pub quads: Vec<Vertex>,
    pub indices: Vec<u32>, /* TODO make internals private
                            * Render via trait */
}

impl DrawCall {
    fn new() -> DrawCall {
        DrawCall {
            quads: Vec::new(),
            indices: Vec::new(),
        }
    }
}

pub struct SpriteBatch {
    pub draw_calls: HashMap<(i32, Rc<SpriteSheet>), DrawCall>, //TODO Hide internals
}

impl SpriteBatch {
    pub fn new() -> SpriteBatch {
        SpriteBatch {
            draw_calls: HashMap::new(),
        }
    }

    //TODO - add a reference to the spritesheet!
    pub fn add(
        &mut self,
        x: i32,
        y: i32,
        z: i32,
        sprite_index: usize,
        spritesheet: Rc<SpriteSheet>,
        camera: &Camera,
    ) {
        let sprite = spritesheet.coords(sprite_index);
        let sprite_boundingbox = BoundingBox::new(x, y, sprite.width, sprite.height);
        if camera.boundingbox.intersects(&sprite_boundingbox) {
            let mut draw_calls = {
                let calls = self.draw_calls.get(&(z, spritesheet.clone()));
                match calls {
                    Some(calls) => calls.clone(),
                    None => DrawCall::new(),
                }
            };

            let i = draw_calls.quads.len() as u32;
            draw_calls.quads.push(Vertex {
                position: [x, y],
                tex_coord: sprite.tex_coords[0],
            });
            draw_calls.quads.push(Vertex {
                position: [x, y + sprite.height as i32],
                tex_coord: sprite.tex_coords[1],
            });
            draw_calls.quads.push(Vertex {
                position: [
                    x + sprite.width as i32,
                    y + sprite.height as i32,
                ],
                tex_coord: sprite.tex_coords[2],
            });
            draw_calls.quads.push(Vertex {
                position: [x + sprite.width as i32, y as i32],
                tex_coord: sprite.tex_coords[3],
            });

            draw_calls.indices.push(i);
            draw_calls.indices.push(i + 1);
            draw_calls.indices.push(i + 2);
            draw_calls.indices.push(i);
            draw_calls.indices.push(i + 3);
            draw_calls.indices.push(i + 2);
            let spritesheet = spritesheet.clone();

            self.draw_calls.insert((z, spritesheet), draw_calls);
        }
    }

    pub fn clear(&mut self) {
        self.draw_calls.clear();
    }
}
