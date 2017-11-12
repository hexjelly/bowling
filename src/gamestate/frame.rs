#[derive(Default)]
pub struct Frame {
    throws: Vec<Throw>,
    points: Option<u32>,
}

pub enum Throw {
    Regular(u8, Vec<bool>),
    Strike(u8, Vec<bool>),
    Spare(u8, Vec<bool>),
    Bonus(u8, Vec<bool>),
}

impl Frame {
    fn calculate_score(&self) -> Option<u32> {
        unimplemented!();
    }
}
