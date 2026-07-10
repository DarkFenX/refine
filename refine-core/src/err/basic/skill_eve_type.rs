use crate::{
    api::ItemTypeId,
    ud::{FitId, ItemId},
};

#[derive(thiserror::Error, Debug)]
#[error("skill {type_id} already exists on fit {fit_id}, item {item_id} has the same type ID")]
pub struct SkillEveTypeError {
    pub type_id: ItemTypeId,
    pub fit_id: FitId,
    pub item_id: ItemId,
}
