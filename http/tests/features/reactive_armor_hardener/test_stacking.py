from tests import approx
from tests.features.reactive_armor_hardener import make_eve_rah, make_eve_ship, setup_rah_basics


def test_regular_resist(client, consts):
    # Check that RAH is stacking penalized against usual resistance modules, which use PostPercent
    # operator to modify resonances
    eve_basic_info = setup_rah_basics(client=client, consts=consts)
    eve_rah_id = make_eve_rah(client=client, basic_info=eve_basic_info, resos=(0.85, 0.85, 0.85, 0.85), shift_amount=6)
    eve_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, resos=(0.5, 0.65, 0.75, 0.9))
    eve_dc_attr_id = client.mk_eve_attr()
    eve_dc_mods = [
        client.mk_eve_effect_mod(
            func=consts.EveModFunc.item,
            dom=consts.EveModDom.ship,
            op=consts.EveModOp.post_percent,
            affector_attr_id=eve_dc_attr_id,
            affectee_attr_id=attr_id)
        for attr_id in (
            eve_basic_info.res_em_attr_id,
            eve_basic_info.res_therm_attr_id,
            eve_basic_info.res_kin_attr_id,
            eve_basic_info.res_expl_attr_id)]
    eve_dc_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=eve_dc_mods)
    eve_dc_id = client.mk_eve_item(attrs={eve_dc_attr_id: -15}, eff_ids=[eve_dc_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit(rah_incoming_dmg=(1, 1, 1, 1))
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah = api_fit.add_mod(type_id=eve_rah_id, state=consts.ApiState.active)
    api_fit.add_mod(type_id=eve_dc_id)
    # Verification
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].dogma == approx(1)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.925)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.82)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.655)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.425)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.5110625)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.52275)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.501075)


def test_damage_control(client, consts):
    # Check that RAH is stacking penalized against DC (which uses PreMul operator to modify
    # resonances)
    eve_basic_info = setup_rah_basics(client=client, consts=consts)
    eve_rah_id = make_eve_rah(client=client, basic_info=eve_basic_info, resos=(0.85, 0.85, 0.85, 0.85), shift_amount=6)
    eve_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, resos=(0.5, 0.65, 0.75, 0.9))
    eve_dc_attr_id = client.mk_eve_attr()
    eve_dc_mods = [
        client.mk_eve_effect_mod(
            func=consts.EveModFunc.item,
            dom=consts.EveModDom.ship,
            op=consts.EveModOp.pre_mul,
            affector_attr_id=eve_dc_attr_id,
            affectee_attr_id=attr_id)
        for attr_id in (
            eve_basic_info.res_em_attr_id,
            eve_basic_info.res_therm_attr_id,
            eve_basic_info.res_kin_attr_id,
            eve_basic_info.res_expl_attr_id)]
    eve_dc_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=eve_dc_mods)
    eve_dc_id = client.mk_eve_item(attrs={eve_dc_attr_id: 0.85}, eff_ids=[eve_dc_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit(rah_incoming_dmg=(1, 1, 1, 1))
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah = api_fit.add_mod(type_id=eve_rah_id, state=consts.ApiState.active)
    api_fit.add_mod(type_id=eve_dc_id)
    # Verification
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].dogma == approx(1)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.925)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.82)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.655)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.425)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.5164858)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.5348237)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.5126481)
