use crate::{
    ed::{EBuffId, EGenFloat, EItemId, EItemListId},
    util::LibNamed,
};

pub struct EItemSpaceComp {
    pub item_id: EItemId,
    pub system_wide_buffs: Option<EItemSpaceCompBuffData>,
    pub system_emitter_buffs: Option<EItemSpaceCompBuffData>,
    pub proxy_effect_buffs: Option<EItemSpaceCompBuffData>,
    pub proxy_trigger_buffs: Option<EItemSpaceCompBuffData>,
    pub ship_link_buffs: Option<EItemSpaceCompBuffData>,
}
impl EItemSpaceComp {
    pub(crate) fn iter_data(&self) -> impl Iterator<Item = &EItemSpaceCompBuffData> {
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
impl LibNamed for EItemSpaceComp {
    fn lib_get_name() -> &'static str {
        "EItemSpaceComp"
    }
}

pub struct EItemSpaceCompBuffData {
    pub buffs: Vec<EItemSpaceCompBuffEntry>,
    pub item_list_filter: Option<EItemListId>,
}

pub struct EItemSpaceCompBuffEntry {
    pub id: EBuffId,
    pub value: EGenFloat,
}
