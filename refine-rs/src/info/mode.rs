use std::{collections::HashMap, hash::Hash};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone)]
pub struct InfoModes<M, I>
where
    M: const Default,
{
    pub default: M = M::default(),
    pub overrides: Vec<(I, M)> = Vec::new(),
}
const impl<M, I> Default for InfoModes<M, I>
where
    M: const Default,
{
    fn default() -> Self {
        Self { .. }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(crate) struct InfoModesInt<M, I> {
    default: M,
    overrides: HashMap<I, M>,
}
impl<M, I> InfoModesInt<M, I> {
    pub(in crate::info) fn get(&self, id: &I) -> M
    where
        M: Copy,
        I: Eq + Hash,
    {
        match self.overrides.get(id) {
            Some(mode) => *mode,
            None => self.default,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<M, I> InfoModesInt<M, I> {
    pub(crate) fn from_pub_mode(pub_mode: M) -> Self {
        Self {
            default: pub_mode,
            overrides: HashMap::new(),
        }
    }
    pub(crate) fn from_pub_modes_regular(pub_modes: InfoModes<M, I>) -> Self
    where
        M: const Default,
        I: Eq + Hash,
    {
        Self {
            default: pub_modes.default,
            overrides: pub_modes.overrides.into_iter().collect(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde")]
mod custom_serde {
    use serde::de::{Deserialize, Deserializer};

    use super::*;

    impl<'de, M, I> Deserialize<'de> for InfoModes<M, I>
    where
        M: const Default + Deserialize<'de>,
        I: Deserialize<'de>,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            Ok(match InfoModesFormats::deserialize(deserializer)? {
                InfoModesFormats::Simple(default) => Self { default, .. },
                InfoModesFormats::Extended(default, overrides) => Self { default, overrides },
            })
        }
    }

    #[derive(serde::Deserialize)]
    #[serde(untagged)]
    enum InfoModesFormats<M, I> {
        Simple(M),
        Extended(M, Vec<(I, M)>),
    }
}
