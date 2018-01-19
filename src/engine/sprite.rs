#[derive(Copy, Clone)]
pub struct Sprite {
    pub width : f32, 
    pub height : f32,
    pub tex_coords : [[f32;2];4]
}

impl Sprite {
    pub fn new(width : f32, height : f32, tex_coords : [[f32;2];4]) -> Sprite {
        Sprite { width : width, height : height, tex_coords : tex_coords }
    }
}