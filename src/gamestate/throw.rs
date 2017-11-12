pub enum Throw {
    Regular(u8, Vec<bool>),
    Strike(u8, Vec<bool>),
    Spare(u8, Vec<bool>),
    Bonus(u8, Vec<bool>),
}
