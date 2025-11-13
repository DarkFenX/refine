use crate::def::AttrVal;

#[derive(Copy, Clone)]
pub struct StatMiningAmount {
    pub yield_: AttrVal,
    pub residue: AttrVal,
}
impl StatMiningAmount {
    pub(crate) fn new(yield_: AttrVal, residue: AttrVal) -> StatMiningAmount {
        Self { yield_, residue }
    }
}
impl Default for StatMiningAmount {
    fn default() -> Self {
        Self::new(AttrVal::default(), AttrVal::default())
    }
}
impl std::ops::Add<StatMiningAmount> for StatMiningAmount {
    type Output = StatMiningAmount;
    fn add(self, rhs: StatMiningAmount) -> Self::Output {
        Self {
            yield_: self.yield_ + rhs.yield_,
            residue: self.residue + rhs.residue,
        }
    }
}
impl std::ops::AddAssign<StatMiningAmount> for StatMiningAmount {
    fn add_assign(&mut self, rhs: StatMiningAmount) {
        self.yield_ += rhs.yield_;
        self.residue += rhs.residue;
    }
}
impl std::ops::Mul<AttrVal> for StatMiningAmount {
    type Output = StatMiningAmount;
    fn mul(self, rhs: AttrVal) -> Self::Output {
        Self {
            yield_: self.yield_ * rhs,
            residue: self.residue * rhs,
        }
    }
}
impl std::ops::Div<AttrVal> for StatMiningAmount {
    type Output = StatMiningAmount;
    fn div(self, rhs: AttrVal) -> Self::Output {
        Self {
            yield_: self.yield_ / rhs,
            residue: self.residue / rhs,
        }
    }
}
