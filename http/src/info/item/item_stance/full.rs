use super::HStanceInfoPartial;
use crate::info::item::extended::HItemExtendedInfo;

#[derive(serde::Serialize)]
pub(crate) struct HStanceInfoFull {
    #[serde(flatten)]
    partial_info: HStanceInfoPartial,
    #[serde(flatten)]
    extended_info: HItemExtendedInfo,
}
impl From<&mut rc::StanceMut<'_>> for HStanceInfoFull {
    fn from(core_stance: &mut rc::StanceMut) -> Self {
        Self {
            partial_info: core_stance.into(),
            extended_info: core_stance.into(),
        }
    }
}
