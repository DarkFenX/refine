from tests import approx


def test_wdfg(client, consts):
    eve_range_attr = client.mk_eve_attr(id_=consts.EveAttr.warp_scramble_range, def_val=0)
    eve_range_hidden_attr = client.mk_eve_attr(id_=consts.EveAttr.max_range_hidden, def_val=0)
    eve_range_bonus_attr = client.mk_eve_attr(id_=consts.EveAttr.warp_scramble_range_bonus, def_val=0)
    eve_cap_attr = client.mk_eve_attr(id_=consts.EveAttr.capacitor_need, def_val=0)
    eve_cap_hidden_attr = client.mk_eve_attr(id_=consts.EveAttr.capacitor_need_hidden, def_val=0)
    eve_cap_bonus_attr = client.mk_eve_attr(id_=consts.EveAttr.cap_need_bonus, def_val=0)
    eve_cycle_attr = client.mk_eve_attr(id_=consts.EveAttr.duration, def_val=0)
    eve_cycle_bonus_attr = client.mk_eve_attr(id_=consts.EveAttr.duration_bonus, def_val=0)
    eve_scram_str_attr = client.mk_eve_attr(id_=consts.EveAttr.warp_scramble_strength, def_val=0)
    eve_scram_status_attr = client.mk_eve_attr(id_=consts.EveAttr.warp_scramble_status, def_val=0)
    # WDFG itself
    eve_wdfg_main_effect = client.mk_eve_effect(
        id_=consts.EveEffect.warp_disrupt_sphere,
        cat_id=consts.EveEffCat.active,
        discharge_attr_id=eve_cap_attr.id,
        duration_attr_id=eve_cycle_attr.id,
        range_attr_id=eve_range_attr.id)
    eve_wdfg_range_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.other,
        op=consts.EveModOp.pre_assign,
        affector_attr_id=eve_range_attr.id,
        affectee_attr_id=eve_range_hidden_attr.id)
    eve_wdfg_range_effect = client.mk_eve_effect(
        id_=consts.EveEffect.max_range_hidden_preass_warp_scramble_range,
        cat_id=consts.EveEffCat.passive,
        mod_info=[eve_wdfg_range_mod])
    eve_wdfg_cap_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.other,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_cap_hidden_attr.id,
        affectee_attr_id=eve_cap_attr.id)
    eve_wdfg_cap_effect = client.mk_eve_effect(
        id_=consts.EveEffect.script_wdfg_set_script_capneed_hidden,
        cat_id=consts.EveEffCat.passive,
        mod_info=[eve_wdfg_cap_mod])
    eve_wdfg = client.mk_eve_item(
        attrs={
            eve_range_attr.id: 20000, eve_cap_attr.id: 150, eve_cycle_attr.id: 30000,
            eve_scram_str_attr.id: 100},
        eff_ids=[eve_wdfg_main_effect.id, eve_wdfg_range_effect.id, eve_wdfg_cap_effect.id],
        defeff_id=eve_wdfg_main_effect.id)
    # Disruption script
    eve_dscript_main_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.tgt,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_scram_str_attr.id,
        affectee_attr_id=eve_scram_status_attr.id)
    eve_dscript_main_effect = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_focused_warp_disruption_script,
        cat_id=consts.EveEffCat.target,
        discharge_attr_id=eve_cap_hidden_attr.id,
        duration_attr_id=eve_cycle_attr.id,
        range_attr_id=eve_range_attr.id)
    eve_dscript_range_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.other,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_range_bonus_attr.id,
        affectee_attr_id=eve_range_attr.id)
    eve_dscript_range_effect = client.mk_eve_effect(
        id_=consts.EveEffect.script_warp_scramble_range_bonus,
        cat_id=consts.EveEffCat.passive,
        mod_info=[eve_dscript_range_mod])
    eve_dscript_cap_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.other,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_cap_bonus_attr.id,
        affectee_attr_id=eve_cap_attr.id)
    eve_dscript_cap_effect = client.mk_eve_effect(
        id_=consts.EveEffect.ammo_influence_cap_need,
        cat_id=consts.EveEffCat.passive,
        mod_info=[eve_dscript_cap_mod])
    eve_dscript_cycle_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.other,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_cycle_bonus_attr.id,
        affectee_attr_id=eve_cycle_attr.id)
    eve_dscript_cycle_effect = client.mk_eve_effect(
        id_=consts.EveEffect.script_duration_bonus,
        cat_id=consts.EveEffCat.passive,
        mod_info=[eve_dscript_cycle_mod])
    eve_dscript = client.mk_eve_item(
        attrs={
            eve_range_hidden_attr.id: 24000, eve_range_bonus_attr.id: 50,
            eve_cap_bonus_attr.id: -60, eve_cycle_bonus_attr.id: -80},
        eff_ids=[
            eve_dscript_main_effect.id, eve_dscript_range_effect.id,
            eve_dscript_cap_effect.id, eve_dscript_cycle_effect.id],
        defeff_id=eve_dscript_main_effect.id)
    # Misc
    eve_ship = client.mk_eve_ship(attrs={eve_scram_status_attr.id: 0})
    # Run the test
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affector_ship = api_affector_fit.set_ship(type_id=eve_ship.id)
    api_affector_wdfg = api_affector_fit.add_mod(type_id=eve_wdfg.id, state=consts.ApiState.online)
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_ship.id)
    # Verification - inactive unscripted
    assert api_affectee_ship.update().attrs[eve_scram_status_attr.id].dogma == approx(0)


def test_bubble_sig_local(client, consts):
    eve_sig_attr = client.mk_eve_attr(id_=consts.EveAttr.sig_radius)
    eve_sig_bonus_attr = client.mk_eve_attr(id_=consts.EveAttr.sig_radius_bonus)
    eve_wdfg_effect = client.mk_eve_effect(id_=consts.EveEffect.warp_disrupt_sphere, cat_id=consts.EveEffCat.active)
    eve_wdfg = client.mk_eve_item(
        attrs={eve_sig_bonus_attr.id: 50},
        eff_ids=[eve_wdfg_effect.id],
        defeff_id=eve_wdfg_effect.id)
    eve_ship = client.mk_eve_ship(attrs={eve_sig_attr.id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship.id)
    api_fit.add_mod(type_id=eve_wdfg.id, state=consts.ApiState.active)
    # Verification
    assert api_ship.update().attrs[eve_sig_attr.id].dogma == approx(150)


def test_bubble_sig_projected(client, consts):
    eve_sig_attr = client.mk_eve_attr(id_=consts.EveAttr.sig_radius)
    eve_sig_bonus_attr = client.mk_eve_attr(id_=consts.EveAttr.sig_radius_bonus)
    eve_wdfg_effect = client.mk_eve_effect(id_=consts.EveEffect.warp_disrupt_sphere, cat_id=consts.EveEffCat.active)
    eve_wdfg = client.mk_eve_item(
        attrs={eve_sig_bonus_attr.id: 50},
        eff_ids=[eve_wdfg_effect.id],
        defeff_id=eve_wdfg_effect.id)
    eve_ship = client.mk_eve_ship(attrs={eve_sig_attr.id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_wdfg = api_affector_fit.add_mod(type_id=eve_wdfg.id, state=consts.ApiState.active)
    api_ship = api_affectee_fit.set_ship(type_id=eve_ship.id)
    api_wdfg.change_mod(add_projs=[api_ship.id])
    # Verification
    assert api_ship.update().attrs[eve_sig_attr.id].dogma == approx(100)


def test_bubble_assist_local(client, consts):
    eve_assist_attr = client.mk_eve_attr(id_=consts.EveAttr.disallow_assistance)
    eve_wdfg_effect = client.mk_eve_effect(id_=consts.EveEffect.warp_disrupt_sphere, cat_id=consts.EveEffCat.active)
    eve_wdfg = client.mk_eve_item(
        attrs={eve_assist_attr.id: 1},
        eff_ids=[eve_wdfg_effect.id],
        defeff_id=eve_wdfg_effect.id)
    eve_ship = client.mk_eve_ship(attrs={eve_assist_attr.id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship.id)
    api_fit.add_mod(type_id=eve_wdfg.id, state=consts.ApiState.active)
    # Verification
    assert api_ship.update().attrs[eve_assist_attr.id].dogma == approx(1)


def test_bubble_assist_projected(client, consts):
    eve_assist_attr = client.mk_eve_attr(id_=consts.EveAttr.disallow_assistance)
    eve_wdfg_effect = client.mk_eve_effect(id_=consts.EveEffect.warp_disrupt_sphere, cat_id=consts.EveEffCat.active)
    eve_wdfg = client.mk_eve_item(
        attrs={eve_assist_attr.id: 1},
        eff_ids=[eve_wdfg_effect.id],
        defeff_id=eve_wdfg_effect.id)
    eve_ship = client.mk_eve_ship(attrs={eve_assist_attr.id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_wdfg = api_affector_fit.add_mod(type_id=eve_wdfg.id, state=consts.ApiState.active)
    api_ship = api_affectee_fit.set_ship(type_id=eve_ship.id)
    api_wdfg.change_mod(add_projs=[api_ship.id])
    # Verification
    assert api_ship.update().attrs[eve_assist_attr.id].dogma == approx(0)
