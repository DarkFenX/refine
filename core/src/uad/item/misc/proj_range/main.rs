use crate::{
    ad,
    def::{AttrVal, OF},
    misc::ProjRange,
};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct UadProjRange {
    // Center-to-center range
    pub(super) c2c: AttrVal,
    // Surface-to-surface range
    pub(super) s2s: AttrVal,
    pub(super) src_rad: AttrVal,
    pub(super) tgt_rad: AttrVal,
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
                s2s: calc_s2s_range(range, src_rad, tgt_rad),
                src_rad,
                tgt_rad,
            }),
            ProjRange::None => None,
        }
    }
    pub(crate) fn get_c2c(&self) -> AttrVal {
        self.c2c
    }
    pub(crate) fn get_s2s(&self) -> AttrVal {
        self.s2s
    }
    pub(crate) fn get_c2s(&self) -> AttrVal {
        AttrVal::max(OF(0.0), self.c2c - self.tgt_rad)
    }
    pub(crate) fn get_src_rad(&self) -> AttrVal {
        self.src_rad
    }
    pub(crate) fn get_tgt_rad(&self) -> AttrVal {
        self.tgt_rad
    }
    pub(crate) fn update_src_rad(&mut self, src_rad: AttrVal) -> bool {
        if self.src_rad == src_rad {
            return false;
        };
        self.src_rad = src_rad;
        self.update_s2s_range();
        true
    }
    pub(crate) fn update_tgt_rad(&mut self, tgt_rad: AttrVal) -> bool {
        if self.tgt_rad == tgt_rad {
            return false;
        };
        self.tgt_rad = tgt_rad;
        self.update_s2s_range();
        true
    }
    pub(crate) fn update_radii(&mut self, src_rad: AttrVal, tgt_rad: AttrVal) {
        self.src_rad = src_rad;
        self.tgt_rad = tgt_rad;
        self.update_s2s_range();
    }
    fn update_s2s_range(&mut self) {
        self.s2s = calc_s2s_range(self.c2c, self.src_rad, self.tgt_rad)
    }
}

fn calc_s2s_range(c2c_range: AttrVal, src_rad: AttrVal, tgt_rad: AttrVal) -> AttrVal {
    AttrVal::max(OF(0.0), c2c_range - src_rad - tgt_rad)
}
