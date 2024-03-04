use crate::{
    ad,
    defs::SsItemId,
    ss::{
        svc::{calc::support::SsAttrMod, SsSvcs},
        SsView,
    },
};

impl SsSvcs {
    pub(in crate::ss::svc::calc) fn buff_to_modifiers(
        &mut self,
        ss_view: &SsView,
        item_id: &SsItemId,
        effect: &ad::ArcEffect,
    ) -> Vec<SsAttrMod> {
        let mut mods = Vec::new();
        if let Some(buff_info) = &effect.buff {
            match buff_info.data_source {
                ad::AEffectBuffDataSrc::Hardcoded(buff_id, buff_val) => {}
                ad::AEffectBuffDataSrc::DefaultAttrs => {}
            }
        }
        mods
    }
}
