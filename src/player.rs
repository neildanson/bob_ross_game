use engine::{Animation, Controller};
use std::time::SystemTime;

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub current_animation: Animation,
    animations: [Animation; 5],
    direction: Direction,
}

impl Player {
    pub fn new() -> Player {
        let animations = [
            Animation::new(0, 4, 100),   //Walk Right
            Animation::new(4, 4, 200),  //Walk Down
            Animation::new(8, 4, 100),   //Walk Left
            Animation::new(12, 4, 200), //Walk Up
            Animation::new(16, 2, 500), //Idle
        ];
        Player {
            x: 0.0,
            y: 0.0,
            direction: Direction::Right,
            current_animation: animations[4],
            animations: animations,
        }
    }

    pub fn update(&mut self, controller: &Controller, update_time: SystemTime) {
        let mut key_pressed = false;

        self.x = if controller.left {
            key_pressed = true;
            self.direction = Direction::Left;
            self.x - 2.0
        } else {
            self.x
        };

        self.x = if controller.right {
            key_pressed = true;
            self.direction = Direction::Right;
            self.x + 2.0
        } else {
            self.x
        };

        self.y = if controller.up {
            key_pressed = true;
            self.direction = Direction::Up;
            self.y - 2.0
        } else {
            self.y
        };

        self.y = if controller.down {
            key_pressed = true;
            self.direction = Direction::Down;
            self.y + 2.0
        } else {
            self.y
        };

        if !key_pressed {
            self.current_animation = self.animations[4];
        } else {
            self.current_animation = match self.direction {
                Direction::Right => self.animations[0],
                Direction::Down => self.animations[1],
                Direction::Left => self.animations[2],
                Direction::Up => self.animations[3],
            }
        }
        for a in &mut self.animations {
            a.update(update_time);
        }
    }
}
