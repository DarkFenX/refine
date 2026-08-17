use std::{collections::HashMap, hash::Hash};

use crate::{CmdResps, err::BackrefRenderError, shared::CtlCmdBr};

// Representation form which is more convenient for use by info builders
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
// builders
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
impl<M, I> InfoModes<M, I> {
    pub(in crate::info) fn from_simple(mode: M) -> Self {
        Self {
            default: mode,
            overrides: HashMap::new(),
        }
    }
    pub(in crate::info) fn from_compact(modes: InfoModesCompact<M, I>) -> Self
    where
        M: Copy,
        I: Eq + Hash,
    {
        Self {
            default: modes.default,
            overrides: modes
                .overrides
                .into_iter()
                .flat_map(|overrides| overrides.1.into_iter().map(move |id| (id, overrides.0)))
                .collect(),
        }
    }
    pub(in crate::info) fn from_compact_br<B>(
        compact_br: InfoModesCompact<M, B>,
        ctl_cmd_resps: &CmdResps,
    ) -> Result<Self, BackrefRenderError>
    where
        M: Copy,
        B: CtlCmdBr<Target = I>,
        I: Eq + Hash,
    {
        Ok(Self {
            default: compact_br.default,
            overrides: compact_br
                .overrides
                .into_iter()
                .flat_map(|(mode, backrefs)| {
                    backrefs
                        .into_iter()
                        .map(move |backref| backref.render(ctl_cmd_resps).map(|id| (id, mode)))
                })
                .collect::<Result<_, _>>()?,
        })
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
