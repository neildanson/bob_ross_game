use engine::{Animation, AnimationType, Audio, Controller};
use std::time::SystemTime;
use direction::Direction;
use constants::{MAP_SIZE_SCALED, PLAYER_SPEED};

pub struct Player {
    pub x: i32,
    pub y: i32,
    pub current_animation: Animation,
    animations: [Animation; 5],
    direction: Direction,
    audio: Audio,
}

impl Player {
    pub fn new() -> Player {
        let audio = Audio::new("./Assets/Audio/Walk.wav");

        let animations = [
            Animation::new(AnimationType::Loop, 0, 4, 100), //Walk Right
            Animation::new(AnimationType::Loop, 4, 4, 200), //Walk Down
            Animation::new(AnimationType::Loop, 8, 4, 100), //Walk Left
            Animation::new(AnimationType::Loop, 12, 4, 200), //Walk Up
            Animation::new(AnimationType::Loop, 16, 2, 500), //Idle
        ];
        Player {
            x: 0,
            y: 0,
            direction: Direction::Right,
            current_animation: animations[4],
            animations: animations,
            audio: audio,
        }
    }

    pub fn update(&mut self, controller: &Controller, update_time: SystemTime) {
        let mut key_pressed = false;

        self.x = if controller.left {
            key_pressed = true;
            self.direction = Direction::Left;
            self.x - PLAYER_SPEED
        } else {
            self.x
        };

        self.x = if controller.right {
            key_pressed = true;
            self.direction = Direction::Right;
            self.x + PLAYER_SPEED
        } else {
            self.x
        };

        self.y = if controller.up {
            key_pressed = true;
            self.direction = Direction::Up;
            self.y - PLAYER_SPEED
        } else {
            self.y
        };

        self.y = if controller.down {
            key_pressed = true;
            self.direction = Direction::Down;
            self.y + PLAYER_SPEED
        } else {
            self.y
        };

        if self.x < 0 {
            self.x = 0
        }
        if self.x > MAP_SIZE_SCALED {
            self.x = MAP_SIZE_SCALED;
        }

        if self.y < 0 {
            self.y = 0
        }
        if self.y > MAP_SIZE_SCALED {
            self.y = MAP_SIZE_SCALED;
        }

        if !key_pressed {
            self.current_animation = self.animations[4];
            self.audio.pause();
        } else {
            self.audio.play();
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
