use crate::{
    CmdResps, ItemId, ItemIdBr,
    shared::BrResolveInfallible,
    stats::{
        StatOptionExt, StatOptionFitDmg, StatOptionFitMining, StatOptionFitOutCps, StatOptionFitOutNps,
        StatOptionFitOutRps, StatOptionInt, StatOptionMass, fleet::FleetStatsOptionsResolved,
    },
};

/// Which stats to fetch for a fleet.
///
/// By default, all stats are not fetched.
#[derive(Clone)]
pub struct FleetStatsOptions<I = ItemId> {
    default: bool = false,
    overrides: Vec<FleetStatOption<I>> = Vec::new(),
}
impl<I> Default for FleetStatsOptions<I> {
    fn default() -> Self {
        Self { .. }
    }
}

pub type FleetStatsOptionsBr = FleetStatsOptions<ItemIdBr>;

#[derive(Clone)]
enum FleetStatOption<I> {
    Dmg(StatOptionInt<StatOptionFitDmg<I>>),
    Mps(StatOptionExt<StatOptionFitMining>),
    OutgoingNps(StatOptionInt<StatOptionFitOutNps<I>>),
    OutgoingRps(StatOptionInt<StatOptionFitOutRps<I>>),
    OutgoingCps(StatOptionInt<StatOptionFitOutCps<I>>),
    Mass(StatOptionExt<StatOptionMass>),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<I> FleetStatsOptions<I> {
    /// True to have all supported stats enabled by default, false to have them disabled.
    pub fn new(default: bool) -> Self {
        Self { default, .. }
    }
    /// True to have all supported stats enabled by default, false to have them disabled.
    pub fn with_override_capacity(default: bool, capacity: usize) -> Self {
        Self {
            default,
            overrides: Vec::with_capacity(capacity),
        }
    }
    pub fn with_dmg(mut self, option: StatOptionExt<StatOptionFitDmg<I>>) -> Self {
        self.overrides.push(FleetStatOption::Dmg(option.into_internal()));
        self
    }
    pub fn with_mps(mut self, option: StatOptionExt<StatOptionFitMining>) -> Self {
        self.overrides.push(FleetStatOption::Mps(option));
        self
    }
    pub fn with_outgoing_nps(mut self, option: StatOptionExt<StatOptionFitOutNps<I>>) -> Self {
        self.overrides
            .push(FleetStatOption::OutgoingNps(option.into_internal()));
        self
    }
    pub fn with_outgoing_rps(mut self, option: StatOptionExt<StatOptionFitOutRps<I>>) -> Self {
        self.overrides
            .push(FleetStatOption::OutgoingRps(option.into_internal()));
        self
    }
    pub fn with_outgoing_cps(mut self, option: StatOptionExt<StatOptionFitOutCps<I>>) -> Self {
        self.overrides
            .push(FleetStatOption::OutgoingCps(option.into_internal()));
        self
    }
    pub fn with_mass(mut self, option: StatOptionExt<StatOptionMass>) -> Self {
        self.overrides.push(FleetStatOption::Mass(option));
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl BrResolveInfallible for FleetStatsOptionsBr {
    type Target = FleetStatsOptions;
    fn br_resolve_infallible(self, resps: &CmdResps) -> Self::Target {
        let mut overrides = Vec::with_capacity(self.overrides.len());
        for option in self.overrides.into_iter() {
            overrides.push(match option {
                FleetStatOption::Dmg(option) => FleetStatOption::Dmg(option.br_resolve_stored(resps)),
                FleetStatOption::Mps(option) => FleetStatOption::Mps(option),
                FleetStatOption::OutgoingNps(option) => FleetStatOption::OutgoingNps(option.br_resolve_stored(resps)),
                FleetStatOption::OutgoingRps(option) => FleetStatOption::OutgoingRps(option.br_resolve_stored(resps)),
                FleetStatOption::OutgoingCps(option) => FleetStatOption::OutgoingCps(option.br_resolve_stored(resps)),
                FleetStatOption::Mass(option) => FleetStatOption::Mass(option),
            });
        }
        Self::Target {
            default: self.default,
            overrides,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Default + stat resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FleetStatsOptions {
    pub(super) fn stat_resolve(self) -> FleetStatsOptionsResolved {
        let mut resolved = FleetStatsOptionsResolved::blank_from_default(self.default);
        for option in self.overrides.into_iter() {
            match option {
                FleetStatOption::Dmg(option) => resolved.dmg = option.into_resolved(),
                FleetStatOption::Mps(option) => resolved.mps = option.into_resolved(),
                FleetStatOption::OutgoingNps(option) => resolved.outgoing_nps = option.into_resolved(),
                FleetStatOption::OutgoingRps(option) => resolved.outgoing_rps = option.into_resolved(),
                FleetStatOption::OutgoingCps(option) => resolved.outgoing_cps = option.into_resolved(),
                FleetStatOption::Mass(option) => resolved.mass = option.into_resolved(),
            }
        }
        resolved.complete_extended_defaults();
        resolved
    }
}

impl From<FleetStatsOptions<ItemId>> for FleetStatsOptionsResolved {
    fn from(value: FleetStatsOptions<ItemId>) -> Self {
        value.stat_resolve()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde")]
mod custom_serde {
    use serde::de::{Deserialize, Deserializer, IgnoredAny, MapAccess, Visitor};

    use super::*;

    impl<'de, I> Deserialize<'de> for FleetStatsOptions<I>
    where
        I: Deserialize<'de>,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_map(VisitorImpl(std::marker::PhantomData))
        }
    }

    #[derive(serde::Deserialize)]
    #[serde(field_identifier, rename_all = "snake_case")]
    enum Key {
        Default,
        Dmg,
        Mps,
        OutgoingNps,
        OutgoingRps,
        OutgoingCps,
        Mass,
        #[serde(other)]
        Unknown,
    }

    struct VisitorImpl<I>(std::marker::PhantomData<I>);
    impl<'de, I> Visitor<'de> for VisitorImpl<I>
    where
        I: Deserialize<'de>,
    {
        type Value = FleetStatsOptions<I>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("struct FleetStatsOptions")
        }

        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: MapAccess<'de>,
        {
            let mut options = Self::Value::default();
            let ovrd = &mut options.overrides;
            while let Some(key) = map.next_key::<Key>()? {
                match key {
                    Key::Default => options.default = map.next_value()?,
                    Key::Dmg => {
                        ovrd.extend(map.next_value::<Option<_>>()?.map(FleetStatOption::Dmg));
                    }
                    Key::Mps => {
                        ovrd.extend(map.next_value::<Option<_>>()?.map(FleetStatOption::Mps));
                    }
                    Key::OutgoingNps => {
                        ovrd.extend(map.next_value::<Option<_>>()?.map(FleetStatOption::OutgoingNps));
                    }
                    Key::OutgoingRps => {
                        ovrd.extend(map.next_value::<Option<_>>()?.map(FleetStatOption::OutgoingRps));
                    }
                    Key::OutgoingCps => {
                        ovrd.extend(map.next_value::<Option<_>>()?.map(FleetStatOption::OutgoingCps));
                    }
                    Key::Mass => {
                        ovrd.extend(map.next_value::<Option<_>>()?.map(FleetStatOption::Mass));
                    }
                    Key::Unknown => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            Ok(options)
        }
    }
}
