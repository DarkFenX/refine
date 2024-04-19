#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SsModRack {
    High,
    Mid,
    Low,
}
impl std::fmt::Display for SsModRack {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::High => write!(f, "high"),
            Self::Mid => write!(f, "mid"),
            Self::Low => write!(f, "low"),
        }
    }
}
