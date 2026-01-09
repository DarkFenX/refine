use rc::ItemCommon;
use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};

#[serde_as]
#[derive(Serialize)]
pub(crate) struct HAutochargeInfoPartial {
    #[serde_as(as = "DisplayFromStr")]
    id: rc::ItemId,
    kind: &'static str,
    type_id: i32,
    #[serde_as(as = "DisplayFromStr")]
    fit_id: rc::FitId,
    #[serde_as(as = "DisplayFromStr")]
    cont_item_id: rc::ItemId,
    #[serde_as(as = "DisplayFromStr")]
    cont_effect_id: rc::EffectId,
    enabled: bool,
}
impl From<&mut rc::AutochargeMut<'_>> for HAutochargeInfoPartial {
    fn from(core_autocharge: &mut rc::AutochargeMut) -> Self {
        Self {
            id: core_autocharge.get_item_id(),
            kind: "autocharge",
            type_id: core_autocharge.get_type_id().into_i32(),
            fit_id: core_autocharge.get_fit().get_fit_id(),
            cont_item_id: core_autocharge.get_cont_item().get_item_id(),
            cont_effect_id: core_autocharge.get_cont_effect_id().into(),
            enabled: core_autocharge.get_state(),
        }
    }
}
