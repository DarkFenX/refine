use crate::def::AttrVal;

#[derive(Copy, Clone)]
pub struct MiningAmount {
    pub yield_: AttrVal,
    pub waste: AttrVal,
}
impl MiningAmount {
    pub(crate) fn new(yield_: AttrVal, waste: AttrVal) -> MiningAmount {
        Self { yield_, waste }
    }
}
impl Default for MiningAmount {
    fn default() -> Self {
        Self::new(AttrVal::default(), AttrVal::default())
    }
}
impl std::ops::Add<MiningAmount> for MiningAmount {
    type Output = MiningAmount;
    fn add(self, rhs: MiningAmount) -> Self::Output {
        Self {
            yield_: self.yield_ + rhs.yield_,
            waste: self.waste + rhs.waste,
        }
    }
}
impl std::ops::AddAssign<MiningAmount> for MiningAmount {
    fn add_assign(&mut self, rhs: MiningAmount) {
        self.yield_ += rhs.yield_;
        self.waste += rhs.waste;
    }
}
impl std::ops::Mul<AttrVal> for MiningAmount {
    type Output = MiningAmount;
    fn mul(self, rhs: AttrVal) -> Self::Output {
        Self {
            yield_: self.yield_ * rhs,
            waste: self.waste * rhs,
        }
    }
}
impl std::ops::Div<AttrVal> for MiningAmount {
    type Output = MiningAmount;
    fn div(self, rhs: AttrVal) -> Self::Output {
        Self {
            yield_: self.yield_ / rhs,
            waste: self.waste / rhs,
        }
    }
}
