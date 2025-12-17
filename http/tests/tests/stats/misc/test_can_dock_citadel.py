from fw import check_no_field
from fw.api import FitStatsOptions, ItemStatsOptions


def test_ship_dock_modified(client, consts):
    eve_dock_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_docking)
    eve_warp_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warp_scramble_status)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_dock_attr_id)
    eve_mod_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_rig_id = client.mk_eve_item(attrs={eve_mod_attr_id: 1}, eff_ids=[eve_mod_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_dock_attr_id: 0, eve_warp_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(can_dock_citadel=True))
    assert api_fit_stats.can_dock_citadel is True
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(can_dock_citadel=True))
    assert api_ship_stats.can_dock_citadel is True
    # Action
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(can_dock_citadel=True))
    assert api_fit_stats.can_dock_citadel is False
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(can_dock_citadel=True))
    assert api_ship_stats.can_dock_citadel is False
    # Action
    api_rig.remove()
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(can_dock_citadel=True))
    assert api_fit_stats.can_dock_citadel is True
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(can_dock_citadel=True))
    assert api_ship_stats.can_dock_citadel is True


def test_ship_warp_modified(client, consts):
    eve_dock_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_docking)
    eve_warp_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warp_scramble_status)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_warp_attr_id)
    eve_mod_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_rig_id = client.mk_eve_item(attrs={eve_mod_attr_id: 1}, eff_ids=[eve_mod_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_dock_attr_id: 0, eve_warp_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(can_dock_citadel=True))
    assert api_fit_stats.can_dock_citadel is True
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(can_dock_citadel=True))
    assert api_ship_stats.can_dock_citadel is True
    # Action
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(can_dock_citadel=True))
    assert api_fit_stats.can_dock_citadel is False
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(can_dock_citadel=True))
    assert api_ship_stats.can_dock_citadel is False
    # Action
    api_rig.remove()
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(can_dock_citadel=True))
    assert api_fit_stats.can_dock_citadel is True
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(can_dock_citadel=True))
    assert api_ship_stats.can_dock_citadel is True


def test_ship_aggro(client, consts):
    eve_dock_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_docking)
    eve_warp_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warp_scramble_status)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active, is_offensive=True)
    eve_module_id = client.mk_eve_item(eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_dock_attr_id: 0, eve_warp_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.online)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(can_dock_citadel=True))
    assert api_fit_stats.can_dock_citadel is True
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(can_dock_citadel=True))
    assert api_ship_stats.can_dock_citadel is True
    # Action
    api_module.change_module(state=consts.ApiModuleState.active)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(can_dock_citadel=True))
    assert api_fit_stats.can_dock_citadel is False
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(can_dock_citadel=True))
    assert api_ship_stats.can_dock_citadel is False
    # Action
    api_module.change_module(state=consts.ApiModuleState.online)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(can_dock_citadel=True))
    assert api_fit_stats.can_dock_citadel is True
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(can_dock_citadel=True))
    assert api_ship_stats.can_dock_citadel is True


def test_ship_dock_values(client, consts):
    eve_dock_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_docking)
    eve_warp_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warp_scramble_status)
    eve_ship1_id = client.mk_eve_ship(attrs={eve_warp_attr_id: 0, eve_dock_attr_id: -100})
    eve_ship2_id = client.mk_eve_ship(attrs={eve_warp_attr_id: 0, eve_dock_attr_id: 0})
    eve_ship3_id = client.mk_eve_ship(attrs={eve_warp_attr_id: 0, eve_dock_attr_id: 100})
    eve_ship4_id = client.mk_eve_ship(attrs={eve_warp_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship1_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(can_dock_citadel=True))
    assert api_fit_stats.can_dock_citadel is True
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(can_dock_citadel=True))
    assert api_ship_stats.can_dock_citadel is True
    # Action
    api_ship.change_ship(type_id=eve_ship2_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(can_dock_citadel=True))
    assert api_fit_stats.can_dock_citadel is True
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(can_dock_citadel=True))
    assert api_ship_stats.can_dock_citadel is True
    # Action
    api_ship.change_ship(type_id=eve_ship3_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(can_dock_citadel=True))
    assert api_fit_stats.can_dock_citadel is False
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(can_dock_citadel=True))
    assert api_ship_stats.can_dock_citadel is False
    # Action
    api_ship.change_ship(type_id=eve_ship4_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(can_dock_citadel=True))
    assert api_fit_stats.can_dock_citadel is True
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(can_dock_citadel=True))
    assert api_ship_stats.can_dock_citadel is True


def test_ship_dock_no_attr(client, consts):
    eve_dock_attr_id = consts.EveAttr.disallow_docking
    eve_warp_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warp_scramble_status)
    eve_ship_id = client.mk_eve_ship(attrs={eve_dock_attr_id: 1, eve_warp_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification - value is ignored if attribute does not exist
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(can_dock_citadel=True))
    assert api_fit_stats.can_dock_citadel is True
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(can_dock_citadel=True))
    assert api_ship_stats.can_dock_citadel is True


def test_ship_warp_values(client, consts):
    eve_dock_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_docking)
    eve_warp_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warp_scramble_status)
    eve_ship1_id = client.mk_eve_ship(attrs={eve_dock_attr_id: 0, eve_warp_attr_id: -100})
    eve_ship2_id = client.mk_eve_ship(attrs={eve_dock_attr_id: 0, eve_warp_attr_id: 0})
    eve_ship3_id = client.mk_eve_ship(attrs={eve_dock_attr_id: 0, eve_warp_attr_id: 100})
    eve_ship4_id = client.mk_eve_ship(attrs={eve_dock_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship1_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(can_dock_citadel=True))
    assert api_fit_stats.can_dock_citadel is True
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(can_dock_citadel=True))
    assert api_ship_stats.can_dock_citadel is True
    # Action
    api_ship.change_ship(type_id=eve_ship2_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(can_dock_citadel=True))
    assert api_fit_stats.can_dock_citadel is True
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(can_dock_citadel=True))
    assert api_ship_stats.can_dock_citadel is True
    # Action
    api_ship.change_ship(type_id=eve_ship3_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(can_dock_citadel=True))
    assert api_fit_stats.can_dock_citadel is False
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(can_dock_citadel=True))
    assert api_ship_stats.can_dock_citadel is False
    # Action
    api_ship.change_ship(type_id=eve_ship4_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(can_dock_citadel=True))
    assert api_fit_stats.can_dock_citadel is True
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(can_dock_citadel=True))
    assert api_ship_stats.can_dock_citadel is True


def test_ship_warp_no_attr(client, consts):
    eve_dock_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_docking)
    eve_warp_attr_id = consts.EveAttr.warp_scramble_status
    eve_ship_id = client.mk_eve_ship(attrs={eve_dock_attr_id: 0, eve_warp_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification - value is ignored if attribute does not exist
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(can_dock_citadel=True))
    assert api_fit_stats.can_dock_citadel is True
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(can_dock_citadel=True))
    assert api_ship_stats.can_dock_citadel is True


def test_ship_absent(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.disallow_docking)
    client.mk_eve_attr(id_=consts.EveAttr.warp_scramble_status)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(can_dock_citadel=True))
    assert api_fit_stats.can_dock_citadel is None


def test_ship_not_loaded(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.disallow_docking)
    client.mk_eve_attr(id_=consts.EveAttr.warp_scramble_status)
    eve_ship_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(can_dock_citadel=True))
    assert api_fit_stats.can_dock_citadel is None
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(can_dock_citadel=True))
    assert api_ship_stats.can_dock_citadel is None


def test_struct(client, consts):
    eve_dock_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_docking)
    eve_warp_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warp_scramble_status)
    eve_struct_id = client.mk_eve_struct(attrs={eve_dock_attr_id: 1, eve_warp_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_struct_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(can_dock_citadel=True))
    assert api_fit_stats.can_dock_citadel is None
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(can_dock_citadel=True))
    assert api_ship_stats.can_dock_citadel is None


def test_other(client, consts):
    eve_dock_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_docking)
    eve_warp_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warp_scramble_status)
    eve_fighter_id = client.mk_eve_fighter(attrs={eve_dock_attr_id: 100, eve_warp_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id)
    # Verification
    api_drone_stats = api_fighter.get_stats(options=ItemStatsOptions(can_dock_citadel=True))
    assert api_drone_stats.can_dock_citadel is None


def test_not_requested(client, consts):
    eve_dock_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_docking)
    eve_warp_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warp_scramble_status)
    eve_ship_id = client.mk_eve_ship(attrs={eve_dock_attr_id: 100, eve_warp_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(can_dock_citadel=False))
    with check_no_field():
        api_fit_stats.can_dock_citadel  # noqa: B018
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(can_dock_citadel=False))
    with check_no_field():
        api_ship_stats.can_dock_citadel  # noqa: B018
