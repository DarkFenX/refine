use crate::{
    consts::{ModRack, OrdAddMode, State},
    defs::{ReeId, ReeIdx, ReeInt},
    ss::SolarSystem,
    ssi, ssn,
    util::{Error, ErrorKind, Named, Result},
};

impl SolarSystem {
    // Public
    pub fn get_module_info(&self, item_id: &ReeId) -> Result<ssn::SsModuleInfo> {
        Ok(self.make_mod_info(self.get_module(item_id)?))
    }
    pub fn get_module_infos(&self, fit_id: &ReeId, rack: ModRack) -> Vec<ssn::SsModuleInfo> {
        self.items
            .values()
            .filter_map(|v| match v {
                ssi::SsItem::Module(m) if m.fit_id == *fit_id && m.rack == rack => Some(self.make_mod_info(m)),
                _ => None,
            })
            .collect()
    }
    pub fn add_module(
        &mut self,
        fit_id: ReeId,
        rack: ModRack,
        pos_mode: OrdAddMode,
        a_item_id: ReeInt,
        state: State,
        charge_a_item_id: Option<ReeInt>,
    ) -> Result<ssn::SsModuleInfo> {
        // Allocate resources first
        let m_item_id = self.alloc_item_id()?;
        let c_item_id = match charge_a_item_id {
            Some(_) => Some(self.alloc_item_id()?),
            None => None,
        };
        // Calculate position for the module and make necessary changes to positions of other modules
        let infos = self.get_module_infos(&fit_id, rack);
        let pos = match pos_mode {
            OrdAddMode::Append => infos.iter().map(|v| v.pos).max().map(|v| 1 + v).unwrap_or(0),
            OrdAddMode::Equip => {
                let positions = infos.iter().map(|v| v.pos).collect();
                find_equip_pos(positions)
            }
            OrdAddMode::Insert(pos) => {
                for info in infos.iter() {
                    match self.items.get_mut(&info.id) {
                        Some(ssi::SsItem::Module(m)) if m.rack == rack && m.pos >= pos => m.pos += 1,
                        _ => (),
                    }
                }
                pos
            }
            OrdAddMode::Place(pos, repl) => {
                let mut old_item_id = None;
                for info in infos.iter() {
                    match self.items.get(&info.id) {
                        Some(ssi::SsItem::Module(m)) if m.rack == rack && m.pos == pos => {
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
        let module = ssi::SsModule::new(&self.src, m_item_id, fit_id, a_item_id, state, rack, pos, c_item_id);
        let m_info = ssn::SsModuleInfo::from_mod_and_charge(&module, c_info);
        let m_item = ssi::SsItem::Module(module);
        self.add_item(m_item);
        Ok(m_info)
    }
    pub fn set_module_state(&mut self, item_id: &ReeId, state: State) -> Result<()> {
        self.get_module_mut(item_id)?.state = state;
        Ok(())
    }
    pub fn set_module_charge(&mut self, item_id: &ReeId, charge_a_item_id: ReeInt) -> Result<ssn::SsChargeInfo> {
        let c_item_id = self.alloc_item_id()?;
        self.remove_module_charge(item_id)?;
        let module = self.get_module(item_id)?;
        let c_info = self.add_charge_with_id(c_item_id, module.fit_id, charge_a_item_id, module.id);
        let module = self.get_module_mut(item_id)?;
        module.charge_a_item_id = Some(c_item_id);
        Ok(c_info)
    }
    pub fn remove_module_charge(&mut self, item_id: &ReeId) -> Result<bool> {
        let module = self.get_module_mut(item_id)?;
        match module.charge_a_item_id {
            Some(cid) => {
                module.charge_a_item_id = None;
                Ok(self.items.remove(&cid).is_some())
            }
            None => Ok(false),
        }
    }
    // Non-public
    fn get_module(&self, item_id: &ReeId) -> Result<&ssi::SsModule> {
        let item = self.get_item(item_id)?;
        match item {
            ssi::SsItem::Module(module) => Ok(module),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                ssi::SsModule::get_name(),
            ))),
        }
    }
    fn get_module_mut(&mut self, item_id: &ReeId) -> Result<&mut ssi::SsModule> {
        let item = self.get_item_mut(item_id)?;
        match item {
            ssi::SsItem::Module(module) => Ok(module),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                ssi::SsModule::get_name(),
            ))),
        }
    }
    pub(crate) fn make_mod_info(&self, module: &ssi::SsModule) -> ssn::SsModuleInfo {
        let charge_info = match module.charge_a_item_id {
            Some(cid) => match self.get_charge_info(&cid) {
                Ok(ci) => Some(ci),
                _ => None,
            },
            None => None,
        };
        ssn::SsModuleInfo::from_mod_and_charge(module, charge_info)
    }
}

// Find first slot not taken by any module
fn find_equip_pos(mut positions: Vec<ReeIdx>) -> ReeIdx {
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
