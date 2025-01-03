from tests import approx
from tests.features.reactive_armor_hardener import make_eve_rah, make_eve_ship, setup_rah_basics


def test_complete_single(client, consts):
    # Check simulation of single RAH, with sim state loop detected before sim tick limit is broken.
    # Also check non-dogma attribute values.
    eve_basic_info = setup_rah_basics(client=client, consts=consts)
    eve_rah_id = make_eve_rah(client=client, basic_info=eve_basic_info, resos=(0.85, 0.85, 0.85, 0.85), shift_amount=6)
    eve_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, resos=(0.5, 0.65, 0.75, 0.9))
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit(rah_incoming_dmg=(1, 1, 1, 1))
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah = api_fit.add_mod(type_id=eve_rah_id, state=consts.ApiState.active)
    # Verification
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].base == approx(0.85)
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].dogma == approx(1)
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].extra == approx(1)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].base == approx(0.85)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.925)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].extra == approx(0.925)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].base == approx(0.85)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.82)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].extra == approx(0.82)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].base == approx(0.85)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.655)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].extra == approx(0.655)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.5)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.60125)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.615)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.5895)


def test_complete_double_synced(client, consts):
    # Check simulation of dual RAHs, with sim state loop detected before sim tick limit is broken.
    eve_basic_info = setup_rah_basics(client=client, consts=consts)
    eve_rah_id = make_eve_rah(client=client, basic_info=eve_basic_info, resos=(0.85, 0.85, 0.85, 0.85), shift_amount=6)
    eve_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, resos=(0.5, 0.65, 0.75, 0.9))
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit(rah_incoming_dmg=(1, 1, 1, 1))
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah1 = api_fit.add_mod(type_id=eve_rah_id, state=consts.ApiState.active)
    api_rah2 = api_fit.add_mod(type_id=eve_rah_id, state=consts.ApiState.active)
    # Verification
    api_rah1.update()
    assert api_rah1.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.97)
    assert api_rah1.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.88)
    assert api_rah1.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.805)
    assert api_rah1.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.745)
    api_rah2.update()
    assert api_rah2.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.97)
    assert api_rah2.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.88)
    assert api_rah2.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.805)
    assert api_rah2.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.745)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.472354)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.512344)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.501427)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.5219)


def test_complete_double_unsynced(client, consts):
    # Check simulation of dual RAHs with different cycle times (realistic scenario with one RAH
    # being heated and the other one cold, as much as realistic scenario with 2 RAHs can be that
    # is), with sim state loop detected before sim tick limit is broken.
    eve_basic_info = setup_rah_basics(client=client, consts=consts)
    eve_rah_id = make_eve_rah(client=client, basic_info=eve_basic_info, resos=(0.85, 0.85, 0.85, 0.85), shift_amount=6)
    eve_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, resos=(0.5, 0.65, 0.75, 0.9))
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit(rah_incoming_dmg=(1, 1, 1, 1))
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah1 = api_fit.add_mod(type_id=eve_rah_id, state=consts.ApiState.active)
    api_rah2 = api_fit.add_mod(type_id=eve_rah_id, state=consts.ApiState.overload)
    # Verification
    api_rah1.update()
    assert api_rah1.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.975294)
    assert api_rah1.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.835)
    assert api_rah1.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.829706)
    assert api_rah1.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.76)
    api_rah2.update()
    assert api_rah2.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.979)
    assert api_rah2.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.91)
    assert api_rah2.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.796)
    assert api_rah2.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.715)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.478747)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.500296)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.50864)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.509273)


def test_no_loop(client, consts):
    # In this test, RAH sim does not detect loop within sim tick limit (1 + 500) due to cycle time
    # of both modules
    eve_basic_info = setup_rah_basics(client=client, consts=consts)
    eve_rah1_id = make_eve_rah(
        client=client,
        basic_info=eve_basic_info,
        resos=(0.85, 0.85, 0.85, 0.85),
        shift_amount=6,
        cycle_time=5.9)
    eve_rah2_id = make_eve_rah(
        client=client,
        basic_info=eve_basic_info,
        resos=(0.85, 0.85, 0.85, 0.85),
        shift_amount=6,
        cycle_time=6.1)
    eve_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, resos=(0.5, 0.65, 0.75, 0.9))
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit(rah_incoming_dmg=(1, 1, 1, 1))
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah1 = api_fit.add_mod(type_id=eve_rah1_id, state=consts.ApiState.active)
    api_rah2 = api_fit.add_mod(type_id=eve_rah2_id, state=consts.ApiState.overload)
    # Verification
    api_rah1.update()
    assert api_rah1.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.97739)
    assert api_rah1.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.881249)
    assert api_rah1.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.776787)
    assert api_rah1.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.764573)
    api_rah2.update()
    assert api_rah2.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.978735)
    assert api_rah2.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.873506)
    assert api_rah2.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.839397)
    assert api_rah2.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.708362)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.479663)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.509179)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.50127)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.507079)


def test_no_loop_history_ignore_limit(client, consts):
    # Like in previous test, RAH sim does not detect state loop here due to RAH durations. When it
    # does not detect loop, it attempts to ignore part of history during which slowest RAH "drains"
    # its highest resist to zero, and some more. To avoid ignoring too much of a history, there is a
    # limit, and in this test we check that this limit is applied - by using durations which are
    # built not to detect loop, and having fast-cycling RAH (to generate ticks to reach tick limit)
    # and slow RAH, which would serve as a trigger to ignore large part of simulation history. If
    # sim had no ignore limit, it wouldn't modify initial RAH resonances.
    eve_basic_info = setup_rah_basics(client=client, consts=consts)
    eve_rah1_id = make_eve_rah(
        client=client,
        basic_info=eve_basic_info,
        resos=(0.85, 0.85, 0.85, 0.85),
        shift_amount=6,
        cycle_time=5.9)
    eve_rah2_id = make_eve_rah(
        client=client,
        basic_info=eve_basic_info,
        resos=(0.4, 1, 1, 1),
        shift_amount=0.6,
        cycle_time=61)
    eve_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, resos=(0.5, 0.65, 0.75, 0.9))
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit(rah_incoming_dmg=(1, 1, 1, 1))
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah1 = api_fit.add_mod(type_id=eve_rah1_id, state=consts.ApiState.active)
    api_rah2 = api_fit.add_mod(type_id=eve_rah2_id, state=consts.ApiState.overload)
    # Verification
    api_rah1.update()
    assert api_rah1.attrs[eve_basic_info.res_em_attr_id].dogma == approx(1)
    assert api_rah1.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.925)
    assert api_rah1.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.82)
    assert api_rah1.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.655)
    api_rah2.update()
    assert api_rah2.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.631)
    assert api_rah2.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.924423)
    assert api_rah2.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.899731)
    assert api_rah2.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.9448462)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.3155)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.561708)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.561405)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.561242)
