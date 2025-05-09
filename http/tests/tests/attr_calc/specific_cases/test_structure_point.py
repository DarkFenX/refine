from tests import approx

# TODO: add fighter MWD/MJD block tests, redo module MWD/MJD block tests to use restrictions
# As of 2024-11-05, point blocks fighter MWD and MJD even without scram script


def test_warp_scram_status(client, consts):
    eve_str_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warp_scramble_strength, def_val=0)
    eve_status_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warp_scramble_status, def_val=0)
    eve_point_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.structure_warp_scramble_block_mwd_with_npc,
        cat_id=consts.EveEffCat.target)
    eve_point_id = client.mk_eve_item(
        attrs={eve_str_attr_id: 100},
        eff_ids=[eve_point_effect_id],
        defeff_id=eve_point_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_status_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_point = api_affector_fit.add_module(type_id=eve_point_id, state=consts.ApiModuleState.active)
    api_ship = api_affectee_fit.set_ship(type_id=eve_ship_id)
    # Verification
    assert api_ship.update().attrs[eve_status_attr_id].dogma == approx(0)
    # Action
    api_point.change_module(add_projs=[api_ship.id])
    # Verification
    assert api_ship.update().attrs[eve_status_attr_id].dogma == approx(100)


def test_mwd_block(client, consts):
    eve_skill_id = client.mk_eve_item(id_=consts.EveItem.high_speed_maneuvering)
    eve_str_attr_id = client.mk_eve_attr(id_=consts.EveAttr.activation_blocked_strength, def_val=0)
    eve_block_attr_id = client.mk_eve_attr(id_=consts.EveAttr.activation_blocked, def_val=0)
    eve_point_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.structure_warp_scramble_block_mwd_with_npc,
        cat_id=consts.EveEffCat.target)
    eve_point_id = client.mk_eve_item(
        attrs={eve_str_attr_id: 0},
        eff_ids=[eve_point_effect_id],
        defeff_id=eve_point_effect_id)
    eve_script_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.other,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_str_attr_id,
        affectee_attr_id=eve_str_attr_id)
    eve_script_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.script_standup_warp_scram,
        cat_id=consts.EveEffCat.passive,
        mod_info=[eve_script_mod])
    eve_script_id = client.mk_eve_item(attrs={eve_str_attr_id: 1}, eff_ids=[eve_script_effect_id])
    eve_ship_id = client.mk_eve_ship()
    eve_mwd_id = client.mk_eve_item(attrs={eve_block_attr_id: 0}, srqs={eve_skill_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_point = api_affector_fit.add_module(type_id=eve_point_id, state=consts.ApiModuleState.active)
    api_ship = api_affectee_fit.set_ship(type_id=eve_ship_id)
    api_mwd = api_affectee_fit.add_module(type_id=eve_mwd_id)
    api_point.change_module(add_projs=[api_ship.id])
    # Verification
    assert api_mwd.update().attrs[eve_block_attr_id].dogma == approx(0)
    # Action
    api_point.change_module(charge_type_id=eve_script_id)
    # Verification
    assert api_mwd.update().attrs[eve_block_attr_id].dogma == approx(1)
    # Action
    api_point.change_module(charge_type_id=None)
    # Verification
    assert api_mwd.update().attrs[eve_block_attr_id].dogma == approx(0)


def test_mjd_block(client, consts):
    # Disruption script disables micro jump drives
    eve_skill_sub_id = client.mk_eve_item(id_=consts.EveItem.micro_jump_drive_operation)
    # Capital MJFG doesn't use regular skill, so test capital skill separately
    eve_skill_cap_id = client.mk_eve_item(id_=consts.EveItem.capital_micro_jump_drive_operation)
    eve_str_attr_id = client.mk_eve_attr(id_=consts.EveAttr.activation_blocked_strength, def_val=0)
    eve_block_attr_id = client.mk_eve_attr(id_=consts.EveAttr.activation_blocked, def_val=0)
    eve_point_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.structure_warp_scramble_block_mwd_with_npc,
        cat_id=consts.EveEffCat.target)
    eve_point_id = client.mk_eve_item(
        attrs={eve_str_attr_id: 0},
        eff_ids=[eve_point_effect_id],
        defeff_id=eve_point_effect_id)
    eve_script_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.other,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_str_attr_id,
        affectee_attr_id=eve_str_attr_id)
    eve_script_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.script_standup_warp_scram,
        cat_id=consts.EveEffCat.passive,
        mod_info=[eve_script_mod])
    eve_script_id = client.mk_eve_item(attrs={eve_str_attr_id: 1}, eff_ids=[eve_script_effect_id])
    eve_ship_id = client.mk_eve_ship()
    eve_mjd_sub_id = client.mk_eve_item(attrs={eve_block_attr_id: 0}, srqs={eve_skill_sub_id: 1})
    eve_mjd_cap_id = client.mk_eve_item(attrs={eve_block_attr_id: 0}, srqs={eve_skill_cap_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_point = api_affector_fit.add_module(type_id=eve_point_id, state=consts.ApiModuleState.active)
    api_ship = api_affectee_fit.set_ship(type_id=eve_ship_id)
    api_mjd_sub = api_affectee_fit.add_module(type_id=eve_mjd_sub_id)
    api_mjd_cap = api_affectee_fit.add_module(type_id=eve_mjd_cap_id)
    api_point.change_module(add_projs=[api_ship.id])
    # Verification
    assert api_mjd_sub.update().attrs[eve_block_attr_id].dogma == approx(0)
    assert api_mjd_cap.update().attrs[eve_block_attr_id].dogma == approx(0)
    # Action
    api_point.change_module(charge_type_id=eve_script_id)
    # Verification
    assert api_mjd_sub.update().attrs[eve_block_attr_id].dogma == approx(1)
    assert api_mjd_cap.update().attrs[eve_block_attr_id].dogma == approx(1)
    # Action
    api_point.change_module(charge_type_id=None)
    # Verification
    assert api_mjd_sub.update().attrs[eve_block_attr_id].dogma == approx(0)
    assert api_mjd_cap.update().attrs[eve_block_attr_id].dogma == approx(0)
