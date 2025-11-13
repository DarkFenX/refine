use crate::def::AttrVal;

#[derive(Copy, Clone)]
pub struct Mining {
    pub yield_: AttrVal,
    pub residue: AttrVal,
}
impl Mining {
    pub(crate) fn new(yield_: AttrVal, residue: AttrVal) -> Mining {
        Self { yield_, residue }
    }
}
impl Default for Mining {
    fn default() -> Self {
        Self::new(AttrVal::default(), AttrVal::default())
    }
}
impl std::ops::Add<Mining> for Mining {
    type Output = Mining;
    fn add(self, rhs: Mining) -> Self::Output {
        Self {
            yield_: self.yield_ + rhs.yield_,
            residue: self.residue + rhs.residue,
        }
    }
}
impl std::ops::AddAssign<Mining> for Mining {
    fn add_assign(&mut self, rhs: Mining) {
        self.yield_ += rhs.yield_;
        self.residue += rhs.residue;
    }
}
impl std::ops::Mul<AttrVal> for Mining {
    type Output = Mining;
    fn mul(self, rhs: AttrVal) -> Self::Output {
        Self {
            yield_: self.yield_ * rhs,
            residue: self.residue * rhs,
        }
    }
}
impl std::ops::Div<AttrVal> for Mining {
    type Output = Mining;
    fn div(self, rhs: AttrVal) -> Self::Output {
        Self {
            yield_: self.yield_ / rhs,
            residue: self.residue / rhs,
        }
    }
}
impl std::iter::Sum<Mining> for Mining {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.reduce(|acc, v| acc + v).unwrap_or_default()
    }
}
