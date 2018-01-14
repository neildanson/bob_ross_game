use glium::texture::SrgbTexture2d;
use engine::Sprite; //I have no idea why this needs to be engine::...


pub struct SpriteSheet {
    pub texture: SrgbTexture2d,
    coords : Vec<Sprite>
}

impl SpriteSheet {
    pub fn new(texture:SrgbTexture2d, num_x_tiles:u32, num_y_tiles:u32) -> SpriteSheet {
        let mut coords = Vec::new();
        let tex_width = texture.width() as f32;
        let tex_height = texture.height() as f32;
        let sprite_width = tex_width / num_x_tiles as f32;
        let sprite_height = tex_height / num_y_tiles as f32;
        let tex_coord_width = sprite_width / tex_width;
        let tex_coord_height = sprite_height / tex_height;

        for y in 0 .. num_y_tiles {
            for x in 0 .. num_x_tiles {
                let x = x as f32 * tex_coord_width;
                let y = y as f32 * tex_coord_height;
                let sprite = Sprite::new(sprite_width, sprite_height,
                    [[x, y],
                     [x, y + tex_coord_height],
                     [x + tex_coord_width, y + tex_coord_height],
                     [x + tex_coord_width, y]
                    ]);
                coords.push(sprite);
            }
        }

        SpriteSheet { texture : texture, coords : coords }
    }
    pub fn coords(&self, index:usize) -> Sprite {
        self.coords[index]
    } 
}

