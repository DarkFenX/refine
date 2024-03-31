use crate::{
    ad,
    ss::{
        item::SsItem,
        svc::svce_calc::modifier::{extend_with_custom_mods, SsAttrMod, SsModType},
    },
};

pub(in crate::ss::svc::svce_calc) fn a_data_to_ss_mods(item: &SsItem, effects: &Vec<ad::ArcEffect>) -> Vec<SsAttrMod> {
    let item_id = item.get_id();
    let mod_type = match item {
        SsItem::SwEffect(_) => SsModType::SystemWide,
        SsItem::FwEffect(_) => SsModType::FitWide,
        _ => SsModType::Local,
    };
    let mut mods = Vec::new();
    for effect in effects.iter() {
        // Regular modifiers
        mods.extend(
            effect
                .mods
                .iter()
                .map(|v| SsAttrMod::from_a_data(item, effect, v, mod_type)),
        );
        // Custom modifiers
        extend_with_custom_mods(item_id, effect.id, &mut mods);
    }
    mods
}
