from tests import approx


# TODO: add fighter MWD/MJD block tests, redo module MWD/MJD block tests to use restrictions
# As of 2024-11-05, point blocks fighter MWD and MJD even without scram script


def test_warp_scram_status(client, consts):
    eve_str_attr = client.mk_eve_attr(id_=consts.EveAttr.warp_scramble_strength, def_val=0)
    eve_status_attr = client.mk_eve_attr(id_=consts.EveAttr.warp_scramble_status, def_val=0)
    eve_point_effect = client.mk_eve_effect(
        id_=consts.EveEffect.structure_warp_scramble_block_mwd_with_npc,
        cat_id=consts.EveEffCat.target)
    eve_point = client.mk_eve_item(
        attrs={eve_str_attr.id: 100},
        eff_ids=[eve_point_effect.id],
        defeff_id=eve_point_effect.id)
    eve_ship = client.mk_eve_ship(attrs={eve_status_attr.id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_point = api_affector_fit.add_mod(type_id=eve_point.id, state=consts.ApiState.active)
    api_ship = api_affectee_fit.set_ship(type_id=eve_ship.id)
    # Verification
    assert api_ship.update().attrs[eve_status_attr.id].dogma == approx(0)
    # Action
    api_point.change_mod(add_projs=[api_ship.id])
    # Verification
    assert api_ship.update().attrs[eve_status_attr.id].dogma == approx(100)


def test_mwd_block(client, consts):
    eve_skill = client.mk_eve_item(id_=consts.EveItem.high_speed_maneuvering)
    eve_str_attr = client.mk_eve_attr(id_=consts.EveAttr.activation_blocked_strength, def_val=0)
    eve_block_attr = client.mk_eve_attr(id_=consts.EveAttr.activation_blocked, def_val=0)
    eve_point_effect = client.mk_eve_effect(
        id_=consts.EveEffect.structure_warp_scramble_block_mwd_with_npc,
        cat_id=consts.EveEffCat.target)
    eve_point = client.mk_eve_item(
        attrs={eve_str_attr.id: 0},
        eff_ids=[eve_point_effect.id],
        defeff_id=eve_point_effect.id)
    eve_script_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.other,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_str_attr.id,
        affectee_attr_id=eve_str_attr.id)
    eve_script_effect = client.mk_eve_effect(
        id_=consts.EveEffect.script_standup_warp_scram,
        cat_id=consts.EveEffCat.passive,
        mod_info=[eve_script_mod])
    eve_script = client.mk_eve_item(attrs={eve_str_attr.id: 1}, eff_ids=[eve_script_effect.id])
    eve_ship = client.mk_eve_ship()
    eve_mwd = client.mk_eve_item(attrs={eve_block_attr.id: 0}, srqs={eve_skill.id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_point = api_affector_fit.add_mod(type_id=eve_point.id, state=consts.ApiState.active)
    api_ship = api_affectee_fit.set_ship(type_id=eve_ship.id)
    api_mwd = api_affectee_fit.add_mod(type_id=eve_mwd.id)
    api_point.change_mod(add_projs=[api_ship.id])
    # Verification
    assert api_mwd.update().attrs[eve_block_attr.id].dogma == approx(0)
    # Action
    api_point.change_mod(charge=eve_script.id)
    # Verification
    assert api_mwd.update().attrs[eve_block_attr.id].dogma == approx(1)
    # Action
    api_point.change_mod(charge=None)
    # Verification
    assert api_mwd.update().attrs[eve_block_attr.id].dogma == approx(0)


def test_mjd_block(client, consts):
    # Disruption script disables micro jump drives
    eve_skill_sub = client.mk_eve_item(id_=consts.EveItem.micro_jump_drive_operation)
    # Capital MJFG doesn't use regular skill, so test capital skill separately
    eve_skill_cap = client.mk_eve_item(id_=consts.EveItem.capital_micro_jump_drive_operation)
    eve_str_attr = client.mk_eve_attr(id_=consts.EveAttr.activation_blocked_strength, def_val=0)
    eve_block_attr = client.mk_eve_attr(id_=consts.EveAttr.activation_blocked, def_val=0)
    eve_point_effect = client.mk_eve_effect(
        id_=consts.EveEffect.structure_warp_scramble_block_mwd_with_npc,
        cat_id=consts.EveEffCat.target)
    eve_point = client.mk_eve_item(
        attrs={eve_str_attr.id: 0},
        eff_ids=[eve_point_effect.id],
        defeff_id=eve_point_effect.id)
    eve_script_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.other,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_str_attr.id,
        affectee_attr_id=eve_str_attr.id)
    eve_script_effect = client.mk_eve_effect(
        id_=consts.EveEffect.script_standup_warp_scram,
        cat_id=consts.EveEffCat.passive,
        mod_info=[eve_script_mod])
    eve_script = client.mk_eve_item(attrs={eve_str_attr.id: 1}, eff_ids=[eve_script_effect.id])
    eve_ship = client.mk_eve_ship()
    eve_mjd_sub = client.mk_eve_item(attrs={eve_block_attr.id: 0}, srqs={eve_skill_sub.id: 1})
    eve_mjd_cap = client.mk_eve_item(attrs={eve_block_attr.id: 0}, srqs={eve_skill_cap.id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_point = api_affector_fit.add_mod(type_id=eve_point.id, state=consts.ApiState.active)
    api_ship = api_affectee_fit.set_ship(type_id=eve_ship.id)
    api_mjd_sub = api_affectee_fit.add_mod(type_id=eve_mjd_sub.id)
    api_mjd_cap = api_affectee_fit.add_mod(type_id=eve_mjd_cap.id)
    api_point.change_mod(add_projs=[api_ship.id])
    # Verification
    assert api_mjd_sub.update().attrs[eve_block_attr.id].dogma == approx(0)
    assert api_mjd_cap.update().attrs[eve_block_attr.id].dogma == approx(0)
    # Action
    api_point.change_mod(charge=eve_script.id)
    # Verification
    assert api_mjd_sub.update().attrs[eve_block_attr.id].dogma == approx(1)
    assert api_mjd_cap.update().attrs[eve_block_attr.id].dogma == approx(1)
    # Action
    api_point.change_mod(charge=None)
    # Verification
    assert api_mjd_sub.update().attrs[eve_block_attr.id].dogma == approx(0)
    assert api_mjd_cap.update().attrs[eve_block_attr.id].dogma == approx(0)
