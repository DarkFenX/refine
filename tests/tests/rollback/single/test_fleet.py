from fw import approx


def test_fit_change_fleet_not_found(client, consts):
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
    api_boosted_fit = api_sol.create_fit(fleet_id=api_fleet.id, sec_status=2)
    api_boosted_ship = api_boosted_fit.set_ship(type_id=eve_ship_id)
    api_other_fleet = api_sol.create_fleet()
    api_other_fleet.remove()
    # Action & verification - fleet is set first, so security status is not reached
    api_boosted_fit.change(
        fleet_id=api_other_fleet.id,
        sec_status=-5,
        status_code=400,
        json_predicate={
            'code': 'FIT-004',
            'message': f'failed to change fit: failed to set fleet: fleet {api_other_fleet.id} not found'})
    api_boosted_fit.update()
    assert api_boosted_fit.fleet_id == api_fleet.id
    assert api_boosted_fit.sec_status == approx(2)
    assert api_boosted_ship.update().attrs[eve_affectee_attr_id].modified == approx(37.5)


def test_fleet_change_add_fit_not_found(client, consts):
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
    # Action & verification
    api_fleet.change(
        add_fit_ids=[api_other_fit.id],
        status_code=400,
        json_predicate={'code': 'FLT-004', 'message': f'failed to add fit: fit {api_other_fit.id} not found'})
    assert api_fleet.update().fits.keys() == {api_booster_fit.id, api_boosted_fit.id}
    assert api_boosted_fit.update().fleet_id == api_fleet.id
    assert api_boosted_ship.update().attrs[eve_affectee_attr_id].modified == approx(37.5)


def test_fleet_change_rm_fit_not_found(client, consts):
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
    # Action & verification
    api_fleet.change(
        rm_fit_ids=[api_other_fit.id],
        status_code=400,
        json_predicate={'code': 'FLT-005', 'message': f'failed to remove fit: fit {api_other_fit.id} not found'})
    assert api_fleet.update().fits.keys() == {api_booster_fit.id, api_boosted_fit.id}
    assert api_boosted_ship.update().attrs[eve_affectee_attr_id].modified == approx(37.5)
