use crate::{
    defs::{EItemId, SolFitId},
    err::basic::{FitFoundError, ItemAllocError, OrderedSlotError},
    sol::{
        item::{SolItem, SolItemState, SolModule},
        item_info::SolModuleInfo,
        sole_item::misc::find_equip_pos,
        SolModRack, SolOrdAddMode, SolarSystem,
    },
};

impl SolarSystem {
    pub fn add_module(
        &mut self,
        fit_id: SolFitId,
        rack: SolModRack,
        pos_mode: SolOrdAddMode,
        a_item_id: EItemId,
        state: SolItemState,
        charge_a_item_id: Option<EItemId>,
    ) -> Result<SolModuleInfo, AddModuleError> {
        // Allocate resources early, to make sure if we fail we don't need to roll anything back
        // TODO: redo so that it does not allocate the same ID when there is only 1 ID left
        let module_item_id = self.items.alloc_item_id()?;
        let charge_item_id = match charge_a_item_id {
            Some(_) => Some(self.items.alloc_item_id()?),
            None => None,
        };
        // Calculate position for the module and make necessary changes to positions of other modules
        let fit = self.fits.get_fit(&fit_id)?;
        let infos = self.int_get_fit_module_infos(fit, rack);
        let pos = match pos_mode {
            SolOrdAddMode::Append => infos.iter().map(|v| v.pos).max().map(|v| 1 + v).unwrap_or(0),
            SolOrdAddMode::Equip => {
                let positions = infos.iter().map(|v| v.pos).collect();
                find_equip_pos(positions)
            }
            SolOrdAddMode::Insert(pos) => {
                for info in infos.iter() {
                    match self.items.get_item_mut(&info.id) {
                        Ok(SolItem::Module(m)) if m.rack == rack && m.pos >= pos => m.pos += 1,
                        _ => (),
                    }
                }
                pos
            }
            SolOrdAddMode::Place(pos, repl) => {
                let mut old_item_id = None;
                for info in infos.iter() {
                    match self.items.get_item(&info.id) {
                        Ok(SolItem::Module(m)) if m.rack == rack && m.pos == pos => {
                            old_item_id = Some(info.id);
                            break;
                        }
                        _ => (),
                    }
                }
                match (old_item_id, repl) {
                    (Some(oid), true) => {
                        self.remove_module(&oid).unwrap();
                        ()
                    }
                    (Some(oid), false) => return Err(OrderedSlotError::new(rack, pos, oid).into()),
                    _ => (),
                }
                pos
            }
        };
        // Create and register all necessary items
        let charge_info = self.add_charge_with_id_opt(charge_item_id, fit_id, charge_a_item_id, module_item_id);
        let module = SolModule::new(
            &self.src,
            module_item_id,
            fit_id,
            a_item_id,
            state,
            rack,
            pos,
            charge_item_id,
        );
        let module_info = SolModuleInfo::from_mod_and_charge(&module, charge_info);
        let module_item = SolItem::Module(module);
        let fit = self.fits.get_fit_mut(&fit_id).unwrap();
        match rack {
            SolModRack::High => fit.mods_high.insert(module_item_id),
            SolModRack::Mid => fit.mods_mid.insert(module_item_id),
            SolModRack::Low => fit.mods_low.insert(module_item_id),
        };
        self.items.add_item(module_item);
        self.add_item_id_to_svcs(&module_item_id);
        Ok(module_info)
    }
}

#[derive(Debug)]
pub enum AddModuleError {
    FitNotFound(FitFoundError),
    ItemIdAllocFailed(ItemAllocError),
    SlotTaken(OrderedSlotError),
}
impl std::error::Error for AddModuleError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
            Self::ItemIdAllocFailed(e) => Some(e),
            Self::SlotTaken(e) => Some(e),
        }
    }
}
impl std::fmt::Display for AddModuleError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
            Self::ItemIdAllocFailed(e) => e.fmt(f),
            Self::SlotTaken(e) => e.fmt(f),
        }
    }
}
impl From<FitFoundError> for AddModuleError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
impl From<ItemAllocError> for AddModuleError {
    fn from(error: ItemAllocError) -> Self {
        Self::ItemIdAllocFailed(error)
    }
}
impl From<OrderedSlotError> for AddModuleError {
    fn from(error: OrderedSlotError) -> Self {
        Self::SlotTaken(error)
    }
}
