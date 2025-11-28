use crate::def::AttrVal;

#[derive(Copy, Clone)]
pub struct StatOutgoingJam {
    pub chance: AttrVal,
    pub uptime: AttrVal,
}
