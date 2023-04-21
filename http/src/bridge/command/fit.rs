pub(in super::super) enum FitCommand {
    SetShip(SetShip),
}

pub(in super::super) struct SetShip {
    ship_type_id: reefast::ReeInt,
}
