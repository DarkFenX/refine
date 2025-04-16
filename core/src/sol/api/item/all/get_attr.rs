use crate::{
    ad,
    err::basic::{AttrMetaFoundError, ItemLoadedError},
    sol::{
        AttrId, ItemKey, SolarSystem,
        api::{
            AutochargeMut, BoosterMut, CharacterMut, ChargeMut, DroneMut, FighterMut, FwEffectMut, ImplantMut, ItemMut,
            ModuleMut, ProjEffectMut, RigMut, ServiceMut, ShipMut, SkillMut, StanceMut, SubsystemMut, SwEffectMut,
        },
        svc::calc::{AttrCalcError, CalcAttrVal},
    },
};

impl SolarSystem {
    pub(in crate::sol) fn internal_get_item_attr(
        &mut self,
        item_key: ItemKey,
        a_attr_id: &ad::AAttrId,
    ) -> Result<CalcAttrVal, AttrCalcError> {
        self.svc.calc.get_item_attr_val_full(&self.uad, item_key, a_attr_id)
    }
}

impl<'a> ItemMut<'a> {
    pub fn get_attr(&mut self, attr_id: &AttrId) -> Result<CalcAttrVal, GetItemAttrError> {
        match self {
            ItemMut::Autocharge(autocharge) => autocharge.get_attr(attr_id),
            ItemMut::Booster(booster) => booster.get_attr(attr_id),
            ItemMut::Character(character) => character.get_attr(attr_id),
            ItemMut::Charge(charge) => charge.get_attr(attr_id),
            ItemMut::Drone(drone) => drone.get_attr(attr_id),
            ItemMut::Fighter(fighter) => fighter.get_attr(attr_id),
            ItemMut::FwEffect(fw_effect) => fw_effect.get_attr(attr_id),
            ItemMut::Implant(implant) => implant.get_attr(attr_id),
            ItemMut::Module(module) => module.get_attr(attr_id),
            ItemMut::ProjEffect(proj_effect) => proj_effect.get_attr(attr_id),
            ItemMut::Rig(rig) => rig.get_attr(attr_id),
            ItemMut::Service(service) => service.get_attr(attr_id),
            ItemMut::Ship(ship) => ship.get_attr(attr_id),
            ItemMut::Skill(skill) => skill.get_attr(attr_id),
            ItemMut::Stance(stance) => stance.get_attr(attr_id),
            ItemMut::Subsystem(subsystem) => subsystem.get_attr(attr_id),
            ItemMut::SwEffect(sw_effect) => sw_effect.get_attr(attr_id),
        }
    }
}
impl<'a> AutochargeMut<'a> {
    pub fn get_attr(&mut self, attr_id: &AttrId) -> Result<CalcAttrVal, GetItemAttrError> {
        get_attr(self.sol, self.key, attr_id)
    }
}
impl<'a> BoosterMut<'a> {
    pub fn get_attr(&mut self, attr_id: &AttrId) -> Result<CalcAttrVal, GetItemAttrError> {
        get_attr(self.sol, self.key, attr_id)
    }
}
impl<'a> CharacterMut<'a> {
    pub fn get_attr(&mut self, attr_id: &AttrId) -> Result<CalcAttrVal, GetItemAttrError> {
        get_attr(self.sol, self.key, attr_id)
    }
}
impl<'a> ChargeMut<'a> {
    pub fn get_attr(&mut self, attr_id: &AttrId) -> Result<CalcAttrVal, GetItemAttrError> {
        get_attr(self.sol, self.key, attr_id)
    }
}
impl<'a> DroneMut<'a> {
    pub fn get_attr(&mut self, attr_id: &AttrId) -> Result<CalcAttrVal, GetItemAttrError> {
        get_attr(self.sol, self.key, attr_id)
    }
}
impl<'a> FighterMut<'a> {
    pub fn get_attr(&mut self, attr_id: &AttrId) -> Result<CalcAttrVal, GetItemAttrError> {
        get_attr(self.sol, self.key, attr_id)
    }
}
impl<'a> FwEffectMut<'a> {
    pub fn get_attr(&mut self, attr_id: &AttrId) -> Result<CalcAttrVal, GetItemAttrError> {
        get_attr(self.sol, self.key, attr_id)
    }
}
impl<'a> ImplantMut<'a> {
    pub fn get_attr(&mut self, attr_id: &AttrId) -> Result<CalcAttrVal, GetItemAttrError> {
        get_attr(self.sol, self.key, attr_id)
    }
}
impl<'a> ModuleMut<'a> {
    pub fn get_attr(&mut self, attr_id: &AttrId) -> Result<CalcAttrVal, GetItemAttrError> {
        get_attr(self.sol, self.key, attr_id)
    }
}
impl<'a> ProjEffectMut<'a> {
    pub fn get_attr(&mut self, attr_id: &AttrId) -> Result<CalcAttrVal, GetItemAttrError> {
        get_attr(self.sol, self.key, attr_id)
    }
}
impl<'a> RigMut<'a> {
    pub fn get_attr(&mut self, attr_id: &AttrId) -> Result<CalcAttrVal, GetItemAttrError> {
        get_attr(self.sol, self.key, attr_id)
    }
}
impl<'a> ServiceMut<'a> {
    pub fn get_attr(&mut self, attr_id: &AttrId) -> Result<CalcAttrVal, GetItemAttrError> {
        get_attr(self.sol, self.key, attr_id)
    }
}
impl<'a> ShipMut<'a> {
    pub fn get_attr(&mut self, attr_id: &AttrId) -> Result<CalcAttrVal, GetItemAttrError> {
        get_attr(self.sol, self.key, attr_id)
    }
}
impl<'a> SkillMut<'a> {
    pub fn get_attr(&mut self, attr_id: &AttrId) -> Result<CalcAttrVal, GetItemAttrError> {
        get_attr(self.sol, self.key, attr_id)
    }
}
impl<'a> StanceMut<'a> {
    pub fn get_attr(&mut self, attr_id: &AttrId) -> Result<CalcAttrVal, GetItemAttrError> {
        get_attr(self.sol, self.key, attr_id)
    }
}
impl<'a> SubsystemMut<'a> {
    pub fn get_attr(&mut self, attr_id: &AttrId) -> Result<CalcAttrVal, GetItemAttrError> {
        get_attr(self.sol, self.key, attr_id)
    }
}
impl<'a> SwEffectMut<'a> {
    pub fn get_attr(&mut self, attr_id: &AttrId) -> Result<CalcAttrVal, GetItemAttrError> {
        get_attr(self.sol, self.key, attr_id)
    }
}

fn get_attr(sol: &mut SolarSystem, item_key: ItemKey, attr_id: &AttrId) -> Result<CalcAttrVal, GetItemAttrError> {
    match sol.internal_get_item_attr(item_key, attr_id) {
        Ok(calc_val) => Ok(calc_val),
        Err(error) => Err(match error {
            AttrCalcError::KeyedItemNotLoaded(_) => ItemLoadedError {
                item_id: sol.uad.items.id_by_key(item_key),
            }
            .into(),
            AttrCalcError::AttrMetaNotFound(e) => e.into(),
        }),
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetItemAttrError {
    #[error("{0}")]
    ItemNotLoaded(#[from] ItemLoadedError),
    #[error("{0}")]
    AttrMetaNotFound(#[from] AttrMetaFoundError),
}
