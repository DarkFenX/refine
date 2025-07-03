use crate::{
    ad,
    def::{AttrVal, OF},
    misc::ProjRange,
};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct UadProjRange {
    // Center-to-center range
    pub(crate) c2c: AttrVal,
    // Surface-to-surface range
    pub(crate) s2s: AttrVal,
    pub(crate) src_rad: AttrVal,
    pub(crate) tgt_rad: AttrVal,
}
impl UadProjRange {
    pub(crate) fn from_prange_with_extras(
        prange: ProjRange,
        src_a_extras: Option<&ad::AItemExtras>,
        tgt_a_extras: Option<&ad::AItemExtras>,
    ) -> Option<Self> {
        UadProjRange::from_prange_with_radii(
            prange,
            src_a_extras.and_then(|v| v.radius),
            tgt_a_extras.and_then(|v| v.radius),
        )
    }
    pub(crate) fn from_prange_with_radii(
        prange: ProjRange,
        src_rad: Option<AttrVal>,
        tgt_rad: Option<AttrVal>,
    ) -> Option<Self> {
        let src_rad = src_rad.unwrap_or(OF(0.0));
        let tgt_rad = tgt_rad.unwrap_or(OF(0.0));
        match prange {
            ProjRange::S2S(range) => Some(Self {
                c2c: range + src_rad + tgt_rad,
                s2s: range,
                src_rad,
                tgt_rad,
            }),
            ProjRange::C2C(range) => Some(Self {
                c2c: range,
                s2s: AttrVal::max(OF(0.0), range - src_rad - tgt_rad),
                src_rad,
                tgt_rad,
            }),
            ProjRange::None => None,
        }
    }
}
