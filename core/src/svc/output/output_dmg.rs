use crate::def::{AttrVal, Count};

#[derive(Copy, Clone)]
pub(crate) struct OutputDmgNormal {
    pub(crate) em: AttrVal,
    pub(crate) thermal: AttrVal,
    pub(crate) kinetic: AttrVal,
    pub(crate) explosive: AttrVal,
}
impl std::ops::Mul<f64> for OutputDmgNormal {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            em: self.em * rhs,
            thermal: self.thermal * rhs,
            kinetic: self.kinetic * rhs,
            explosive: self.explosive * rhs,
        }
    }
}

#[derive(Copy, Clone)]
pub(crate) struct OutputDmgBreacher {
    pub(crate) absolute_max: AttrVal,
    pub(crate) relative_max: AttrVal,
    pub(crate) instance_count: Count,
}
