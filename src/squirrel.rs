use rand::*;
use rand::distributions::{IndependentSample, Range};
use std::time::SystemTime;
use engine::Animation;
use direction::Direction;

pub struct Squirrel {
    pub x : f32,
    pub y : f32, 
    between : Range<i32>,
    rng : ThreadRng,
    target_x : f32,
    target_y : f32,
    animations: [Animation; 4],
    direction: Direction,
    pub current_animation : Animation, 
}

impl Squirrel {
    pub fn new() -> Squirrel {
        let between = Range::new(0, 50 * 16);
        let mut rng = thread_rng();
        let pos_x = between.ind_sample(&mut rng) as f32 ;
        let pos_y = between.ind_sample(&mut rng) as f32;
        
        let animations = [
            Animation::new(0, 4, 100),   //Walk Right
            Animation::new(4, 4, 200),  //Walk Down
            Animation::new(8, 4, 100),   //Walk Left
            Animation::new(12, 4, 200), //Walk Up
        ];
        let target_x = between.ind_sample(&mut rng) as f32;
        let target_y = between.ind_sample(&mut rng) as f32;
        Squirrel { x : pos_x, y : pos_y, 
            between : between, 
            rng : rng, 
            target_x : target_x, target_y : target_y, 
            direction : Direction::Left,
            animations : animations,
            current_animation : animations[0] }
    }

    pub fn update(&mut self, update_time : SystemTime) {
        let delta_x = if self.x < self.target_x { 
            self.direction = Direction::Left;
            1.0
        } else { 
            if self.x > self.target_x { 
                 self.direction = Direction::Right; 
                 -1.0 
            } else { 0.0 }};
        let delta_y = if self.y < self.target_y { 
            self.direction = Direction::Down;
                1.0 
            } else { 
                if self.y > self.target_y { 
                    self.direction = Direction::Up;
                    -1.0 
                } else { 0.0 }
                };

        self.current_animation = match self.direction {
                Direction::Right => self.animations[0],
                Direction::Down => self.animations[1],
                Direction::Left => self.animations[2],
                Direction::Up => self.animations[3],
            };

        let x = self.x + delta_x;
        let y = self.y + delta_y;

        if x as i32 == self.target_x as i32 && y as i32 == self.target_y as i32 {
            self.target_x = self.between.ind_sample(&mut self.rng) as f32;
            self.target_y = self.between.ind_sample(&mut self.rng) as f32;
        }
        self.x = x;
        self.y = y;

        for a in &mut self.animations {
            a.update(update_time);
        }
    }
}