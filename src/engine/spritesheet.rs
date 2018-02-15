use std::cmp::Ordering;
use glium::texture::SrgbTexture2d;
use glium::GlObject;
use engine::Sprite; //I have no idea why this needs to be engine::...

use std::hash::{Hash, Hasher};

pub struct SpriteSheet {
    pub texture: SrgbTexture2d,
    coords: Vec<Sprite>,
}

impl SpriteSheet {
    pub fn new(texture: SrgbTexture2d, num_x_tiles: u32, num_y_tiles: u32) -> SpriteSheet {
        let mut coords = Vec::new();
        let tex_width = texture.width();
        let tex_height = texture.height();
        let sprite_width = tex_width / num_x_tiles;
        let sprite_height = tex_height / num_y_tiles;
        let tex_coord_width = sprite_width as f32 / tex_width as f32;
        let tex_coord_height = sprite_height as f32 / tex_height as f32;

        for y in 0..num_y_tiles {
            for x in 0..num_x_tiles {
                let x = x as f32;
                let y = y as f32;
                let x = x * tex_coord_width;
                let y = y * tex_coord_height;
                let sprite = Sprite::new(
                    sprite_width,
                    sprite_height,
                    [
                        [x, y],
                        [x, y + tex_coord_height],
                        [x + tex_coord_width, y + tex_coord_height],
                        [x + tex_coord_width, y],
                    ],
                );
                coords.push(sprite);
            }
        }

        SpriteSheet {
            texture: texture,
            coords: coords,
        }
    }
    pub fn coords(&self, index: usize) -> Sprite {
        self.coords[index]
    }
}

impl Hash for SpriteSheet {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.texture.get_id().hash(state);
    }
}

impl PartialEq for SpriteSheet {
    fn eq(&self, other: &SpriteSheet) -> bool {
        self.texture.get_id() == other.texture.get_id()
    }
}

impl Eq for SpriteSheet {}

impl Ord for SpriteSheet {
    fn cmp(&self, other: &SpriteSheet) -> Ordering {
        self.texture.get_id().cmp(&other.texture.get_id())
    }
}

impl PartialOrd for SpriteSheet {
    fn partial_cmp(&self, other: &SpriteSheet) -> Option<Ordering> {
        Some(self.texture.get_id().cmp(&other.texture.get_id()))
    }
}
