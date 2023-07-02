#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ModRack {
    High,
    Mid,
    Low,
}
impl std::fmt::Display for ModRack {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::High => write!(f, "high"),
            Self::Mid => write!(f, "mid"),
            Self::Low => write!(f, "low"),
        }
    }
}
