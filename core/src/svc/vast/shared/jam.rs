use crate::def::AttrVal;

#[derive(Copy, Clone)]
pub struct StatJamApplied {
    pub chance: AttrVal,
    pub uptime: AttrVal,
}
