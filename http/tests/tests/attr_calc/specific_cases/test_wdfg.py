from tests import approx

# TODO: add fighter MWD/MJD block tests, redo module MWD/MJD block tests to use restrictions
# As of 2024-11-05, HIC ray blocks fighter MWD and MJD even with disruption script


def test_bubble_sig_local(client, consts):
    # Bubble blows sig of the ship it's on
    eve_sig_attr_id = client.mk_eve_attr(id_=consts.EveAttr.sig_radius)
    eve_sig_bonus_attr_id = client.mk_eve_attr(id_=consts.EveAttr.sig_radius_bonus)
    eve_wdfg_effect_id = client.mk_eve_effect(id_=consts.EveEffect.warp_disrupt_sphere, cat_id=consts.EveEffCat.active)
    eve_wdfg_id = client.mk_eve_item(
        attrs={eve_sig_bonus_attr_id: 50},
        eff_ids=[eve_wdfg_effect_id],
        defeff_id=eve_wdfg_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_sig_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_wdfg_id, state=consts.ApiModuleState.active)
    # Verification
    assert api_ship.update().attrs[eve_sig_attr_id].dogma == approx(150)


def test_bubble_sig_projected(client, consts):
    # Bubble doesn't blow sig of the ship it's projected to
    eve_sig_attr_id = client.mk_eve_attr(id_=consts.EveAttr.sig_radius)
    eve_sig_bonus_attr_id = client.mk_eve_attr(id_=consts.EveAttr.sig_radius_bonus)
    eve_wdfg_effect_id = client.mk_eve_effect(id_=consts.EveEffect.warp_disrupt_sphere, cat_id=consts.EveEffCat.active)
    eve_wdfg_id = client.mk_eve_item(
        attrs={eve_sig_bonus_attr_id: 50},
        eff_ids=[eve_wdfg_effect_id],
        defeff_id=eve_wdfg_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_sig_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_wdfg = api_affector_fit.add_module(type_id=eve_wdfg_id, state=consts.ApiModuleState.active)
    api_ship = api_affectee_fit.set_ship(type_id=eve_ship_id)
    api_wdfg.change_module(add_projs=[api_ship.id])
    # Verification
    assert api_ship.update().attrs[eve_sig_attr_id].dogma == approx(100)


def test_bubble_assist_local(client, consts):
    # Bubble disables assistance on the ship it's running on
    eve_assist_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_assistance)
    eve_wdfg_effect_id = client.mk_eve_effect(id_=consts.EveEffect.warp_disrupt_sphere, cat_id=consts.EveEffCat.active)
    eve_wdfg_id = client.mk_eve_item(
        attrs={eve_assist_attr_id: 1},
        eff_ids=[eve_wdfg_effect_id],
        defeff_id=eve_wdfg_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_assist_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_wdfg_id, state=consts.ApiModuleState.active)
    # Verification
    assert api_ship.update().attrs[eve_assist_attr_id].dogma == approx(1)


def test_bubble_assist_projected(client, consts):
    # Bubble doesn't disable assistance on any other ships
    eve_assist_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_assistance)
    eve_wdfg_effect_id = client.mk_eve_effect(id_=consts.EveEffect.warp_disrupt_sphere, cat_id=consts.EveEffCat.active)
    eve_wdfg_id = client.mk_eve_item(
        attrs={eve_assist_attr_id: 1},
        eff_ids=[eve_wdfg_effect_id],
        defeff_id=eve_wdfg_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_assist_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_wdfg = api_affector_fit.add_module(type_id=eve_wdfg_id, state=consts.ApiModuleState.active)
    api_ship = api_affectee_fit.set_ship(type_id=eve_ship_id)
    api_wdfg.change_module(add_projs=[api_ship.id])
    # Verification
    assert api_ship.update().attrs[eve_assist_attr_id].dogma == approx(0)


def test_warp_scram_status_dscript(client, consts):
    # Disruption script disables warp for target it's projected on
    eve_str_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warp_scramble_strength, def_val=0)
    eve_status_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warp_scramble_status, def_val=0)
    eve_wdfg_effect_id = client.mk_eve_effect(id_=consts.EveEffect.warp_disrupt_sphere, cat_id=consts.EveEffCat.active)
    eve_wdfg_id = client.mk_eve_item(
        attrs={eve_str_attr_id: 100},
        eff_ids=[eve_wdfg_effect_id],
        defeff_id=eve_wdfg_effect_id)
    eve_script_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_focused_warp_disruption_script,
        cat_id=consts.EveEffCat.target)
    eve_script_id = client.mk_eve_item(
        eff_ids=[eve_script_effect_id],
        defeff_id=eve_script_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_status_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_wdfg = api_affector_fit.add_module(type_id=eve_wdfg_id, state=consts.ApiModuleState.active)
    api_ship = api_affectee_fit.set_ship(type_id=eve_ship_id)
    api_wdfg.change_module(add_projs=[api_ship.id])
    # Verification
    assert api_ship.update().attrs[eve_status_attr_id].dogma == approx(0)
    # Action
    api_wdfg.change_module(charge_type_id=eve_script_id)
    # Verification
    assert api_ship.update().attrs[eve_status_attr_id].dogma == approx(100)
    # Action
    api_wdfg.change_module(charge_type_id=None)
    # Verification
    assert api_ship.update().attrs[eve_status_attr_id].dogma == approx(0)


def test_warp_scram_status_sscript(client, consts):
    # Scrambling script disables warp for target it's projected on
    eve_str_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warp_scramble_strength, def_val=0)
    eve_status_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warp_scramble_status, def_val=0)
    eve_wdfg_effect_id = client.mk_eve_effect(id_=consts.EveEffect.warp_disrupt_sphere, cat_id=consts.EveEffCat.active)
    eve_wdfg_id = client.mk_eve_item(
        attrs={eve_str_attr_id: 100},
        eff_ids=[eve_wdfg_effect_id],
        defeff_id=eve_wdfg_effect_id)
    eve_script_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_focused_warp_scrambling_script,
        cat_id=consts.EveEffCat.target)
    eve_script_id = client.mk_eve_item(
        eff_ids=[eve_script_effect_id],
        defeff_id=eve_script_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_status_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_wdfg = api_affector_fit.add_module(type_id=eve_wdfg_id, state=consts.ApiModuleState.active)
    api_ship = api_affectee_fit.set_ship(type_id=eve_ship_id)
    api_wdfg.change_module(add_projs=[api_ship.id])
    # Verification
    assert api_ship.update().attrs[eve_status_attr_id].dogma == approx(0)
    # Action
    api_wdfg.change_module(charge_type_id=eve_script_id)
    # Verification
    assert api_ship.update().attrs[eve_status_attr_id].dogma == approx(100)
    # Action
    api_wdfg.change_module(charge_type_id=None)
    # Verification
    assert api_ship.update().attrs[eve_status_attr_id].dogma == approx(0)


def test_gate_scram_status_dscript(client, consts):
    # Disruption script disables gate jumps for target capitals it's projected onto
    client.mk_eve_attr(id_=consts.EveAttr.gate_scramble_strength, def_val=1)
    eve_status_attr_id = client.mk_eve_attr(id_=consts.EveAttr.gate_scramble_status, def_val=0)
    eve_wdfg_effect_id = client.mk_eve_effect(id_=consts.EveEffect.warp_disrupt_sphere, cat_id=consts.EveEffCat.active)
    eve_wdfg_id = client.mk_eve_item(eff_ids=[eve_wdfg_effect_id], defeff_id=eve_wdfg_effect_id)
    eve_script_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_focused_warp_disruption_script,
        cat_id=consts.EveEffCat.target)
    eve_script_id = client.mk_eve_item(
        eff_ids=[eve_script_effect_id],
        defeff_id=eve_script_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_status_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_wdfg = api_affector_fit.add_module(type_id=eve_wdfg_id, state=consts.ApiModuleState.active)
    api_ship = api_affectee_fit.set_ship(type_id=eve_ship_id)
    api_wdfg.change_module(add_projs=[api_ship.id])
    # Verification
    assert api_ship.update().attrs[eve_status_attr_id].dogma == approx(0)
    # Action
    api_wdfg.change_module(charge_type_id=eve_script_id)
    # Verification
    assert api_ship.update().attrs[eve_status_attr_id].dogma == approx(1)
    # Action
    api_wdfg.change_module(charge_type_id=None)
    # Verification
    assert api_ship.update().attrs[eve_status_attr_id].dogma == approx(0)


def test_gate_scram_status_sscript(client, consts):
    # Scrambling script disables gate jumps for target capitals it's projected onto
    client.mk_eve_attr(id_=consts.EveAttr.gate_scramble_strength, def_val=1)
    eve_status_attr_id = client.mk_eve_attr(id_=consts.EveAttr.gate_scramble_status, def_val=0)
    eve_wdfg_effect_id = client.mk_eve_effect(id_=consts.EveEffect.warp_disrupt_sphere, cat_id=consts.EveEffCat.active)
    eve_wdfg_id = client.mk_eve_item(eff_ids=[eve_wdfg_effect_id], defeff_id=eve_wdfg_effect_id)
    eve_script_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_focused_warp_scrambling_script,
        cat_id=consts.EveEffCat.target)
    eve_script_id = client.mk_eve_item(
        eff_ids=[eve_script_effect_id],
        defeff_id=eve_script_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_status_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_wdfg = api_affector_fit.add_module(type_id=eve_wdfg_id, state=consts.ApiModuleState.active)
    api_ship = api_affectee_fit.set_ship(type_id=eve_ship_id)
    api_wdfg.change_module(add_projs=[api_ship.id])
    # Verification
    assert api_ship.update().attrs[eve_status_attr_id].dogma == approx(0)
    # Action
    api_wdfg.change_module(charge_type_id=eve_script_id)
    # Verification
    assert api_ship.update().attrs[eve_status_attr_id].dogma == approx(1)
    # Action
    api_wdfg.change_module(charge_type_id=None)
    # Verification
    assert api_ship.update().attrs[eve_status_attr_id].dogma == approx(0)


def test_mwd_block_dscript(client, consts):
    # Disruption script doesn't disable micro warp drives
    eve_skill_id = client.mk_eve_item(id_=consts.EveItem.high_speed_maneuvering)
    eve_str_attr_id = client.mk_eve_attr(id_=consts.EveAttr.activation_blocked_strength, def_val=0)
    eve_block_attr_id = client.mk_eve_attr(id_=consts.EveAttr.activation_blocked, def_val=0)
    eve_wdfg_effect_id = client.mk_eve_effect(id_=consts.EveEffect.warp_disrupt_sphere, cat_id=consts.EveEffCat.active)
    eve_wdfg_id = client.mk_eve_item(
        attrs={eve_str_attr_id: 1},
        eff_ids=[eve_wdfg_effect_id],
        defeff_id=eve_wdfg_effect_id)
    eve_script_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_focused_warp_disruption_script,
        cat_id=consts.EveEffCat.target)
    eve_script_id = client.mk_eve_item(eff_ids=[eve_script_effect_id], defeff_id=eve_script_effect_id)
    eve_ship_id = client.mk_eve_ship()
    eve_mwd_id = client.mk_eve_item(attrs={eve_block_attr_id: 0}, srqs={eve_skill_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_wdfg = api_affector_fit.add_module(type_id=eve_wdfg_id, state=consts.ApiModuleState.active)
    api_ship = api_affectee_fit.set_ship(type_id=eve_ship_id)
    api_mwd = api_affectee_fit.add_module(type_id=eve_mwd_id)
    api_wdfg.change_module(add_projs=[api_ship.id])
    # Verification
    assert api_mwd.update().attrs[eve_block_attr_id].dogma == approx(0)
    # Action
    api_wdfg.change_module(charge_type_id=eve_script_id)
    # Verification
    assert api_mwd.update().attrs[eve_block_attr_id].dogma == approx(0)
    # Action
    api_wdfg.change_module(charge_type_id=None)
    # Verification
    assert api_mwd.update().attrs[eve_block_attr_id].dogma == approx(0)


def test_mwd_block_sscript(client, consts):
    # Scrambling script disables micro warp drives
    eve_skill_id = client.mk_eve_item(id_=consts.EveItem.high_speed_maneuvering)
    eve_str_attr_id = client.mk_eve_attr(id_=consts.EveAttr.activation_blocked_strength, def_val=0)
    eve_block_attr_id = client.mk_eve_attr(id_=consts.EveAttr.activation_blocked, def_val=0)
    eve_wdfg_effect_id = client.mk_eve_effect(id_=consts.EveEffect.warp_disrupt_sphere, cat_id=consts.EveEffCat.active)
    eve_wdfg_id = client.mk_eve_item(
        attrs={eve_str_attr_id: 1},
        eff_ids=[eve_wdfg_effect_id],
        defeff_id=eve_wdfg_effect_id)
    eve_script_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_focused_warp_scrambling_script,
        cat_id=consts.EveEffCat.target)
    eve_script_id = client.mk_eve_item(eff_ids=[eve_script_effect_id], defeff_id=eve_script_effect_id)
    eve_ship_id = client.mk_eve_ship()
    eve_mwd_id = client.mk_eve_item(attrs={eve_block_attr_id: 0}, srqs={eve_skill_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_wdfg = api_affector_fit.add_module(type_id=eve_wdfg_id, state=consts.ApiModuleState.active)
    api_ship = api_affectee_fit.set_ship(type_id=eve_ship_id)
    api_mwd = api_affectee_fit.add_module(type_id=eve_mwd_id)
    api_wdfg.change_module(add_projs=[api_ship.id])
    # Verification
    assert api_mwd.update().attrs[eve_block_attr_id].dogma == approx(0)
    # Action
    api_wdfg.change_module(charge_type_id=eve_script_id)
    # Verification
    assert api_mwd.update().attrs[eve_block_attr_id].dogma == approx(1)
    # Action
    api_wdfg.change_module(charge_type_id=None)
    # Verification
    assert api_mwd.update().attrs[eve_block_attr_id].dogma == approx(0)


def test_mjd_block_dscript(client, consts):
    # Disruption script disables micro jump drives
    eve_skill_sub_id = client.mk_eve_item(id_=consts.EveItem.micro_jump_drive_operation)
    # Capital MJFG doesn't use regular skill, so test capital skill separately
    eve_skill_cap_id = client.mk_eve_item(id_=consts.EveItem.capital_micro_jump_drive_operation)
    eve_str_attr_id = client.mk_eve_attr(id_=consts.EveAttr.activation_blocked_strength, def_val=0)
    eve_block_attr_id = client.mk_eve_attr(id_=consts.EveAttr.activation_blocked, def_val=0)
    eve_wdfg_effect_id = client.mk_eve_effect(id_=consts.EveEffect.warp_disrupt_sphere, cat_id=consts.EveEffCat.active)
    eve_wdfg_id = client.mk_eve_item(
        attrs={eve_str_attr_id: 1},
        eff_ids=[eve_wdfg_effect_id],
        defeff_id=eve_wdfg_effect_id)
    eve_script_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_focused_warp_disruption_script,
        cat_id=consts.EveEffCat.target)
    eve_script_id = client.mk_eve_item(eff_ids=[eve_script_effect_id], defeff_id=eve_script_effect_id)
    eve_ship_id = client.mk_eve_ship()
    eve_mjd_sub_id = client.mk_eve_item(attrs={eve_block_attr_id: 0}, srqs={eve_skill_sub_id: 1})
    eve_mjd_cap_id = client.mk_eve_item(attrs={eve_block_attr_id: 0}, srqs={eve_skill_cap_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_wdfg = api_affector_fit.add_module(type_id=eve_wdfg_id, state=consts.ApiModuleState.active)
    api_ship = api_affectee_fit.set_ship(type_id=eve_ship_id)
    api_mjd_sub = api_affectee_fit.add_module(type_id=eve_mjd_sub_id)
    api_mjd_cap = api_affectee_fit.add_module(type_id=eve_mjd_cap_id)
    api_wdfg.change_module(add_projs=[api_ship.id])
    # Verification
    assert api_mjd_sub.update().attrs[eve_block_attr_id].dogma == approx(0)
    assert api_mjd_cap.update().attrs[eve_block_attr_id].dogma == approx(0)
    # Action
    api_wdfg.change_module(charge_type_id=eve_script_id)
    # Verification
    assert api_mjd_sub.update().attrs[eve_block_attr_id].dogma == approx(1)
    assert api_mjd_cap.update().attrs[eve_block_attr_id].dogma == approx(1)
    # Action
    api_wdfg.change_module(charge_type_id=None)
    # Verification
    assert api_mjd_sub.update().attrs[eve_block_attr_id].dogma == approx(0)
    assert api_mjd_cap.update().attrs[eve_block_attr_id].dogma == approx(0)


def test_mjd_block_sscript(client, consts):
    # Scrambling script disables micro jump drives
    eve_skill_sub_id = client.mk_eve_item(id_=consts.EveItem.micro_jump_drive_operation)
    # Capital MJFG doesn't use regular skill, so test capital skill separately
    eve_skill_cap_id = client.mk_eve_item(id_=consts.EveItem.capital_micro_jump_drive_operation)
    eve_str_attr_id = client.mk_eve_attr(id_=consts.EveAttr.activation_blocked_strength, def_val=0)
    eve_block_attr_id = client.mk_eve_attr(id_=consts.EveAttr.activation_blocked, def_val=0)
    eve_wdfg_effect_id = client.mk_eve_effect(id_=consts.EveEffect.warp_disrupt_sphere, cat_id=consts.EveEffCat.active)
    eve_wdfg_id = client.mk_eve_item(
        attrs={eve_str_attr_id: 1},
        eff_ids=[eve_wdfg_effect_id],
        defeff_id=eve_wdfg_effect_id)
    eve_script_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_focused_warp_scrambling_script,
        cat_id=consts.EveEffCat.target)
    eve_script_id = client.mk_eve_item(eff_ids=[eve_script_effect_id], defeff_id=eve_script_effect_id)
    eve_ship_id = client.mk_eve_ship()
    eve_mjd_sub_id = client.mk_eve_item(attrs={eve_block_attr_id: 0}, srqs={eve_skill_sub_id: 1})
    eve_mjd_cap_id = client.mk_eve_item(attrs={eve_block_attr_id: 0}, srqs={eve_skill_cap_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_wdfg = api_affector_fit.add_module(type_id=eve_wdfg_id, state=consts.ApiModuleState.active)
    api_ship = api_affectee_fit.set_ship(type_id=eve_ship_id)
    api_mjd_sub = api_affectee_fit.add_module(type_id=eve_mjd_sub_id)
    api_mjd_cap = api_affectee_fit.add_module(type_id=eve_mjd_cap_id)
    api_wdfg.change_module(add_projs=[api_ship.id])
    # Verification
    assert api_mjd_sub.update().attrs[eve_block_attr_id].dogma == approx(0)
    assert api_mjd_cap.update().attrs[eve_block_attr_id].dogma == approx(0)
    # Action
    api_wdfg.change_module(charge_type_id=eve_script_id)
    # Verification
    assert api_mjd_sub.update().attrs[eve_block_attr_id].dogma == approx(1)
    assert api_mjd_cap.update().attrs[eve_block_attr_id].dogma == approx(1)
    # Action
    api_wdfg.change_module(charge_type_id=None)
    # Verification
    assert api_mjd_sub.update().attrs[eve_block_attr_id].dogma == approx(0)
    assert api_mjd_cap.update().attrs[eve_block_attr_id].dogma == approx(0)


def test_range_dscript(client, consts):
    eve_range_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warp_scramble_range, def_val=0)
    eve_range_hidden_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_range_hidden, def_val=0)
    eve_range_bonus_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warp_scramble_range_bonus, def_val=0)
    eve_str_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warp_scramble_strength, def_val=0)
    eve_status_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warp_scramble_status, def_val=0)
    eve_wdfg_range_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.other,
        op=consts.EveModOp.pre_assign,
        affector_attr_id=eve_range_attr_id,
        affectee_attr_id=eve_range_hidden_attr_id)
    eve_wdfg_range_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.max_range_hidden_preass_warp_scramble_range,
        cat_id=consts.EveEffCat.passive,
        mod_info=[eve_wdfg_range_mod])
    eve_wdfg_main_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.warp_disrupt_sphere,
        cat_id=consts.EveEffCat.active,
        range_attr_id=eve_range_attr_id)
    eve_wdfg_id = client.mk_eve_item(
        attrs={eve_range_attr_id: 20000, eve_str_attr_id: 100},
        eff_ids=[eve_wdfg_main_effect_id, eve_wdfg_range_effect_id],
        defeff_id=eve_wdfg_main_effect_id)
    eve_script_range_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.other,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_range_bonus_attr_id,
        affectee_attr_id=eve_range_attr_id)
    eve_script_range_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.script_warp_scramble_range_bonus,
        cat_id=consts.EveEffCat.passive,
        mod_info=[eve_script_range_mod])
    eve_script_main_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_focused_warp_disruption_script,
        cat_id=consts.EveEffCat.target,
        range_attr_id=eve_range_attr_id)
    eve_script_id = client.mk_eve_item(
        attrs={eve_range_bonus_attr_id: 50},
        eff_ids=[eve_script_main_effect_id, eve_script_range_effect_id],
        defeff_id=eve_script_main_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_status_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_wdfg = api_affector_fit.add_module(
        type_id=eve_wdfg_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_script_id)
    api_ship = api_affectee_fit.set_ship(type_id=eve_ship_id)
    api_wdfg.change_module(add_projs=[(api_ship.id, 30000)])
    # Verification - range should be 30k (20k base from module +50% from script)
    assert api_ship.update().attrs[eve_status_attr_id].dogma == approx(100)
    # Action
    api_wdfg.change_module(change_projs=[(api_ship.id, 30001)])
    # Verification
    assert api_ship.update().attrs[eve_status_attr_id].dogma == approx(0)


def test_range_sscript(client, consts):
    eve_range_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warp_scramble_range, def_val=0)
    eve_range_hidden_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_range_hidden, def_val=0)
    eve_range_bonus_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warp_scramble_range_bonus, def_val=0)
    eve_str_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warp_scramble_strength, def_val=0)
    eve_status_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warp_scramble_status, def_val=0)
    eve_wdfg_range_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.other,
        op=consts.EveModOp.pre_assign,
        affector_attr_id=eve_range_attr_id,
        affectee_attr_id=eve_range_hidden_attr_id)
    eve_wdfg_range_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.max_range_hidden_preass_warp_scramble_range,
        cat_id=consts.EveEffCat.passive,
        mod_info=[eve_wdfg_range_mod])
    eve_wdfg_main_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.warp_disrupt_sphere,
        cat_id=consts.EveEffCat.active,
        range_attr_id=eve_range_attr_id)
    eve_wdfg_id = client.mk_eve_item(
        attrs={eve_range_attr_id: 20000, eve_str_attr_id: 100},
        eff_ids=[eve_wdfg_main_effect_id, eve_wdfg_range_effect_id],
        defeff_id=eve_wdfg_main_effect_id)
    eve_script_range_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.other,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_range_bonus_attr_id,
        affectee_attr_id=eve_range_attr_id)
    eve_script_range_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.script_warp_scramble_range_bonus,
        cat_id=consts.EveEffCat.passive,
        mod_info=[eve_script_range_mod])
    eve_script_main_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_focused_warp_scrambling_script,
        cat_id=consts.EveEffCat.target,
        range_attr_id=eve_range_attr_id)
    eve_script_id = client.mk_eve_item(
        attrs={eve_range_bonus_attr_id: -20},
        eff_ids=[eve_script_main_effect_id, eve_script_range_effect_id],
        defeff_id=eve_script_main_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_status_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_wdfg = api_affector_fit.add_module(
        type_id=eve_wdfg_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_script_id)
    api_ship = api_affectee_fit.set_ship(type_id=eve_ship_id)
    api_wdfg.change_module(add_projs=[(api_ship.id, 16000)])
    # Verification - range should be 16k (20k base from module -20% from script)
    assert api_ship.update().attrs[eve_status_attr_id].dogma == approx(100)
    # Action
    api_wdfg.change_module(change_projs=[(api_ship.id, 16001)])
    # Verification
    assert api_ship.update().attrs[eve_status_attr_id].dogma == approx(0)


def test_assist_dscript(client, consts):
    # WDFG disables assistance even when it's running with any script
    eve_assist_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_assistance)
    eve_wdfg_effect_id = client.mk_eve_effect(id_=consts.EveEffect.warp_disrupt_sphere, cat_id=consts.EveEffCat.active)
    eve_wdfg_id = client.mk_eve_item(
        attrs={eve_assist_attr_id: 1},
        eff_ids=[eve_wdfg_effect_id],
        defeff_id=eve_wdfg_effect_id)
    eve_script_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_focused_warp_disruption_script,
        cat_id=consts.EveEffCat.target)
    eve_script_id = client.mk_eve_item(eff_ids=[eve_script_effect_id], defeff_id=eve_script_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_assist_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(
        type_id=eve_wdfg_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_script_id)
    # Verification
    assert api_ship.update().attrs[eve_assist_attr_id].dogma == approx(1)
    # Action
    api_module.change_module(state=consts.ApiModuleState.online)
    # Verification
    assert api_ship.update().attrs[eve_assist_attr_id].dogma == approx(0)


def test_assist_sscript(client, consts):
    # WDFG disables assistance even when it's running with any script
    eve_assist_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_assistance)
    eve_wdfg_effect_id = client.mk_eve_effect(id_=consts.EveEffect.warp_disrupt_sphere, cat_id=consts.EveEffCat.active)
    eve_wdfg_id = client.mk_eve_item(
        attrs={eve_assist_attr_id: 1},
        eff_ids=[eve_wdfg_effect_id],
        defeff_id=eve_wdfg_effect_id)
    eve_script_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_focused_warp_scrambling_script,
        cat_id=consts.EveEffCat.target)
    eve_script_id = client.mk_eve_item(eff_ids=[eve_script_effect_id], defeff_id=eve_script_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_assist_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(
        type_id=eve_wdfg_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_script_id)
    # Verification
    assert api_ship.update().attrs[eve_assist_attr_id].dogma == approx(1)
    # Action
    api_module.change_module(state=consts.ApiModuleState.online)
    # Verification
    assert api_ship.update().attrs[eve_assist_attr_id].dogma == approx(0)
