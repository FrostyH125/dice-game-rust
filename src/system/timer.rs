pub struct Timer {
    pub duration: f32,
    current_time: f32,
}

impl Timer {
    pub fn new(duration: f32) -> Self {
        Timer {
            duration,
            current_time: 0.0
        }
    }
    
    pub fn track(&mut self, dt: f32) {
        self.current_time += dt;
    }
       
    pub fn is_done(&self) -> bool {
        if self.current_time >= self.duration {
            true
        } else {
            false
        }
    }
    pub fn reset(&mut self) {
        self.current_time = 0.0;
    }
    
}