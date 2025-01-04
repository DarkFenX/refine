from tests import approx
from tests.features.reactive_armor_hardener import make_eve_rah, make_eve_ship, setup_rah_basics


def test_res_changed_em(client, consts):
    eve_basic_info = setup_rah_basics(client=client, consts=consts)
    eve_grp1_id = client.mk_eve_item_group()
    eve_grp2_id = client.mk_eve_item_group()
    eve_rah1_id = make_eve_rah(
        client=client,
        basic_info=eve_basic_info,
        resos=(0.85, 0.85, 0.85, 0.85),
        shift_amount=6,
        grp_id=eve_grp1_id)
    eve_rah2_id = make_eve_rah(
        client=client,
        basic_info=eve_basic_info,
        resos=(0.85, 0.85, 0.85, 0.85),
        shift_amount=6,
        grp_id=eve_grp2_id)
    eve_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, resos=(0.5, 0.65, 0.59, 0.51))
    eve_res_boost_attr_id = client.mk_eve_attr()
    eve_rig_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        dom=consts.EveModDom.ship,
        grp=eve_grp1_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_res_boost_attr_id,
        affectee_attr_id=eve_basic_info.res_em_attr_id)
    eve_rig_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=[eve_rig_mod])
    eve_rig_id = client.mk_eve_item(attrs={eve_res_boost_attr_id: -30}, eff_ids=[eve_rig_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit(rah_incoming_dmg=(1, 1, 1, 1))
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah1 = api_fit.add_mod(type_id=eve_rah1_id, state=consts.ApiState.active)
    api_rah2 = api_fit.add_mod(type_id=eve_rah2_id, state=consts.ApiState.active)
    # Verification
    api_rah1.update()
    assert api_rah1.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.94)
    assert api_rah1.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.76)
    assert api_rah1.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.82)
    assert api_rah1.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.88)
    api_rah2.update()
    assert api_rah2.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.94)
    assert api_rah2.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.76)
    assert api_rah2.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.82)
    assert api_rah2.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.88)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.445491)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.390957)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.4081136)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.401993)
    # Action
    api_fit.add_rig(type_id=eve_rig_id)
    # Verification - results for both RAHs should be reset, despite only one having its attr updated
    api_rah1.update()
    assert api_rah1.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.745)
    assert api_rah1.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.76)
    assert api_rah1.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.82)
    assert api_rah1.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.82)
    api_rah2.update()
    assert api_rah2.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.97)
    assert api_rah2.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.76)
    assert api_rah2.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.835)
    assert api_rah2.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.835)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.3627876)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.390957)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.414421)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.358228)
