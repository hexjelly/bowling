use std::fmt;

#[derive(Default)]
pub(crate) struct Frame {
    pub(crate) throws: Vec<Throw>,
    pub(crate) points: Option<u32>,
}

#[derive(Debug)]
pub enum Throw {
    Regular(u8, Vec<bool>),
    Strike(u8, Vec<bool>),
    Spare(u8, Vec<bool>),
}

impl Frame {
    fn calculate_score(&self, next_frames: &[&Frame]) -> Option<u32> {
        unimplemented!();
    }

    pub(crate) fn add_throw(&mut self, throw: Throw) {
        unimplemented!();
    }
}
