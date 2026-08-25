use std::hash::Hash;

use crate::{
    CmdResps,
    shared::{BrResolveFallible, BrResolveInfallible},
    util::RMap,
};

// Representation form which is compact; it is hard to use work with it directly, so types which are
// often queried should be converted into something else.
#[derive(Clone)]
pub(crate) struct OvrdCompact<K, V> {
    default: V,
    overrides: Vec<(V, Vec<K>)>,
}
impl<K, V> Default for OvrdCompact<K, V>
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
impl<K, V> OvrdCompact<K, V> {
    pub(crate) fn set_default(&mut self, default: V) {
        self.default = default;
    }
    pub(crate) fn add_overrides(&mut self, value: V, keys: impl Iterator<Item = K>) {
        self.overrides.push((value, keys.collect()))
    }
}

// Representation form which takes space, but is easy to query. This version should be used for
// relatively small copiable values.
#[derive(Clone)]
pub(crate) struct OvrdMapLight<K, V> {
    default: V,
    overrides: RMap<K, V>,
}
impl<K, V> Default for OvrdMapLight<K, V>
where
    V: Default,
{
    fn default() -> Self {
        Self {
            default: V::default(),
            overrides: RMap::new(),
        }
    }
}
impl<K, V> OvrdMapLight<K, V>
where
    K: Eq + Hash,
    V: Copy,
{
    pub(crate) fn get(&self, key: &K) -> V {
        match self.overrides.get(key) {
            Some(value) => *value,
            None => self.default,
        }
    }
    pub(crate) fn set_default(&mut self, default: V) {
        self.default = default;
    }
    pub(crate) fn add_overrides(&mut self, value: V, keys: impl Iterator<Item = K>) {
        self.overrides.extend(keys.map(|key| (key, value)))
    }
}

// Representation form which takes space, but is easy to query. This version should be used for
// relatively large or non-copiable values.
#[derive(Clone)]
pub(crate) struct OvrdMapHeavy<K, V> {
    default: V,
    override_refs: RMap<K, usize>,
    override_vals: Vec<V>,
}
impl<K, V> Default for OvrdMapHeavy<K, V>
where
    V: Default,
{
    fn default() -> Self {
        Self {
            default: V::default(),
            override_refs: RMap::new(),
            override_vals: Vec::new(),
        }
    }
}
impl<K, V> OvrdMapHeavy<K, V>
where
    K: Eq + Hash,
{
    pub(crate) fn add_overrides(&mut self, value: V, keys: impl Iterator<Item = K>) {
        let index = self.override_vals.len();
        self.override_vals.push(value);
        self.override_refs.extend(keys.map(|key| (key, index)));
    }
    pub(crate) fn get(&self, key: &K) -> &V {
        match self.override_refs.get(key) {
            Some(index) => &self.override_vals[*index],
            None => &self.default,
        }
    }
    pub(crate) fn get_default(&self) -> &V {
        &self.default
    }
    pub(crate) fn iter_overrides(&self) -> impl ExactSizeIterator<Item = (K, &V)>
    where
        K: Copy,
    {
        self.override_refs
            .iter()
            .map(|(key, index)| (*key, &self.override_vals[*index]))
    }
    pub(crate) fn override_len(&self) -> usize {
        self.override_refs.len()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<K1, K2, V1, V2> OvrdCompact<K1, V1>
where
    K1: BrResolveFallible<Target = K2>,
    V1: BrResolveInfallible<Target = V2>,
{
    pub(crate) fn br_resolve(self, cmd_resps: &CmdResps) -> OvrdCompact<K2, V2> {
        let mut overrides = Vec::with_capacity(self.overrides.len());
        for override_ in self.overrides.into_iter() {
            let value = override_.0.br_resolve_infallible(cmd_resps);
            let mut keys = Vec::with_capacity(override_.1.len());
            // Silently skip keys which cannot be resolved
            keys.extend(
                override_
                    .1
                    .into_iter()
                    .filter_map(|key| key.br_resolve_fallible(cmd_resps).ok()),
            );
            if !keys.is_empty() {
                overrides.push((value, keys))
            }
        }
        OvrdCompact {
            default: self.default.br_resolve_infallible(cmd_resps),
            overrides,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<K, V> OvrdMapHeavy<K, V>
where
    K: Eq + Hash,
{
    pub(crate) fn from_compact_with_conversion<VC>(compact: OvrdCompact<K, VC>) -> Self
    where
        VC: Into<V>,
    {
        let mut heavy = Self {
            default: compact.default.into(),
            override_refs: RMap::with_capacity(compact.overrides.iter().map(|(_, keys)| keys.len()).sum()),
            override_vals: Vec::with_capacity(compact.overrides.len()),
        };
        for (value, keys) in compact.overrides.into_iter() {
            heavy.add_overrides(value.into(), keys.into_iter());
        }
        heavy
    }
}

impl<K, V> OvrdMapLight<K, V> {
    pub(crate) fn from_default(default: V) -> Self {
        Self {
            default,
            overrides: RMap::new(),
        }
    }
    pub(crate) fn from_compact_with_br_resolution<B>(compact_br: OvrdCompact<B, V>, ctl_cmd_resps: &CmdResps) -> Self
    where
        K: Eq + Hash,
        V: Copy + PartialEq,
        B: BrResolveFallible<Target = K>,
    {
        let default_value = compact_br.default;
        let mut overrides = RMap::with_capacity(calc_needed_space(default_value, &compact_br.overrides));
        for (override_value, override_keys_br) in compact_br.overrides.into_iter() {
            // Getter falls back to default, do not add entries with it
            if override_value == default_value {
                continue;
            }
            for override_key_br in override_keys_br {
                if let Ok(override_key) = override_key_br.br_resolve_fallible(ctl_cmd_resps) {
                    overrides.insert(override_key, override_value);
                }
            }
        }
        Self {
            default: default_value,
            overrides,
        }
    }
}

// This repeats filtering done in actual converting methods
fn calc_needed_space<K, V>(default: V, overrides: &[(V, Vec<K>)]) -> usize
where
    V: Copy + PartialEq,
{
    overrides
        .iter()
        .filter_map(|(value, keys)| match *value == default {
            true => None,
            false => Some(keys.len()),
        })
        .sum()
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde")]
mod custom_serde {
    use serde::de::{Deserialize, Deserializer};

    use super::*;

    impl<'de, K, V> Deserialize<'de> for OvrdMapLight<K, V>
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
                    overrides: RMap::new(),
                },
                OverridableFormats::Extended(default, overrides) => {
                    let mut map = RMap::with_capacity(overrides.iter().map(|(_, keys)| keys.len()).sum());
                    map.extend(
                        overrides
                            .into_iter()
                            .flat_map(|(value, keys)| keys.into_iter().map(move |key| (key, value))),
                    );
                    Self {
                        default,
                        overrides: map,
                    }
                }
            })
        }
    }

    impl<'de, K, V> Deserialize<'de> for OvrdCompact<K, V>
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
