use crate::{
    ed::{EBuffId, EFloat, EItemId, EItemListId},
    util::LibNamed,
};

pub struct EItemBuff {
    pub item_id: EItemId,
    pub system_wide_buffs: Option<EItemBuffData>,
    pub system_emitter_buffs: Option<EItemBuffData>,
    pub proxy_effect_buffs: Option<EItemBuffData>,
    pub proxy_trigger_buffs: Option<EItemBuffData>,
    pub ship_link_buffs: Option<EItemBuffData>,
}
impl EItemBuff {
    pub(crate) fn iter_data(&self) -> impl Iterator<Item = &EItemBuffData> {
        [
            &self.system_wide_buffs,
            &self.system_emitter_buffs,
            &self.proxy_effect_buffs,
            &self.proxy_trigger_buffs,
            &self.ship_link_buffs,
        ]
        .into_iter()
        .filter_map(|v| v.as_ref())
    }
    pub(crate) fn has_buffs(&self) -> bool {
        for buff_data in self.iter_data() {
            if !buff_data.buffs.is_empty() {
                return true;
            }
        }
        false
    }
}
impl LibNamed for EItemBuff {
    fn lib_get_name() -> &'static str {
        "EItemBuff"
    }
}

pub struct EItemBuffData {
    pub buffs: Vec<EItemBuffEntry>,
    pub item_list_filter: Option<EItemListId>,
}

pub struct EItemBuffEntry {
    pub id: EBuffId,
    pub value: EFloat,
}
