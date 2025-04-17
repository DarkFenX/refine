use rc::ItemCommon;

use crate::shared::HEffectId;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HAutochargeInfoPartial {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::ItemId,
    pub(crate) kind: &'static str,
    pub(crate) type_id: rc::ItemTypeId,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) fit_id: rc::FitId,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) cont_item_id: rc::ItemId,
    pub(crate) cont_effect_id: HEffectId,
    pub(crate) enabled: bool,
}
impl From<&mut rc::AutochargeMut<'_>> for HAutochargeInfoPartial {
    fn from(core_autocharge: &mut rc::AutochargeMut) -> Self {
        Self {
            id: core_autocharge.get_item_id(),
            kind: "autocharge",
            type_id: core_autocharge.get_type_id(),
            fit_id: core_autocharge.get_fit().get_fit_id(),
            cont_item_id: core_autocharge.get_cont_item().get_item_id(),
            cont_effect_id: core_autocharge.get_cont_effect_id().into(),
            enabled: core_autocharge.get_state(),
        }
    }
}
