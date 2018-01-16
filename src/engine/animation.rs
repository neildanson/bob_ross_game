use std::time::{SystemTime, Duration};

pub struct Animation {
    pub current_frame : usize,
    num_frames : usize,
    last_update_time : SystemTime,
    frame_interval : Duration,
}

impl Animation {
    pub fn new(num_frames : usize, frame_interfal_in_ms : u64) -> Animation {
        Animation { num_frames : num_frames, 
                    current_frame: 0, 
                    last_update_time : SystemTime::now(),
                    frame_interval : Duration::from_millis(frame_interfal_in_ms) }
    }

    pub fn update(&mut self, now : SystemTime) {
        if now.duration_since(self.last_update_time).unwrap() >= self.frame_interval {
            self.last_update_time = now;
            self.current_frame = if self.current_frame < self.num_frames - 1 {
                self.current_frame + 1
            } else {
                0
            }
        }
    }
}