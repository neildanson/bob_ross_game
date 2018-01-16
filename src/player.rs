pub struct Player {
    pub x : f32,
    pub y : f32
}

impl Player {
    pub fn new() -> Player {
        Player { x: 0.0, y : 0.0 }
    }
}