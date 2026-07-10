from fw import approx
from tests.features.reactive_armor_hardener import make_eve_rah, make_eve_ship, setup_rah_basics


def test_complete_single(client, consts):
    # Check simulation of single RAH, with sim state loop detected before sim tick limit is broken.
    # Also check non-dogma attribute values.
    eve_basic_info = setup_rah_basics(client=client, consts=consts)
    eve_rah_id = make_eve_rah(client=client, basic_info=eve_basic_info, resos=(0.85, 0.85, 0.85, 0.85), shift_amount=6)
    eve_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, resos=(0.5, 0.65, 0.75, 0.9))
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit(rah_incoming_dps=(1, 1, 1, 1))
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah = api_fit.add_module(type_id=eve_rah_id, state=consts.ApiModuleState.active)
    # Verification
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].base == approx(0.85)
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].modified == approx(1)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].base == approx(0.85)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.925)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].base == approx(0.85)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.82)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].base == approx(0.85)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.655)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.5)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.60125)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.615)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.5895)


def test_complete_double_synced(client, consts):
    # Check simulation of dual RAHs, with sim state loop detected before sim tick limit is broken.
    eve_basic_info = setup_rah_basics(client=client, consts=consts)
    eve_rah_id = make_eve_rah(client=client, basic_info=eve_basic_info, resos=(0.85, 0.85, 0.85, 0.85), shift_amount=6)
    eve_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, resos=(0.5, 0.65, 0.75, 0.9))
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit(rah_incoming_dps=(1, 1, 1, 1))
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah1 = api_fit.add_module(type_id=eve_rah_id, state=consts.ApiModuleState.active)
    api_rah2 = api_fit.add_module(type_id=eve_rah_id, state=consts.ApiModuleState.active)
    # Verification
    api_rah1.update()
    assert api_rah1.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.97)
    assert api_rah1.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.88)
    assert api_rah1.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.805)
    assert api_rah1.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.745)
    api_rah2.update()
    assert api_rah2.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.97)
    assert api_rah2.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.88)
    assert api_rah2.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.805)
    assert api_rah2.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.745)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.4723543)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.5123436)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.5014274)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.5219)


def test_complete_double_unsynced(client, consts):
    # Check simulation of dual RAHs with different cycle times (realistic scenario with one RAH
    # being heated and the other one cold, as much as realistic scenario with 2 RAHs can be that
    # is), with sim state loop detected before sim tick limit is broken.
    eve_basic_info = setup_rah_basics(client=client, consts=consts)
    eve_rah_id = make_eve_rah(client=client, basic_info=eve_basic_info, resos=(0.85, 0.85, 0.85, 0.85), shift_amount=6)
    eve_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, resos=(0.5, 0.65, 0.75, 0.9))
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit(rah_incoming_dps=(1, 1, 1, 1))
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah1 = api_fit.add_module(type_id=eve_rah_id, state=consts.ApiModuleState.active)
    api_rah2 = api_fit.add_module(type_id=eve_rah_id, state=consts.ApiModuleState.overload)
    # Verification
    api_rah1.update()
    assert api_rah1.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.9752941)
    assert api_rah1.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.835)
    assert api_rah1.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.8297059)
    assert api_rah1.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.76)
    api_rah2.update()
    assert api_rah2.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.979)
    assert api_rah2.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.91)
    assert api_rah2.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.796)
    assert api_rah2.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.715)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.4787468)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.5002957)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.5086404)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.5092731)


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
    api_fit = api_sol.create_fit(rah_incoming_dps=(1, 1, 1, 1))
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah1 = api_fit.add_module(type_id=eve_rah1_id, state=consts.ApiModuleState.active)
    api_rah2 = api_fit.add_module(type_id=eve_rah2_id, state=consts.ApiModuleState.overload)
    # Verification
    api_rah1.update()
    assert api_rah1.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.9772912)
    assert api_rah1.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.8811238)
    assert api_rah1.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.7766642)
    assert api_rah1.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.7649209)
    api_rah2.update()
    assert api_rah2.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.9786538)
    assert api_rah2.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.8737115)
    assert api_rah2.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.8395288)
    assert api_rah2.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.7081058)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.4795801)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.5092371)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.5012579)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.5070881)


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
    api_fit = api_sol.create_fit(rah_incoming_dps=(1, 1, 1, 1))
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah1 = api_fit.add_module(type_id=eve_rah1_id, state=consts.ApiModuleState.active)
    api_rah2 = api_fit.add_module(type_id=eve_rah2_id, state=consts.ApiModuleState.overload)
    # Verification
    api_rah1.update()
    assert api_rah1.attrs[eve_basic_info.res_em_attr_id].modified == approx(1)
    assert api_rah1.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.925)
    assert api_rah1.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.82)
    assert api_rah1.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.655)
    api_rah2.update()
    assert api_rah2.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.631)
    assert api_rah2.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.9244231)
    assert api_rah2.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.8997308)
    assert api_rah2.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.9448462)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.3155)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.5617076)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.5614052)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.5612421)


def test_already_adapted_accuracy(client, consts):
    # Normally, RAH sim does lots of rounding in process. However, when RAH didn't need to adapt,
    # non-rounded base results should be set
    eve_basic_info = setup_rah_basics(client=client, consts=consts)
    eve_rah_id = make_eve_rah(client=client, basic_info=eve_basic_info, resos=(1, 1, 0.40000000001, 1), shift_amount=6)
    eve_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, resos=(0.5, 0.65, 0.75, 0.9))
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit(rah_incoming_dps=(0, 0, 1, 0))
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah = api_fit.add_module(type_id=eve_rah_id, state=consts.ApiModuleState.active)
    # Verification
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].modified == approx(1, accuracy=11)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].modified == approx(1, accuracy=11)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.40000000001, accuracy=11)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].modified == approx(1, accuracy=11)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.5)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.65)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.3)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.9)
