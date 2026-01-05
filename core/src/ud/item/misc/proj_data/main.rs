use crate::{
    misc::{PValue, Xyz},
    rd::RItemAXt,
    ud::UPhysics,
    util::LibDefault,
};

#[derive(Copy, Clone, PartialEq, Eq)]
pub(crate) struct UProjData {
    pub(super) src_physics: UPhysics,
    pub(super) tgt_physics: UPhysics,
    // Center-to-center range
    pub(super) range_c2c: PValue,
    // Surface-to-surface range
    pub(super) range_s2s: PValue,
    pub(super) src_radius: PValue,
    pub(super) tgt_radius: PValue,
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
        src_radius: Option<PValue>,
        tgt_radius: Option<PValue>,
    ) -> Self {
        let src_radius = src_radius.unwrap_or(LibDefault::lib_default());
        let tgt_radius = tgt_radius.unwrap_or(LibDefault::lib_default());
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
    pub(crate) fn get_range_c2c(&self) -> PValue {
        self.range_c2c
    }
    pub(crate) fn get_range_s2s(&self) -> PValue {
        self.range_s2s
    }
    pub(crate) fn get_range_c2s(&self) -> PValue {
        calc_range_c2s(self.range_c2c, self.tgt_radius)
    }
    pub(crate) fn get_src_radius(&self) -> PValue {
        self.src_radius
    }
    pub(crate) fn get_tgt_radius(&self) -> PValue {
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
    pub(crate) fn get_src_speed(&self) -> PValue {
        self.src_physics.speed
    }
    pub(crate) fn get_tgt_speed(&self) -> PValue {
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
    pub(crate) fn update_src_radius(&mut self, src_radius: PValue) -> bool {
        if self.src_radius == src_radius {
            return false;
        };
        self.src_radius = src_radius;
        self.update_s2s_range();
        true
    }
    pub(crate) fn update_tgt_radius(&mut self, tgt_radius: PValue) -> bool {
        if self.tgt_radius == tgt_radius {
            return false;
        };
        self.tgt_radius = tgt_radius;
        self.update_s2s_range();
        true
    }
    pub(crate) fn update_radii(&mut self, src_radius: PValue, tgt_radius: PValue) {
        self.src_radius = src_radius;
        self.tgt_radius = tgt_radius;
        self.update_s2s_range();
    }
    fn update_s2s_range(&mut self) {
        self.range_s2s = calc_range_s2s(self.range_c2c, self.src_radius, self.tgt_radius)
    }
}

fn calc_range_c2c(src_coordinates: Xyz, tgt_coordinates: Xyz) -> PValue {
    // Do not check anything because result will always be positive
    PValue::new_of64_unchecked(
        ((tgt_coordinates.x - src_coordinates.x).powi(2)
            + (tgt_coordinates.y - src_coordinates.y).powi(2)
            + (tgt_coordinates.z - src_coordinates.z).powi(2))
        .sqrt()
        .into_inner_of64(),
    )
}

fn calc_range_c2s(range_c2c: PValue, tgt_radius: PValue) -> PValue {
    PValue::new_clamped(range_c2c.into_inner() - tgt_radius.into_inner())
}

fn calc_range_s2s(range_c2c: PValue, src_radius: PValue, tgt_radius: PValue) -> PValue {
    PValue::new_clamped(range_c2c.into_inner() - src_radius.into_inner() - tgt_radius.into_inner())
}
