from fw import approx
from tests.features.reactive_armor_hardener import make_eve_rah, make_eve_ship, setup_rah_basics


def test_rah_to_rah(client, consts):
    eve_basic_info = setup_rah_basics(client=client, consts=consts)
    eve_rah_id = make_eve_rah(client=client, basic_info=eve_basic_info, resos=(0.85, 0.85, 0.85, 0.85), shift_amount=6)
    eve_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, resos=(0.5, 0.65, 0.75, 0.9))
    eve_implant_attr_id = client.mk_eve_attr()
    eve_implant_mods = [
        client.mk_eve_effect_mod(
            func=consts.EveModFunc.loc,
            loc=consts.EveModLoc.ship,
            op=consts.EveModOp.post_percent,
            affector_attr_id=eve_implant_attr_id,
            affectee_attr_id=attr_id)
        for attr_id in (
            eve_basic_info.res_em_attr_id,
            eve_basic_info.res_therm_attr_id,
            eve_basic_info.res_kin_attr_id,
            eve_basic_info.res_expl_attr_id)]
    eve_implant_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=eve_implant_mods)
    eve_implant_id = client.mk_eve_item(attrs={eve_implant_attr_id: -10}, eff_ids=[eve_implant_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit(rah_incoming_dps=(1, 1, 0, 0))
    api_fit.add_implant(type_id=eve_implant_id)
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah = api_fit.add_module(type_id=eve_rah_id, state=consts.ApiModuleState.active)
    # Verification
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.53)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.53)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].modified == approx(1)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].modified == approx(1)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.265)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.3445)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.75)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.9)
    # Action
    api_fit.change(rah_incoming_dps=(0, 0, 1, 1))
    # Verification
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].modified == approx(1)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].modified == approx(1)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.53)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.53)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.5)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.65)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.3975)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.477)


def test_rah_to_rah_no_dmg(client, consts):
    eve_basic_info = setup_rah_basics(client=client, consts=consts)
    eve_rah_id = make_eve_rah(
        client=client,
        basic_info=eve_basic_info,
        resos=(0.85000000001, 0.85000000001, 0.85000000001, 0.85000000001),
        shift_amount=6)
    eve_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, resos=(0.5, 0.65, 0.75, 0.9))
    eve_implant_attr_id = client.mk_eve_attr()
    eve_implant_mods = [
        client.mk_eve_effect_mod(
            func=consts.EveModFunc.loc,
            loc=consts.EveModLoc.ship,
            op=consts.EveModOp.post_percent,
            affector_attr_id=eve_implant_attr_id,
            affectee_attr_id=attr_id)
        for attr_id in (
            eve_basic_info.res_em_attr_id,
            eve_basic_info.res_therm_attr_id,
            eve_basic_info.res_kin_attr_id,
            eve_basic_info.res_expl_attr_id)]
    eve_implant_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=eve_implant_mods)
    eve_implant_id = client.mk_eve_item(attrs={eve_implant_attr_id: -10}, eff_ids=[eve_implant_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit(rah_incoming_dps=(0, 0, 0, 0))
    api_implant = api_fit.add_implant(type_id=eve_implant_id)
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah = api_fit.add_module(type_id=eve_rah_id, state=consts.ApiModuleState.active)
    # Verification - 0 total damage is a special case, it means "do not adapt" in case of RAH.
    # Modified unadapted attributes should be returned in this case.
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].base == approx(0.85000000001, accuracy=11)
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.765)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].base == approx(0.85000000001, accuracy=11)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.765)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].base == approx(0.85000000001, accuracy=11)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.765)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].base == approx(0.85000000001, accuracy=11)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.765)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.3825)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.49725)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.57375)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.6885)
    # Action
    api_fit.change(rah_incoming_dps=(1, 1, 0, 0))
    # Verification
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.53)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.53)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].modified == approx(1)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].modified == approx(1)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.265)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.3445)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.75)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.9)
    # Action
    api_fit.change(rah_incoming_dps=(0, 0, 0, 0, (0, 0)))
    # Verification - same as 0 case before, but with breacher info
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].base == approx(0.85000000001, accuracy=11)
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.765)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].base == approx(0.85000000001, accuracy=11)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.765)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].base == approx(0.85000000001, accuracy=11)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.765)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].base == approx(0.85000000001, accuracy=11)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.765)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.3825)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.49725)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.57375)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.6885)
    # Action
    api_implant.remove()
    # Verification
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].base == approx(0.85000000001, accuracy=11)
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.85000000001, accuracy=11)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].base == approx(0.85000000001, accuracy=11)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.85000000001, accuracy=11)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].base == approx(0.85000000001, accuracy=11)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.85000000001, accuracy=11)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].base == approx(0.85000000001, accuracy=11)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.85000000001, accuracy=11)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.425)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.5525)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.6375)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.765)


def test_default_to_default(client, consts):
    eve_basic_info = setup_rah_basics(client=client, consts=consts)
    eve_rah_id = make_eve_rah(client=client, basic_info=eve_basic_info, resos=(0.85, 0.85, 0.85, 0.85), shift_amount=6)
    eve_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, resos=(0.5, 0.65, 0.75, 0.9))
    client.create_sources()
    api_sol = client.create_sol(default_incoming_dps=(1, 1, 0, 0))
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah = api_fit.add_module(type_id=eve_rah_id, state=consts.ApiModuleState.active)
    # Verification
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.7)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.7)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].modified == approx(1)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].modified == approx(1)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.35)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.455)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.75)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.9)
    # Action
    api_sol.change(default_incoming_dps=(0, 0, 1, 1))
    # Verification
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].modified == approx(1)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].modified == approx(1)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.7)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.7)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.5)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.65)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.525)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.63)


def test_rah_to_default(client, consts):
    eve_basic_info = setup_rah_basics(client=client, consts=consts)
    eve_rah_id = make_eve_rah(client=client, basic_info=eve_basic_info, resos=(0.85, 0.85, 0.85, 0.85), shift_amount=6)
    eve_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, resos=(0.5, 0.65, 0.75, 0.9))
    client.create_sources()
    api_sol = client.create_sol(default_incoming_dps=(0, 0, 1, 1))
    api_fit = api_sol.create_fit(rah_incoming_dps=(1, 1, 0, 0))
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah = api_fit.add_module(type_id=eve_rah_id, state=consts.ApiModuleState.active)
    # Verification
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.7)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.7)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].modified == approx(1)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].modified == approx(1)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.35)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.455)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.75)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.9)
    # Action
    api_fit.change(rah_incoming_dps=None)
    # Verification
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].modified == approx(1)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].modified == approx(1)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.7)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.7)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.5)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.65)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.525)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.63)


def test_default_to_rah(client, consts):
    eve_basic_info = setup_rah_basics(client=client, consts=consts)
    eve_rah_id = make_eve_rah(client=client, basic_info=eve_basic_info, resos=(0.85, 0.85, 0.85, 0.85), shift_amount=6)
    eve_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, resos=(0.5, 0.65, 0.75, 0.9))
    client.create_sources()
    api_sol = client.create_sol(default_incoming_dps=(1, 1, 0, 0))
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah = api_fit.add_module(type_id=eve_rah_id, state=consts.ApiModuleState.active)
    # Verification
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.7)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.7)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].modified == approx(1)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].modified == approx(1)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.35)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.455)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.75)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.9)
    # Action
    api_fit.change(rah_incoming_dps=(0, 0, 1, 1))
    # Verification
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].modified == approx(1)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].modified == approx(1)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.7)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.7)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].modified == approx(0.5)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].modified == approx(0.65)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].modified == approx(0.525)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].modified == approx(0.63)
