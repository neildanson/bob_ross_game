#[derive(Copy, Clone)]
pub struct Sprite {
    pub width : f32, 
    pub height : f32,
    pub coords : [[f32;2];4]
}

impl Sprite {
    pub fn new(width : f32, height : f32, coords : [[f32;2];4]) -> Sprite {
        Sprite { width : width, height : height, coords : coords }
    }
}