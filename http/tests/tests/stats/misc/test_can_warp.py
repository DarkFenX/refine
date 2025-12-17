from fw import check_no_field
from fw.api import FitStatsOptions, ItemStatsOptions


def test_ship_warp_modified(client, consts):
    eve_warp_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warp_scramble_status)
    eve_speed_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_warp_attr_id)
    eve_mod_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_rig_id = client.mk_eve_item(attrs={eve_mod_attr_id: 1}, eff_ids=[eve_mod_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_warp_attr_id: 0, eve_speed_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(can_warp=True))
    assert api_fit_stats.can_warp is True
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(can_warp=True))
    assert api_ship_stats.can_warp is True
    # Action
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(can_warp=True))
    assert api_fit_stats.can_warp is False
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(can_warp=True))
    assert api_ship_stats.can_warp is False
    # Action
    api_rig.remove()
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(can_warp=True))
    assert api_fit_stats.can_warp is True
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(can_warp=True))
    assert api_ship_stats.can_warp is True


def test_ship_warp_values(client, consts):
    eve_warp_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warp_scramble_status)
    eve_speed_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_ship1_id = client.mk_eve_ship(attrs={eve_speed_attr_id: 1, eve_warp_attr_id: -100})
    eve_ship2_id = client.mk_eve_ship(attrs={eve_speed_attr_id: 1, eve_warp_attr_id: 0})
    eve_ship3_id = client.mk_eve_ship(attrs={eve_speed_attr_id: 1, eve_warp_attr_id: 100})
    eve_ship4_id = client.mk_eve_ship(attrs={eve_speed_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship1_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(can_warp=True))
    assert api_fit_stats.can_warp is True
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(can_warp=True))
    assert api_ship_stats.can_warp is True
    # Action
    api_ship.change_ship(type_id=eve_ship2_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(can_warp=True))
    assert api_fit_stats.can_warp is True
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(can_warp=True))
    assert api_ship_stats.can_warp is True
    # Action
    api_ship.change_ship(type_id=eve_ship3_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(can_warp=True))
    assert api_fit_stats.can_warp is False
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(can_warp=True))
    assert api_ship_stats.can_warp is False
    # Action
    api_ship.change_ship(type_id=eve_ship4_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(can_warp=True))
    assert api_fit_stats.can_warp is True
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(can_warp=True))
    assert api_ship_stats.can_warp is True


def test_ship_warp_no_attr(client, consts):
    eve_warp_attr_id = consts.EveAttr.warp_scramble_status
    eve_speed_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_ship_id = client.mk_eve_ship(attrs={eve_warp_attr_id: 1, eve_speed_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification - value is ignored if attribute does not exist
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(can_warp=True))
    assert api_fit_stats.can_warp is True
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(can_warp=True))
    assert api_ship_stats.can_warp is True


def test_ship_speed_values(client, consts):
    eve_warp_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warp_scramble_status)
    eve_speed_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_ship1_id = client.mk_eve_ship(attrs={eve_warp_attr_id: 0, eve_speed_attr_id: -100})
    eve_ship2_id = client.mk_eve_ship(attrs={eve_warp_attr_id: 0, eve_speed_attr_id: 0})
    eve_ship3_id = client.mk_eve_ship(attrs={eve_warp_attr_id: 0, eve_speed_attr_id: 1})
    eve_ship4_id = client.mk_eve_ship(attrs={eve_warp_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship1_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(can_warp=True))
    assert api_fit_stats.can_warp is False
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(can_warp=True))
    assert api_ship_stats.can_warp is False
    # Action
    api_ship.change_ship(type_id=eve_ship2_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(can_warp=True))
    assert api_fit_stats.can_warp is False
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(can_warp=True))
    assert api_ship_stats.can_warp is False
    # Action
    api_ship.change_ship(type_id=eve_ship3_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(can_warp=True))
    assert api_fit_stats.can_warp is True
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(can_warp=True))
    assert api_ship_stats.can_warp is True
    # Action
    api_ship.change_ship(type_id=eve_ship4_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(can_warp=True))
    assert api_fit_stats.can_warp is False
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(can_warp=True))
    assert api_ship_stats.can_warp is False


def test_ship_speed_no_attr(client, consts):
    eve_warp_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warp_scramble_status)
    eve_speed_attr_id = consts.EveAttr.max_velocity
    eve_ship_id = client.mk_eve_ship(attrs={eve_warp_attr_id: 0, eve_speed_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification - value is ignored if attribute does not exist
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(can_warp=True))
    assert api_fit_stats.can_warp is True
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(can_warp=True))
    assert api_ship_stats.can_warp is True


def test_ship_absent(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.warp_scramble_status)
    client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(can_warp=True))
    assert api_fit_stats.can_warp is None


def test_ship_not_loaded(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.warp_scramble_status)
    client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_ship_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(can_warp=True))
    assert api_fit_stats.can_warp is None
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(can_warp=True))
    assert api_ship_stats.can_warp is None


def test_fighter_warp_modified(client, consts):
    eve_warp_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warp_scramble_status)
    eve_speed_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_max_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.mod_add,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_warp_attr_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.buff_everything, cat_id=consts.EveEffCat.active)
    eve_fw_effect_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 100},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_fighter_id = client.mk_eve_fighter(
        attrs={eve_warp_attr_id: 0, eve_speed_attr_id: 1, eve_max_count_attr_id: 9})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id)
    # Verification
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(can_warp=True))
    assert api_fighter_stats.can_warp is True
    # Action
    api_fw_effect = api_fit.add_fw_effect(type_id=eve_fw_effect_id)
    # Verification
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(can_warp=True))
    assert api_fighter_stats.can_warp is False
    # Action
    api_fw_effect.remove()
    # Verification
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(can_warp=True))
    assert api_fighter_stats.can_warp is True


def test_struct(client, consts):
    eve_warp_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warp_scramble_status)
    eve_speed_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_struct_id = client.mk_eve_struct(attrs={eve_warp_attr_id: 1, eve_speed_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_struct_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(can_warp=True))
    assert api_fit_stats.can_warp is None
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(can_warp=True))
    assert api_ship_stats.can_warp is None


def test_other(client, consts):
    eve_warp_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warp_scramble_status)
    eve_speed_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_drone_id = client.mk_eve_drone(attrs={eve_warp_attr_id: 100, eve_speed_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_drone = api_fit.add_drone(type_id=eve_drone_id)
    # Verification
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(can_warp=True))
    assert api_drone_stats.can_warp is None


def test_not_requested(client, consts):
    eve_warp_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warp_scramble_status)
    eve_speed_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_ship_id = client.mk_eve_ship(attrs={eve_warp_attr_id: 100, eve_speed_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(can_warp=False))
    with check_no_field():
        api_fit_stats.can_warp  # noqa: B018
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(can_warp=False))
    with check_no_field():
        api_ship_stats.can_warp  # noqa: B018
