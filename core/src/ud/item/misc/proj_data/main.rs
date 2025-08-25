use crate::{
    def::{AttrVal, OF},
    rd::RItemAXt,
    ud::UPhysics,
    util::Xyz,
};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct UProjData {
    pub(super) src_physics: UPhysics,
    pub(super) tgt_physics: UPhysics,
    // Center-to-center range
    pub(super) range_c2c: AttrVal,
    // Surface-to-surface range
    pub(super) range_s2s: AttrVal,
    pub(super) src_radius: AttrVal,
    pub(super) tgt_radius: AttrVal,
}
impl UProjData {
    pub(crate) fn from_physics_with_axt(
        src_physics: UPhysics,
        tgt_physics: UPhysics,
        src_axt: Option<&RItemAXt>,
        tgt_axt: Option<&RItemAXt>,
    ) -> Self {
        UProjData::from_physics_with_radii(
            src_physics,
            tgt_physics,
            src_axt.map(|v| v.radius),
            tgt_axt.map(|v| v.radius),
        )
    }
    pub(crate) fn from_physics_with_radii(
        src_physics: UPhysics,
        tgt_physics: UPhysics,
        src_radius: Option<AttrVal>,
        tgt_radius: Option<AttrVal>,
    ) -> Self {
        let src_radius = src_radius.unwrap_or(OF(0.0));
        let tgt_radius = tgt_radius.unwrap_or(OF(0.0));
        let range_c2c = calc_range_c2c(src_physics.coordinates, tgt_physics.coordinates);
        Self {
            src_physics,
            tgt_physics,
            range_c2c,
            range_s2s: calc_range_s2s(range_c2c, src_radius, tgt_radius),
            src_radius,
            tgt_radius,
        }
    }
    pub(crate) fn get_range_c2c(&self) -> AttrVal {
        self.range_c2c
    }
    pub(crate) fn get_range_s2s(&self) -> AttrVal {
        self.range_s2s
    }
    pub(crate) fn get_range_c2s(&self) -> AttrVal {
        calc_range_c2s(self.range_c2c, self.tgt_radius)
    }
    pub(crate) fn get_src_radius(&self) -> AttrVal {
        self.src_radius
    }
    pub(crate) fn get_tgt_radius(&self) -> AttrVal {
        self.tgt_radius
    }
    pub(crate) fn get_src_coordinates(&self) -> Xyz {
        self.src_physics.coordinates
    }
    pub(crate) fn get_tgt_coordinates(&self) -> Xyz {
        self.tgt_physics.coordinates
    }
    pub(crate) fn get_src_direction(&self) -> Xyz {
        self.src_physics.direction
    }
    pub(crate) fn get_tgt_direction(&self) -> Xyz {
        self.tgt_physics.direction
    }
    pub(crate) fn get_src_speed(&self) -> AttrVal {
        self.src_physics.speed
    }
    pub(crate) fn get_tgt_speed(&self) -> AttrVal {
        self.tgt_physics.speed
    }
    pub(crate) fn update_src_physics(&mut self, src_physics: UPhysics) {
        self.src_physics = src_physics;
        self.range_c2c = calc_range_c2c(self.src_physics.coordinates, self.tgt_physics.coordinates);
        self.range_s2s = calc_range_s2s(self.range_c2c, self.src_radius, self.tgt_radius);
    }
    pub(crate) fn update_tgt_physics(&mut self, tgt_physics: UPhysics) {
        self.tgt_physics = tgt_physics;
        self.range_c2c = calc_range_c2c(self.src_physics.coordinates, self.tgt_physics.coordinates);
        self.range_s2s = calc_range_s2s(self.range_c2c, self.src_radius, self.tgt_radius);
    }
    pub(crate) fn update_src_radius(&mut self, src_radius: AttrVal) -> bool {
        if self.src_radius == src_radius {
            return false;
        };
        self.src_radius = src_radius;
        self.update_s2s_range();
        true
    }
    pub(crate) fn update_tgt_radius(&mut self, tgt_radius: AttrVal) -> bool {
        if self.tgt_radius == tgt_radius {
            return false;
        };
        self.tgt_radius = tgt_radius;
        self.update_s2s_range();
        true
    }
    pub(crate) fn update_radii(&mut self, src_radius: AttrVal, tgt_radius: AttrVal) {
        self.src_radius = src_radius;
        self.tgt_radius = tgt_radius;
        self.update_s2s_range();
    }
    fn update_s2s_range(&mut self) {
        self.range_s2s = calc_range_s2s(self.range_c2c, self.src_radius, self.tgt_radius)
    }
}

fn calc_range_c2c(src_coordinates: Xyz, tgt_coordinates: Xyz) -> AttrVal {
    OF(((tgt_coordinates.x - src_coordinates.x).powi(2)
        + (tgt_coordinates.y - src_coordinates.y).powi(2)
        + (tgt_coordinates.z - src_coordinates.z).powi(2))
    .sqrt())
}

fn calc_range_c2s(range_c2c: AttrVal, tgt_radius: AttrVal) -> AttrVal {
    AttrVal::max(OF(0.0), range_c2c - tgt_radius)
}

fn calc_range_s2s(range_c2c: AttrVal, src_radius: AttrVal, tgt_radius: AttrVal) -> AttrVal {
    AttrVal::max(OF(0.0), range_c2c - src_radius - tgt_radius)
}
