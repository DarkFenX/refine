use crate::{
    ItemId, UnitInterval,
    misc::{AttrSpec, EffectSpec},
    rd::REffectResist,
    svc::{Calc, SvcCtx, Vast, funcs::is_oattr_flag_set, vast::VastFitData},
    ud::{UItem, UItemId},
    util::{RMap, RMapRSet, RSet},
};

#[cfg_attr(
    feature = "serde",
    cfg_eval,
    serde_with::serde_as,
    derive(serde::Serialize),
    serde(transparent)
)]
#[derive(Clone)]
pub struct ValProjImmunityFail {
    /// Projecting items and targets they can't be projected to.
    #[cfg_attr(feature = "serde", serde_as(as = "refine_serde::VecAsMap"))]
    pub items: Vec<ValProjImmunityItemInfo>,
}

#[cfg_attr(feature = "serde", derive(refine_serde::VecAsMapEntry))]
#[derive(Clone)]
pub struct ValProjImmunityItemInfo {
    /// Item-projector which fails the validation.
    #[cfg_attr(feature = "serde", vec_map(key))]
    pub item_id: ItemId,
    /// Projectee item IDs the projector can't be projected to.
    #[cfg_attr(feature = "serde", vec_map(value))]
    pub projectee_item_ids: Vec<ItemId>,
}

impl Vast {
    // Fast validations
    pub(in crate::svc::vast::val) fn validate_assist_immunity_fast(
        &self,
        fit_data: &VastFitData,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
    ) -> bool {
        validate_fast(kfs, ctx, calc, &fit_data.blockable_assistance, is_assist_blocked, self)
    }
    pub(in crate::svc::vast::val) fn validate_offense_immunity_fast(
        &self,
        fit_data: &VastFitData,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
    ) -> bool {
        validate_fast(kfs, ctx, calc, &fit_data.blockable_offense, is_offense_blocked, self)
    }
    pub(in crate::svc::vast::val) fn validate_resist_immunity_fast(
        &self,
        fit_data: &VastFitData,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
    ) -> bool {
        validate_fast(kfs, ctx, calc, &fit_data.resist_immunity, is_resist_blocked, self)
    }
    // Verbose validations
    pub(in crate::svc::vast::val) fn validate_assist_immunity_verbose(
        &self,
        fit_data: &VastFitData,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
    ) -> Option<ValProjImmunityFail> {
        validate_verbose(kfs, ctx, calc, &fit_data.blockable_assistance, is_assist_blocked, self)
    }
    pub(in crate::svc::vast::val) fn validate_offense_immunity_verbose(
        &self,
        fit_data: &VastFitData,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
    ) -> Option<ValProjImmunityFail> {
        validate_verbose(kfs, ctx, calc, &fit_data.blockable_offense, is_offense_blocked, self)
    }
    pub(in crate::svc::vast::val) fn validate_resist_immunity_verbose(
        &self,
        fit_data: &VastFitData,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
    ) -> Option<ValProjImmunityFail> {
        validate_verbose(kfs, ctx, calc, &fit_data.resist_immunity, is_resist_blocked, self)
    }
}

fn validate_fast<F, P>(
    kfs: &RSet<UItemId>,
    ctx: SvcCtx,
    calc: &mut Calc,
    blockable: &RMapRSet<P, EffectSpec>,
    is_blocked: F,
    vast: &Vast,
) -> bool
where
    P: Copy + Eq + std::hash::Hash,
    F: Fn(SvcCtx, &mut Calc, P, &Vast) -> bool,
{
    for (&projectee_data, mut projector_especs) in blockable.iter() {
        if is_blocked(ctx, calc, projectee_data, vast) {
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
    vast: &Vast,
) -> Option<ValProjImmunityFail>
where
    P: Copy + Eq + std::hash::Hash + GetItemUid,
    F: Fn(SvcCtx, &mut Calc, P, &Vast) -> bool,
{
    let mut items = RMap::new();
    for (&projectee_data, projector_especs) in blockable.iter() {
        if is_blocked(ctx, calc, projectee_data, vast) {
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

fn is_assist_blocked(ctx: SvcCtx, calc: &mut Calc, projectee_uid: UItemId, vast: &Vast) -> bool {
    if is_oattr_flag_set(ctx, calc, projectee_uid, ctx.ac().disallow_assistance).unwrap_or(false) {
        return true;
    };
    let UItem::Ship(ship) = ctx.u_data.items.get(projectee_uid) else {
        return false;
    };
    let projectee_fit_data = vast.get_fit_data(ship.get_fit_uid());
    !projectee_fit_data.mods_active_block_in_assist.is_empty()
}

fn is_offense_blocked(ctx: SvcCtx, calc: &mut Calc, projectee_uid: UItemId, _vast: &Vast) -> bool {
    is_oattr_flag_set(ctx, calc, projectee_uid, ctx.ac().disallow_offensive_modifiers).unwrap_or(false)
}

fn is_resist_blocked(ctx: SvcCtx, calc: &mut Calc, projectee_aspec: AttrSpec, _vast: &Vast) -> bool {
    REffectResist::get_mult_by_aspec(ctx, calc, &projectee_aspec) == Some(UnitInterval::ZERO)
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
