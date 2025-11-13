use crate::misc::Mining;

#[derive(Copy, Clone)]
pub struct MiningKinds {
    pub ore: Mining,
    pub ice: Mining,
    pub gas: Mining,
}
impl MiningKinds {
    pub(crate) fn new(ore: Mining, ice: Mining, gas: Mining) -> MiningKinds {
        Self { ore, ice, gas }
    }
}
impl Default for MiningKinds {
    fn default() -> Self {
        Self::new(Mining::default(), Mining::default(), Mining::default())
    }
}
impl std::ops::Add<MiningKinds> for MiningKinds {
    type Output = MiningKinds;
    fn add(self, rhs: MiningKinds) -> Self::Output {
        Self {
            ore: self.ore + rhs.ore,
            ice: self.ice + rhs.ice,
            gas: self.gas + rhs.gas,
        }
    }
}
impl std::iter::Sum<MiningKinds> for MiningKinds {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.reduce(|acc, v| acc + v).unwrap_or_default()
    }
}
