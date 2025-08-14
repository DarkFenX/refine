use crate::{
    def::{AttrVal, OF},
    rd::RItemAXt,
    ud::{UCoordinates, UPosition},
};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct UProjData {
    pub(super) src_pos: UPosition,
    pub(super) tgt_pos: UPosition,
    // Center-to-center range
    pub(super) range_c2c: AttrVal,
    // Surface-to-surface range
    pub(super) range_s2s: AttrVal,
    pub(super) src_rad: AttrVal,
    pub(super) tgt_rad: AttrVal,
}
impl UProjData {
    pub(crate) fn from_positions_with_axt(
        src_pos: UPosition,
        tgt_pos: UPosition,
        src_axt: Option<&RItemAXt>,
        tgt_axt: Option<&RItemAXt>,
    ) -> Self {
        UProjData::from_positions_with_radii(src_pos, tgt_pos, src_axt.map(|v| v.radius), tgt_axt.map(|v| v.radius))
    }
    pub(crate) fn from_positions_with_radii(
        src_pos: UPosition,
        tgt_pos: UPosition,
        src_rad: Option<AttrVal>,
        tgt_rad: Option<AttrVal>,
    ) -> Self {
        let src_rad = src_rad.unwrap_or(OF(0.0));
        let tgt_rad = tgt_rad.unwrap_or(OF(0.0));
        let range_c2c = calc_range_c2c(src_pos.coordinates, tgt_pos.coordinates);
        Self {
            src_pos,
            tgt_pos,
            range_c2c,
            range_s2s: calc_range_s2s(range_c2c, src_rad, tgt_rad),
            src_rad,
            tgt_rad,
        }
    }
    pub(crate) fn get_range_c2c(&self) -> AttrVal {
        self.range_c2c
    }
    pub(crate) fn get_range_s2s(&self) -> AttrVal {
        self.range_s2s
    }
    pub(crate) fn get_range_c2s(&self) -> AttrVal {
        AttrVal::max(OF(0.0), self.range_c2c - self.tgt_rad)
    }
    pub(crate) fn get_src_rad(&self) -> AttrVal {
        self.src_rad
    }
    pub(crate) fn get_tgt_rad(&self) -> AttrVal {
        self.tgt_rad
    }
    pub(crate) fn update_src_pos(&mut self, src_pos: UPosition) {
        self.src_pos = src_pos;
        self.range_c2c = calc_range_c2c(self.src_pos.coordinates, self.tgt_pos.coordinates);
        self.range_s2s = calc_range_s2s(self.range_c2c, self.src_rad, self.tgt_rad);
    }
    pub(crate) fn update_tgt_pos(&mut self, tgt_pos: UPosition) {
        self.tgt_pos = tgt_pos;
        self.range_c2c = calc_range_c2c(self.src_pos.coordinates, self.tgt_pos.coordinates);
        self.range_s2s = calc_range_s2s(self.range_c2c, self.src_rad, self.tgt_rad);
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
        self.range_s2s = calc_range_s2s(self.range_c2c, self.src_rad, self.tgt_rad)
    }
}

fn calc_range_c2c(src_coords: UCoordinates, tgt_coords: UCoordinates) -> AttrVal {
    OF(((tgt_coords.x - src_coords.x).powi(2)
        + (tgt_coords.y - src_coords.y).powi(2)
        + (tgt_coords.z - src_coords.z).powi(2))
    .sqrt())
}

fn calc_range_s2s(c2c_range: AttrVal, src_rad: AttrVal, tgt_rad: AttrVal) -> AttrVal {
    AttrVal::max(OF(0.0), c2c_range - src_rad - tgt_rad)
}
