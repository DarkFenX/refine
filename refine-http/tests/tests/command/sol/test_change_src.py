from fw import approx, check_no_field


def test_error_params_malformed(client):
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    eve_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_sw_effect_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    client.mk_eve_ship(datas=[eve_d1], id_=eve_sw_effect_id, attrs={eve_attr_id: 50})
    client.mk_eve_ship(datas=[eve_d2], id_=eve_sw_effect_id, attrs={eve_attr_id: 30})
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1)
    api_sw_effect = api_sol.add_sw_effect(type_id=eve_sw_effect_id)
    # Verification
    api_sol.change_src(
        data=eve_d2,
        sol_info_mode='random',
        status_code=400,
        json_predicate={'code': 'PRM-001', 'message': 're:.+'})
    api_sol.change_src(
        data=eve_d2,
        fleet_info_mode='random',
        status_code=400,
        json_predicate={'code': 'PRM-001', 'message': 're:.+'})
    api_sol.change_src(
        data=eve_d2,
        fit_info_mode='random',
        status_code=400,
        json_predicate={'code': 'PRM-001', 'message': 're:.+'})
    api_sol.change_src(
        data=eve_d2,
        item_info_mode='random',
        status_code=400,
        json_predicate={'code': 'PRM-001', 'message': 're:.+'})
    assert api_sw_effect.update().attrs[eve_attr_id].modified == approx(50)


def test_ship_loaded_to_loaded(client):
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    eve_d1_attr1_id = client.alloc_attr_id(datas=[eve_d1, eve_d2])
    client.mk_eve_attr(datas=[eve_d1], id_=eve_d1_attr1_id)
    eve_d2_attr1_id = client.alloc_attr_id(datas=[eve_d1, eve_d2])
    client.mk_eve_attr(datas=[eve_d2], id_=eve_d2_attr1_id)
    eve_d2_attr2_id = client.alloc_attr_id(datas=[eve_d1, eve_d2])
    client.mk_eve_attr(datas=[eve_d2], id_=eve_d2_attr2_id)
    eve_ship_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    client.mk_eve_ship(datas=[eve_d1], id_=eve_ship_id, attrs={eve_d1_attr1_id: 50})
    client.mk_eve_ship(datas=[eve_d2], id_=eve_ship_id, attrs={eve_d2_attr1_id: 30, eve_d2_attr2_id: 85})
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1)
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    assert api_ship.update().attrs[eve_d1_attr1_id].modified == approx(50)
    api_sol.change_src(data=eve_d2)
    api_ship.update()
    assert api_ship.attrs[eve_d2_attr1_id].modified == approx(30)
    assert api_ship.attrs[eve_d2_attr2_id].modified == approx(85)


def test_ship_loaded_to_not_loaded_to_loaded(client):
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    eve_d1_attr1_id = client.mk_eve_attr(datas=[eve_d1])
    eve_d1_ship_ip = client.mk_eve_ship(datas=[eve_d1], attrs={eve_d1_attr1_id: 50})
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1)
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_d1_ship_ip)
    # Verification
    assert api_ship.update().attrs[eve_d1_attr1_id].modified == approx(50)
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification
    api_ship.update()
    with check_no_field():
        api_ship.attrs  # noqa: B018
    # Action
    api_sol.change_src(data=eve_d1)
    # Verification
    assert api_ship.update().attrs[eve_d1_attr1_id].modified == approx(50)
