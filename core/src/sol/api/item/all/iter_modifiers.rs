use crate::{
    err::basic::ItemLoadedError,
    sol::{
        AttrId, ItemKey, SolarSystem,
        api::{
            AutochargeMut, BoosterMut, CharacterMut, ChargeMut, DroneMut, FighterMut, FwEffectMut, ImplantMut, ItemMut,
            ModuleMut, ProjEffectMut, RigMut, ServiceMut, ShipMut, SkillMut, StanceMut, SubsystemMut, SwEffectMut,
        },
        svc::calc::ModificationInfo,
    },
};

impl<'a> ItemMut<'a> {
    pub fn iter_modifiers(
        &'a mut self,
    ) -> Result<impl ExactSizeIterator<Item = (AttrId, Vec<ModificationInfo>)>, IterItemModifiersError> {
        let item_key = self.get_key();
        iter_modifiers(self.get_sol_mut(), item_key)
    }
}
impl<'a> AutochargeMut<'a> {
    pub fn iter_modifiers(
        &'a mut self,
    ) -> Result<impl ExactSizeIterator<Item = (AttrId, Vec<ModificationInfo>)>, IterItemModifiersError> {
        iter_modifiers(self.sol, self.key)
    }
}
impl<'a> BoosterMut<'a> {
    pub fn iter_modifiers(
        &'a mut self,
    ) -> Result<impl ExactSizeIterator<Item = (AttrId, Vec<ModificationInfo>)>, IterItemModifiersError> {
        iter_modifiers(self.sol, self.key)
    }
}
impl<'a> CharacterMut<'a> {
    pub fn iter_modifiers(
        &'a mut self,
    ) -> Result<impl ExactSizeIterator<Item = (AttrId, Vec<ModificationInfo>)>, IterItemModifiersError> {
        iter_modifiers(self.sol, self.key)
    }
}
impl<'a> ChargeMut<'a> {
    pub fn iter_modifiers(
        &'a mut self,
    ) -> Result<impl ExactSizeIterator<Item = (AttrId, Vec<ModificationInfo>)>, IterItemModifiersError> {
        iter_modifiers(self.sol, self.key)
    }
}
impl<'a> DroneMut<'a> {
    pub fn iter_modifiers(
        &'a mut self,
    ) -> Result<impl ExactSizeIterator<Item = (AttrId, Vec<ModificationInfo>)>, IterItemModifiersError> {
        iter_modifiers(self.sol, self.key)
    }
}
impl<'a> FighterMut<'a> {
    pub fn iter_modifiers(
        &'a mut self,
    ) -> Result<impl ExactSizeIterator<Item = (AttrId, Vec<ModificationInfo>)>, IterItemModifiersError> {
        iter_modifiers(self.sol, self.key)
    }
}
impl<'a> FwEffectMut<'a> {
    pub fn iter_modifiers(
        &'a mut self,
    ) -> Result<impl ExactSizeIterator<Item = (AttrId, Vec<ModificationInfo>)>, IterItemModifiersError> {
        iter_modifiers(self.sol, self.key)
    }
}
impl<'a> ImplantMut<'a> {
    pub fn iter_modifiers(
        &'a mut self,
    ) -> Result<impl ExactSizeIterator<Item = (AttrId, Vec<ModificationInfo>)>, IterItemModifiersError> {
        iter_modifiers(self.sol, self.key)
    }
}
impl<'a> ModuleMut<'a> {
    pub fn iter_modifiers(
        &'a mut self,
    ) -> Result<impl ExactSizeIterator<Item = (AttrId, Vec<ModificationInfo>)>, IterItemModifiersError> {
        iter_modifiers(self.sol, self.key)
    }
}
impl<'a> ProjEffectMut<'a> {
    pub fn iter_modifiers(
        &'a mut self,
    ) -> Result<impl ExactSizeIterator<Item = (AttrId, Vec<ModificationInfo>)>, IterItemModifiersError> {
        iter_modifiers(self.sol, self.key)
    }
}
impl<'a> RigMut<'a> {
    pub fn iter_modifiers(
        &'a mut self,
    ) -> Result<impl ExactSizeIterator<Item = (AttrId, Vec<ModificationInfo>)>, IterItemModifiersError> {
        iter_modifiers(self.sol, self.key)
    }
}
impl<'a> ServiceMut<'a> {
    pub fn iter_modifiers(
        &'a mut self,
    ) -> Result<impl ExactSizeIterator<Item = (AttrId, Vec<ModificationInfo>)>, IterItemModifiersError> {
        iter_modifiers(self.sol, self.key)
    }
}
impl<'a> ShipMut<'a> {
    pub fn iter_modifiers(
        &'a mut self,
    ) -> Result<impl ExactSizeIterator<Item = (AttrId, Vec<ModificationInfo>)>, IterItemModifiersError> {
        iter_modifiers(self.sol, self.key)
    }
}
impl<'a> SkillMut<'a> {
    pub fn iter_modifiers(
        &'a mut self,
    ) -> Result<impl ExactSizeIterator<Item = (AttrId, Vec<ModificationInfo>)>, IterItemModifiersError> {
        iter_modifiers(self.sol, self.key)
    }
}
impl<'a> StanceMut<'a> {
    pub fn iter_modifiers(
        &'a mut self,
    ) -> Result<impl ExactSizeIterator<Item = (AttrId, Vec<ModificationInfo>)>, IterItemModifiersError> {
        iter_modifiers(self.sol, self.key)
    }
}
impl<'a> SubsystemMut<'a> {
    pub fn iter_modifiers(
        &'a mut self,
    ) -> Result<impl ExactSizeIterator<Item = (AttrId, Vec<ModificationInfo>)>, IterItemModifiersError> {
        iter_modifiers(self.sol, self.key)
    }
}
impl<'a> SwEffectMut<'a> {
    pub fn iter_modifiers(
        &'a mut self,
    ) -> Result<impl ExactSizeIterator<Item = (AttrId, Vec<ModificationInfo>)>, IterItemModifiersError> {
        iter_modifiers(self.sol, self.key)
    }
}

fn iter_modifiers(
    sol: &mut SolarSystem,
    item_key: ItemKey,
) -> Result<impl ExactSizeIterator<Item = (AttrId, Vec<ModificationInfo>)>, IterItemModifiersError> {
    match sol.svc.calc.iter_item_mods(&sol.uad, item_key) {
        Ok(mods_iter) => Ok(mods_iter),
        Err(err) => Err(ItemLoadedError {
            item_id: sol.uad.items.id_by_key(err.item_key),
        }
        .into()),
    }
}

#[derive(thiserror::Error, Debug)]
pub enum IterItemModifiersError {
    #[error("{0}")]
    ItemNotLoaded(#[from] ItemLoadedError),
}
