#[derive(Eq, PartialEq)]
pub enum Cost {
    Passable(u32),
    Blocked,
}
