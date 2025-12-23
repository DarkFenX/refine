from fw import approx
from tests.features.reactive_armor_hardener import make_eve_rah, make_eve_ship, setup_rah_basics


def test_add_remove(client, consts):
    eve_basic_info = setup_rah_basics(client=client, consts=consts)
    eve_rah1_id = make_eve_rah(
        client=client,
        basic_info=eve_basic_info,
        resos=(0.85, 0.85, 0.85, 0.85),
        shift_amount=6,
        cycle_time=10000)
    eve_rah2_id = make_eve_rah(
        client=client,
        basic_info=eve_basic_info,
        resos=(0.7, 0.7, 0.7, 0.7),
        shift_amount=5,
        cycle_time=15000)
    eve_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, resos=(0.5, 0.65, 0.59, 0.51))
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit(rah_incoming_dps=(1, 1, 1, 1))
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah1 = api_fit.add_module(type_id=eve_rah1_id, state=consts.ApiModuleState.active)
    # Verification
    api_rah1.update()
    assert api_rah1.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.94)
    assert api_rah1.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.76)
    assert api_rah1.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.76)
    assert api_rah1.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.94)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.47)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.494)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.4484)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.4794)
    # Action
    api_rah2 = api_fit.add_module(type_id=eve_rah2_id, state=consts.ApiModuleState.active)
    # Verification - results for existing RAH should be reset too
    api_rah1.update()
    assert api_rah1.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.93)
    assert api_rah1.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.735)
    assert api_rah1.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.755)
    assert api_rah1.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.98)
    api_rah2.update()
    assert api_rah2.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.725)
    assert api_rah2.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.675)
    assert api_rah2.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.725)
    assert api_rah2.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.675)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.3404461)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.3376985)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.3366673)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.3382661)
    # Action
    api_rah1.remove()
    # Verification - results for remaining RAH should be reset too
    api_rah2.update()
    assert api_rah2.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.775)
    assert api_rah2.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.625)
    assert api_rah2.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.625)
    assert api_rah2.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.775)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.3875)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.40625)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.36875)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.39525)


def test_state_on_off(client, consts):
    eve_basic_info = setup_rah_basics(client=client, consts=consts)
    eve_rah1_id = make_eve_rah(
        client=client,
        basic_info=eve_basic_info,
        resos=(0.85000000001, 0.85000000001, 0.85000000001, 0.85000000001),
        shift_amount=6,
        cycle_time=10000)
    eve_rah2_id = make_eve_rah(
        client=client,
        basic_info=eve_basic_info,
        resos=(0.70000000001, 0.70000000001, 0.70000000001, 0.70000000001),
        shift_amount=5,
        cycle_time=15000)
    eve_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, resos=(0.5, 0.65, 0.59, 0.51))
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit(rah_incoming_dps=(1, 1, 1, 1))
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah1 = api_fit.add_module(type_id=eve_rah1_id, state=consts.ApiModuleState.active)
    api_rah2 = api_fit.add_module(type_id=eve_rah2_id, state=consts.ApiModuleState.online)
    # Verification
    api_rah1.update()
    assert api_rah1.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.94)
    assert api_rah1.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.76)
    assert api_rah1.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.76)
    assert api_rah1.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.94)
    api_rah2.update()
    assert api_rah2.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.70000000001, accuracy=11)
    assert api_rah2.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.70000000001, accuracy=11)
    assert api_rah2.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.70000000001, accuracy=11)
    assert api_rah2.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.70000000001, accuracy=11)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.47)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.494)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.4484)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.4794)
    # Action
    api_rah2.change_module(state=consts.ApiModuleState.active)
    # Verification - results for existing RAH should be reset too
    api_rah1.update()
    assert api_rah1.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.93)
    assert api_rah1.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.735)
    assert api_rah1.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.755)
    assert api_rah1.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.98)
    api_rah2.update()
    assert api_rah2.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.725)
    assert api_rah2.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.675)
    assert api_rah2.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.725)
    assert api_rah2.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.675)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.3404461)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.3376985)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.3366673)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.3382661)
    # Action
    api_rah1.change_module(state=consts.ApiModuleState.online)
    # Verification - results for remaining RAH should be reset too
    api_rah1.update()
    assert api_rah1.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.85000000001, accuracy=11)
    assert api_rah1.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.85000000001, accuracy=11)
    assert api_rah1.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.85000000001, accuracy=11)
    assert api_rah1.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.85000000001, accuracy=11)
    api_rah2.update()
    assert api_rah2.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.775)
    assert api_rah2.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.625)
    assert api_rah2.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.625)
    assert api_rah2.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.775)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.3875)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.40625)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.36875)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.39525)


def test_effect_mode_on_off(client, consts):
    eve_basic_info = setup_rah_basics(client=client, consts=consts)
    eve_rah1_id = make_eve_rah(
        client=client,
        basic_info=eve_basic_info,
        resos=(0.85000000001, 0.85000000001, 0.85000000001, 0.85000000001),
        shift_amount=6,
        cycle_time=10000)
    eve_rah2_id = make_eve_rah(
        client=client,
        basic_info=eve_basic_info,
        resos=(0.70000000001, 0.70000000001, 0.70000000001, 0.70000000001),
        shift_amount=5,
        cycle_time=15000)
    eve_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, resos=(0.5, 0.65, 0.59, 0.51))
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit(rah_incoming_dps=(1, 1, 1, 1))
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah1 = api_fit.add_module(type_id=eve_rah1_id, state=consts.ApiModuleState.active)
    api_rah2 = api_fit.add_module(type_id=eve_rah2_id, state=consts.ApiModuleState.online)
    # Verification
    api_rah1.update()
    assert api_rah1.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.94)
    assert api_rah1.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.76)
    assert api_rah1.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.76)
    assert api_rah1.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.94)
    api_rah2.update()
    assert api_rah2.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.70000000001, accuracy=11)
    assert api_rah2.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.70000000001, accuracy=11)
    assert api_rah2.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.70000000001, accuracy=11)
    assert api_rah2.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.70000000001, accuracy=11)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.47)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.494)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.4484)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.4794)
    # Action
    api_rah2.change_module(effect_modes={eve_basic_info.api_rah_effect_id: consts.ApiEffMode.force_run})
    # Verification - results for existing RAH should be reset too
    api_rah1.update()
    assert api_rah1.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.93)
    assert api_rah1.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.735)
    assert api_rah1.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.755)
    assert api_rah1.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.98)
    api_rah2.update()
    assert api_rah2.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.725)
    assert api_rah2.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.675)
    assert api_rah2.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.725)
    assert api_rah2.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.675)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.3404461)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.3376985)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.3366673)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.3382661)
    # Action
    api_rah1.change_module(effect_modes={eve_basic_info.api_rah_effect_id: consts.ApiEffMode.force_stop})
    # Verification - results for remaining RAH should be reset too
    api_rah1.update()
    assert api_rah1.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.85000000001, accuracy=11)
    assert api_rah1.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.85000000001, accuracy=11)
    assert api_rah1.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.85000000001, accuracy=11)
    assert api_rah1.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.85000000001, accuracy=11)
    api_rah2.update()
    assert api_rah2.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.775)
    assert api_rah2.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.625)
    assert api_rah2.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.625)
    assert api_rah2.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.775)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.3875)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.40625)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.36875)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.39525)


def test_attr_changed_res_em(client, consts):
    eve_basic_info = setup_rah_basics(client=client, consts=consts)
    eve_grp1_id = client.mk_eve_item_group()
    eve_grp2_id = client.mk_eve_item_group()
    eve_rah1_id = make_eve_rah(
        client=client,
        basic_info=eve_basic_info,
        grp_id=eve_grp1_id,
        resos=(0.85, 0.85, 0.85, 0.85),
        shift_amount=6)
    eve_rah2_id = make_eve_rah(
        client=client,
        basic_info=eve_basic_info,
        grp_id=eve_grp2_id,
        resos=(0.85, 0.85, 0.85, 0.85),
        shift_amount=6)
    eve_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, resos=(0.5, 0.65, 0.59, 0.51))
    eve_res_boost_attr_id = client.mk_eve_attr()
    eve_rig_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        loc=consts.EveModLoc.ship,
        grp=eve_grp1_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_res_boost_attr_id,
        affectee_attr_id=eve_basic_info.res_em_attr_id)
    eve_rig_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=[eve_rig_mod])
    eve_rig_id = client.mk_eve_item(attrs={eve_res_boost_attr_id: -30}, eff_ids=[eve_rig_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit(rah_incoming_dps=(1, 1, 1, 1))
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah1 = api_fit.add_module(type_id=eve_rah1_id, state=consts.ApiModuleState.active)
    api_rah2 = api_fit.add_module(type_id=eve_rah2_id, state=consts.ApiModuleState.active)
    # Verification
    api_rah1.update()
    assert api_rah1.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.94)
    assert api_rah1.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.76)
    assert api_rah1.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.82)
    assert api_rah1.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.88)
    api_rah2.update()
    assert api_rah2.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.94)
    assert api_rah2.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.76)
    assert api_rah2.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.82)
    assert api_rah2.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.88)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.4454908)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.3909571)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.4081136)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.4019927)
    # Action
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification - results for both RAHs should be reset, despite only one having its attr updated
    api_rah1.update()
    assert api_rah1.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.745)
    assert api_rah1.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.76)
    assert api_rah1.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.82)
    assert api_rah1.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.82)
    api_rah2.update()
    assert api_rah2.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.97)
    assert api_rah2.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.76)
    assert api_rah2.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.835)
    assert api_rah2.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.835)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.3627876)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.3909571)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.4144208)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.3582281)
    # Action
    api_rig.remove()
    # Verification
    api_rah1.update()
    assert api_rah1.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.94)
    assert api_rah1.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.76)
    assert api_rah1.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.82)
    assert api_rah1.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.88)
    api_rah2.update()
    assert api_rah2.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.94)
    assert api_rah2.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.76)
    assert api_rah2.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.82)
    assert api_rah2.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.88)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.4454908)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.3909571)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.4081136)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.4019927)


def test_attr_changed_res_therm(client, consts):
    eve_basic_info = setup_rah_basics(client=client, consts=consts)
    eve_grp1_id = client.mk_eve_item_group()
    eve_grp2_id = client.mk_eve_item_group()
    eve_rah1_id = make_eve_rah(
        client=client,
        basic_info=eve_basic_info,
        grp_id=eve_grp1_id,
        resos=(0.85, 0.85, 0.85, 0.85),
        shift_amount=6)
    eve_rah2_id = make_eve_rah(
        client=client,
        basic_info=eve_basic_info,
        grp_id=eve_grp2_id,
        resos=(0.85, 0.85, 0.85, 0.85),
        shift_amount=6)
    eve_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, resos=(0.5, 0.65, 0.59, 0.51))
    eve_res_boost_attr_id = client.mk_eve_attr()
    eve_rig_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        loc=consts.EveModLoc.ship,
        grp=eve_grp1_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_res_boost_attr_id,
        affectee_attr_id=eve_basic_info.res_therm_attr_id)
    eve_rig_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=[eve_rig_mod])
    eve_rig_id = client.mk_eve_item(attrs={eve_res_boost_attr_id: -30}, eff_ids=[eve_rig_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit(rah_incoming_dps=(1, 1, 1, 1))
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah1 = api_fit.add_module(type_id=eve_rah1_id, state=consts.ApiModuleState.active)
    api_rah2 = api_fit.add_module(type_id=eve_rah2_id, state=consts.ApiModuleState.active)
    # Verification
    api_rah1.update()
    assert api_rah1.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.94)
    assert api_rah1.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.76)
    assert api_rah1.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.82)
    assert api_rah1.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.88)
    api_rah2.update()
    assert api_rah2.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.94)
    assert api_rah2.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.76)
    assert api_rah2.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.82)
    assert api_rah2.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.88)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.4454908)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.3909571)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.4081136)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.4019927)
    # Action
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification - results for both RAHs should be reset, despite only one having its attr updated
    api_rah1.update()
    assert api_rah1.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.88)
    assert api_rah1.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.625)
    assert api_rah1.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.82)
    assert api_rah1.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.82)
    api_rah2.update()
    assert api_rah2.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.88)
    assert api_rah2.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.88)
    assert api_rah2.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.82)
    assert api_rah2.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.82)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.3941105)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.3638804)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.4081136)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.3527761)
    # Action
    api_rig.remove()
    # Verification
    api_rah1.update()
    assert api_rah1.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.94)
    assert api_rah1.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.76)
    assert api_rah1.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.82)
    assert api_rah1.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.88)
    api_rah2.update()
    assert api_rah2.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.94)
    assert api_rah2.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.76)
    assert api_rah2.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.82)
    assert api_rah2.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.88)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.4454908)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.3909571)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.4081136)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.4019927)


def test_attr_changed_res_kin(client, consts):
    eve_basic_info = setup_rah_basics(client=client, consts=consts)
    eve_grp1_id = client.mk_eve_item_group()
    eve_grp2_id = client.mk_eve_item_group()
    eve_rah1_id = make_eve_rah(
        client=client,
        basic_info=eve_basic_info,
        grp_id=eve_grp1_id,
        resos=(0.85, 0.85, 0.85, 0.85),
        shift_amount=6)
    eve_rah2_id = make_eve_rah(
        client=client,
        basic_info=eve_basic_info,
        grp_id=eve_grp2_id,
        resos=(0.85, 0.85, 0.85, 0.85),
        shift_amount=6)
    eve_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, resos=(0.5, 0.65, 0.59, 0.51))
    eve_res_boost_attr_id = client.mk_eve_attr()
    eve_rig_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        loc=consts.EveModLoc.ship,
        grp=eve_grp1_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_res_boost_attr_id,
        affectee_attr_id=eve_basic_info.res_kin_attr_id)
    eve_rig_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=[eve_rig_mod])
    eve_rig_id = client.mk_eve_item(attrs={eve_res_boost_attr_id: -30}, eff_ids=[eve_rig_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit(rah_incoming_dps=(1, 1, 1, 1))
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah1 = api_fit.add_module(type_id=eve_rah1_id, state=consts.ApiModuleState.active)
    api_rah2 = api_fit.add_module(type_id=eve_rah2_id, state=consts.ApiModuleState.active)
    # Verification
    api_rah1.update()
    assert api_rah1.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.94)
    assert api_rah1.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.76)
    assert api_rah1.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.82)
    assert api_rah1.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.88)
    api_rah2.update()
    assert api_rah2.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.94)
    assert api_rah2.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.76)
    assert api_rah2.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.82)
    assert api_rah2.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.88)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.4454908)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.3909571)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.4081136)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.4019927)
    # Action
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification - results for both RAHs should be reset, despite only one having its attr updated
    api_rah1.update()
    assert api_rah1.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.88)
    assert api_rah1.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.76)
    assert api_rah1.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.685)
    assert api_rah1.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.82)
    api_rah2.update()
    assert api_rah2.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.88)
    assert api_rah2.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.76)
    assert api_rah2.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.94)
    assert api_rah2.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.82)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.3941105)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.3909571)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.3830747)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.3527761)
    # Action
    api_rig.remove()
    # Verification
    api_rah1.update()
    assert api_rah1.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.94)
    assert api_rah1.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.76)
    assert api_rah1.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.82)
    assert api_rah1.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.88)
    api_rah2.update()
    assert api_rah2.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.94)
    assert api_rah2.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.76)
    assert api_rah2.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.82)
    assert api_rah2.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.88)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.4454908)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.3909571)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.4081136)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.4019927)


def test_attr_changed_res_expl(client, consts):
    eve_basic_info = setup_rah_basics(client=client, consts=consts)
    eve_grp1_id = client.mk_eve_item_group()
    eve_grp2_id = client.mk_eve_item_group()
    eve_rah1_id = make_eve_rah(
        client=client,
        basic_info=eve_basic_info,
        grp_id=eve_grp1_id,
        resos=(0.85, 0.85, 0.85, 0.85),
        shift_amount=6)
    eve_rah2_id = make_eve_rah(
        client=client,
        basic_info=eve_basic_info,
        grp_id=eve_grp2_id,
        resos=(0.85, 0.85, 0.85, 0.85),
        shift_amount=6)
    eve_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, resos=(0.5, 0.65, 0.59, 0.51))
    eve_res_boost_attr_id = client.mk_eve_attr()
    eve_rig_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        loc=consts.EveModLoc.ship,
        grp=eve_grp1_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_res_boost_attr_id,
        affectee_attr_id=eve_basic_info.res_expl_attr_id)
    eve_rig_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=[eve_rig_mod])
    eve_rig_id = client.mk_eve_item(attrs={eve_res_boost_attr_id: -30}, eff_ids=[eve_rig_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit(rah_incoming_dps=(1, 1, 1, 1))
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah1 = api_fit.add_module(type_id=eve_rah1_id, state=consts.ApiModuleState.active)
    api_rah2 = api_fit.add_module(type_id=eve_rah2_id, state=consts.ApiModuleState.active)
    # Verification
    api_rah1.update()
    assert api_rah1.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.94)
    assert api_rah1.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.76)
    assert api_rah1.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.82)
    assert api_rah1.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.88)
    api_rah2.update()
    assert api_rah2.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.94)
    assert api_rah2.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.76)
    assert api_rah2.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.82)
    assert api_rah2.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.88)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.4454908)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.3909571)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.4081136)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.4019927)
    # Action
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification - results for both RAHs should be reset, despite only one having its attr updated
    api_rah1.update()
    assert api_rah1.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.82)
    assert api_rah1.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.76)
    assert api_rah1.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.82)
    assert api_rah1.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.745)
    api_rah2.update()
    assert api_rah2.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.835)
    assert api_rah2.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.76)
    assert api_rah2.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.835)
    assert api_rah2.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.97)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.351204)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.3909571)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.4144208)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.3700433)
    # Action
    api_rig.remove()
    # Verification
    api_rah1.update()
    assert api_rah1.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.94)
    assert api_rah1.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.76)
    assert api_rah1.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.82)
    assert api_rah1.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.88)
    api_rah2.update()
    assert api_rah2.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.94)
    assert api_rah2.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.76)
    assert api_rah2.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.82)
    assert api_rah2.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.88)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.4454908)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.3909571)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.4081136)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.4019927)


def test_attr_changed_shift_amount(client, consts):
    eve_basic_info = setup_rah_basics(client=client, consts=consts)
    eve_grp1_id = client.mk_eve_item_group()
    eve_grp2_id = client.mk_eve_item_group()
    eve_rah1_id = make_eve_rah(
        client=client,
        basic_info=eve_basic_info,
        grp_id=eve_grp1_id,
        resos=(0.85, 0.85, 0.85, 0.85),
        shift_amount=6)
    eve_rah2_id = make_eve_rah(
        client=client,
        basic_info=eve_basic_info,
        grp_id=eve_grp2_id,
        resos=(0.85, 0.85, 0.85, 0.85),
        shift_amount=6)
    eve_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, resos=(0.5, 0.65, 0.59, 0.51))
    eve_shift_boost_attr_id = client.mk_eve_attr()
    eve_rig_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        loc=consts.EveModLoc.ship,
        grp=eve_grp1_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_shift_boost_attr_id,
        affectee_attr_id=eve_basic_info.res_shift_attr_id)
    eve_rig_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=[eve_rig_mod])
    eve_rig_id = client.mk_eve_item(attrs={eve_shift_boost_attr_id: 100}, eff_ids=[eve_rig_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit(rah_incoming_dps=(1, 1, 1, 1))
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah1 = api_fit.add_module(type_id=eve_rah1_id, state=consts.ApiModuleState.active)
    api_rah2 = api_fit.add_module(type_id=eve_rah2_id, state=consts.ApiModuleState.active)
    # Verification
    api_rah1.update()
    assert api_rah1.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.94)
    assert api_rah1.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.76)
    assert api_rah1.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.82)
    assert api_rah1.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.88)
    api_rah2.update()
    assert api_rah2.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.94)
    assert api_rah2.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.76)
    assert api_rah2.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.82)
    assert api_rah2.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.88)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.4454908)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.3909571)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.4081136)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.4019927)
    # Action
    api_fit.add_rig(type_id=eve_rig_id)
    # Verification - results for both RAHs should be reset, despite only one having its attr updated
    api_rah1.update()
    assert api_rah1.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.91)
    assert api_rah1.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.79)
    assert api_rah1.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.79)
    assert api_rah1.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.91)
    api_rah2.update()
    assert api_rah2.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.88)
    assert api_rah2.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.82)
    assert api_rah2.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.82)
    assert api_rah2.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.88)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.4055828)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.4331672)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.3931826)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.4136945)


def test_attr_changed_cycle_time(client, consts):
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    eve_d1_cycle_time_attr_id = client.alloc_attr_id(datas=[eve_d1, eve_d2])
    eve_d2_cycle_time_attr_id = client.alloc_attr_id(datas=[eve_d1, eve_d2])
    eve_d1_basic_info = setup_rah_basics(
        client=client,
        consts=consts,
        datas=[eve_d1],
        attr_cycle_time=eve_d1_cycle_time_attr_id)
    eve_d2_basic_info = setup_rah_basics(
        client=client,
        consts=consts,
        datas=[eve_d2],
        attr_cycle_time=eve_d2_cycle_time_attr_id)
    eve_rah_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    make_eve_rah(
        client=client,
        datas=[eve_d1],
        basic_info=eve_d1_basic_info,
        id_=eve_rah_id,
        resos=(0.85, 0.85, 0.85, 0.85),
        shift_amount=6)
    make_eve_rah(
        client=client,
        datas=[eve_d2],
        basic_info=eve_d2_basic_info,
        id_=eve_rah_id,
        resos=(0.85, 0.85, 0.85, 0.85),
        shift_amount=6)
    eve_ship_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    make_eve_ship(
        client=client,
        datas=[eve_d1],
        basic_info=eve_d1_basic_info,
        id_=eve_ship_id,
        resos=(0.5, 0.65, 0.59, 0.51))
    make_eve_ship(
        client=client,
        datas=[eve_d2],
        basic_info=eve_d2_basic_info,
        id_=eve_ship_id,
        resos=(0.5, 0.65, 0.75, 0.9))
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1)
    api_fit = api_sol.create_fit(rah_incoming_dps=(1, 1, 1, 1))
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah1 = api_fit.add_module(type_id=eve_rah_id, state=consts.ApiModuleState.active)
    api_rah2 = api_fit.add_module(type_id=eve_rah_id, state=consts.ApiModuleState.active)
    # Verification
    api_rah1.update()
    assert api_rah1.attrs[eve_d1_basic_info.res_em_attr_id].modified == approx(0.94)
    assert api_rah1.attrs[eve_d1_basic_info.res_therm_attr_id].modified == approx(0.76)
    assert api_rah1.attrs[eve_d1_basic_info.res_kin_attr_id].modified == approx(0.82)
    assert api_rah1.attrs[eve_d1_basic_info.res_expl_attr_id].modified == approx(0.88)
    api_rah2.update()
    assert api_rah2.attrs[eve_d1_basic_info.res_em_attr_id].modified == approx(0.94)
    assert api_rah2.attrs[eve_d1_basic_info.res_therm_attr_id].modified == approx(0.76)
    assert api_rah2.attrs[eve_d1_basic_info.res_kin_attr_id].modified == approx(0.82)
    assert api_rah2.attrs[eve_d1_basic_info.res_expl_attr_id].modified == approx(0.88)
    api_ship.update()
    assert api_ship.attrs[eve_d1_basic_info.res_em_attr_id].modified == approx(0.4454908)
    assert api_ship.attrs[eve_d1_basic_info.res_therm_attr_id].modified == approx(0.3909571)
    assert api_ship.attrs[eve_d1_basic_info.res_kin_attr_id].modified == approx(0.4081136)
    assert api_ship.attrs[eve_d1_basic_info.res_expl_attr_id].modified == approx(0.4019927)
    # Action
    api_rah2.change_module(state=consts.ApiModuleState.overload)
    # Verification - results for both RAHs should be reset, despite only one having its attr updated
    api_rah1.update()
    assert api_rah1.attrs[eve_d1_basic_info.res_em_attr_id].modified == approx(0.9752941)
    assert api_rah1.attrs[eve_d1_basic_info.res_therm_attr_id].modified == approx(0.7961765)
    assert api_rah1.attrs[eve_d1_basic_info.res_kin_attr_id].modified == approx(0.775)
    assert api_rah1.attrs[eve_d1_basic_info.res_expl_attr_id].modified == approx(0.8535294)
    api_rah2.update()
    assert api_rah2.attrs[eve_d1_basic_info.res_em_attr_id].modified == approx(0.817)
    assert api_rah2.attrs[eve_d1_basic_info.res_therm_attr_id].modified == approx(0.748)
    assert api_rah2.attrs[eve_d1_basic_info.res_kin_attr_id].modified == approx(0.895)
    assert api_rah2.attrs[eve_d1_basic_info.res_expl_attr_id].modified == approx(0.94)
    api_ship.update()
    assert api_ship.attrs[eve_d1_basic_info.res_em_attr_id].modified == approx(0.3997285)
    assert api_ship.attrs[eve_d1_basic_info.res_therm_attr_id].modified == approx(0.4000711)
    assert api_ship.attrs[eve_d1_basic_info.res_kin_attr_id].modified == approx(0.4155225)
    assert api_ship.attrs[eve_d1_basic_info.res_expl_attr_id].modified == approx(0.4126003)
    # Action - unlike for other attribute change tests, here we do source switch. Since duration
    # attribute is not hardcoded into simulator, it can change between sources. Sim stores attribute
    # ID for optimization reasons, and has to update it during source switch.
    api_sol.change_src(data=eve_d2)
    # Verification
    api_rah1.update()
    assert api_rah1.attrs[eve_d2_basic_info.res_em_attr_id].modified == approx(0.9752941)
    assert api_rah1.attrs[eve_d2_basic_info.res_therm_attr_id].modified == approx(0.835)
    assert api_rah1.attrs[eve_d2_basic_info.res_kin_attr_id].modified == approx(0.8297059)
    assert api_rah1.attrs[eve_d2_basic_info.res_expl_attr_id].modified == approx(0.76)
    api_rah2.update()
    assert api_rah2.attrs[eve_d2_basic_info.res_em_attr_id].modified == approx(0.979)
    assert api_rah2.attrs[eve_d2_basic_info.res_therm_attr_id].modified == approx(0.91)
    assert api_rah2.attrs[eve_d2_basic_info.res_kin_attr_id].modified == approx(0.796)
    assert api_rah2.attrs[eve_d2_basic_info.res_expl_attr_id].modified == approx(0.715)
    api_ship.update()
    assert api_ship.attrs[eve_d2_basic_info.res_em_attr_id].modified == approx(0.4787468)
    assert api_ship.attrs[eve_d2_basic_info.res_therm_attr_id].modified == approx(0.5002957)
    assert api_ship.attrs[eve_d2_basic_info.res_kin_attr_id].modified == approx(0.5086404)
    assert api_ship.attrs[eve_d2_basic_info.res_expl_attr_id].modified == approx(0.5092731)
    # Action
    api_rah1.change_module(state=consts.ApiModuleState.overload)
    # Verification - on second source it should work as well, if attribute reference was updated
    # during source switch
    api_rah1.update()
    assert api_rah1.attrs[eve_d2_basic_info.res_em_attr_id].modified == approx(0.97)
    assert api_rah1.attrs[eve_d2_basic_info.res_therm_attr_id].modified == approx(0.88)
    assert api_rah1.attrs[eve_d2_basic_info.res_kin_attr_id].modified == approx(0.805)
    assert api_rah1.attrs[eve_d2_basic_info.res_expl_attr_id].modified == approx(0.745)
    api_rah2.update()
    assert api_rah2.attrs[eve_d2_basic_info.res_em_attr_id].modified == approx(0.97)
    assert api_rah2.attrs[eve_d2_basic_info.res_therm_attr_id].modified == approx(0.88)
    assert api_rah2.attrs[eve_d2_basic_info.res_kin_attr_id].modified == approx(0.805)
    assert api_rah2.attrs[eve_d2_basic_info.res_expl_attr_id].modified == approx(0.745)
    api_ship.update()
    assert api_ship.attrs[eve_d2_basic_info.res_em_attr_id].modified == approx(0.4723543)
    assert api_ship.attrs[eve_d2_basic_info.res_therm_attr_id].modified == approx(0.5123436)
    assert api_ship.attrs[eve_d2_basic_info.res_kin_attr_id].modified == approx(0.5014274)
    assert api_ship.attrs[eve_d2_basic_info.res_expl_attr_id].modified == approx(0.5219)
