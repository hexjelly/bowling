use super::Throw;

pub struct Frame {
    throws: Vec<Throw>,
    points: Option<u32>,
}
