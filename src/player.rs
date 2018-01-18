use engine::{Animation, Controller};
use std::time::SystemTime;

pub enum Direction {
    Left, 
    Right,
    Up, 
    Down
}

pub struct Player {
    pub x : f32,
    pub y : f32,
    pub currect_animation : Animation,
    direction : Direction,
}

impl Player {
    pub fn new(animation : Animation) -> Player {

        Player { x: 0.0, y : 0.0, direction : Direction::Right, currect_animation : animation }
    }

    pub fn update(&mut self, controller:&Controller, update_time:SystemTime) {
        self.currect_animation.update(update_time);

        self.x = if controller.left {
            self.direction = Direction::Left;
            self.x - 1.0
        } else { self.x };

        self.x = if controller.right {
            self.direction = Direction::Right;
            self.x + 1.0
        } else { self.x };

        self.y = if controller.up {
            self.direction = Direction::Up;
            self.y - 1.0
        } else { self.y };

        self.y = if controller.down {
            self.direction = Direction::Down;
            self.y + 1.0
        } else { self.y };
    }
}