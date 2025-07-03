use crate::def::AttrVal;

pub enum Range {
    S2S(AttrVal),
    C2C(AttrVal),
    None,
}
