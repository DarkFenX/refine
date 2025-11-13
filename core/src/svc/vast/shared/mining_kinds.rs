use super::mining_amount::StatMiningAmount;

#[derive(Copy, Clone)]
pub struct StatMiningKinds {
    pub ore: StatMiningAmount,
    pub ice: StatMiningAmount,
    pub gas: StatMiningAmount,
}
impl StatMiningKinds {
    pub(crate) fn new(ore: StatMiningAmount, ice: StatMiningAmount, gas: StatMiningAmount) -> StatMiningKinds {
        Self { ore, ice, gas }
    }
}
impl Default for StatMiningKinds {
    fn default() -> Self {
        Self::new(
            StatMiningAmount::default(),
            StatMiningAmount::default(),
            StatMiningAmount::default(),
        )
    }
}
impl std::ops::Add<StatMiningKinds> for StatMiningKinds {
    type Output = StatMiningKinds;
    fn add(self, rhs: StatMiningKinds) -> Self::Output {
        Self {
            ore: self.ore + rhs.ore,
            ice: self.ice + rhs.ice,
            gas: self.gas + rhs.gas,
        }
    }
}
impl std::iter::Sum<StatMiningKinds> for StatMiningKinds {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.reduce(|acc, v| acc + v).unwrap_or_default()
    }
}
