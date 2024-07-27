from tests import approx


def test_loaded_to_loaded(client):
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    eve_d1_attr1 = eve_d1.mk_attr()
    eve_d2_attr1 = eve_d2.mk_attr()
    eve_d2_attr2 = eve_d2.mk_attr()
    eve_d1_ship = eve_d1.mk_ship(attrs={eve_d1_attr1.id: 50})
    eve_d2.mk_ship(id_=eve_d1_ship.id, attrs={eve_d2_attr1.id: 30, eve_d2_attr2.id: 85})
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1)
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_d1_ship.id)
    assert api_ship.update().attrs[eve_d1_attr1.id].dogma == approx(50)
    api_sol.change_src(data=eve_d2)
    api_ship.update()
    assert api_ship.attrs[eve_d2_attr1.id].dogma == approx(30)
    assert api_ship.attrs[eve_d2_attr2.id].dogma == approx(85)


def test_loaded_to_unloaded_to_loaded(client):
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    eve_d1_attr1 = eve_d1.mk_attr()
    eve_d1_ship = eve_d1.mk_ship(attrs={eve_d1_attr1.id: 50})
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1)
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_d1_ship.id)
    # Verification
    assert api_ship.update().attrs[eve_d1_attr1.id].dogma == approx(50)
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification
    assert len(api_ship.update().attrs) == 0
    # Action
    api_sol.change_src(data=eve_d1)
    # Verification
    assert api_ship.update().attrs[eve_d1_attr1.id].dogma == approx(50)
