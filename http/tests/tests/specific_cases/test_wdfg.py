from tests import Effect, approx, check_no_field
from tests.fw.api import FitStatsOptions, ValOptions


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
    api_src_fit = api_sol.create_fit()
    api_tgt_fit = api_sol.create_fit()
    api_wdfg = api_src_fit.add_module(type_id=eve_wdfg_id, state=consts.ApiModuleState.active)
    api_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_wdfg.change_module(add_projs=[api_ship.id])
    # Verification
    assert api_ship.update().attrs[eve_sig_attr_id].dogma == approx(100)


def test_warp_dscript(client, consts):
    # Disruption script disables warp for target it's projected on
    eve_str_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warp_scramble_strength, def_val=0)
    eve_status_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warp_scramble_status, def_val=0)
    eve_immunity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_offensive_modifiers)
    eve_wdfg_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.warp_disrupt_sphere,
        cat_id=consts.EveEffCat.active,
        is_offensive=True)
    eve_wdfg_id = client.mk_eve_item(
        attrs={eve_str_attr_id: 100},
        eff_ids=[eve_wdfg_effect_id],
        defeff_id=eve_wdfg_effect_id)
    eve_script_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_focused_warp_disruption_script,
        cat_id=consts.EveEffCat.target,
        is_offensive=True)
    eve_script_id = client.mk_eve_item(
        eff_ids=[eve_script_effect_id],
        defeff_id=eve_script_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_status_attr_id: 0, eve_immunity_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_tgt_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_ship_id)
    api_wdfg = api_src_fit.add_module(type_id=eve_wdfg_id, state=consts.ApiModuleState.active)
    api_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_wdfg.change_module(add_projs=[api_ship.id])
    # Verification - without a script bubble disables warp and jump drive of the HIC itself and its
    # target
    api_src_val = api_src_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_src_val.passed is True
    api_src_stats = api_src_fit.get_stats(
        options=FitStatsOptions(can_warp=True, can_jump_drive=True, can_dock_citadel=True, can_tether=True))
    assert api_src_stats.can_warp is False
    assert api_src_stats.can_jump_drive is False
    assert api_src_stats.can_dock_citadel is False
    assert api_src_stats.can_tether is False
    api_tgt_stats = api_tgt_fit.get_stats(
        options=FitStatsOptions(can_warp=True, can_jump_drive=True, can_dock_citadel=True, can_tether=True))
    assert api_tgt_stats.can_warp is False
    assert api_tgt_stats.can_jump_drive is False
    assert api_tgt_stats.can_dock_citadel is True
    assert api_tgt_stats.can_tether is True
    # Action
    api_wdfg.change_module(charge_type_id=eve_script_id)
    # Verification - script should be successfully applied even to ewar immune target, HIC should be
    # able to warp
    api_src_val = api_src_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_src_val.passed is True
    api_src_stats = api_src_fit.get_stats(
        options=FitStatsOptions(can_warp=True, can_jump_drive=True, can_dock_citadel=True, can_tether=True))
    assert api_src_stats.can_warp is True
    assert api_src_stats.can_jump_drive is True
    assert api_src_stats.can_dock_citadel is False
    assert api_src_stats.can_tether is False
    api_tgt_stats = api_tgt_fit.get_stats(
        options=FitStatsOptions(can_warp=True, can_jump_drive=True, can_dock_citadel=True, can_tether=True))
    assert api_tgt_stats.can_warp is False
    assert api_tgt_stats.can_jump_drive is False
    assert api_tgt_stats.can_dock_citadel is False
    assert api_tgt_stats.can_tether is False
    # Action
    api_wdfg.change_module(charge_type_id=None)
    # Verification
    api_src_val = api_src_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_src_val.passed is True
    api_src_stats = api_src_fit.get_stats(
        options=FitStatsOptions(can_warp=True, can_jump_drive=True, can_dock_citadel=True, can_tether=True))
    assert api_src_stats.can_warp is False
    assert api_src_stats.can_jump_drive is False
    assert api_src_stats.can_dock_citadel is False
    assert api_src_stats.can_tether is False
    api_tgt_stats = api_tgt_fit.get_stats(
        options=FitStatsOptions(can_warp=True, can_jump_drive=True, can_dock_citadel=True, can_tether=True))
    assert api_tgt_stats.can_warp is False
    assert api_tgt_stats.can_jump_drive is False
    assert api_tgt_stats.can_dock_citadel is True
    assert api_tgt_stats.can_tether is True


def test_warp_sscript(client, consts):
    # Scrambling script disables warp for target it's projected on
    eve_str_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warp_scramble_strength, def_val=0)
    eve_status_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warp_scramble_status, def_val=0)
    eve_immunity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_offensive_modifiers)
    eve_wdfg_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.warp_disrupt_sphere,
        cat_id=consts.EveEffCat.active,
        is_offensive=True)
    eve_wdfg_id = client.mk_eve_item(
        attrs={eve_str_attr_id: 100},
        eff_ids=[eve_wdfg_effect_id],
        defeff_id=eve_wdfg_effect_id)
    eve_script_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_focused_warp_scrambling_script,
        cat_id=consts.EveEffCat.target,
        is_offensive=True)
    eve_script_id = client.mk_eve_item(
        eff_ids=[eve_script_effect_id],
        defeff_id=eve_script_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_status_attr_id: 0, eve_immunity_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_tgt_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_ship_id)
    api_wdfg = api_src_fit.add_module(type_id=eve_wdfg_id, state=consts.ApiModuleState.active)
    api_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_wdfg.change_module(add_projs=[api_ship.id])
    # Verification - without a script bubble still disables warp and jump drive
    api_src_val = api_src_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_src_val.passed is True
    api_src_stats = api_src_fit.get_stats(
        options=FitStatsOptions(can_warp=True, can_jump_drive=True, can_dock_citadel=True, can_tether=True))
    assert api_src_stats.can_warp is False
    assert api_src_stats.can_jump_drive is False
    assert api_src_stats.can_dock_citadel is False
    assert api_src_stats.can_tether is False
    api_tgt_stats = api_tgt_fit.get_stats(
        options=FitStatsOptions(can_warp=True, can_jump_drive=True, can_dock_citadel=True, can_tether=True))
    assert api_tgt_stats.can_warp is False
    assert api_tgt_stats.can_jump_drive is False
    assert api_tgt_stats.can_dock_citadel is True
    assert api_tgt_stats.can_tether is True
    # Action
    api_wdfg.change_module(charge_type_id=eve_script_id)
    # Verification - script should be successfully applied even to ewar immune target
    api_src_val = api_src_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_src_val.passed is True
    api_src_stats = api_src_fit.get_stats(
        options=FitStatsOptions(can_warp=True, can_jump_drive=True, can_dock_citadel=True, can_tether=True))
    assert api_src_stats.can_warp is True
    assert api_src_stats.can_jump_drive is True
    assert api_src_stats.can_dock_citadel is False
    assert api_src_stats.can_tether is False
    api_tgt_stats = api_tgt_fit.get_stats(
        options=FitStatsOptions(can_warp=True, can_jump_drive=True, can_dock_citadel=True, can_tether=True))
    assert api_tgt_stats.can_warp is False
    assert api_tgt_stats.can_jump_drive is False
    assert api_tgt_stats.can_dock_citadel is False
    assert api_tgt_stats.can_tether is False
    # Action
    api_wdfg.change_module(charge_type_id=None)
    # Verification
    api_src_val = api_src_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_src_val.passed is True
    api_src_stats = api_src_fit.get_stats(
        options=FitStatsOptions(can_warp=True, can_jump_drive=True, can_dock_citadel=True, can_tether=True))
    assert api_src_stats.can_warp is False
    assert api_src_stats.can_jump_drive is False
    assert api_src_stats.can_dock_citadel is False
    assert api_src_stats.can_tether is False
    api_tgt_stats = api_tgt_fit.get_stats(
        options=FitStatsOptions(can_warp=True, can_jump_drive=True, can_dock_citadel=True, can_tether=True))
    assert api_tgt_stats.can_warp is False
    assert api_tgt_stats.can_jump_drive is False
    assert api_tgt_stats.can_dock_citadel is True
    assert api_tgt_stats.can_tether is True


def test_gate_dscript(client, consts):
    # Disruption script disables gate jumps for target capitals it's projected onto. The way it
    # works on capitals only is that caps have base strength set to 0 while default value is -1000
    client.mk_eve_attr(id_=consts.EveAttr.gate_scramble_strength, def_val=1)
    eve_status_attr_id = client.mk_eve_attr(id_=consts.EveAttr.gate_scramble_status, def_val=-1000)
    eve_immunity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_offensive_modifiers)
    eve_wdfg_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.warp_disrupt_sphere,
        cat_id=consts.EveEffCat.active,
        is_offensive=True)
    eve_wdfg_id = client.mk_eve_item(eff_ids=[eve_wdfg_effect_id], defeff_id=eve_wdfg_effect_id)
    eve_script_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_focused_warp_disruption_script,
        cat_id=consts.EveEffCat.target,
        is_offensive=True)
    eve_script_id = client.mk_eve_item(
        eff_ids=[eve_script_effect_id],
        defeff_id=eve_script_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_status_attr_id: 0, eve_immunity_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_tgt_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_ship_id)
    api_wdfg = api_src_fit.add_module(type_id=eve_wdfg_id, state=consts.ApiModuleState.active)
    api_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_wdfg.change_module(add_projs=[api_ship.id])
    # Verification
    api_src_val = api_src_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_src_val.passed is True
    api_src_stats = api_src_fit.get_stats(options=FitStatsOptions(can_jump_gate=True))
    assert api_src_stats.can_jump_gate is False
    api_tgt_stats = api_tgt_fit.get_stats(options=FitStatsOptions(can_jump_gate=True))
    assert api_tgt_stats.can_jump_gate is True
    # Action
    api_wdfg.change_module(charge_type_id=eve_script_id)
    # Verification - script should be successfully applied even to ewar immune target
    api_src_val = api_src_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_src_val.passed is True
    api_src_stats = api_src_fit.get_stats(options=FitStatsOptions(can_jump_gate=True))
    assert api_src_stats.can_jump_gate is False
    api_tgt_stats = api_tgt_fit.get_stats(options=FitStatsOptions(can_jump_gate=True))
    assert api_tgt_stats.can_jump_gate is False
    # Action
    api_wdfg.change_module(charge_type_id=None)
    # Verification
    api_src_val = api_src_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_src_val.passed is True
    api_src_stats = api_src_fit.get_stats(options=FitStatsOptions(can_jump_gate=True))
    assert api_src_stats.can_jump_gate is False
    api_tgt_stats = api_tgt_fit.get_stats(options=FitStatsOptions(can_jump_gate=True))
    assert api_tgt_stats.can_jump_gate is True


def test_gate_sscript(client, consts):
    # Scrambling script disables gate jumps for target capitals it's projected onto. The way it
    # works on capitals only is that caps have base strength set to 0 while default value is -1000
    client.mk_eve_attr(id_=consts.EveAttr.gate_scramble_strength, def_val=1)
    eve_status_attr_id = client.mk_eve_attr(id_=consts.EveAttr.gate_scramble_status, def_val=-1000)
    eve_immunity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_offensive_modifiers)
    eve_wdfg_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.warp_disrupt_sphere,
        cat_id=consts.EveEffCat.active,
        is_offensive=True)
    eve_wdfg_id = client.mk_eve_item(eff_ids=[eve_wdfg_effect_id], defeff_id=eve_wdfg_effect_id)
    eve_script_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_focused_warp_scrambling_script,
        cat_id=consts.EveEffCat.target,
        is_offensive=True)
    eve_script_id = client.mk_eve_item(
        eff_ids=[eve_script_effect_id],
        defeff_id=eve_script_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_status_attr_id: 0, eve_immunity_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_tgt_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_ship_id)
    api_wdfg = api_src_fit.add_module(type_id=eve_wdfg_id, state=consts.ApiModuleState.active)
    api_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_wdfg.change_module(add_projs=[api_ship.id])
    # Verification
    api_src_val = api_src_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_src_val.passed is True
    api_src_stats = api_src_fit.get_stats(options=FitStatsOptions(can_jump_gate=True))
    assert api_src_stats.can_jump_gate is False
    api_tgt_stats = api_tgt_fit.get_stats(options=FitStatsOptions(can_jump_gate=True))
    assert api_tgt_stats.can_jump_gate is True
    # Action
    api_wdfg.change_module(charge_type_id=eve_script_id)
    # Verification - script should be successfully applied even to ewar immune target
    api_src_val = api_src_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_src_val.passed is True
    api_src_stats = api_src_fit.get_stats(options=FitStatsOptions(can_jump_gate=True))
    assert api_src_stats.can_jump_gate is False
    api_tgt_stats = api_tgt_fit.get_stats(options=FitStatsOptions(can_jump_gate=True))
    assert api_tgt_stats.can_jump_gate is False
    # Action
    api_wdfg.change_module(charge_type_id=None)
    # Verification
    api_src_val = api_src_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_src_val.passed is True
    api_src_stats = api_src_fit.get_stats(options=FitStatsOptions(can_jump_gate=True))
    assert api_src_stats.can_jump_gate is False
    api_tgt_stats = api_tgt_fit.get_stats(options=FitStatsOptions(can_jump_gate=True))
    assert api_tgt_stats.can_jump_gate is True


def test_block_module_mwd_dscript(client, consts):
    # Disruption script doesn't disable micro warp drives
    eve_skill_id = client.mk_eve_item(id_=consts.EveItem.high_speed_maneuvering)
    eve_str_attr_id = client.mk_eve_attr(id_=consts.EveAttr.activation_blocked_strength, def_val=0)
    eve_block_attr_id = client.mk_eve_attr(id_=consts.EveAttr.activation_blocked, def_val=0)
    eve_immunity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_offensive_modifiers)
    eve_wdfg_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.warp_disrupt_sphere,
        cat_id=consts.EveEffCat.active,
        is_offensive=True)
    eve_wdfg_id = client.mk_eve_item(
        attrs={eve_str_attr_id: 1},
        eff_ids=[eve_wdfg_effect_id],
        defeff_id=eve_wdfg_effect_id)
    eve_script_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_focused_warp_disruption_script,
        cat_id=consts.EveEffCat.target,
        is_offensive=True)
    eve_script_id = client.mk_eve_item(eff_ids=[eve_script_effect_id], defeff_id=eve_script_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_immunity_attr_id: 1})
    eve_mwd_id = client.mk_eve_item(attrs={eve_block_attr_id: 0}, srqs={eve_skill_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_tgt_fit = api_sol.create_fit()
    api_wdfg = api_src_fit.add_module(type_id=eve_wdfg_id, state=consts.ApiModuleState.active)
    api_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_tgt_fit.add_module(type_id=eve_mwd_id, state=consts.ApiModuleState.active)
    api_wdfg.change_module(add_projs=[api_ship.id])
    # Verification
    api_src_val = api_src_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_src_val.passed is True
    api_val = api_tgt_fit.validate(options=ValOptions(activation_blocked=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_wdfg.change_module(charge_type_id=eve_script_id)
    # Verification - script should be successfully applied even to ewar immune target
    api_src_val = api_src_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_src_val.passed is True
    api_val = api_tgt_fit.validate(options=ValOptions(activation_blocked=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_wdfg.change_module(charge_type_id=None)
    # Verification
    api_src_val = api_src_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_src_val.passed is True
    api_val = api_tgt_fit.validate(options=ValOptions(activation_blocked=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_block_module_mwd_sscript(client, consts):
    # Scrambling script disables micro warp drives
    eve_skill_id = client.mk_eve_item(id_=consts.EveItem.high_speed_maneuvering)
    eve_str_attr_id = client.mk_eve_attr(id_=consts.EveAttr.activation_blocked_strength, def_val=0)
    eve_block_attr_id = client.mk_eve_attr(id_=consts.EveAttr.activation_blocked, def_val=0)
    eve_immunity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_offensive_modifiers)
    eve_wdfg_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.warp_disrupt_sphere,
        cat_id=consts.EveEffCat.active,
        is_offensive=True)
    eve_wdfg_id = client.mk_eve_item(
        attrs={eve_str_attr_id: 1},
        eff_ids=[eve_wdfg_effect_id],
        defeff_id=eve_wdfg_effect_id)
    eve_script_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_focused_warp_scrambling_script,
        cat_id=consts.EveEffCat.target,
        is_offensive=True)
    eve_script_id = client.mk_eve_item(eff_ids=[eve_script_effect_id], defeff_id=eve_script_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_immunity_attr_id: 1})
    eve_mwd_id = client.mk_eve_item(attrs={eve_block_attr_id: 0}, srqs={eve_skill_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_tgt_fit = api_sol.create_fit()
    api_wdfg = api_src_fit.add_module(type_id=eve_wdfg_id, state=consts.ApiModuleState.active)
    api_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_mwd = api_tgt_fit.add_module(type_id=eve_mwd_id, state=consts.ApiModuleState.active)
    api_wdfg.change_module(add_projs=[api_ship.id])
    # Verification
    api_src_val = api_src_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_src_val.passed is True
    api_val = api_tgt_fit.validate(options=ValOptions(activation_blocked=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_wdfg.change_module(charge_type_id=eve_script_id)
    # Verification - script should be successfully applied even to ewar immune target
    api_src_val = api_src_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_src_val.passed is True
    api_val = api_tgt_fit.validate(options=ValOptions(activation_blocked=True))
    assert api_val.passed is False
    assert api_val.details.activation_blocked == [api_mwd.id]
    # Action
    api_wdfg.change_module(charge_type_id=None)
    # Verification
    api_src_val = api_src_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_src_val.passed is True
    api_val = api_tgt_fit.validate(options=ValOptions(activation_blocked=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_block_module_mjd_dscript(client, consts):
    # Disruption script disables micro jump drives
    eve_skill_sub_id = client.mk_eve_item(id_=consts.EveItem.micro_jump_drive_operation)
    # Capital MJFG doesn't use regular skill, so test capital skill separately
    eve_skill_cap_id = client.mk_eve_item(id_=consts.EveItem.capital_micro_jump_drive_operation)
    eve_str_attr_id = client.mk_eve_attr(id_=consts.EveAttr.activation_blocked_strength, def_val=0)
    eve_block_attr_id = client.mk_eve_attr(id_=consts.EveAttr.activation_blocked, def_val=0)
    eve_immunity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_offensive_modifiers)
    eve_wdfg_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.warp_disrupt_sphere,
        cat_id=consts.EveEffCat.active,
        is_offensive=True)
    eve_wdfg_id = client.mk_eve_item(
        attrs={eve_str_attr_id: 1},
        eff_ids=[eve_wdfg_effect_id],
        defeff_id=eve_wdfg_effect_id)
    eve_script_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_focused_warp_disruption_script,
        cat_id=consts.EveEffCat.target,
        is_offensive=True)
    eve_script_id = client.mk_eve_item(eff_ids=[eve_script_effect_id], defeff_id=eve_script_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_immunity_attr_id: 1})
    eve_mjd_sub_id = client.mk_eve_item(attrs={eve_block_attr_id: 0}, srqs={eve_skill_sub_id: 1})
    eve_mjd_cap_id = client.mk_eve_item(attrs={eve_block_attr_id: 0}, srqs={eve_skill_cap_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_tgt_fit = api_sol.create_fit()
    api_wdfg = api_src_fit.add_module(type_id=eve_wdfg_id, state=consts.ApiModuleState.active)
    api_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_mjd_sub = api_tgt_fit.add_module(type_id=eve_mjd_sub_id, state=consts.ApiModuleState.active)
    api_mjd_cap = api_tgt_fit.add_module(type_id=eve_mjd_cap_id, state=consts.ApiModuleState.active)
    api_wdfg.change_module(add_projs=[api_ship.id])
    # Verification
    api_src_val = api_src_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_src_val.passed is True
    api_val = api_tgt_fit.validate(options=ValOptions(activation_blocked=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_wdfg.change_module(charge_type_id=eve_script_id)
    # Verification - script should be successfully applied even to ewar immune target
    api_src_val = api_src_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_src_val.passed is True
    api_val = api_tgt_fit.validate(options=ValOptions(activation_blocked=True))
    assert api_val.passed is False
    assert api_val.details.activation_blocked == sorted([api_mjd_sub.id, api_mjd_cap.id])
    # Action
    api_wdfg.change_module(charge_type_id=None)
    # Verification
    api_src_val = api_src_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_src_val.passed is True
    api_val = api_tgt_fit.validate(options=ValOptions(activation_blocked=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_block_module_mjd_sscript(client, consts):
    # Scrambling script disables micro jump drives
    eve_skill_sub_id = client.mk_eve_item(id_=consts.EveItem.micro_jump_drive_operation)
    # Capital MJFG doesn't use regular skill, so test capital skill separately
    eve_skill_cap_id = client.mk_eve_item(id_=consts.EveItem.capital_micro_jump_drive_operation)
    eve_str_attr_id = client.mk_eve_attr(id_=consts.EveAttr.activation_blocked_strength, def_val=0)
    eve_block_attr_id = client.mk_eve_attr(id_=consts.EveAttr.activation_blocked, def_val=0)
    eve_immunity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_offensive_modifiers)
    eve_wdfg_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.warp_disrupt_sphere,
        cat_id=consts.EveEffCat.active,
        is_offensive=True)
    eve_wdfg_id = client.mk_eve_item(
        attrs={eve_str_attr_id: 1},
        eff_ids=[eve_wdfg_effect_id],
        defeff_id=eve_wdfg_effect_id)
    eve_script_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_focused_warp_scrambling_script,
        cat_id=consts.EveEffCat.target,
        is_offensive=True)
    eve_script_id = client.mk_eve_item(eff_ids=[eve_script_effect_id], defeff_id=eve_script_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_immunity_attr_id: 1})
    eve_mjd_sub_id = client.mk_eve_item(attrs={eve_block_attr_id: 0}, srqs={eve_skill_sub_id: 1})
    eve_mjd_cap_id = client.mk_eve_item(attrs={eve_block_attr_id: 0}, srqs={eve_skill_cap_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_tgt_fit = api_sol.create_fit()
    api_wdfg = api_src_fit.add_module(type_id=eve_wdfg_id, state=consts.ApiModuleState.active)
    api_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_mjd_sub = api_tgt_fit.add_module(type_id=eve_mjd_sub_id, state=consts.ApiModuleState.active)
    api_mjd_cap = api_tgt_fit.add_module(type_id=eve_mjd_cap_id, state=consts.ApiModuleState.active)
    api_wdfg.change_module(add_projs=[api_ship.id])
    # Verification
    api_src_val = api_src_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_src_val.passed is True
    api_val = api_tgt_fit.validate(options=ValOptions(activation_blocked=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_wdfg.change_module(charge_type_id=eve_script_id)
    # Verification - script should be successfully applied even to ewar immune target
    api_src_val = api_src_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_src_val.passed is True
    api_val = api_tgt_fit.validate(options=ValOptions(activation_blocked=True))
    assert api_val.passed is False
    assert api_val.details.activation_blocked == sorted([api_mjd_sub.id, api_mjd_cap.id])
    # Action
    api_wdfg.change_module(charge_type_id=None)
    # Verification
    api_src_val = api_src_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_src_val.passed is True
    api_val = api_tgt_fit.validate(options=ValOptions(activation_blocked=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_block_fighter_mwd_mjd_dscript(client, consts):
    # As of 2024-11-05, even disruption script disables fighter MWD and MJD abilities
    eve_wdfg_effect_id = client.mk_eve_effect(id_=consts.EveEffect.warp_disrupt_sphere, cat_id=consts.EveEffCat.active)
    eve_wdfg_id = client.mk_eve_item(eff_ids=[eve_wdfg_effect_id], defeff_id=eve_wdfg_effect_id)
    eve_script_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_focused_warp_disruption_script,
        cat_id=consts.EveEffCat.target)
    eve_script_id = client.mk_eve_item(eff_ids=[eve_script_effect_id], defeff_id=eve_script_effect_id)
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
    api_src_fit = api_sol.create_fit()
    api_tgt_fit = api_sol.create_fit()
    api_wdfg = api_src_fit.add_module(type_id=eve_wdfg_id, state=consts.ApiModuleState.active)
    api_fighter = api_tgt_fit.add_fighter(type_id=eve_fighter_id)
    api_fighter.change_fighter(effect_modes={
        api_ftr_mwd_effect_id: consts.ApiEffMode.force_run,
        api_ftr_mjd_effect_id: consts.ApiEffMode.force_run})
    api_wdfg.change_module(add_projs=[api_fighter.id])
    # Verification
    api_val = api_tgt_fit.validate(options=ValOptions(effect_stopper=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_wdfg.change_module(charge_type_id=eve_script_id)
    # Verification
    api_val = api_tgt_fit.validate(options=ValOptions(effect_stopper=True))
    assert api_val.passed is False
    assert api_val.details.effect_stopper == {api_fighter.id: sorted([api_ftr_mwd_effect_id, api_ftr_mjd_effect_id])}


def test_block_fighter_mwd_mjd_sscript(client, consts):
    eve_wdfg_effect_id = client.mk_eve_effect(id_=consts.EveEffect.warp_disrupt_sphere, cat_id=consts.EveEffCat.active)
    eve_wdfg_id = client.mk_eve_item(eff_ids=[eve_wdfg_effect_id], defeff_id=eve_wdfg_effect_id)
    eve_script_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_focused_warp_scrambling_script,
        cat_id=consts.EveEffCat.target)
    eve_script_id = client.mk_eve_item(eff_ids=[eve_script_effect_id], defeff_id=eve_script_effect_id)
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
    api_src_fit = api_sol.create_fit()
    api_tgt_fit = api_sol.create_fit()
    api_wdfg = api_src_fit.add_module(type_id=eve_wdfg_id, state=consts.ApiModuleState.active)
    api_fighter = api_tgt_fit.add_fighter(type_id=eve_fighter_id)
    api_fighter.change_fighter(effect_modes={
        api_ftr_mwd_effect_id: consts.ApiEffMode.force_run,
        api_ftr_mjd_effect_id: consts.ApiEffMode.force_run})
    api_wdfg.change_module(add_projs=[api_fighter.id])
    # Verification
    api_val = api_tgt_fit.validate(options=ValOptions(effect_stopper=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_wdfg.change_module(charge_type_id=eve_script_id)
    # Verification
    api_val = api_tgt_fit.validate(options=ValOptions(effect_stopper=True))
    assert api_val.passed is False
    assert api_val.details.effect_stopper == {api_fighter.id: sorted([api_ftr_mwd_effect_id, api_ftr_mjd_effect_id])}


def test_range_bubble(client, consts):
    eve_range_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warp_scramble_range, def_val=0)
    eve_str_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warp_scramble_strength, def_val=0)
    eve_status_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warp_scramble_status, def_val=0)
    eve_wdfg_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.warp_disrupt_sphere,
        cat_id=consts.EveEffCat.active,
        range_attr_id=eve_range_attr_id)
    eve_wdfg_id = client.mk_eve_item(
        attrs={eve_range_attr_id: 20000, eve_str_attr_id: 100},
        eff_ids=[eve_wdfg_effect_id],
        defeff_id=eve_wdfg_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_status_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_ship_id, coordinates=(0, 0, 0))
    api_tgt_fit = api_sol.create_fit()
    api_wdfg = api_src_fit.add_module(type_id=eve_wdfg_id, state=consts.ApiModuleState.active)
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id, coordinates=(19999, 0, 0))
    api_wdfg.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_tgt_stats = api_tgt_fit.get_stats(
        options=FitStatsOptions(can_warp=True, can_jump_drive=True, can_dock_citadel=True, can_tether=True))
    assert api_tgt_stats.can_warp is False
    assert api_tgt_stats.can_jump_drive is False
    assert api_tgt_stats.can_dock_citadel is True
    assert api_tgt_stats.can_tether is True
    # Action
    api_tgt_ship.change_ship(coordinates=(20001, 0, 0))
    # Verification
    api_tgt_stats = api_tgt_fit.get_stats(
        options=FitStatsOptions(can_warp=True, can_jump_drive=True, can_dock_citadel=True, can_tether=True))
    assert api_tgt_stats.can_warp is True
    assert api_tgt_stats.can_jump_drive is True
    assert api_tgt_stats.can_dock_citadel is True
    assert api_tgt_stats.can_tether is True


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
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_ship_id, coordinates=(0, 0, 0))
    api_tgt_fit = api_sol.create_fit()
    api_wdfg = api_src_fit.add_module(
        type_id=eve_wdfg_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_script_id)
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id, coordinates=(29999, 0, 0))
    api_wdfg.change_module(add_projs=[api_tgt_ship.id])
    # Verification - range should be 30k (20k base from module +50% from script)
    api_tgt_stats = api_tgt_fit.get_stats(
        options=FitStatsOptions(can_warp=True, can_jump_drive=True, can_dock_citadel=True, can_tether=True))
    assert api_tgt_stats.can_warp is False
    assert api_tgt_stats.can_jump_drive is False
    assert api_tgt_stats.can_dock_citadel is False
    assert api_tgt_stats.can_tether is False
    # Action
    api_tgt_ship.change_ship(coordinates=(30001, 0, 0))
    # Verification
    api_tgt_stats = api_tgt_fit.get_stats(
        options=FitStatsOptions(can_warp=True, can_jump_drive=True, can_dock_citadel=True, can_tether=True))
    assert api_tgt_stats.can_warp is True
    assert api_tgt_stats.can_jump_drive is True
    assert api_tgt_stats.can_dock_citadel is True
    assert api_tgt_stats.can_tether is True


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
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_ship_id, coordinates=(0, 0, 0))
    api_tgt_fit = api_sol.create_fit()
    api_wdfg = api_src_fit.add_module(
        type_id=eve_wdfg_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_script_id)
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id, coordinates=(16000, 0, 0))
    api_wdfg.change_module(add_projs=[api_tgt_ship.id])
    # Verification - range should be 16k (20k base from module -20% from script)
    assert api_tgt_ship.update().attrs[eve_status_attr_id].dogma == approx(100)
    # Action
    api_tgt_ship.change_ship(coordinates=(16001, 0, 0))
    # Verification
    assert api_tgt_ship.update().attrs[eve_status_attr_id].dogma == approx(0)


def test_assist_bubble_local(client, consts):
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
    api_module = api_fit.add_module(type_id=eve_wdfg_id, state=consts.ApiModuleState.active)
    # Verification
    assert api_ship.update().attrs[eve_assist_attr_id].dogma == approx(1)
    # Action
    api_module.change_module(state=consts.ApiModuleState.online)
    # Verification
    assert api_ship.update().attrs[eve_assist_attr_id].dogma == approx(0)


def test_assist_bubble_projected(client, consts):
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
    api_src_fit = api_sol.create_fit()
    api_tgt_fit = api_sol.create_fit()
    api_wdfg = api_src_fit.add_module(type_id=eve_wdfg_id, state=consts.ApiModuleState.active)
    api_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_wdfg.change_module(add_projs=[api_ship.id])
    # Verification
    assert api_ship.update().attrs[eve_assist_attr_id].dogma == approx(0)


def test_assist_dscript(client, consts):
    # WDFG disables assistance even when it's running with a script
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
    # WDFG disables assistance even when it's running with a script
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
