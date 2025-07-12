use crate::util::HExecError;

pub(crate) fn get_primary_fit<'a>(
    core_sol: &'a mut rc::SolarSystem,
    fit_id: &rc::FitId,
) -> Result<rc::FitMut<'a>, HExecError> {
    core_sol.get_fit_mut(fit_id).map_err(|error| match error {
        rc::err::GetFitError::FitNotFound(e) => HExecError::FitNotFoundPrimary(e),
    })
}

pub(crate) fn get_primary_fleet<'a>(
    core_sol: &'a mut rc::SolarSystem,
    fleet_id: &rc::FleetId,
) -> Result<rc::FleetMut<'a>, HExecError> {
    core_sol.get_fleet_mut(fleet_id).map_err(|error| match error {
        rc::err::GetFleetError::FleetNotFound(e) => HExecError::FleetNotFoundPrimary(e),
    })
}

pub(crate) fn get_primary_item<'a>(
    core_sol: &'a mut rc::SolarSystem,
    item_id: &rc::ItemId,
) -> Result<rc::ItemMut<'a>, HExecError> {
    core_sol.get_item_mut(item_id).map_err(|error| match error {
        rc::err::GetItemError::ItemNotFound(e) => HExecError::ItemNotFoundPrimary(e),
    })
}
