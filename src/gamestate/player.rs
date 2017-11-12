use super::Frame;
use super::Throw;

pub struct Player {
    name: String,
    frames: Vec<Frame>,
}

impl Player {
    pub fn new<S: Into<String>>(name: S) -> Player {
        Player {
            name: name.into(),
            frames: vec![],
        }
    }

    pub fn add_frame(&mut self) {
        self.frames.push(Frame::default())
    }

    pub fn add_throw(&mut self, throw: Throw) {
        unimplemented!();
    }
}
