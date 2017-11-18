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

    pub fn with_throws(throws: &[Throw]) -> Frame {
        unimplemented!();
    }

    fn calculate_score(&self, next_frames: &[&Frame]) -> Option<u32> {
        unimplemented!();
    }

    pub(crate) fn add_throw(&mut self, throw: Throw) {
        unimplemented!();
    }
}
