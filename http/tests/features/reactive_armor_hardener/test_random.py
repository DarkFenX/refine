from tests import approx, check_no_field


def test_random(client, consts):
    eve_res_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_max_dmg_resonance, def_val=1)
    eve_res_em_attr_id = client.mk_eve_attr(
        id_=consts.EveAttr.armor_em_dmg_resonance,
        max_attr_id=eve_res_max_attr_id)
    eve_res_therm_attr_id = client.mk_eve_attr(
        id_=consts.EveAttr.armor_therm_dmg_resonance,
        max_attr_id=eve_res_max_attr_id)
    eve_res_kin_attr_id = client.mk_eve_attr(
        id_=consts.EveAttr.armor_kin_dmg_resonance,
        max_attr_id=eve_res_max_attr_id)
    eve_res_expl_attr_id = client.mk_eve_attr(
        id_=consts.EveAttr.armor_expl_dmg_resonance,
        max_attr_id=eve_res_max_attr_id)
    eve_cycle_time_attr_id = client.mk_eve_attr(id_=consts.EveAttr.duration)
    eve_res_shift_attr_id = client.mk_eve_attr(id_=consts.EveAttr.resist_shift_amount)
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.adaptive_armor_hardener,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_rah_id = client.mk_eve_item(
        attrs={
            eve_res_em_attr_id: 0.85,
            eve_res_therm_attr_id: 0.85,
            eve_res_kin_attr_id: 0.85,
            eve_res_expl_attr_id: 0.85,
            eve_cycle_time_attr_id: 10000,
            eve_res_shift_attr_id: 6},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={
        eve_res_em_attr_id: 0.5,
        eve_res_therm_attr_id: 0.65,
        eve_res_kin_attr_id: 0.75,
        eve_res_expl_attr_id: 0.9})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah = api_fit.add_mod(type_id=eve_rah_id, state=consts.ApiState.active)
    # Verification
    api_ship.update()
    assert api_ship.attrs[eve_res_em_attr_id].dogma == approx(0.5)
    assert api_ship.attrs[eve_res_therm_attr_id].dogma == approx(0.60125)
    assert api_ship.attrs[eve_res_kin_attr_id].dogma == approx(0.615)
    assert api_ship.attrs[eve_res_expl_attr_id].dogma == approx(0.5895)
    api_rah.update()
    assert api_rah.attrs[eve_res_em_attr_id].dogma == approx(1.0)
    assert api_rah.attrs[eve_res_therm_attr_id].dogma == approx(0.925)
    assert api_rah.attrs[eve_res_kin_attr_id].dogma == approx(0.82)
    assert api_rah.attrs[eve_res_expl_attr_id].dogma == approx(0.655)
    api_sol.change_default_incoming_dmg(dmg_profile=(1, 1, 0, 0))
    api_sol.update()
    api_ship.update()
    assert api_ship.attrs[eve_res_em_attr_id].dogma == approx(0.35)
    assert api_ship.attrs[eve_res_therm_attr_id].dogma == approx(0.455)
    assert api_ship.attrs[eve_res_kin_attr_id].dogma == approx(0.75)
    assert api_ship.attrs[eve_res_expl_attr_id].dogma == approx(0.9)
    api_rah.update()
    assert api_rah.attrs[eve_res_em_attr_id].dogma == approx(0.7)
    assert api_rah.attrs[eve_res_therm_attr_id].dogma == approx(0.7)
    assert api_rah.attrs[eve_res_kin_attr_id].dogma == approx(1)
    assert api_rah.attrs[eve_res_expl_attr_id].dogma == approx(1)
