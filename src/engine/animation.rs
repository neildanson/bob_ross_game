use std::time::{Duration, SystemTime};

#[derive(Copy, Clone)]
pub enum AnimationType {
    Loop,
    Once
}

#[derive(Copy, Clone)]
pub struct Animation {
    animation_type : AnimationType,
    pub current_frame: usize,
    base_frame: usize,
    num_frames: usize,
    last_update_time: SystemTime,
    frame_interval: Duration,
}

impl Animation {
    pub fn new(animation_type : AnimationType,  base_frame: usize, num_frames: usize, frame_interfal_in_ms: u64) -> Animation {
        Animation {
            animation_type,
            num_frames,
            base_frame,
            current_frame: base_frame,
            last_update_time: SystemTime::now(),
            frame_interval: Duration::from_millis(frame_interfal_in_ms),
        }
    }

    pub fn update(&mut self, now: SystemTime) {
        if now.duration_since(self.last_update_time).unwrap() >= self.frame_interval {
            self.last_update_time = now;
            self.current_frame = if self.current_frame < self.base_frame + self.num_frames - 1 {
                self.current_frame + 1
            } else {
                match self.animation_type {
                    AnimationType::Loop => self.base_frame,
                    AnimationType::Once => self.current_frame
                }
            }
        }
    }
}
