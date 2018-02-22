use rand::*;
use rand::distributions::{IndependentSample, Range};
use std::time::SystemTime;
use engine::{Animation, Entity};
use direction::Direction;
use constants::*;

enum State {
    Alive(i32, i32, i32, i32, Animation),
    Dead(i32, i32, Animation)
}

impl State {
    fn position(&self) -> (i32, i32) {
        match *self {
            State::Alive(x,y,..) => (x,y),
            State::Dead(x,y,..) => (x,y)
        }
    }
}

pub struct Squirrel {
    state : State,
    between: Range<i32>,
    rng: ThreadRng,
    animations: [Animation; 4],
    direction: Direction,
    current_animation: Animation,
}

impl Entity for Squirrel {
    fn position(&self) -> (i32, i32) {
        self.state.position()
    }

    fn animation_index(&self) -> usize {
        self.current_animation.current_frame
    }
}

impl Squirrel {
    pub fn new() -> Squirrel {
        let between = Range::new(0, MAP_SIZE_SCALED);
        let mut rng = thread_rng();
        let pos_x = between.ind_sample(&mut rng);
        let pos_y = between.ind_sample(&mut rng);

        let animations = [
            Animation::new(0, 4, 100),  //Walk Right
            Animation::new(4, 4, 200),  //Walk Down
            Animation::new(8, 4, 100),  //Walk Left
            Animation::new(12, 4, 200), //Walk Up
        ];
        let target_x = between.ind_sample(&mut rng);
        let target_y = between.ind_sample(&mut rng);
        let state = State::Alive (pos_x, pos_y, target_x, target_y, animations[0]);
        
        Squirrel {
            state: state,
            between: between,
            rng: rng,
            direction: Direction::Left,
            animations: animations,
            current_animation: animations[0],
        }
    }

    pub fn update(&mut self, update_time: SystemTime) {
        match self.state {
            State::Alive(x,y, mut target_x, mut target_y, animation) =>{
                let delta_x = if x < target_x {
                    self.direction = Direction::Left;
                    SQUIRREL_SPEED
                } else {
                    if x > target_x {
                        self.direction = Direction::Right;
                        -SQUIRREL_SPEED
                    } else {
                        0
                    }
                };
                let delta_y = if y < target_y {
                    self.direction = Direction::Down;
                    SQUIRREL_SPEED
                } else {
                    if y > target_y {
                        self.direction = Direction::Up;
                        -SQUIRREL_SPEED
                    } else {
                        0
                    }
                };

                self.current_animation = match self.direction {
                    Direction::Right => self.animations[0],
                    Direction::Down => self.animations[1],
                    Direction::Left => self.animations[2],
                    Direction::Up => self.animations[3],
                };

                let x = x + delta_x;
                let y = y + delta_y;

                if x as i32 == target_x as i32 && y as i32 == target_y as i32 {
                    target_x = self.between.ind_sample(&mut self.rng);
                    target_y = self.between.ind_sample(&mut self.rng);
                }
                
                self.state = State::Alive(x,y,target_x, target_y, animation);

                for a in &mut self.animations {
                    a.update(update_time);
                }
            },
            _ => () //TODO DEATH
        }
    }
}
