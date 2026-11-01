from fw import approx


def test_add_fit_not_found(client, consts):
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.buff_fleet_ships, cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 5},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fleet = api_sol.create_fleet()
    api_booster_fit = api_sol.create_fit(fleet_id=api_fleet.id)
    api_booster_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_boosted_fit = api_sol.create_fit(fleet_id=api_fleet.id)
    api_boosted_ship = api_boosted_fit.set_ship(type_id=eve_ship_id)
    # Verification
    assert api_boosted_ship.update().attrs[eve_affectee_attr_id].modified == approx(37.5)
    api_other_fit = api_sol.create_fit()
    api_other_fit.remove()
    # Action & verification - boosted fit leaves its fleet for the new one before the next fit fails
    # to join it
    api_sol.create_fleet(
        fit_ids=[api_boosted_fit.id, api_other_fit.id],
        status_code=400,
        json_predicate={'code': 'FLT-003', 'message': f'failed to add fit to fleet: fit {api_other_fit.id} not found'})
    assert list(api_sol.update().fleets) == [api_fleet.id]
    assert sorted(api_fleet.update().fits) == sorted([api_booster_fit.id, api_boosted_fit.id])
    assert api_boosted_fit.update().fleet_id == api_fleet.id
    assert api_boosted_ship.update().attrs[eve_affectee_attr_id].modified == approx(37.5)


def test_add_only_fit_not_found(client):
    client.create_sources()
    api_sol = client.create_sol()
    api_fleet = api_sol.create_fleet()
    api_other_fit = api_sol.create_fit()
    api_other_fit.remove()
    # Action & verification - fleet is created before its only fit fails to join it
    api_sol.create_fleet(
        fit_ids=[api_other_fit.id],
        status_code=400,
        json_predicate={'code': 'FLT-003', 'message': f'failed to add fit to fleet: fit {api_other_fit.id} not found'})
    assert list(api_sol.update().fleets) == [api_fleet.id]


def test_change_add_fit_not_found(client, consts):
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.buff_fleet_ships, cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 5},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fleet1 = api_sol.create_fleet()
    api_booster_fit = api_sol.create_fit(fleet_id=api_fleet1.id)
    api_booster_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_boosted_fit = api_sol.create_fit(fleet_id=api_fleet1.id)
    api_boosted_ship = api_boosted_fit.set_ship(type_id=eve_ship_id)
    api_fleet2 = api_sol.create_fleet()
    api_joining_fit = api_sol.create_fit(fleet_id=api_fleet2.id)
    api_joining_ship = api_joining_fit.set_ship(type_id=eve_ship_id)
    # Verification
    assert api_boosted_ship.update().attrs[eve_affectee_attr_id].modified == approx(37.5)
    assert api_joining_ship.update().attrs[eve_affectee_attr_id].modified == approx(7.5)
    api_other_fit = api_sol.create_fit()
    api_other_fit.remove()
    # Action & verification - boosted fit leaves the fleet, and fit from another fleet joins it,
    # before the next fit fails to join
    api_fleet1.change(
        rm_fit_ids=[api_boosted_fit.id],
        add_fit_ids=[api_joining_fit.id, api_other_fit.id],
        status_code=400,
        json_predicate={'code': 'FLT-004', 'message': f'failed to add fit: fit {api_other_fit.id} not found'})
    assert sorted(api_fleet1.update().fits) == sorted([api_booster_fit.id, api_boosted_fit.id])
    assert list(api_fleet2.update().fits) == [api_joining_fit.id]
    assert api_boosted_fit.update().fleet_id == api_fleet1.id
    assert api_joining_fit.update().fleet_id == api_fleet2.id
    assert api_boosted_ship.update().attrs[eve_affectee_attr_id].modified == approx(37.5)
    assert api_joining_ship.update().attrs[eve_affectee_attr_id].modified == approx(7.5)


def test_change_rm_fit_not_found(client, consts):
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.buff_fleet_ships, cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 5},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fleet = api_sol.create_fleet()
    api_booster_fit = api_sol.create_fit(fleet_id=api_fleet.id)
    api_booster_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_boosted_fit = api_sol.create_fit(fleet_id=api_fleet.id)
    api_boosted_ship = api_boosted_fit.set_ship(type_id=eve_ship_id)
    api_other_fit = api_sol.create_fit()
    api_other_fit.remove()
    # Action & verification - boosted fit leaves the fleet before the next fit is not found
    api_fleet.change(
        rm_fit_ids=[api_boosted_fit.id, api_other_fit.id],
        status_code=400,
        json_predicate={'code': 'FLT-005', 'message': f'failed to remove fit: fit {api_other_fit.id} not found'})
    assert sorted(api_fleet.update().fits) == sorted([api_booster_fit.id, api_boosted_fit.id])
    assert api_boosted_fit.update().fleet_id == api_fleet.id
    assert api_boosted_ship.update().attrs[eve_affectee_attr_id].modified == approx(37.5)
