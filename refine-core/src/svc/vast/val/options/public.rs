use crate::{FitId, ItemId, OptionExt, svc::vast::val::ValKind};

/// Solar system validation options.
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct ValOptionsSol {
    /// Fit IDs to validate.
    #[cfg_attr(feature = "serde", serde(default))]
    pub fit_ids: Vec<FitId>,
    /// Validation options.
    #[cfg_attr(feature = "serde", serde(default))]
    pub options: ValOptions,
}

/// Validation options.
#[derive(Clone)]
pub struct ValOptions<I = ItemId> {
    pub(super) default: bool = true,
    pub(super) overrides: Vec<(ValKind, OptionExt<ValEnabled<I>>)> = Vec::new(),
}
impl<I> Default for ValOptions<I> {
    fn default() -> Self {
        Self { .. }
    }
}
impl<I> ValOptions<I> {
    /// True to have all validations enabled by default, false to have them disabled.
    pub fn new(default: bool) -> Self {
        Self { default, .. }
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Generic
    ////////////////////////////////////////////////////////////////////////////////////////////////
    /// Fails for any items which are not loaded. Items can become not loaded when they were added
    /// to a fit, but current data source does not have an EVE item with corresponding type ID.
    pub fn with_not_loaded_item(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::NotLoadedItem, option));
        self
    }
    /// Any EVE item usually can be represented by a single item kind in the lib. For example, an
    /// item from Implant category with "boosterness" attribute is a booster. This validation checks
    /// relations between user-defined item kind and item kind detected for a backing EVE item.
    pub fn with_item_kind(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::ItemKind, option));
        self
    }
    /// Fails when a direct skill requirement is not satisfied for an item.
    pub fn with_skill_reqs(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::SkillReqs, option));
        self
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Implants/boosters
    ////////////////////////////////////////////////////////////////////////////////////////////////
    /// Fails when multiple implants attempt to take the same slot.
    pub fn with_implant_slot_index(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::ImplantSlotIndex, option));
        self
    }
    /// Fails when multiple boosters attempt to take the same slot.
    pub fn with_booster_slot_index(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::BoosterSlotIndex, option));
        self
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Shared between mod-alike items
    ////////////////////////////////////////////////////////////////////////////////////////////////
    /// Fails when items take more CPU than ship can produce.
    pub fn with_cpu(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::Cpu, option));
        self
    }
    /// Fails when items take more PG than ship can produce.
    pub fn with_powergrid(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::Powergrid, option));
        self
    }
    /// When a fit has any items which can be fit to specific set of ships (identified by ship list
    /// and ship group list), and ship does not fall into it, this validation is failed for those
    /// items.
    pub fn with_ship_limit(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::ShipLimit, option));
        self
    }
    /// When an item has limit on how many items from its group can be fitted, and count of fitted
    /// items exceeds that, this validation fails.
    pub fn with_max_group_fitted(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::MaxGroupFitted, option));
        self
    }
    /// When an item has limit on how many items from its group can be online, and count of online
    /// items exceeds that, this validation fails.
    pub fn with_max_group_online(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::MaxGroupOnline, option));
        self
    }
    /// When an item has limit on how many items from its group can be active, and count of active
    /// items exceeds that, this validation fails.
    pub fn with_max_group_active(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::MaxGroupActive, option));
        self
    }
    /// When an item has limit on how many items with the same type ID can be fitted, and count of
    /// fitted items exceeds that, this validation fails.
    pub fn with_max_type_fitted(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::MaxTypeFitted, option));
        self
    }
    /// Checks that structure items are not fit to a ship fit, and ship items are not fit to a
    /// structure fit. Type of fit is defined by its ship kind.
    pub fn with_item_vs_ship_kind(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::ItemVsShipKind, option));
        self
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Modules
    ////////////////////////////////////////////////////////////////////////////////////////////////
    /// If any of high slot modules occupy slots with indices higher than ship supports, this
    /// validation fails, only for those modules.
    pub fn with_high_slot_count(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::HighSlotCount, option));
        self
    }
    /// If any of medium slot modules occupy slots with indices higher than ship supports, this
    /// validation fails, only for those modules.
    pub fn with_mid_slot_count(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::MidSlotCount, option));
        self
    }
    /// If any of low slot modules occupy slots with indices higher than ship supports, this
    /// validation fails, only for those modules.
    pub fn with_low_slot_count(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::LowSlotCount, option));
        self
    }
    /// If count of taken turret slots is higher than ship provides, this validation fails for all
    /// modules which need a turret slot.
    pub fn with_turret_slot_count(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::TurretSlotCount, option));
        self
    }
    /// If count of taken launcher slots is higher than ship provides, this validation fails for all
    /// modules which need a launcher slot.
    pub fn with_launcher_slot_count(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::LauncherSlotCount, option));
        self
    }
    /// If any module has state higher than it supports (e.g. active bulkhead), this validation
    /// fails.
    pub fn with_module_state(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::ModuleState, option));
        self
    }
    /// Fails when any capital modules (large-volume modules) are fit to subcapital ships.
    pub fn with_capital_module(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::CapitalModule, option));
        self
    }
    /// Fails when fit has any items overloaded, and overload skill requirement is not satisfied.
    pub fn with_overload_skill(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::OverloadSkill, option));
        self
    }
    /// Fails when any item consumes more cap than ship has. Only on-fit items which consume cap are
    /// considered for this, anything else (e.g. incoming neutralizers) are ignored.
    pub fn with_unusable_cap(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::UnusableCap, option));
        self
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Charges
    ////////////////////////////////////////////////////////////////////////////////////////////////
    /// Some modules restrict charges which can be loaded into them by group; if charge from
    /// disallowed group is loaded, validation fails for charge.
    pub fn with_charge_group(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::ChargeGroup, option));
        self
    }
    /// Some charges restrict into which modules they can be loaded by module group; if charge from
    /// disallowed group is loaded, validation fails for charge.
    pub fn with_charge_parent_group(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::ChargeParentGroup, option));
        self
    }
    /// Some charges and modules have charge size set. When a module specifies it, and has a charge
    /// without size or with mismatching size loaded, this validation fails for the charge.
    pub fn with_charge_size(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::ChargeSize, option));
        self
    }
    /// Fails when volume of a single charge is larger than capacity of a module it's loaded into.
    pub fn with_charge_volume(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::ChargeVolume, option));
        self
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Rigs
    ////////////////////////////////////////////////////////////////////////////////////////////////
    /// Fails when fit has more rigs than ship has rig slots.
    pub fn with_rig_slot_count(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::RigSlotCount, option));
        self
    }
    /// Fails when rigs take more calibration than ship can produce.
    pub fn with_calibration(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::Calibration, option));
        self
    }
    /// Ships and rigs specify rig size; when those mismatch, this validation fails for rigs.
    pub fn with_rig_size(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::RigSize, option));
        self
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Services
    ////////////////////////////////////////////////////////////////////////////////////////////////
    /// Fails when fit has more services than ship/structure has service slots.
    pub fn with_service_slot_count(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::ServiceSlotCount, option));
        self
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // T3 subsystems/stances
    ////////////////////////////////////////////////////////////////////////////////////////////////
    /// Fails when fit has more subsystems than ship has subsystem slots.
    pub fn with_subsystem_slot_count(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::SubsystemSlotCount, option));
        self
    }
    /// Fails when multiple subsystems attempt to take the same slot.
    pub fn with_subsystem_slot_index(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::SubsystemSlotIndex, option));
        self
    }
    /// Fails when a ship which can't have a stance but has one.
    pub fn with_ship_stance(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::ShipStance, option));
        self
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Drones
    ////////////////////////////////////////////////////////////////////////////////////////////////
    /// Fails when drones take more volume than ship's drone bay has.
    pub fn with_drone_bay_volume(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::DroneBayVolume, option));
        self
    }
    /// Fails when fit has more in-space drones than ship supports.
    pub fn with_launched_drone_count(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::LaunchedDroneCount, option));
        self
    }
    /// Fails when in-space drones take more bandwidth than ship provides.
    pub fn with_drone_bandwidth(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::DroneBandwidth, option));
        self
    }
    /// Fails when fit has any drones when ship supports none.
    pub fn with_unlaunchable_drone_slot(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::UnlaunchableDroneSlot, option));
        self
    }
    /// Fails when fit has any drones which take more bandwidth than ship provides.
    pub fn with_unlaunchable_drone_bandwidth(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::UnlaunchableDroneBandwidth, option));
        self
    }
    /// Ship can limit which drone groups can be put into its drone bay. If it does, and drones from
    /// mismatching groups are fit, this validation fails for those drones.
    pub fn with_drone_group(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::DroneGroup, option));
        self
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Fighters
    ////////////////////////////////////////////////////////////////////////////////////////////////
    /// Fails when fighters take more volume than ship's fighter bay has.
    pub fn with_fighter_bay_volume(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::FighterBayVolume, option));
        self
    }
    /// Fails when fit has more in-space fighters than ship supports.
    pub fn with_launched_fighter_count(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::LaunchedFighterCount, option));
        self
    }
    /// Fails when fit has more in-space light fighters than ship supports.
    pub fn with_launched_light_fighter_count(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::LaunchedLightFighterCount, option));
        self
    }
    /// Fails when fit has more in-space heavy fighters than ship supports.
    pub fn with_launched_heavy_fighter_count(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::LaunchedHeavyFighterCount, option));
        self
    }
    /// Fails when fit has more in-space support fighters than ship supports.
    pub fn with_launched_support_fighter_count(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::LaunchedSupportFighterCount, option));
        self
    }
    /// Fails when fit has more in-space standup light fighters than ship supports.
    pub fn with_launched_st_light_fighter_count(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::LaunchedStLightFighterCount, option));
        self
    }
    /// Fails when fit has more in-space standup heavy fighters than ship supports.
    pub fn with_launched_st_heavy_fighter_count(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::LaunchedStHeavyFighterCount, option));
        self
    }
    /// Fails when fit has more in-space standup support fighters than ship supports.
    pub fn with_launched_st_support_fighter_count(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::LaunchedStSupportFighterCount, option));
        self
    }
    /// Fails when fit has any fighters when ship supports none.
    pub fn with_unlaunchable_fighter(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::UnlaunchableFighter, option));
        self
    }
    /// Fails when fit has any light fighters when ship supports none.
    pub fn with_unlaunchable_light_fighter(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::UnlaunchableLightFighter, option));
        self
    }
    /// Fails when fit has any heavy fighters when ship supports none.
    pub fn with_unlaunchable_heavy_fighter(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::UnlaunchableHeavyFighter, option));
        self
    }
    /// Fails when fit has any support fighters when ship supports none.
    pub fn with_unlaunchable_support_fighter(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::UnlaunchableSupportFighter, option));
        self
    }
    /// Fails when fit has any standup light fighters when ship supports none.
    pub fn with_unlaunchable_st_light_fighter(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::UnlaunchableStLightFighter, option));
        self
    }
    /// Fails when fit has any standup heavy fighters when ship supports none.
    pub fn with_unlaunchable_st_heavy_fighter(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::UnlaunchableStHeavyFighter, option));
        self
    }
    /// Fails when fit has any standup support fighters when ship supports none.
    pub fn with_unlaunchable_st_support_fighter(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::UnlaunchableStSupportFighter, option));
        self
    }
    /// Fails for fighter squads which have more fighters than squad supports.
    pub fn with_fighter_squad_size(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::FighterSquadSize, option));
        self
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Projection, destination side
    ////////////////////////////////////////////////////////////////////////////////////////////////
    /// Fails when any modules are active but their activation is blocked (e.g. scrambled MWDs).
    pub fn with_activation_blocked(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::ActivationBlocked, option));
        self
    }
    /// Fails when any items have running effects which are stopped by external factors (e.g.
    /// scrambled fighter MWD).
    pub fn with_effect_stopper(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::EffectStopper, option));
        self
    }
    /// When a cloak is active and something blocks it (weather, modules incompatible with cloaking
    /// like sieges, multiple cloaks fit to ship), this validation fails.
    pub fn with_cloaking_blocked(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::CloakingBlocked, option));
        self
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Projection, source side
    ////////////////////////////////////////////////////////////////////////////////////////////////
    /// Fails when item defines which targets it can be applied to, but some of its targets do not
    /// belong to it.
    pub fn with_projectee_filter(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::ProjecteeFilter, option));
        self
    }
    /// Fails when item is marked as assistive, and is applied to a target which is immune to
    /// assistance.
    pub fn with_assist_immunity(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::AssistImmunity, option));
        self
    }
    /// Fails when item is marked as offensive, and is applied to a target which is immune to
    /// offense.
    pub fn with_offense_immunity(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::OffenseImmunity, option));
        self
    }
    /// Fails when item's effect can be resisted, and is applied to a target which completely
    /// resists its effect.
    pub fn with_resist_immunity(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::ResistImmunity, option));
        self
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Sec zone
    ////////////////////////////////////////////////////////////////////////////////////////////////
    /// Fails when some items are not allowed to be fitted in current sol security zone.
    pub fn with_sec_zone_fitted(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::SecZoneFitted, option));
        self
    }
    /// Fails when some items are not allowed to be online in current sol security zone.
    pub fn with_sec_zone_online(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::SecZoneOnline, option));
        self
    }
    /// Fails when some items are not allowed to be active in current sol security zone.
    pub fn with_sec_zone_active(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::SecZoneActive, option));
        self
    }
    /// Fails when fit has items which cannot be onlined in current sol security zone.
    pub fn with_sec_zone_unonlineable(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::SecZoneUnonlineable, option));
        self
    }
    /// Fails when fit has items which cannot be activated in current sol security zone.
    pub fn with_sec_zone_unactivable(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::SecZoneUnactivable, option));
        self
    }
    /// Fails when some effects are not allowed to run in current sol security zone.
    pub fn with_sec_zone_effect(mut self, option: OptionExt<ValEnabled<I>>) -> Self {
        self.overrides.push((ValKind::SecZoneEffect, option));
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<I1> ValOptions<I1> {
    pub fn try_map_ids<I2, E, M>(self, mut item_mapper: M) -> Result<ValOptions<I2>, E>
    where
        M: FnMut(I1) -> Result<I2, E>,
    {
        let mut new_overrides = Vec::with_capacity(self.overrides.len());
        for (kind, option) in self.overrides {
            let new_option = match option {
                OptionExt::Disabled => OptionExt::Disabled,
                OptionExt::Enabled => OptionExt::Enabled,
                OptionExt::EnabledExtended(enabled) => {
                    let mut kfs = Vec::with_capacity(enabled.kfs.len());
                    for kf in enabled.kfs {
                        kfs.push(item_mapper(kf)?);
                    }
                    OptionExt::EnabledExtended(ValEnabled { kfs })
                }
            };
            new_overrides.push((kind, new_option));
        }
        Ok(ValOptions {
            default: self.default,
            overrides: new_overrides,
        })
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize), serde(transparent))]
#[derive(Clone)]
pub struct ValEnabled<I = ItemId> {
    /// Known failures of a validation.
    ///
    /// Every validation failure is attached to an item. Items listed here will not be returned as
    /// validation failures. If all validation's failures are known, it is passed.
    pub kfs: Vec<I> = Vec::new(),
}
impl<I> Default for ValEnabled<I> {
    fn default() -> Self {
        Self { .. }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde")]
mod custom_serde {
    use serde::de::{Deserialize, Deserializer, Error, IgnoredAny, IntoDeserializer, MapAccess, Visitor};

    use super::*;
    use crate::DefOptionExt;

    impl<'de, I> Deserialize<'de> for ValOptions<I>
    where
        I: Deserialize<'de>,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct VisitorImpl<I>(std::marker::PhantomData<I>);

            impl<'de, I> Visitor<'de> for VisitorImpl<I>
            where
                I: Deserialize<'de>,
            {
                type Value = ValOptions<I>;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("struct ValOptions")
                }

                fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
                where
                    A: MapAccess<'de>,
                {
                    let mut options = ValOptions::default();
                    while let Some(key) = map.next_key()? {
                        match key {
                            FieldKind::Default => {
                                options.default = map.next_value()?;
                            }
                            FieldKind::Option(kind) => {
                                // Deserialize as default-capable option, but discard default
                                // variant right away - we do not need to store it
                                if let Some(option) = map.next_value::<DefOptionExt<_>>()?.into_option_ext() {
                                    options.overrides.push((kind, option));
                                }
                            }
                            // Skip entries with unrecognizable field names
                            FieldKind::Unknown => {
                                map.next_value::<IgnoredAny>()?;
                            }
                        }
                    }
                    Ok(options)
                }
            }

            deserializer.deserialize_map(VisitorImpl(std::marker::PhantomData))
        }
    }

    enum FieldKind {
        Default,
        Option(ValKind),
        Unknown,
    }
    impl<'de> Deserialize<'de> for FieldKind {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct VisitorImpl;

            impl<'de> Visitor<'de> for VisitorImpl {
                type Value = FieldKind;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("validation option name")
                }

                fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: Error,
                {
                    if v == "default" {
                        return Ok(FieldKind::Default);
                    }
                    let kind_deserializer = IntoDeserializer::<E>::into_deserializer(v);
                    Ok(match ValKind::deserialize(kind_deserializer) {
                        Ok(kind) => FieldKind::Option(kind),
                        Err(_) => FieldKind::Unknown,
                    })
                }
            }

            deserializer.deserialize_identifier(VisitorImpl)
        }
    }
}
