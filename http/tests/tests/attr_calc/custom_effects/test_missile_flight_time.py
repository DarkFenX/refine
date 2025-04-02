from tests import approx


def test_missile_kinds(client, consts):
    eve_flight_time_attr_id = client.mk_eve_attr(id_=consts.EveAttr.explosion_delay)
    eve_speed_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.radius)
    eve_regular_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.missile_launching,
        cat_id=consts.EveEffCat.target)
    eve_defender_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.defender_missile_launching,
        cat_id=consts.EveEffCat.active)
    eve_fof_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.fof_missile_launching,
        cat_id=consts.EveEffCat.active)
    eve_dot_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.dot_missile_launching,
        cat_id=consts.EveEffCat.target)
    eve_regular_missile_id = client.mk_eve_item(
        attrs={eve_speed_attr_id: 500, eve_flight_time_attr_id: 10000},
        eff_ids=[eve_regular_effect_id],
        defeff_id=eve_regular_effect_id)
    eve_defender_missile_id = client.mk_eve_item(
        attrs={eve_speed_attr_id: 500, eve_flight_time_attr_id: 10000},
        eff_ids=[eve_defender_effect_id],
        defeff_id=eve_defender_effect_id)
    eve_fof_missile_id = client.mk_eve_item(
        attrs={eve_speed_attr_id: 500, eve_flight_time_attr_id: 10000},
        eff_ids=[eve_fof_effect_id],
        defeff_id=eve_fof_effect_id)
    eve_dot_missile_id = client.mk_eve_item(
        attrs={eve_speed_attr_id: 500, eve_flight_time_attr_id: 10000},
        eff_ids=[eve_dot_effect_id],
        defeff_id=eve_dot_effect_id)
    eve_module_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship(attrs={eve_radius_attr_id: 2000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_regular_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_regular_missile_id)
    api_defender_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_defender_missile_id)
    api_fof_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_fof_missile_id)
    api_dot_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_dot_missile_id)
    # Verification
    api_regular_module.update()
    assert api_regular_module.charge.attrs[eve_flight_time_attr_id].dogma == approx(10000)
    assert api_regular_module.charge.attrs[eve_flight_time_attr_id].extra == approx(14000)
    api_defender_module.update()
    assert api_defender_module.charge.attrs[eve_flight_time_attr_id].dogma == approx(10000)
    assert api_defender_module.charge.attrs[eve_flight_time_attr_id].extra == approx(14000)
    api_fof_module.update()
    assert api_fof_module.charge.attrs[eve_flight_time_attr_id].dogma == approx(10000)
    assert api_fof_module.charge.attrs[eve_flight_time_attr_id].extra == approx(14000)
    api_dot_module.update()
    assert api_dot_module.charge.attrs[eve_flight_time_attr_id].dogma == approx(10000)
    assert api_dot_module.charge.attrs[eve_flight_time_attr_id].extra == approx(14000)
