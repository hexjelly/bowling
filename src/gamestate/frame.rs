use super::Throw;

#[derive(Debug, Default, Eq, PartialEq)]
pub struct Frame {
    pub(crate) throws: Vec<Throw>,
    pub(crate) points: Option<u32>,
}

impl Frame {
    pub fn new() -> Frame {
        Frame::default()
    }

    pub fn with_throws(_throws: &[Throw]) -> Frame {
        unimplemented!();
    }

    pub fn calculate_score(&self, _next_frames: &[&Frame]) -> Option<u32> {
        unimplemented!();
    }

    pub fn add_throw(&mut self, _throw: Throw) {
        unimplemented!();
    }
}
