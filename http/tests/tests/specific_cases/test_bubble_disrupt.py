from tests.fw.api import ItemStatsOptions


def test_module_self(client, consts):
    # Shouldn't affect ship of owner even if activated
    eve_warp_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warp_scramble_status)
    eve_tether_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_tethering)
    client.mk_eve_buff(
        id_=consts.EveBuff.disallow_tether,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.mod_add,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_tether_attr_id)])
    eve_charge1_id = client.mk_eve_item(id_=consts.EveItem.warp_disrupt_probe)
    eve_charge2_id = client.mk_eve_item(id_=consts.EveItem.surgical_warp_disrupt_probe)
    eve_module_effect_id = client.mk_eve_effect(id_=consts.EveEffect.use_missiles, cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_item(eff_ids=[eve_module_effect_id], defeff_id=eve_module_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_warp_attr_id: 0, eve_tether_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge1_id)
    # Verification - launching a warp bubble breaks tether, but does not do anything else (tested on
    # 2025-12-13)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(
        can_warp=True,
        can_jump_gate=True,
        can_jump_drive=True,
        can_dock_station=True,
        can_dock_citadel=True,
        can_tether=True))
    assert api_ship_stats.can_warp is True
    assert api_ship_stats.can_jump_gate is True
    assert api_ship_stats.can_jump_drive is True
    assert api_ship_stats.can_dock_station is True
    assert api_ship_stats.can_dock_citadel is True
    assert api_ship_stats.can_tether is False
    # Action
    api_module.change_module(charge_type_id=eve_charge2_id)
    # Verification - assuming surgical probe is the same
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(
        can_warp=True,
        can_jump_gate=True,
        can_jump_drive=True,
        can_dock_station=True,
        can_dock_citadel=True,
        can_tether=True))
    assert api_ship_stats.can_warp is True
    assert api_ship_stats.can_jump_gate is True
    assert api_ship_stats.can_jump_drive is True
    assert api_ship_stats.can_dock_station is True
    assert api_ship_stats.can_dock_citadel is True
    assert api_ship_stats.can_tether is False


def test_module_charge_uncharge(client, consts):
    eve_tether_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_tethering)
    eve_charge1_id = client.mk_eve_item(id_=consts.EveItem.warp_disrupt_probe)
    eve_charge2_id = client.mk_eve_item(id_=consts.EveItem.surgical_warp_disrupt_probe)
    eve_module_effect_id = client.mk_eve_effect(id_=consts.EveEffect.use_missiles, cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_item(eff_ids=[eve_module_effect_id], defeff_id=eve_module_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_tether_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_ship_id)
    api_affector_fit = api_sol.create_fit()
    api_affector_module = api_affector_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_affector_module.change_module(add_projs=[api_affectee_ship.id])
    # Verification
    api_afectee_ship_stats = api_affectee_ship.get_stats(options=ItemStatsOptions(
        can_warp=True,
        can_jump_gate=True,
        can_jump_drive=True,
        can_dock_station=True,
        can_dock_citadel=True,
        can_tether=True))
    assert api_afectee_ship_stats.can_warp is True
    assert api_afectee_ship_stats.can_jump_gate is True
    assert api_afectee_ship_stats.can_jump_drive is True
    assert api_afectee_ship_stats.can_dock_station is True
    assert api_afectee_ship_stats.can_dock_citadel is True
    assert api_afectee_ship_stats.can_tether is True
    # Action
    api_affector_module.change_module(charge_type_id=eve_charge1_id)
    # Verification
    api_afectee_ship_stats = api_affectee_ship.get_stats(options=ItemStatsOptions(
        can_warp=True,
        can_jump_gate=True,
        can_jump_drive=True,
        can_dock_station=True,
        can_dock_citadel=True,
        can_tether=True))
    assert api_afectee_ship_stats.can_warp is False
    assert api_afectee_ship_stats.can_jump_gate is True
    assert api_afectee_ship_stats.can_jump_drive is False
    assert api_afectee_ship_stats.can_dock_station is True
    assert api_afectee_ship_stats.can_dock_citadel is True
    assert api_afectee_ship_stats.can_tether is True
    # Action
    api_affector_module.change_module(charge_type_id=eve_charge2_id)
    # Verification
    api_afectee_ship_stats = api_affectee_ship.get_stats(options=ItemStatsOptions(
        can_warp=True,
        can_jump_gate=True,
        can_jump_drive=True,
        can_dock_station=True,
        can_dock_citadel=True,
        can_tether=True))
    assert api_afectee_ship_stats.can_warp is False
    assert api_afectee_ship_stats.can_jump_gate is True
    assert api_afectee_ship_stats.can_jump_drive is False
    assert api_afectee_ship_stats.can_dock_station is True
    assert api_afectee_ship_stats.can_dock_citadel is True
    assert api_afectee_ship_stats.can_tether is True
    # Action
    api_affector_module.change_module(charge_type_id=None)
    # Verification
    api_afectee_ship_stats = api_affectee_ship.get_stats(options=ItemStatsOptions(
        can_warp=True,
        can_jump_gate=True,
        can_jump_drive=True,
        can_dock_station=True,
        can_dock_citadel=True,
        can_tether=True))
    assert api_afectee_ship_stats.can_warp is True
    assert api_afectee_ship_stats.can_jump_gate is True
    assert api_afectee_ship_stats.can_jump_drive is True
    assert api_afectee_ship_stats.can_dock_station is True
    assert api_afectee_ship_stats.can_dock_citadel is True
    assert api_afectee_ship_stats.can_tether is True
