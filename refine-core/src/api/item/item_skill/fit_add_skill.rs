use std::collections::hash_map::Entry;

use crate::{
    Fit, ItemTypeId, Skill, SkillLevel,
    ad::AItemId,
    err::basic::SkillEveTypeError,
    sol::{CoreInner, UndoStep},
    ud::{UFitId, UFitSkill, UItem, UItemId, USkill},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Fit<'s> {
    pub fn add_skill(&mut self, type_id: ItemTypeId, level: SkillLevel) -> Result<Skill<'_>, SkillAddError> {
        let skill_uid = self.core.internal_add_skill(self.uid, type_id.into_aid(), level)?;
        Ok(Skill::new(self.core, skill_uid))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SkillAddError {
    #[error(transparent)]
    SkillIdCollision(#[from] SkillEveTypeError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CoreInner {
    pub(in crate::api) fn internal_add_skill(
        &mut self,
        fit_uid: UFitId,
        type_aid: AItemId,
        level: SkillLevel,
    ) -> Result<UItemId, SkillEveTypeError> {
        let fit = self.u_data.fits.get_mut(fit_uid);
        match fit.skills.entry(type_aid) {
            Entry::Vacant(entry) => {
                let item_id = self.u_data.items.alloc_id();
                let skill = USkill::new(item_id, type_aid, fit_uid, level, true, &self.u_data.r_data);
                let item = UItem::Skill(skill);
                let skill_uid = self.u_data.items.add(item);
                entry.insert(UFitSkill { skill_uid, level });
                CoreInner::util_add_skill(&mut self.u_data, &mut self.svc, skill_uid, &mut self.reuse.eupdates);
                if self.undo.is_recording() {
                    self.undo.record(UndoStep::SkillAdded(skill_uid));
                }
                Ok(skill_uid)
            }
            Entry::Occupied(entry) => Err(SkillEveTypeError {
                type_id: ItemTypeId::from_aid(type_aid),
                fit_id: fit.id,
                item_id: self.u_data.items.ext_id_by_int_id(entry.get().skill_uid),
            }),
        }
    }
    pub(in crate::api) fn internal_reattach_skill(&mut self, skill_uid: UItemId) {
        self.u_data.items.reattach(skill_uid);
        let u_skill = self.u_data.items.get(skill_uid).dc_skill().unwrap();
        let u_fit = self.u_data.fits.get_mut(u_skill.get_fit_uid());
        u_fit.skills.insert(
            u_skill.get_type_aid(),
            UFitSkill {
                skill_uid,
                level: u_skill.get_level(),
            },
        );
        CoreInner::util_add_skill(&mut self.u_data, &mut self.svc, skill_uid, &mut self.reuse.eupdates);
    }
}
