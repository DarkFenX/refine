use std::{collections::HashMap, hash::Hash};

use crate::{CmdResps, err::BrResolveError, shared::BrResolvable};

// Representation form which is more convenient for use by info builders; should be used on commands
// which are directly executable (i.e. commands with backreferences resolved into IDs)
#[derive(Clone)]
pub(in crate::info) struct OverridableMap<K, V> {
    pub(in crate::info) default: V,
    pub(in crate::info) overrides: HashMap<K, V>,
}
impl<K, V> OverridableMap<K, V> {
    pub(in crate::info) fn get(&self, id: &K) -> V
    where
        K: Eq + Hash,
        V: Copy,
    {
        match self.overrides.get(id) {
            Some(mode) => *mode,
            None => self.default,
        }
    }
}
impl<K, V> Default for OverridableMap<K, V>
where
    V: Default,
{
    fn default() -> Self {
        Self {
            default: V::default(),
            overrides: HashMap::new(),
        }
    }
}

// Representation form which is compact; should be used only when it is not directly usable by info
// builders (e.g. command with backreferences)
#[derive(Clone)]
pub(in crate::info) struct OverridableCompact<K, V> {
    pub(in crate::info) default: V,
    pub(in crate::info) overrides: Vec<(V, Vec<K>)>,
}
impl<K, V> Default for OverridableCompact<K, V>
where
    V: Default,
{
    fn default() -> Self {
        Self {
            default: V::default(),
            overrides: Vec::new(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
// Forward conversions, from "external" to "internal" form
impl<K, V> OverridableMap<K, V> {
    pub(in crate::info) fn from_default(default: V) -> Self {
        Self {
            default,
            overrides: HashMap::new(),
        }
    }
    pub(in crate::info) fn from_compact(compact: OverridableCompact<K, V>) -> Self
    where
        K: Eq + Hash,
        V: Copy + PartialEq,
    {
        let default = compact.default;
        let mut overrides = HashMap::with_capacity(calc_needed_space(default, &compact.overrides));
        for (value, keys) in compact.overrides {
            // Getter falls back to default, do not add entries with it
            if value == default {
                continue;
            }
            for key in keys {
                overrides.insert(key, value);
            }
        }
        Self { default, overrides }
    }
    pub(in crate::info) fn from_compact_br<B>(
        compact_br: OverridableCompact<B, V>,
        ctl_cmd_resps: &CmdResps,
    ) -> Result<Self, BrResolveError>
    where
        K: Eq + Hash,
        V: Copy + PartialEq,
        B: BrResolvable<Target = K>,
    {
        let default = compact_br.default;
        let mut overrides = HashMap::with_capacity(calc_needed_space(default, &compact_br.overrides));
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
fn calc_needed_space<K, V>(default: V, overrides: &[(V, Vec<K>)]) -> usize
where
    V: Copy + PartialEq,
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
impl<K, V> OverridableMap<K, V> {
    pub(in crate::info) fn into_compact_br<B>(self) -> OverridableCompact<B, V>
    where
        V: Eq + Hash,
        B: From<K>,
    {
        let mut rev_map: HashMap<V, Vec<B>> = HashMap::new();
        for (id, value) in self.overrides.into_iter() {
            if value == self.default {
                continue;
            }
            rev_map.entry(value).or_default().push(id.into());
        }
        OverridableCompact {
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

    impl<'de, K, V> Deserialize<'de> for OverridableMap<K, V>
    where
        K: Eq + Hash + Deserialize<'de>,
        V: Copy + Deserialize<'de>,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            Ok(match OverridableFormats::deserialize(deserializer)? {
                OverridableFormats::Simple(default) => Self {
                    default,
                    overrides: HashMap::new(),
                },
                OverridableFormats::Extended(default, overrides) => Self {
                    default,
                    overrides: overrides
                        .into_iter()
                        .flat_map(|(mode, ids)| ids.into_iter().map(move |id| (id, mode)))
                        .collect(),
                },
            })
        }
    }

    impl<'de, K, V> Deserialize<'de> for OverridableCompact<K, V>
    where
        V: Deserialize<'de>,
        K: Deserialize<'de>,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            Ok(match OverridableFormats::deserialize(deserializer)? {
                OverridableFormats::Simple(default) => Self {
                    default,
                    overrides: Vec::new(),
                },
                OverridableFormats::Extended(default, overrides) => Self { default, overrides },
            })
        }
    }

    #[derive(serde::Deserialize)]
    #[serde(untagged)]
    enum OverridableFormats<K, V> {
        Simple(V),
        Extended(V, Vec<(V, Vec<K>)>),
    }
}
