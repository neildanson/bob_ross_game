
pub struct Controller { 
    pub left : bool,
    pub right : bool,
    pub up : bool,
    pub down : bool
}

impl Controller {
    pub fn new() -> Controller {
        Controller { left : false, right : false, up : false, down : false }
    }
}