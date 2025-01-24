from tests import approx, check_no_field


def test_loaded_to_loaded(client):
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
    assert api_ship.update().attrs[eve_d1_attr1_id].dogma == approx(50)
    api_sol.change_src(data=eve_d2)
    api_ship.update()
    assert api_ship.attrs[eve_d2_attr1_id].dogma == approx(30)
    assert api_ship.attrs[eve_d2_attr2_id].dogma == approx(85)


def test_loaded_to_not_loaded_to_loaded(client):
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    eve_d1_attr1_id = client.mk_eve_attr(datas=[eve_d1])
    eve_d1_ship_ip = client.mk_eve_ship(datas=[eve_d1], attrs={eve_d1_attr1_id: 50})
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1)
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_d1_ship_ip)
    # Verification
    assert api_ship.update().attrs[eve_d1_attr1_id].dogma == approx(50)
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification
    api_ship.update()
    with check_no_field():
        api_ship.attrs  # pylint: disable=W0104
    # Action
    api_sol.change_src(data=eve_d1)
    # Verification
    assert api_ship.update().attrs[eve_d1_attr1_id].dogma == approx(50)
