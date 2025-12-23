from fw import Effect, approx, check_no_field
from fw.api import ValOptions


def test_warp_scram_status(client, consts):
    eve_str_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warp_scramble_strength, def_val=0)
    eve_status_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warp_scramble_status, def_val=0)
    eve_point_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.struct_warp_scramble_block_mwd_with_npc,
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
    assert api_ship.update().attrs[eve_status_attr_id].modified == approx(0)
    # Action
    api_point.change_module(add_projs=[api_ship.id])
    # Verification
    assert api_ship.update().attrs[eve_status_attr_id].modified == approx(100)


def test_module_mwd_block(client, consts):
    eve_skill_id = client.mk_eve_item(id_=consts.EveItem.high_speed_maneuvering)
    eve_str_attr_id = client.mk_eve_attr(id_=consts.EveAttr.activation_blocked_strength, def_val=0)
    eve_block_attr_id = client.mk_eve_attr(id_=consts.EveAttr.activation_blocked, def_val=0)
    eve_point_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.struct_warp_scramble_block_mwd_with_npc,
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
        id_=consts.EveEffect.script_st_warp_scram,
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
    api_mwd = api_affectee_fit.add_module(type_id=eve_mwd_id, state=consts.ApiModuleState.active)
    api_point.change_module(add_projs=[api_ship.id])
    # Verification
    api_val = api_affectee_fit.validate(options=ValOptions(activation_blocked=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_point.change_module(charge_type_id=eve_script_id)
    # Verification
    api_val = api_affectee_fit.validate(options=ValOptions(activation_blocked=True))
    assert api_val.passed is False
    assert api_val.details.activation_blocked == [api_mwd.id]
    # Action
    api_point.change_module(charge_type_id=None)
    # Verification
    api_val = api_affectee_fit.validate(options=ValOptions(activation_blocked=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_module_mjd_block(client, consts):
    # Disruption script disables micro jump drives
    eve_skill_sub_id = client.mk_eve_item(id_=consts.EveItem.micro_jump_drive_operation)
    # Capital MJFG doesn't use regular skill, so test capital skill separately
    eve_skill_cap_id = client.mk_eve_item(id_=consts.EveItem.capital_micro_jump_drive_operation)
    eve_str_attr_id = client.mk_eve_attr(id_=consts.EveAttr.activation_blocked_strength, def_val=0)
    eve_block_attr_id = client.mk_eve_attr(id_=consts.EveAttr.activation_blocked, def_val=0)
    eve_point_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.struct_warp_scramble_block_mwd_with_npc,
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
        id_=consts.EveEffect.script_st_warp_scram,
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
    api_mjd_sub = api_affectee_fit.add_module(type_id=eve_mjd_sub_id, state=consts.ApiModuleState.active)
    api_mjd_cap = api_affectee_fit.add_module(type_id=eve_mjd_cap_id, state=consts.ApiModuleState.active)
    api_point.change_module(add_projs=[api_ship.id])
    # Verification
    api_val = api_affectee_fit.validate(options=ValOptions(activation_blocked=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_point.change_module(charge_type_id=eve_script_id)
    # Verification
    api_val = api_affectee_fit.validate(options=ValOptions(activation_blocked=True))
    assert api_val.passed is False
    assert api_val.details.activation_blocked == sorted([api_mjd_sub.id, api_mjd_cap.id])
    # Action
    api_point.change_module(charge_type_id=None)
    # Verification
    api_val = api_affectee_fit.validate(options=ValOptions(activation_blocked=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_fighter_mwd_mjd_block(client, consts):
    # As of 2024-11-05, point blocks fighter MWD and MJD even without scram script
    eve_point_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.struct_warp_scramble_block_mwd_with_npc,
        cat_id=consts.EveEffCat.target)
    eve_point_id = client.mk_eve_item(eff_ids=[eve_point_effect_id], defeff_id=eve_point_effect_id)
    eve_script_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.script_st_warp_scram,
        cat_id=consts.EveEffCat.passive)
    eve_script_id = client.mk_eve_item(eff_ids=[eve_script_effect_id])
    eve_ftr_mwd_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ftr_abil_mwd,
        cat_id=consts.EveEffCat.active)
    eve_ftr_mjd_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ftr_abil_mjd,
        cat_id=consts.EveEffCat.active)
    eve_fighter_id = client.mk_eve_fighter(eff_ids=[eve_ftr_mwd_effect_id, eve_ftr_mjd_effect_id])
    client.create_sources()
    api_ftr_mwd_effect_id = Effect.dogma_to_api(dogma_effect_id=eve_ftr_mwd_effect_id)
    api_ftr_mjd_effect_id = Effect.dogma_to_api(dogma_effect_id=eve_ftr_mjd_effect_id)
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_point = api_affector_fit.add_module(type_id=eve_point_id, state=consts.ApiModuleState.active)
    api_fighter = api_affectee_fit.add_fighter(type_id=eve_fighter_id)
    api_fighter.change_fighter(effect_modes={
        api_ftr_mwd_effect_id: consts.ApiEffMode.force_run,
        api_ftr_mjd_effect_id: consts.ApiEffMode.force_run})
    api_point.change_module(add_projs=[api_fighter.id])
    # Verification
    api_val = api_affectee_fit.validate(options=ValOptions(effect_stopper=True))
    assert api_val.passed is False
    assert api_val.details.effect_stopper == {api_fighter.id: sorted([api_ftr_mwd_effect_id, api_ftr_mjd_effect_id])}
    # Action
    api_point.change_module(charge_type_id=eve_script_id)
    # Verification
    api_val = api_affectee_fit.validate(options=ValOptions(effect_stopper=True))
    assert api_val.passed is False
    assert api_val.details.effect_stopper == {api_fighter.id: sorted([api_ftr_mwd_effect_id, api_ftr_mjd_effect_id])}
    # Action
    api_point.change_module(charge_type_id=None)
    # Verification
    api_val = api_affectee_fit.validate(options=ValOptions(effect_stopper=True))
    assert api_val.passed is False
    assert api_val.details.effect_stopper == {api_fighter.id: sorted([api_ftr_mwd_effect_id, api_ftr_mjd_effect_id])}
