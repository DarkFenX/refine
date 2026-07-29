use crate::{
    ItemId, PValue,
    misc::{AttrSpec, EffectSpec},
    rd::REffectResist,
    svc::{Calc, SvcCtx, funcs::is_oattr_flag_set, vast::VastFitData},
    ud::UItemId,
    util::{RMap, RMapRSet, RSet},
};

#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(transparent))]
#[derive(Clone)]
pub struct ValProjImmunityFail {
    /// Projecting items and targets they can't be projected to.
    #[cfg_attr(feature = "serde", serde(serialize_with = "custom_serde::as_map"))]
    pub items: Vec<ValProjImmunityItemInfo>,
}

#[derive(Clone)]
pub struct ValProjImmunityItemInfo {
    /// Item-projector which fails the validation.
    pub item_id: ItemId,
    /// Projectee item IDs the projector can't be projected to.
    pub projectee_item_ids: Vec<ItemId>,
}

impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast) fn validate_assist_immunity_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
    ) -> bool {
        validate_fast(kfs, ctx, calc, &self.blockable_assistance, is_assist_blocked)
    }
    pub(in crate::svc::vast) fn validate_offense_immunity_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
    ) -> bool {
        validate_fast(kfs, ctx, calc, &self.blockable_offense, is_offense_blocked)
    }
    pub(in crate::svc::vast) fn validate_resist_immunity_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
    ) -> bool {
        validate_fast(kfs, ctx, calc, &self.resist_immunity, is_resist_blocked)
    }
    // Verbose validations
    pub(in crate::svc::vast) fn validate_assist_immunity_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
    ) -> Option<ValProjImmunityFail> {
        validate_verbose(kfs, ctx, calc, &self.blockable_assistance, is_assist_blocked)
    }
    pub(in crate::svc::vast) fn validate_offense_immunity_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
    ) -> Option<ValProjImmunityFail> {
        validate_verbose(kfs, ctx, calc, &self.blockable_offense, is_offense_blocked)
    }
    pub(in crate::svc::vast) fn validate_resist_immunity_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
    ) -> Option<ValProjImmunityFail> {
        validate_verbose(kfs, ctx, calc, &self.resist_immunity, is_resist_blocked)
    }
}

fn validate_fast<F, P>(
    kfs: &RSet<UItemId>,
    ctx: SvcCtx,
    calc: &mut Calc,
    blockable: &RMapRSet<P, EffectSpec>,
    is_blocked: F,
) -> bool
where
    P: Copy + Eq + std::hash::Hash,
    F: Fn(SvcCtx, &mut Calc, P) -> bool,
{
    for (&projectee_data, mut projector_especs) in blockable.iter() {
        if is_blocked(ctx, calc, projectee_data) {
            match kfs.is_empty() {
                true => return false,
                false => {
                    if !projector_especs.all(|v| kfs.contains(&v.item_uid)) {
                        return false;
                    }
                }
            }
        }
    }
    true
}

fn validate_verbose<F, P>(
    kfs: &RSet<UItemId>,
    ctx: SvcCtx,
    calc: &mut Calc,
    blockable: &RMapRSet<P, EffectSpec>,
    is_blocked: F,
) -> Option<ValProjImmunityFail>
where
    P: Copy + Eq + std::hash::Hash + GetItemUid,
    F: Fn(SvcCtx, &mut Calc, P) -> bool,
{
    let mut items = RMap::new();
    for (&projectee_data, projector_especs) in blockable.iter() {
        if is_blocked(ctx, calc, projectee_data) {
            let projectee_item_id = ctx.u_data.items.ext_id_by_int_id(projectee_data.get_item_uid());
            for projector_espec in projector_especs {
                if kfs.contains(&projector_espec.item_uid) {
                    continue;
                }
                let projector_item_id = ctx.u_data.items.ext_id_by_int_id(projector_espec.item_uid);
                items
                    .entry(projector_item_id)
                    .or_insert_with(RSet::new)
                    .insert(projectee_item_id);
            }
        }
    }
    match items.is_empty() {
        true => None,
        false => Some(ValProjImmunityFail {
            items: items
                .into_iter()
                .map(|(projector_item_id, projectee_item_ids)| ValProjImmunityItemInfo {
                    item_id: projector_item_id,
                    projectee_item_ids: projectee_item_ids.into_iter().collect(),
                })
                .collect(),
        }),
    }
}

fn is_assist_blocked(ctx: SvcCtx, calc: &mut Calc, projectee_uid: UItemId) -> bool {
    is_oattr_flag_set(ctx, calc, projectee_uid, ctx.ac().disallow_assistance).unwrap_or(false)
}

fn is_offense_blocked(ctx: SvcCtx, calc: &mut Calc, projectee_uid: UItemId) -> bool {
    is_oattr_flag_set(ctx, calc, projectee_uid, ctx.ac().disallow_offensive_modifiers).unwrap_or(false)
}

fn is_resist_blocked(ctx: SvcCtx, calc: &mut Calc, projectee_aspec: AttrSpec) -> bool {
    REffectResist::get_mult_by_aspec(ctx, calc, &projectee_aspec) == Some(PValue::ZERO)
}

trait GetItemUid {
    fn get_item_uid(&self) -> UItemId;
}
impl GetItemUid for UItemId {
    fn get_item_uid(&self) -> UItemId {
        *self
    }
}
impl GetItemUid for AttrSpec {
    fn get_item_uid(&self) -> UItemId {
        self.item_uid
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde")]
mod custom_serde {
    use serde::ser::{SerializeMap, Serializer};

    use super::*;

    pub(super) fn as_map<S>(items: &[ValProjImmunityItemInfo], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(items.len()))?;
        for item in items {
            map.serialize_entry(&item.item_id, &item.projectee_item_ids)?;
        }
        map.end()
    }
}
