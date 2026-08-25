use crate::{
    ChangedItemIdsResp, CmdResps, Coordinates, EffectId, EffectMode, FitId, FitIdBr, ItemId, ItemIdBr, ItemTypeId,
    Movement, ctl::core::shared::EffectModes, err::BrResolveError,
};

// Core commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct ShipChangeCmd {
    type_id: Option<ItemTypeId>,
    state: Option<bool>,
    coordinates: Option<Coordinates>,
    movement: Option<Movement>,
    #[cfg_attr(feature = "serde", serde(default))]
    effect_modes: EffectModes,
}

// Extra context commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize), serde(untagged))]
#[derive(Clone)]
pub enum ShipChangeCmdCtxAny {
    Fit(ShipChangeCmdCtxFit),
    Item(ShipChangeCmdCtxItem),
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize), serde(untagged))]
#[derive(Clone)]
pub enum ShipChangeCmdCtxAnyBr {
    Fit(ShipChangeCmdCtxFitBr),
    Item(ShipChangeCmdCtxItemBr),
}

// Extra context commands - fit
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct ShipChangeCmdCtxFit {
    fit_id: FitId,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: ShipChangeCmd,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct ShipChangeCmdCtxFitBr {
    fit_id: FitIdBr,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: ShipChangeCmd,
}

// Extra context commands - item
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct ShipChangeCmdCtxItem {
    item_id: ItemId,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: ShipChangeCmd,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct ShipChangeCmdCtxItemBr {
    item_id: ItemIdBr,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: ShipChangeCmd,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ShipChangeCmd {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_type_id(mut self, type_id: ItemTypeId) -> Self {
        self.type_id = Some(type_id);
        self
    }
    pub fn with_state(mut self, state: bool) -> Self {
        self.state = Some(state);
        self
    }
    pub fn with_coordinates(mut self, coordinates: Coordinates) -> Self {
        self.coordinates = Some(coordinates);
        self
    }
    pub fn with_movement(mut self, movement: Movement) -> Self {
        self.movement = Some(movement);
        self
    }
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (EffectId, EffectMode)>) -> Self {
        self.effect_modes.extend(effect_modes);
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ShipChangeCmd {
    pub(in crate::ctl) fn into_ctx_via_fit(self, fit_id: FitId) -> ShipChangeCmdCtxAny {
        ShipChangeCmdCtxAny::Fit(ShipChangeCmdCtxFit { fit_id, core: self })
    }
    pub(in crate::ctl) fn into_ctx_via_item(self, item_id: ItemId) -> ShipChangeCmdCtxAny {
        ShipChangeCmdCtxAny::Item(ShipChangeCmdCtxItem { item_id, core: self })
    }
    pub(in crate::ctl) fn into_ctx_br_via_fit(self, fit_id: impl Into<FitIdBr>) -> ShipChangeCmdCtxAnyBr {
        ShipChangeCmdCtxAnyBr::Fit(ShipChangeCmdCtxFitBr {
            fit_id: fit_id.into(),
            core: self,
        })
    }
    pub(in crate::ctl) fn into_ctx_br_via_item(self, item_id: impl Into<ItemIdBr>) -> ShipChangeCmdCtxAnyBr {
        ShipChangeCmdCtxAnyBr::Item(ShipChangeCmdCtxItemBr {
            item_id: item_id.into(),
            core: self,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ShipChangeCmdCtxAnyBr {
    pub(in crate::ctl) fn br_resolve(self, resps: &CmdResps) -> Result<ShipChangeCmdCtxAny, BrResolveError> {
        Ok(match self {
            Self::Fit(cmd) => ShipChangeCmdCtxAny::Fit(cmd.br_resolve(resps)?),
            Self::Item(cmd) => ShipChangeCmdCtxAny::Item(cmd.br_resolve(resps)?),
        })
    }
}

impl ShipChangeCmdCtxFitBr {
    fn br_resolve(self, resps: &CmdResps) -> Result<ShipChangeCmdCtxFit, BrResolveError> {
        Ok(ShipChangeCmdCtxFit {
            fit_id: resps.resolve_fit_id(self.fit_id)?,
            core: self.core,
        })
    }
}

impl ShipChangeCmdCtxItemBr {
    fn br_resolve(self, resps: &CmdResps) -> Result<ShipChangeCmdCtxItem, BrResolveError> {
        Ok(ShipChangeCmdCtxItem {
            item_id: resps.resolve_item_id(self.item_id)?,
            core: self.core,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ShipChangeCmd {
    pub(in crate::ctl) fn execute_via_fit(
        self,
        core_fit: &mut rc::FitMut,
    ) -> Result<ChangedItemIdsResp, FitShipChangeError> {
        let mut core_ship = match core_fit.get_ship_mut() {
            Some(core_ship) => core_ship,
            None => return Err(FitShipChangeError::FitNoShip(core_fit.get_fit_id())),
        };
        Ok(self.execute(&mut core_ship))
    }
    pub(in crate::ctl) fn execute_via_item(
        self,
        core_item: &mut rc::ItemMut,
    ) -> Result<ChangedItemIdsResp, ItemShipChangeError> {
        let core_ship = core_item.dc_ship()?;
        Ok(self.execute(core_ship))
    }
    fn execute(self, core_ship: &mut rc::ShipMut) -> ChangedItemIdsResp {
        if let Some(type_id) = self.type_id {
            core_ship.set_type_id(type_id);
        }
        if let Some(state) = self.state {
            core_ship.set_state(state);
        }
        if let Some(coordinates) = self.coordinates {
            core_ship.set_coordinates(coordinates);
        }
        if let Some(movement) = self.movement {
            core_ship.set_movement(movement);
        }
        self.effect_modes.apply(core_ship);
        ChangedItemIdsResp::default()
    }
}
#[derive(thiserror::Error, Debug)]
pub enum FitShipChangeError {
    #[error("fit {0} has no ship set")]
    FitNoShip(FitId),
}
#[derive(thiserror::Error, Debug)]
pub enum ItemShipChangeError {
    #[error(transparent)]
    ItemIsNotShip(#[from] rc::err::ItemKindMatchError),
}

impl ShipChangeCmdCtxAny {
    pub(in crate::ctl) fn execute(self, core_sol: &mut rc::SolarSystem) -> Result<ChangedItemIdsResp, ShipChangeError> {
        Ok(match self {
            Self::Fit(cmd) => cmd.execute(core_sol)?,
            Self::Item(cmd) => cmd.execute(core_sol)?,
        })
    }
}
#[derive(thiserror::Error, Debug)]
pub enum ShipChangeError {
    #[error(transparent)]
    ViaFit(#[from] FitGetShipChangeError),
    #[error(transparent)]
    ViaItem(#[from] ItemGetShipChangeError),
}

impl ShipChangeCmdCtxFit {
    pub(in crate::ctl) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<ChangedItemIdsResp, FitGetShipChangeError> {
        let mut core_fit = core_sol.get_fit_mut(&self.fit_id)?;
        Ok(self.core.execute_via_fit(&mut core_fit)?)
    }
}
#[derive(thiserror::Error, Debug)]
pub enum FitGetShipChangeError {
    #[error(transparent)]
    FitGet(#[from] rc::err::FitGetError),
    #[error("fit {0} has no ship set")]
    FitNoShip(FitId),
}
impl From<FitShipChangeError> for FitGetShipChangeError {
    fn from(err: FitShipChangeError) -> Self {
        match err {
            FitShipChangeError::FitNoShip(inner) => Self::FitNoShip(inner),
        }
    }
}

impl ShipChangeCmdCtxItem {
    pub(in crate::ctl) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<ChangedItemIdsResp, ItemGetShipChangeError> {
        let mut core_ship = core_sol.get_ship_mut(&self.item_id)?;
        Ok(self.core.execute(&mut core_ship))
    }
}
#[derive(thiserror::Error, Debug)]
pub enum ItemGetShipChangeError {
    #[error(transparent)]
    ItemGet(#[from] rc::err::ShipGetError),
}
