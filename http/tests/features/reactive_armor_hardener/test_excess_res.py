from tests import approx
from tests.features.reactive_armor_hardener import make_eve_rah, make_eve_ship, setup_rah_basics


def test_full(client, consts):
    eve_basic_info = setup_rah_basics(client=client, consts=consts)
    eve_rah_id = make_eve_rah(
        client=client,
        basic_info=eve_basic_info,
        resos=(1, 0, 1, 1),
        shift_amount=10)
    eve_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, resos=(0.5, 0.65, 0.59, 0.51))
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit(rah_incoming_dmg=(0, 0, 1, 0))
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah = api_fit.add_mod(type_id=eve_rah_id, state=consts.ApiState.active)
    # Verification
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].dogma == approx(1)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(1)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(1)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.5)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.65)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.51)


def test_excess_slow_stable(client, consts):
    eve_basic_info = setup_rah_basics(client=client, consts=consts)
    eve_rah_id = make_eve_rah(
        client=client,
        basic_info=eve_basic_info,
        resos=(0.3, 0.5, 0.3, 0.5),
        shift_amount=10)
    eve_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, resos=(0.5, 0.65, 0.59, 0.51))
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit(rah_incoming_dmg=(0, 1, 1, 0))
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah = api_fit.add_mod(type_id=eve_rah_id, state=consts.ApiState.active)
    # Verification - purely theoretical, but possible with overrides / custom mods
    # 0 0.300 0.500 0.300 0.500
    # 1 0.400 0.400 0.200 0.600
    # 2 0.500 0.300 0.100 0.700
    # 3 0.600 0.200 0.000 0.800
    # ---loop---
    # 4 0.700 0.000 0.000 0.900
    # On 3rd tick, 3 resonances donated shift amount (kinetic too, since it didn't take any damage
    # due to 100% resist), which got distributed in favor of thermal - which does not go past 100%,
    # but distributes excess into next resistance from the end, kinetic (which depends on the order
    # of resonances, ship being shot from kinetic damage sources is irrelevant here)
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.7)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.9)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.35)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.459)


def test_excess_slow_loop(client, consts):
    eve_basic_info = setup_rah_basics(client=client, consts=consts)
    eve_rah_id = make_eve_rah(
        client=client,
        basic_info=eve_basic_info,
        resos=(0.3, 0.5, 0.3, 0.5),
        shift_amount=10)
    eve_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, resos=(0.5, 0.65, 0.59, 0.51))
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit(rah_incoming_dmg=(0, 0, 1, 1))
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah = api_fit.add_mod(type_id=eve_rah_id, state=consts.ApiState.active)
    # Verification - purely theoretical, but possible with overrides / custom mods
    # 0 0.300 0.500 0.300 0.500
    # 1 0.400 0.600 0.200 0.400
    # 2 0.500 0.700 0.100 0.300
    # 3 0.600 0.800 0.000 0.200 - EM therm kin give 0.1, expl takes 0.2, therm takes 0.1
    # 4 0.700 0.800 0.100 0.000 - EM therm expl give 0.1, kin takes 0.1, therm takes 0.2
    # 5 0.800 0.700 0.000 0.100 - EM therm kin give 0.1, expl takes 0.1, therm takes 0.2
    # 6 0.900 0.600 0.100 0.000 - EM therm expl give 0.1, kin takes 0.1, therm takes 0.2
    # ---loop---
    # 7 1.000 0.500 0.000 0.100 - therm kin donate 0.1, therm expl take 0.1
    # 8 1.000 0.500 0.100 0.000 - therm expl donate 0.1, therm kin take 0.1
    # For donating, EM is the most preferred and thermal is the least preferred (assuming equal
    # damage taken), for receiving donated resistances it's reverse
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].dogma == approx(1)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.5)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.05)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.05)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.5)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.325)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.0295)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.0255)


def test_excess_max_takers(client, consts):
    eve_basic_info = setup_rah_basics(client=client, consts=consts)
    eve_rah_id = make_eve_rah(
        client=client,
        basic_info=eve_basic_info,
        resos=(0, 0, 0.2, 0.4),
        shift_amount=100)
    eve_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, resos=(0.5, 0.65, 0.59, 0.51))
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit(rah_incoming_dmg=(0, 0, 1, 1))
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah = api_fit.add_mod(type_id=eve_rah_id, state=consts.ApiState.active)
    # Verification
    # 0 0.000 0.000 0.200 0.400 - EM gives 1, therm gives 1, expl takes 0.4, kin takes 0.2, therm
    # takes 1, EM takes 0.4
    # ---loop---
    # 1 0.600 0.000 0.000 0.000
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.6)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.3)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0)
