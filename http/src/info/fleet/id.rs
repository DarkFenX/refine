use crate::util::HResult;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HFleetInfoId {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::SsFleetId,
}
impl HFleetInfoId {
    pub(in crate::info::fleet) fn mk_info(core_ss: &rc::SolarSystem, fleet_id: &rc::SsFleetId) -> HResult<Self> {
        let core_fleet = core_ss.get_fleet(fleet_id)?;
        let info = Self { id: core_fleet.id };
        Ok(info)
    }
}
