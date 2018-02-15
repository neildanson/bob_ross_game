#[derive(Copy, Clone)]
pub struct Sprite {
    pub width: u32,
    pub height: u32,
    pub tex_coords: [[f32; 2]; 4],
}

impl Sprite {
    pub fn new(width: u32, height: u32, tex_coords: [[f32; 2]; 4]) -> Sprite {
        Sprite {
            width: width,
            height: height,
            tex_coords: tex_coords,
        }
    }
}
