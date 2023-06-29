use crate::{
    consts::{ModRack, OrdAddMode, State},
    defs::{EItemId, Idx, SsFitId, SsItemId},
    ss::{
        info::{SsChargeInfo, SsModuleInfo},
        item::{SsItem, SsModule},
        SolarSystem, SsView,
    },
    util::{Error, ErrorKind, Result},
};

impl SolarSystem {
    // Public
    pub fn get_module_info(&self, item_id: &SsItemId) -> Result<SsModuleInfo> {
        Ok(self.make_mod_info(self.items.get_module(item_id)?))
    }
    pub fn get_module_infos(&self, fit_id: &SsFitId, rack: ModRack) -> Result<Vec<SsModuleInfo>> {
        let fit = self.fits.get_fit(fit_id)?;
        let module_ids = match rack {
            ModRack::High => &fit.mods_high,
            ModRack::Mid => &fit.mods_mid,
            ModRack::Low => &fit.mods_low,
        };
        let module_infos = module_ids
            .iter()
            .map(|v| self.make_mod_info(self.items.get_module(v).unwrap()))
            .collect();
        Ok(module_infos)
    }
    pub fn add_module(
        &mut self,
        fit_id: SsFitId,
        rack: ModRack,
        pos_mode: OrdAddMode,
        a_item_id: EItemId,
        state: State,
        charge_a_item_id: Option<EItemId>,
    ) -> Result<SsModuleInfo> {
        // Allocate resources early, to make sure if we fail we don't need to roll anything back
        let m_item_id = self.items.alloc_item_id()?;
        let c_item_id = match charge_a_item_id {
            Some(_) => Some(self.items.alloc_item_id()?),
            None => None,
        };
        // Calculate position for the module and make necessary changes to positions of other modules
        let infos = self.get_module_infos(&fit_id, rack)?;
        let pos = match pos_mode {
            OrdAddMode::Append => infos.iter().map(|v| v.pos).max().map(|v| 1 + v).unwrap_or(0),
            OrdAddMode::Equip => {
                let positions = infos.iter().map(|v| v.pos).collect();
                find_equip_pos(positions)
            }
            OrdAddMode::Insert(pos) => {
                for info in infos.iter() {
                    match self.items.get_item_mut(&info.id) {
                        Ok(SsItem::Module(m)) if m.rack == rack && m.pos >= pos => m.pos += 1,
                        _ => (),
                    }
                }
                pos
            }
            OrdAddMode::Place(pos, repl) => {
                let mut old_item_id = None;
                for info in infos.iter() {
                    match self.items.get_item(&info.id) {
                        Ok(SsItem::Module(m)) if m.rack == rack && m.pos == pos => {
                            old_item_id = Some(info.id);
                            break;
                        }
                        _ => (),
                    }
                }
                match (old_item_id, repl) {
                    (Some(oid), true) => {
                        self.remove_item(&oid);
                        ()
                    }
                    (Some(oid), false) => return Err(Error::new(ErrorKind::ModuleSlotTaken(rack, pos, oid))),
                    _ => (),
                }
                pos
            }
        };
        // Create and register all necessary items
        let c_info = self.add_charge_with_id_opt(c_item_id, fit_id, charge_a_item_id, m_item_id);
        let module = SsModule::new(&self.src, m_item_id, fit_id, a_item_id, state, rack, pos, c_item_id);
        let m_info = SsModuleInfo::from_mod_and_charge(&module, c_info);
        let m_item = SsItem::Module(module);
        self.add_item(m_item);
        Ok(m_info)
    }
    pub fn set_module_state(&mut self, item_id: &SsItemId, state: State) -> Result<()> {
        let module = self.items.get_module_mut(item_id)?;
        let old_state = module.state;
        module.state = state;
        if state != old_state {
            let item = self.items.get_item(item_id).unwrap();
            self.svcs
                .switch_state(&SsView::new(&self.src, &self.fits, &self.items), item, old_state, state);
        };
        Ok(())
    }
    pub fn set_module_charge(&mut self, item_id: &SsItemId, charge_a_item_id: EItemId) -> Result<SsChargeInfo> {
        let c_item_id = self.items.alloc_item_id()?;
        self.remove_module_charge(item_id)?;
        let module = self.items.get_module(item_id)?;
        let c_info = self.add_charge_with_id(c_item_id, module.fit_id, charge_a_item_id, module.id);
        let module = self.items.get_module_mut(item_id)?;
        module.charge_a_item_id = Some(c_item_id);
        Ok(c_info)
    }
    pub fn remove_module_charge(&mut self, item_id: &SsItemId) -> Result<bool> {
        let module = self.items.get_module_mut(item_id)?;
        match module.charge_a_item_id {
            Some(cid) => {
                module.charge_a_item_id = None;
                Ok(self.items.remove_item(&cid).is_some())
            }
            None => Ok(false),
        }
    }
    // Non-public
    pub(in crate::ss) fn make_mod_info(&self, module: &SsModule) -> SsModuleInfo {
        let charge_info = match module.charge_a_item_id {
            Some(cid) => match self.get_charge_info(&cid) {
                Ok(ci) => Some(ci),
                _ => None,
            },
            None => None,
        };
        SsModuleInfo::from_mod_and_charge(module, charge_info)
    }
}

// Find first slot not taken by any module
fn find_equip_pos(mut positions: Vec<Idx>) -> Idx {
    for i in 0..positions.len() {
        while (positions[i] < positions.len()) && (positions[i] != i) {
            let j = positions[i];
            if positions[j] == positions[i] {
                break;
            }
            positions.swap(i, j);
        }
    }
    for i in 0..positions.len() {
        if i != positions[i] {
            return i;
        }
    }
    positions.len()
}
