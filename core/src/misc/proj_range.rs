use crate::def::AttrVal;

pub enum ProjRange {
    S2S(AttrVal),
    C2C(AttrVal),
    None,
}
