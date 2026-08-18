use std::{collections::HashMap, hash::Hash};

use crate::{CmdResps, err::BrResolveError, shared::BrResolvable};

// Representation form which is more convenient for use by info builders; should be used on commands
// which are directly executable (i.e. commands with backreferences resolved into IDs)
#[derive(Clone)]
pub(in crate::info) struct InfoModes<M, I> {
    pub(in crate::info) default: M,
    pub(in crate::info) overrides: HashMap<I, M>,
}
impl<M, I> InfoModes<M, I> {
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
impl<M, I> Default for InfoModes<M, I>
where
    M: Default,
{
    fn default() -> Self {
        Self {
            default: M::default(),
            overrides: HashMap::new(),
        }
    }
}

// Representation form which is compact; should be used only when it is not directly usable by info
// builders (e.g. command with backreferences)
#[derive(Clone)]
pub(in crate::info) struct InfoModesCompact<M, I> {
    pub(in crate::info) default: M,
    pub(in crate::info) overrides: Vec<(M, Vec<I>)>,
}
impl<M, I> Default for InfoModesCompact<M, I>
where
    M: Default,
{
    fn default() -> Self {
        Self {
            default: M::default(),
            overrides: Vec::new(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
// Forward conversions, from "external" to "internal" form
impl<M, I> InfoModes<M, I> {
    pub(in crate::info) fn from_simple(mode: M) -> Self {
        Self {
            default: mode,
            overrides: HashMap::new(),
        }
    }
    pub(in crate::info) fn from_compact(modes: InfoModesCompact<M, I>) -> Self
    where
        M: Copy + PartialEq,
        I: Eq + Hash,
    {
        let default = modes.default;
        let mut overrides = HashMap::with_capacity(calc_needed_space(&modes.overrides, default));
        for (over_mode, over_ids) in modes.overrides {
            // Getter falls back to default, do not add entries with it
            if over_mode == default {
                continue;
            }
            for over_id in over_ids {
                overrides.insert(over_id, over_mode);
            }
        }
        Self { default, overrides }
    }
    pub(in crate::info) fn from_compact_br<B>(
        compact_br: InfoModesCompact<M, B>,
        ctl_cmd_resps: &CmdResps,
    ) -> Result<Self, BrResolveError>
    where
        M: Copy + PartialEq,
        B: BrResolvable<Target = I>,
        I: Eq + Hash,
    {
        let default = compact_br.default;
        let mut overrides = HashMap::with_capacity(calc_needed_space(&compact_br.overrides, default));
        for (over_mode, over_backrefs) in compact_br.overrides {
            // Getter falls back to default, do not add entries with it
            if over_mode == default {
                continue;
            }
            for over_backref in over_backrefs {
                overrides.insert(over_backref.br_resolve(ctl_cmd_resps)?, over_mode);
            }
        }
        Ok(Self { default, overrides })
    }
}

// This repeats filtering done in actual converting methods
fn calc_needed_space<M, I>(overrides: &[(M, Vec<I>)], default: M) -> usize
where
    M: Copy + PartialEq,
{
    overrides
        .iter()
        .filter_map(|(mode, ids)| match *mode == default {
            true => None,
            false => Some(ids.len()),
        })
        .sum()
}

// Backward conversion
impl<M, I> InfoModes<M, I> {
    pub(in crate::info) fn into_compact_br<B>(self) -> InfoModesCompact<M, B>
    where
        M: Eq + Hash,
        B: From<I>,
    {
        let mut rev_map: HashMap<M, Vec<B>> = HashMap::new();
        for (id, mode) in self.overrides.into_iter() {
            if mode == self.default {
                continue;
            }
            rev_map.entry(mode).or_default().push(id.into());
        }
        InfoModesCompact {
            default: self.default,
            overrides: rev_map.into_iter().collect(),
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
        M: Copy + Deserialize<'de>,
        I: Eq + Hash + Deserialize<'de>,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            Ok(match InfoModesFormats::deserialize(deserializer)? {
                InfoModesFormats::Simple(default) => Self {
                    default,
                    overrides: HashMap::new(),
                },
                InfoModesFormats::Extended(default, overrides) => Self {
                    default,
                    overrides: overrides
                        .into_iter()
                        .flat_map(|(mode, ids)| ids.into_iter().map(move |id| (id, mode)))
                        .collect(),
                },
            })
        }
    }

    impl<'de, M, I> Deserialize<'de> for InfoModesCompact<M, I>
    where
        M: Deserialize<'de>,
        I: Deserialize<'de>,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            Ok(match InfoModesFormats::deserialize(deserializer)? {
                InfoModesFormats::Simple(default) => Self {
                    default,
                    overrides: Vec::new(),
                },
                InfoModesFormats::Extended(default, overrides) => Self { default, overrides },
            })
        }
    }

    #[derive(serde::Deserialize)]
    #[serde(untagged)]
    enum InfoModesFormats<M, I> {
        Simple(M),
        Extended(M, Vec<(M, Vec<I>)>),
    }
}
