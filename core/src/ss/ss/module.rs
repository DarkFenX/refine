use crate::{
    consts::{OrdAddMode, State},
    ss::item::{Charge, Item, Module},
    Error, ErrorKind, ReeId, ReeIdx, ReeInt, Result, SolarSystem,
};

impl SolarSystem {
    pub fn get_modules_high(&self, fit_id: ReeId) -> Vec<ReeId> {
        self.items
            .values()
            .filter_map(|v| match v {
                Item::ModuleHigh(m) if m.fit_id == fit_id => Some(m.item_id),
                _ => None,
            })
            .collect()
    }
    pub fn get_modules_mid(&self, fit_id: ReeId) -> Vec<ReeId> {
        self.items
            .values()
            .filter_map(|v| match v {
                Item::ModuleMid(m) if m.fit_id == fit_id => Some(m.item_id),
                _ => None,
            })
            .collect()
    }
    pub fn get_modules_low(&self, fit_id: ReeId) -> Vec<ReeId> {
        self.items
            .values()
            .filter_map(|v| match v {
                Item::ModuleLow(m) if m.fit_id == fit_id => Some(m.item_id),
                _ => None,
            })
            .collect()
    }
    pub fn add_module_high(
        &mut self,
        fit_id: ReeId,
        type_id: ReeInt,
        state: State,
        add_mode: OrdAddMode,
        charge_type_id: Option<ReeInt>,
    ) -> Result<(ReeId, Option<ReeId>)> {
        let item_ids = self.get_modules_high(fit_id);
        let pos = match add_mode {
            OrdAddMode::Append => self.get_positions(&item_ids).iter().max().map(|v| 1 + v).unwrap_or(0),
            OrdAddMode::Equip => {
                let positions = self.get_positions(&item_ids);
                first_free_pos(positions)
            }
            OrdAddMode::Insert(pos) => {
                for item_id in item_ids.iter() {
                    match self.items.get_mut(item_id) {
                        Some(Item::ModuleHigh(m)) if m.pos >= pos => m.pos += 1,
                        _ => (),
                    }
                }
                pos
            }
            OrdAddMode::Place(pos, repl) => {
                let mut old_item_id = None;
                for item_id in item_ids.iter() {
                    match self.items.get(item_id) {
                        Some(Item::ModuleHigh(m)) if m.pos == pos => old_item_id = Some(item_id),
                        _ => (),
                    }
                }
                match (old_item_id, repl) {
                    (Some(oid), true) => {
                        self.remove_item(oid);
                        ()
                    }
                    (Some(oid), false) => {
                        return Err(Error::new(
                            ErrorKind::SlotTaken,
                            format!("high slot position {} is taken by item ID {}", pos, oid),
                        ))
                    }
                    _ => (),
                }
                pos
            }
        };
        let module_id = self.alloc_item_id()?;
        let charge_id = self.add_charge(fit_id, module_id, charge_type_id)?;
        let module = Item::ModuleHigh(Module::new(
            &self.src, module_id, fit_id, type_id, state, pos, charge_id,
        ));
        self.add_item(module);
        Ok((module_id, charge_id))
    }
    pub fn add_module_mid(
        &mut self,
        fit_id: ReeId,
        type_id: ReeInt,
        state: State,
        add_mode: OrdAddMode,
        charge_type_id: Option<ReeInt>,
    ) -> Result<(ReeId, Option<ReeId>)> {
        let item_ids = self.get_modules_mid(fit_id);
        let pos = match add_mode {
            OrdAddMode::Append => self.get_positions(&item_ids).iter().max().map(|v| 1 + v).unwrap_or(0),
            OrdAddMode::Equip => {
                let positions = self.get_positions(&item_ids);
                first_free_pos(positions)
            }
            OrdAddMode::Insert(pos) => {
                for item_id in item_ids.iter() {
                    match self.items.get_mut(item_id) {
                        Some(Item::ModuleMid(m)) if m.pos >= pos => m.pos += 1,
                        _ => (),
                    }
                }
                pos
            }
            OrdAddMode::Place(pos, repl) => {
                let mut old_item_id = None;
                for item_id in item_ids.iter() {
                    match self.items.get(item_id) {
                        Some(Item::ModuleMid(m)) if m.pos == pos => old_item_id = Some(item_id),
                        _ => (),
                    }
                }
                match (old_item_id, repl) {
                    (Some(oid), true) => {
                        self.remove_item(oid);
                        ()
                    }
                    (Some(oid), false) => {
                        return Err(Error::new(
                            ErrorKind::SlotTaken,
                            format!("mid slot position {} is taken by item ID {}", pos, oid),
                        ))
                    }
                    _ => (),
                }
                pos
            }
        };
        let module_id = self.alloc_item_id()?;
        let charge_id = self.add_charge(fit_id, module_id, charge_type_id)?;
        let module = Item::ModuleMid(Module::new(
            &self.src, module_id, fit_id, type_id, state, pos, charge_id,
        ));
        self.add_item(module);
        Ok((module_id, charge_id))
    }
    pub fn add_module_low(
        &mut self,
        fit_id: ReeId,
        type_id: ReeInt,
        state: State,
        add_mode: OrdAddMode,
        charge_type_id: Option<ReeInt>,
    ) -> Result<(ReeId, Option<ReeId>)> {
        let item_ids = self.get_modules_low(fit_id);
        let pos = match add_mode {
            OrdAddMode::Append => self.get_positions(&item_ids).iter().max().map(|v| 1 + v).unwrap_or(0),
            OrdAddMode::Equip => {
                let positions = self.get_positions(&item_ids);
                first_free_pos(positions)
            }
            OrdAddMode::Insert(pos) => {
                for item_id in item_ids.iter() {
                    match self.items.get_mut(item_id) {
                        Some(Item::ModuleLow(m)) if m.pos >= pos => m.pos += 1,
                        _ => (),
                    }
                }
                pos
            }
            OrdAddMode::Place(pos, repl) => {
                let mut old_item_id = None;
                for item_id in item_ids.iter() {
                    match self.items.get(item_id) {
                        Some(Item::ModuleLow(m)) if m.pos == pos => old_item_id = Some(item_id),
                        _ => (),
                    }
                }
                match (old_item_id, repl) {
                    (Some(oid), true) => {
                        self.remove_item(oid);
                        ()
                    }
                    (Some(oid), false) => {
                        return Err(Error::new(
                            ErrorKind::SlotTaken,
                            format!("low slot position {} is taken by item ID {}", pos, oid),
                        ))
                    }
                    _ => (),
                }
                pos
            }
        };
        let module_id = self.alloc_item_id()?;
        let charge_id = self.add_charge(fit_id, module_id, charge_type_id)?;
        let module = Item::ModuleLow(Module::new(
            &self.src, module_id, fit_id, type_id, state, pos, charge_id,
        ));
        self.add_item(module);
        Ok((module_id, charge_id))
    }
    pub fn set_module_state(&mut self, item_id: &ReeId, state: State) -> Result<()> {
        let item = self
            .items
            .get_mut(item_id)
            .ok_or_else(|| Error::new(ErrorKind::ItemNotFound, format!("item with ID {item_id} not found")))?;
        match item {
            Item::ModuleHigh(m) => m.state = state,
            Item::ModuleMid(m) => m.state = state,
            Item::ModuleLow(m) => m.state = state,
            _ => {
                return Err(Error::new(
                    ErrorKind::UnexpectedItemType,
                    format!("expected Module as item with ID {item_id}"),
                ))
            }
        }
        Ok(())
    }
    pub fn set_module_charge(&mut self, item_id: &ReeId, charge_type_id: ReeInt) -> Result<ReeId> {
        self.remove_module_charge(item_id)?;
        let module = self
            .items
            .get(item_id)
            .ok_or_else(|| Error::new(ErrorKind::ItemNotFound, format!("item with ID {item_id} not found")))?;
        let (charge_id, charge) = match module {
            Item::ModuleHigh(m) => {
                let fit_id = m.fit_id;
                let charge_id = self.alloc_item_id()?;
                let charge = Item::Charge(Charge::new(&self.src, charge_id, fit_id, charge_type_id, *item_id));
                (charge_id, charge)
            }
            Item::ModuleMid(m) => {
                let fit_id = m.fit_id;
                let charge_id = self.alloc_item_id()?;
                let charge = Item::Charge(Charge::new(&self.src, charge_id, fit_id, charge_type_id, *item_id));
                (charge_id, charge)
            }
            Item::ModuleLow(m) => {
                let fit_id = m.fit_id;
                let charge_id = self.alloc_item_id()?;
                let charge = Item::Charge(Charge::new(&self.src, charge_id, fit_id, charge_type_id, *item_id));
                (charge_id, charge)
            }
            _ => {
                return Err(Error::new(
                    ErrorKind::UnexpectedItemType,
                    format!("item with ID {item_id} is not a module"),
                ))
            }
        };
        self.add_item(charge);
        let module = self
            .items
            .get_mut(item_id)
            .ok_or_else(|| Error::new(ErrorKind::ItemNotFound, format!("item with ID {item_id} not found")))?;
        match module {
            Item::ModuleHigh(m) => m.charge = Some(charge_id),
            Item::ModuleMid(m) => m.charge = Some(charge_id),
            Item::ModuleLow(m) => m.charge = Some(charge_id),
            _ => {
                return Err(Error::new(
                    ErrorKind::UnexpectedItemType,
                    format!("item with ID {item_id} is not a module"),
                ))
            }
        };
        Ok(charge_id)
    }
    pub fn remove_module_charge(&mut self, item_id: &ReeId) -> Result<bool> {
        let item = self
            .items
            .get_mut(item_id)
            .ok_or_else(|| Error::new(ErrorKind::ItemNotFound, format!("item with ID {item_id} not found")))?;
        let charge_id = match item {
            Item::ModuleHigh(m) => {
                let charge_id = m.charge;
                m.charge = None;
                charge_id
            }
            Item::ModuleMid(m) => {
                let charge_id = m.charge;
                m.charge = None;
                charge_id
            }
            Item::ModuleLow(m) => {
                let charge_id = m.charge;
                m.charge = None;
                charge_id
            }
            _ => {
                return Err(Error::new(
                    ErrorKind::UnexpectedItemType,
                    format!("item with ID {item_id} is not a module"),
                ))
            }
        };
        match charge_id {
            None => Ok(false),
            Some(i) => Ok(self.items.remove(&i).is_some()),
        }
    }
    fn add_charge(&mut self, fit_id: ReeId, module_id: ReeId, charge_type_id: Option<ReeInt>) -> Result<Option<ReeId>> {
        match charge_type_id {
            Some(i) => {
                let charge_id = self.alloc_item_id()?;
                let charge = Item::Charge(Charge::new(&self.src, charge_id, fit_id, i, module_id));
                self.add_item(charge);
                Ok(Some(charge_id))
            }
            None => Ok(None),
        }
    }
}

fn first_free_pos(mut positions: Vec<ReeIdx>) -> ReeIdx {
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
