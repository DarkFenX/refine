use crate::{
    ad,
    def::{AttrVal, OF},
    misc::ProjRange,
    uad::UadItem,
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
    pub(crate) fn new_tmp(range: AttrVal) -> Self {
        Self {
            c2c: range,
            s2s: range,
            src_rad: OF(0.0),
            tgt_rad: OF(0.0),
        }
    }
    pub(crate) fn from_prange_with_items(prange: ProjRange, src_item: &UadItem, tgt_item: &UadItem) -> Option<Self> {
        UadProjRange::from_prange_with_extras(prange, src_item.get_a_extras(), tgt_item.get_a_extras())
    }
    pub(crate) fn from_prange_with_extras(
        prange: ProjRange,
        src_a_extras: Option<&ad::AItemExtras>,
        tgt_a_extras: Option<&ad::AItemExtras>,
    ) -> Option<Self> {
        UadProjRange::from_prange_with_radii(
            prange,
            src_a_extras.and_then(|v| v.radius).unwrap_or(OF(0.0)),
            tgt_a_extras.and_then(|v| v.radius).unwrap_or(OF(0.0)),
        )
    }
    pub(crate) fn from_prange_with_radii(prange: ProjRange, src_rad: AttrVal, tgt_rad: AttrVal) -> Option<Self> {
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
