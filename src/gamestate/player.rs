use super::Frame;

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
}
