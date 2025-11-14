use super::mining_amount::StatMiningAmount;

#[derive(Copy, Clone)]
pub struct StatMining {
    pub ore: StatMiningAmount,
    pub ice: StatMiningAmount,
    pub gas: StatMiningAmount,
}
impl StatMining {
    pub(crate) fn new(ore: StatMiningAmount, ice: StatMiningAmount, gas: StatMiningAmount) -> StatMining {
        Self { ore, ice, gas }
    }
}
impl Default for StatMining {
    fn default() -> Self {
        Self::new(
            StatMiningAmount::default(),
            StatMiningAmount::default(),
            StatMiningAmount::default(),
        )
    }
}
impl std::ops::Add<StatMining> for StatMining {
    type Output = StatMining;
    fn add(self, rhs: StatMining) -> Self::Output {
        Self {
            ore: self.ore + rhs.ore,
            ice: self.ice + rhs.ice,
            gas: self.gas + rhs.gas,
        }
    }
}
impl std::iter::Sum<StatMining> for StatMining {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.reduce(|acc, v| acc + v).unwrap_or_default()
    }
}
