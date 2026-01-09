use rc::ItemCommon;
use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};

use crate::info::item::proj::HProjInfo;

#[serde_as]
#[derive(Serialize)]
pub(crate) struct HProjEffectInfoPartial {
    #[serde_as(as = "DisplayFromStr")]
    id: rc::ItemId,
    kind: &'static str,
    type_id: i32,
    enabled: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    projs: Vec<HProjInfo>,
}
impl From<&mut rc::ProjEffectMut<'_>> for HProjEffectInfoPartial {
    fn from(core_proj_effect: &mut rc::ProjEffectMut) -> Self {
        Self {
            id: core_proj_effect.get_item_id(),
            kind: "proj_effect",
            type_id: core_proj_effect.get_type_id().into_i32(),
            enabled: core_proj_effect.get_state(),
            projs: core_proj_effect.iter_projs().map(Into::into).collect(),
        }
    }
}
