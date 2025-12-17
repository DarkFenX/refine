from fw import approx, check_no_field
from fw.api import FitStatsOptions, ItemStatsOptions
from tests.stats.tank import (
    make_eve_local_aar,
    make_eve_local_asb,
    make_eve_remote_aar,
    make_eve_remote_asb,
    make_eve_tankable,
    setup_tank_basics,
)


def test_buffer_ship(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_ship_id = make_eve_tankable(
        client=client, basic_info=eve_basic_info, hps=(3000, 2000, 1000), maker=client.mk_eve_ship)
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        item_mods=[
            client.mk_eve_buff_mod(attr_id=eve_basic_info.shield_hp_attr_id),
            client.mk_eve_buff_mod(attr_id=eve_basic_info.armor_hp_attr_id),
            client.mk_eve_buff_mod(attr_id=eve_basic_info.hull_hp_attr_id)])
    eve_buff_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.buff_everything, cat_id=consts.EveEffCat.active)
    eve_fw_effect_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 25},
        eff_ids=[eve_buff_effect_id], defeff_id=eve_buff_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(hp=True))
    assert api_fit_stats.hp.shield == (approx(3000), 0, 0)
    assert api_fit_stats.hp.armor == (approx(2000), 0, 0)
    assert api_fit_stats.hp.hull == (approx(1000), 0, 0)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(hp=True))
    assert api_ship_stats.hp.shield == (approx(3000), 0, 0)
    assert api_ship_stats.hp.armor == (approx(2000), 0, 0)
    assert api_ship_stats.hp.hull == (approx(1000), 0, 0)
    # Action
    api_fw_effect = api_fit.add_fw_effect(type_id=eve_fw_effect_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(hp=True))
    assert api_fit_stats.hp.shield == (approx(3750), 0, 0)
    assert api_fit_stats.hp.armor == (approx(2500), 0, 0)
    assert api_fit_stats.hp.hull == (approx(1250), 0, 0)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(hp=True))
    assert api_ship_stats.hp.shield == (approx(3750), 0, 0)
    assert api_ship_stats.hp.armor == (approx(2500), 0, 0)
    assert api_ship_stats.hp.hull == (approx(1250), 0, 0)
    # Action
    api_fw_effect.remove()
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(hp=True))
    assert api_fit_stats.hp.shield == (approx(3000), 0, 0)
    assert api_fit_stats.hp.armor == (approx(2000), 0, 0)
    assert api_fit_stats.hp.hull == (approx(1000), 0, 0)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(hp=True))
    assert api_ship_stats.hp.shield == (approx(3000), 0, 0)
    assert api_ship_stats.hp.armor == (approx(2000), 0, 0)
    assert api_ship_stats.hp.hull == (approx(1000), 0, 0)


def test_buffer_drone(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_drone_id = make_eve_tankable(
        client=client, basic_info=eve_basic_info, hps=(1728, 672, 600), maker=client.mk_eve_drone)
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        item_mods=[
            client.mk_eve_buff_mod(attr_id=eve_basic_info.shield_hp_attr_id),
            client.mk_eve_buff_mod(attr_id=eve_basic_info.armor_hp_attr_id),
            client.mk_eve_buff_mod(attr_id=eve_basic_info.hull_hp_attr_id)])
    eve_buff_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.buff_everything, cat_id=consts.EveEffCat.active)
    eve_fw_effect_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 25},
        eff_ids=[eve_buff_effect_id], defeff_id=eve_buff_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_drone = api_fit.add_drone(type_id=eve_drone_id)
    # Verification
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(hp=True))
    assert api_drone_stats.hp.shield == (approx(1728), 0, 0)
    assert api_drone_stats.hp.armor == (approx(672), 0, 0)
    assert api_drone_stats.hp.hull == (approx(600), 0, 0)
    # Action
    api_fw_effect = api_fit.add_fw_effect(type_id=eve_fw_effect_id)
    # Verification
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(hp=True))
    assert api_drone_stats.hp.shield == (approx(2160), 0, 0)
    assert api_drone_stats.hp.armor == (approx(840), 0, 0)
    assert api_drone_stats.hp.hull == (approx(750), 0, 0)
    # Action
    api_fw_effect.remove()
    # Verification
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(hp=True))
    assert api_drone_stats.hp.shield == (approx(1728), 0, 0)
    assert api_drone_stats.hp.armor == (approx(672), 0, 0)
    assert api_drone_stats.hp.hull == (approx(600), 0, 0)


def test_buffer_fighter(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_fighter_id = make_eve_tankable(
        client=client, basic_info=eve_basic_info, hps=(2190, None, 100), fighter_count=9, maker=client.mk_eve_fighter)
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        item_mods=[
            client.mk_eve_buff_mod(attr_id=eve_basic_info.shield_hp_attr_id),
            client.mk_eve_buff_mod(attr_id=eve_basic_info.armor_hp_attr_id),
            client.mk_eve_buff_mod(attr_id=eve_basic_info.hull_hp_attr_id)])
    eve_buff_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.buff_everything, cat_id=consts.EveEffCat.active)
    eve_fw_effect_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 25},
        eff_ids=[eve_buff_effect_id], defeff_id=eve_buff_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id)
    # Verification
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(hp=True))
    assert api_fighter_stats.hp.shield == (approx(2190), 0, 0)
    assert api_fighter_stats.hp.armor == (0, 0, 0)
    assert api_fighter_stats.hp.hull == (approx(100), 0, 0)
    # Action
    api_fw_effect = api_fit.add_fw_effect(type_id=eve_fw_effect_id)
    # Verification
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(hp=True))
    assert api_fighter_stats.hp.shield == (approx(2737.5), 0, 0)
    assert api_fighter_stats.hp.armor == (0, 0, 0)
    assert api_fighter_stats.hp.hull == (approx(125), 0, 0)
    # Action
    api_fw_effect.remove()
    # Verification
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(hp=True))
    assert api_fighter_stats.hp.shield == (approx(2190), 0, 0)
    assert api_fighter_stats.hp.armor == (0, 0, 0)
    assert api_fighter_stats.hp.hull == (approx(100), 0, 0)


def test_local_asb_ship_accuracy_and_charge_switch(client, consts):
    # Accuracy = cases like 2.3 / 0.1 = 22.999999999999996
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_ship_id = make_eve_tankable(
        client=client, basic_info=eve_basic_info, hps=(3000, 2000, 1000), maker=client.mk_eve_ship)
    eve_rep_item_id = make_eve_local_asb(
        client=client, basic_info=eve_basic_info, rep_amount=300, cycle_time=5000, capacity=2.3)
    eve_charge_item_id = client.mk_eve_item(attrs={eve_basic_info.volume_attr_id: 0.1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_asb = api_fit.add_module(
        type_id=eve_rep_item_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_item_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(hp=True))
    assert api_fit_stats.hp.shield == (approx(3000), approx(6900), 0)
    assert api_fit_stats.hp.armor == (approx(2000), 0, 0)
    assert api_fit_stats.hp.hull == (approx(1000), 0, 0)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(hp=True))
    assert api_ship_stats.hp.shield == (approx(3000), approx(6900), 0)
    assert api_ship_stats.hp.armor == (approx(2000), 0, 0)
    assert api_ship_stats.hp.hull == (approx(1000), 0, 0)
    # Action
    api_asb.change_module(charge_type_id=None)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(hp=True))
    assert api_fit_stats.hp.shield == (approx(3000), 0, 0)
    assert api_fit_stats.hp.armor == (approx(2000), 0, 0)
    assert api_fit_stats.hp.hull == (approx(1000), 0, 0)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(hp=True))
    assert api_ship_stats.hp.shield == (approx(3000), 0, 0)
    assert api_ship_stats.hp.armor == (approx(2000), 0, 0)
    assert api_ship_stats.hp.hull == (approx(1000), 0, 0)
    # Action
    api_asb.change_module(charge_type_id=eve_charge_item_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(hp=True))
    assert api_fit_stats.hp.shield == (approx(3000), approx(6900), 0)
    assert api_fit_stats.hp.armor == (approx(2000), 0, 0)
    assert api_fit_stats.hp.hull == (approx(1000), 0, 0)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(hp=True))
    assert api_ship_stats.hp.shield == (approx(3000), approx(6900), 0)
    assert api_ship_stats.hp.armor == (approx(2000), 0, 0)
    assert api_ship_stats.hp.hull == (approx(1000), 0, 0)
    # Action
    api_asb.remove()
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(hp=True))
    assert api_fit_stats.hp.shield == (approx(3000), 0, 0)
    assert api_fit_stats.hp.armor == (approx(2000), 0, 0)
    assert api_fit_stats.hp.hull == (approx(1000), 0, 0)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(hp=True))
    assert api_ship_stats.hp.shield == (approx(3000), 0, 0)
    assert api_ship_stats.hp.armor == (approx(2000), 0, 0)
    assert api_ship_stats.hp.hull == (approx(1000), 0, 0)


def test_local_asb_ship_state_switch(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_ship_id = make_eve_tankable(
        client=client, basic_info=eve_basic_info, hps=(3000, 2000, 1000), maker=client.mk_eve_ship)
    eve_rep_item_id = make_eve_local_asb(
        client=client, basic_info=eve_basic_info, rep_amount=300, cycle_time=5000, capacity=3)
    eve_charge_item_id = client.mk_eve_item(attrs={eve_basic_info.volume_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_asb = api_fit.add_module(
        type_id=eve_rep_item_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_item_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(hp=True))
    assert api_fit_stats.hp.shield == (approx(3000), approx(900), 0)
    assert api_fit_stats.hp.armor == (approx(2000), 0, 0)
    assert api_fit_stats.hp.hull == (approx(1000), 0, 0)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(hp=True))
    assert api_ship_stats.hp.shield == (approx(3000), approx(900), 0)
    assert api_ship_stats.hp.armor == (approx(2000), 0, 0)
    assert api_ship_stats.hp.hull == (approx(1000), 0, 0)
    # Action
    api_asb.change_module(state=consts.ApiModuleState.online)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(hp=True))
    assert api_fit_stats.hp.shield == (approx(3000), 0, 0)
    assert api_fit_stats.hp.armor == (approx(2000), 0, 0)
    assert api_fit_stats.hp.hull == (approx(1000), 0, 0)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(hp=True))
    assert api_ship_stats.hp.shield == (approx(3000), 0, 0)
    assert api_ship_stats.hp.armor == (approx(2000), 0, 0)
    assert api_ship_stats.hp.hull == (approx(1000), 0, 0)
    # Action
    api_asb.change_module(state=consts.ApiModuleState.active)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(hp=True))
    assert api_fit_stats.hp.shield == (approx(3000), approx(900), 0)
    assert api_fit_stats.hp.armor == (approx(2000), 0, 0)
    assert api_fit_stats.hp.hull == (approx(1000), 0, 0)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(hp=True))
    assert api_ship_stats.hp.shield == (approx(3000), approx(900), 0)
    assert api_ship_stats.hp.armor == (approx(2000), 0, 0)
    assert api_ship_stats.hp.hull == (approx(1000), 0, 0)
    # Action
    api_asb.remove()
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(hp=True))
    assert api_fit_stats.hp.shield == (approx(3000), 0, 0)
    assert api_fit_stats.hp.armor == (approx(2000), 0, 0)
    assert api_fit_stats.hp.hull == (approx(1000), 0, 0)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(hp=True))
    assert api_ship_stats.hp.shield == (approx(3000), 0, 0)
    assert api_ship_stats.hp.armor == (approx(2000), 0, 0)
    assert api_ship_stats.hp.hull == (approx(1000), 0, 0)


def test_local_asb_ship_modified_and_rep_hp_limit(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_ship_id = make_eve_tankable(
        client=client, basic_info=eve_basic_info, hps=(1000, 500, 250), maker=client.mk_eve_ship)
    eve_rep_item_id = make_eve_local_asb(
        client=client, basic_info=eve_basic_info, rep_amount=1500, cycle_time=5000, capacity=112)
    eve_charge_item_id = client.mk_eve_item(attrs={eve_basic_info.volume_attr_id: 12})
    eve_mod_attr_id = client.mk_eve_attr()
    eve_shield_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_basic_info.shield_hp_attr_id)
    eve_rep_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_basic_info.shield_rep_amount_attr_id)
    eve_shield_mod_effect_id = client.mk_eve_effect(mod_info=[eve_shield_mod])
    eve_rep_mod_effect_id = client.mk_eve_effect(mod_info=[eve_rep_mod])
    eve_hp_rig = client.mk_eve_item(attrs={eve_mod_attr_id: 70}, eff_ids=[eve_shield_mod_effect_id])
    eve_rep_rig = client.mk_eve_item(attrs={eve_mod_attr_id: 70}, eff_ids=[eve_rep_mod_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(
        type_id=eve_rep_item_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_item_id)
    # Verification - reps are limited from 1500 / cycle to 1000 / cycle
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(hp=True))
    assert api_fit_stats.hp.shield == (approx(1000), approx(9000), 0)
    assert api_fit_stats.hp.armor == (approx(500), 0, 0)
    assert api_fit_stats.hp.hull == (approx(250), 0, 0)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(hp=True))
    assert api_ship_stats.hp.shield == (approx(1000), approx(9000), 0)
    assert api_ship_stats.hp.armor == (approx(500), 0, 0)
    assert api_ship_stats.hp.hull == (approx(250), 0, 0)
    # Action
    api_fit.add_rig(type_id=eve_hp_rig)
    # Verification - no limit now, with shield HP increased
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(hp=True))
    assert api_fit_stats.hp.shield == (approx(1700), approx(13500), 0)
    assert api_fit_stats.hp.armor == (approx(500), 0, 0)
    assert api_fit_stats.hp.hull == (approx(250), 0, 0)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(hp=True))
    assert api_ship_stats.hp.shield == (approx(1700), approx(13500), 0)
    assert api_ship_stats.hp.armor == (approx(500), 0, 0)
    assert api_ship_stats.hp.hull == (approx(250), 0, 0)
    # Action
    api_fit.add_rig(type_id=eve_rep_rig)
    # Verification - limited again from 2550 / cycle to 1700 / cycle
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(hp=True))
    assert api_fit_stats.hp.shield == (approx(1700), approx(15300), 0)
    assert api_fit_stats.hp.armor == (approx(500), 0, 0)
    assert api_fit_stats.hp.hull == (approx(250), 0, 0)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(hp=True))
    assert api_ship_stats.hp.shield == (approx(1700), approx(15300), 0)
    assert api_ship_stats.hp.armor == (approx(500), 0, 0)
    assert api_ship_stats.hp.hull == (approx(250), 0, 0)


def test_local_asb_drone_fighter(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_drone_id = make_eve_tankable(client=client, basic_info=eve_basic_info, hps=(1728, 672, 600))
    eve_fighter_id = make_eve_tankable(client=client, basic_info=eve_basic_info, hps=(2190, None, 100), fighter_count=9)
    eve_rep_item_id = make_eve_local_asb(
        client=client, basic_info=eve_basic_info, rep_amount=300, cycle_time=5000, capacity=1)
    eve_charge_item_id = client.mk_eve_item(attrs={eve_basic_info.volume_attr_id: 0.1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_drone = api_fit.add_drone(type_id=eve_drone_id)
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id)
    api_fit.add_module(
        type_id=eve_rep_item_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_item_id)
    # Verification - drone and fighter are not affected by ASB
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(hp=True))
    assert api_drone_stats.hp.shield == (approx(1728), 0, 0)
    assert api_drone_stats.hp.armor == (approx(672), 0, 0)
    assert api_drone_stats.hp.hull == (approx(600), 0, 0)
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(hp=True))
    assert api_fighter_stats.hp.shield == (approx(2190), 0, 0)
    assert api_fighter_stats.hp.armor == (0, 0, 0)
    assert api_fighter_stats.hp.hull == (approx(100), 0, 0)


def test_local_aar_ship_accuracy_and_charge_switch(client, consts):
    # Accuracy = cases like 2.3 / 0.1 = 22.999999999999996
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_ship_id = make_eve_tankable(
        client=client, basic_info=eve_basic_info, hps=(3000, 2000, 1000), maker=client.mk_eve_ship)
    eve_rep_item_id = make_eve_local_aar(
        client=client, basic_info=eve_basic_info, rep_amount=100, cycle_time=5000, capacity=2.3, charge_rate=1)
    eve_charge_item_id = client.mk_eve_item(
        id_=consts.EveItem.nanite_repair_paste, attrs={eve_basic_info.volume_attr_id: 0.1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_aar = api_fit.add_module(
        type_id=eve_rep_item_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_item_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(hp=True))
    assert api_fit_stats.hp.shield == (approx(3000), 0, 0)
    assert api_fit_stats.hp.armor == (approx(2000), approx(6900), 0)
    assert api_fit_stats.hp.hull == (approx(1000), 0, 0)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(hp=True))
    assert api_ship_stats.hp.shield == (approx(3000), 0, 0)
    assert api_ship_stats.hp.armor == (approx(2000), approx(6900), 0)
    assert api_ship_stats.hp.hull == (approx(1000), 0, 0)
    # Action
    api_aar.change_module(charge_type_id=None)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(hp=True))
    assert api_fit_stats.hp.shield == (approx(3000), 0, 0)
    assert api_fit_stats.hp.armor == (approx(2000), 0, 0)
    assert api_fit_stats.hp.hull == (approx(1000), 0, 0)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(hp=True))
    assert api_ship_stats.hp.shield == (approx(3000), 0, 0)
    assert api_ship_stats.hp.armor == (approx(2000), 0, 0)
    assert api_ship_stats.hp.hull == (approx(1000), 0, 0)
    # Action
    api_aar.change_module(charge_type_id=eve_charge_item_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(hp=True))
    assert api_fit_stats.hp.shield == (approx(3000), 0, 0)
    assert api_fit_stats.hp.armor == (approx(2000), approx(6900), 0)
    assert api_fit_stats.hp.hull == (approx(1000), 0, 0)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(hp=True))
    assert api_ship_stats.hp.shield == (approx(3000), 0, 0)
    assert api_ship_stats.hp.armor == (approx(2000), approx(6900), 0)
    assert api_ship_stats.hp.hull == (approx(1000), 0, 0)
    # Action
    api_aar.remove()
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(hp=True))
    assert api_fit_stats.hp.shield == (approx(3000), 0, 0)
    assert api_fit_stats.hp.armor == (approx(2000), 0, 0)
    assert api_fit_stats.hp.hull == (approx(1000), 0, 0)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(hp=True))
    assert api_ship_stats.hp.shield == (approx(3000), 0, 0)
    assert api_ship_stats.hp.armor == (approx(2000), 0, 0)
    assert api_ship_stats.hp.hull == (approx(1000), 0, 0)


def test_local_aar_ship_charge_rate_rounding_and_state_switch(client, consts):
    # Rounding in this case means the way lib considers not-fully-charged-cycle
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_ship_id = make_eve_tankable(
        client=client, basic_info=eve_basic_info, hps=(3000, 2000, 1000), maker=client.mk_eve_ship)
    eve_rep_item_id = make_eve_local_aar(
        client=client, basic_info=eve_basic_info, rep_amount=100, cycle_time=5000, capacity=15, charge_rate=4)
    eve_charge_item_id = client.mk_eve_item(
        id_=consts.EveItem.nanite_repair_paste, attrs={eve_basic_info.volume_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_aar = api_fit.add_module(
        type_id=eve_rep_item_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_item_id)
    # Verification - count of cycles is floored (i.e. forced to reload on partially charged cycles)
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(hp=True))
    assert api_fit_stats.hp.shield == (approx(3000), 0, 0)
    assert api_fit_stats.hp.armor == (approx(2000), approx(900), 0)
    assert api_fit_stats.hp.hull == (approx(1000), 0, 0)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(hp=True))
    assert api_ship_stats.hp.shield == (approx(3000), 0, 0)
    assert api_ship_stats.hp.armor == (approx(2000), approx(900), 0)
    assert api_ship_stats.hp.hull == (approx(1000), 0, 0)
    # Action
    api_aar.change_module(state=consts.ApiModuleState.online)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(hp=True))
    assert api_fit_stats.hp.shield == (approx(3000), 0, 0)
    assert api_fit_stats.hp.armor == (approx(2000), 0, 0)
    assert api_fit_stats.hp.hull == (approx(1000), 0, 0)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(hp=True))
    assert api_ship_stats.hp.shield == (approx(3000), 0, 0)
    assert api_ship_stats.hp.armor == (approx(2000), 0, 0)
    assert api_ship_stats.hp.hull == (approx(1000), 0, 0)
    # Action
    api_aar.change_module(state=consts.ApiModuleState.active)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(hp=True))
    assert api_fit_stats.hp.shield == (approx(3000), 0, 0)
    assert api_fit_stats.hp.armor == (approx(2000), approx(900), 0)
    assert api_fit_stats.hp.hull == (approx(1000), 0, 0)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(hp=True))
    assert api_ship_stats.hp.shield == (approx(3000), 0, 0)
    assert api_ship_stats.hp.armor == (approx(2000), approx(900), 0)
    assert api_ship_stats.hp.hull == (approx(1000), 0, 0)
    # Action
    api_aar.remove()
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(hp=True))
    assert api_fit_stats.hp.shield == (approx(3000), 0, 0)
    assert api_fit_stats.hp.armor == (approx(2000), 0, 0)
    assert api_fit_stats.hp.hull == (approx(1000), 0, 0)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(hp=True))
    assert api_ship_stats.hp.shield == (approx(3000), 0, 0)
    assert api_ship_stats.hp.armor == (approx(2000), 0, 0)
    assert api_ship_stats.hp.hull == (approx(1000), 0, 0)


def test_local_aar_ship_modified_and_rep_hp_limit(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_ship_id = make_eve_tankable(
        client=client, basic_info=eve_basic_info, hps=(2000, 1000, 500), maker=client.mk_eve_ship)
    eve_rep_item_id = make_eve_local_aar(
        client=client, basic_info=eve_basic_info, rep_amount=500, cycle_time=5000, capacity=0.64, charge_rate=8)
    eve_charge_item_id = client.mk_eve_item(
        id_=consts.EveItem.nanite_repair_paste, attrs={eve_basic_info.volume_attr_id: 0.01})
    eve_mod_attr_id = client.mk_eve_attr()
    eve_hp_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_basic_info.armor_hp_attr_id)
    eve_rep_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_basic_info.armor_rep_amount_attr_id)
    eve_hp_mod_effect_id = client.mk_eve_effect(mod_info=[eve_hp_mod])
    eve_rep_mod_effect_id = client.mk_eve_effect(mod_info=[eve_rep_mod])
    eve_hp_rig = client.mk_eve_item(attrs={eve_mod_attr_id: 70}, eff_ids=[eve_hp_mod_effect_id])
    eve_rep_rig = client.mk_eve_item(attrs={eve_mod_attr_id: 70}, eff_ids=[eve_rep_mod_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(
        type_id=eve_rep_item_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_item_id)
    # Verification - reps are limited from 1500 / cycle to 1000 / cycle
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(hp=True))
    assert api_fit_stats.hp.shield == (approx(2000), 0, 0)
    assert api_fit_stats.hp.armor == (approx(1000), approx(8000), 0)
    assert api_fit_stats.hp.hull == (approx(500), 0, 0)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(hp=True))
    assert api_ship_stats.hp.shield == (approx(2000), 0, 0)
    assert api_ship_stats.hp.armor == (approx(1000), approx(8000), 0)
    assert api_ship_stats.hp.hull == (approx(500), 0, 0)
    # Action
    api_fit.add_rig(type_id=eve_hp_rig)
    # Verification - no limit now, with armor HP increased
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(hp=True))
    assert api_fit_stats.hp.shield == (approx(2000), 0, 0)
    assert api_fit_stats.hp.armor == (approx(1700), approx(12000), 0)
    assert api_fit_stats.hp.hull == (approx(500), 0, 0)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(hp=True))
    assert api_ship_stats.hp.shield == (approx(2000), 0, 0)
    assert api_ship_stats.hp.armor == (approx(1700), approx(12000), 0)
    assert api_ship_stats.hp.hull == (approx(500), 0, 0)
    # Action
    api_fit.add_rig(type_id=eve_rep_rig)
    # Verification - limited again from 2550 / cycle to 1700 / cycle
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(hp=True))
    assert api_fit_stats.hp.shield == (approx(2000), 0, 0)
    assert api_fit_stats.hp.armor == (approx(1700), approx(13600), 0)
    assert api_fit_stats.hp.hull == (approx(500), 0, 0)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(hp=True))
    assert api_ship_stats.hp.shield == (approx(2000), 0, 0)
    assert api_ship_stats.hp.armor == (approx(1700), approx(13600), 0)
    assert api_ship_stats.hp.hull == (approx(500), 0, 0)


def test_local_aar_drone_fighter(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_drone_id = make_eve_tankable(client=client, basic_info=eve_basic_info, hps=(1728, 672, 600))
    eve_fighter_id = make_eve_tankable(client=client, basic_info=eve_basic_info, hps=(2190, None, 100), fighter_count=9)
    eve_rep_item_id = make_eve_local_aar(
        client=client, basic_info=eve_basic_info, rep_amount=100, cycle_time=5000, capacity=1, charge_rate=1)
    eve_charge_item_id = client.mk_eve_item(
        id_=consts.EveItem.nanite_repair_paste, attrs={eve_basic_info.volume_attr_id: 0.1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_drone = api_fit.add_drone(type_id=eve_drone_id)
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id)
    api_fit.add_module(
        type_id=eve_rep_item_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_item_id)
    # Verification - drone and fighter are not affected by AAR
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(hp=True))
    assert api_drone_stats.hp.shield == (approx(1728), 0, 0)
    assert api_drone_stats.hp.armor == (approx(672), 0, 0)
    assert api_drone_stats.hp.hull == (approx(600), 0, 0)
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(hp=True))
    assert api_fighter_stats.hp.shield == (approx(2190), 0, 0)
    assert api_fighter_stats.hp.armor == (0, 0, 0)
    assert api_fighter_stats.hp.hull == (approx(100), 0, 0)


def test_remote_asb_ship_accuracy_and_charge_switch(client, consts):
    # Accuracy = cases like 2.3 / 0.1 = 22.999999999999996
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_ship_id = make_eve_tankable(
        client=client, basic_info=eve_basic_info, hps=(3000, 2000, 1000), maker=client.mk_eve_ship)
    eve_rep_item_id = make_eve_remote_asb(
        client=client, basic_info=eve_basic_info, rep_amount=300, cycle_time=5000, capacity=2.3)
    eve_charge_item_id = client.mk_eve_item(attrs={eve_basic_info.volume_attr_id: 0.1})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_rasb = api_src_fit.add_module(
        type_id=eve_rep_item_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_item_id)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_rasb.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(hp=True))
    assert api_tgt_fit_stats.hp.shield == (approx(3000), 0, approx(6900))
    assert api_tgt_fit_stats.hp.armor == (approx(2000), 0, 0)
    assert api_tgt_fit_stats.hp.hull == (approx(1000), 0, 0)
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(hp=True))
    assert api_tgt_ship_stats.hp.shield == (approx(3000), 0, approx(6900))
    assert api_tgt_ship_stats.hp.armor == (approx(2000), 0, 0)
    assert api_tgt_ship_stats.hp.hull == (approx(1000), 0, 0)
    # Action
    api_rasb.change_module(charge_type_id=None)
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(hp=True))
    assert api_tgt_fit_stats.hp.shield == (approx(3000), 0, 0)
    assert api_tgt_fit_stats.hp.armor == (approx(2000), 0, 0)
    assert api_tgt_fit_stats.hp.hull == (approx(1000), 0, 0)
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(hp=True))
    assert api_tgt_ship_stats.hp.shield == (approx(3000), 0, 0)
    assert api_tgt_ship_stats.hp.armor == (approx(2000), 0, 0)
    assert api_tgt_ship_stats.hp.hull == (approx(1000), 0, 0)
    # Action
    api_rasb.change_module(charge_type_id=eve_charge_item_id)
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(hp=True))
    assert api_tgt_fit_stats.hp.shield == (approx(3000), 0, approx(6900))
    assert api_tgt_fit_stats.hp.armor == (approx(2000), 0, 0)
    assert api_tgt_fit_stats.hp.hull == (approx(1000), 0, 0)
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(hp=True))
    assert api_tgt_ship_stats.hp.shield == (approx(3000), 0, approx(6900))
    assert api_tgt_ship_stats.hp.armor == (approx(2000), 0, 0)
    assert api_tgt_ship_stats.hp.hull == (approx(1000), 0, 0)
    # Action
    api_rasb.remove()
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(hp=True))
    assert api_tgt_fit_stats.hp.shield == (approx(3000), 0, 0)
    assert api_tgt_fit_stats.hp.armor == (approx(2000), 0, 0)
    assert api_tgt_fit_stats.hp.hull == (approx(1000), 0, 0)
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(hp=True))
    assert api_tgt_ship_stats.hp.shield == (approx(3000), 0, 0)
    assert api_tgt_ship_stats.hp.armor == (approx(2000), 0, 0)
    assert api_tgt_ship_stats.hp.hull == (approx(1000), 0, 0)


def test_remote_asb_ship_state_switch(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_ship_id = make_eve_tankable(
        client=client, basic_info=eve_basic_info, hps=(3000, 2000, 1000), maker=client.mk_eve_ship)
    eve_rep_item_id = make_eve_remote_asb(
        client=client, basic_info=eve_basic_info, rep_amount=300, cycle_time=5000, capacity=3)
    eve_charge_item_id = client.mk_eve_item(attrs={eve_basic_info.volume_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_rasb = api_src_fit.add_module(
        type_id=eve_rep_item_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_item_id)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_rasb.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(hp=True))
    assert api_tgt_fit_stats.hp.shield == (approx(3000), 0, approx(900))
    assert api_tgt_fit_stats.hp.armor == (approx(2000), 0, 0)
    assert api_tgt_fit_stats.hp.hull == (approx(1000), 0, 0)
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(hp=True))
    assert api_tgt_ship_stats.hp.shield == (approx(3000), 0, approx(900))
    assert api_tgt_ship_stats.hp.armor == (approx(2000), 0, 0)
    assert api_tgt_ship_stats.hp.hull == (approx(1000), 0, 0)
    # Action
    api_rasb.change_module(state=consts.ApiModuleState.online)
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(hp=True))
    assert api_tgt_fit_stats.hp.shield == (approx(3000), 0, 0)
    assert api_tgt_fit_stats.hp.armor == (approx(2000), 0, 0)
    assert api_tgt_fit_stats.hp.hull == (approx(1000), 0, 0)
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(hp=True))
    assert api_tgt_ship_stats.hp.shield == (approx(3000), 0, 0)
    assert api_tgt_ship_stats.hp.armor == (approx(2000), 0, 0)
    assert api_tgt_ship_stats.hp.hull == (approx(1000), 0, 0)
    # Action
    api_rasb.change_module(state=consts.ApiModuleState.active)
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(hp=True))
    assert api_tgt_fit_stats.hp.shield == (approx(3000), 0, approx(900))
    assert api_tgt_fit_stats.hp.armor == (approx(2000), 0, 0)
    assert api_tgt_fit_stats.hp.hull == (approx(1000), 0, 0)
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(hp=True))
    assert api_tgt_ship_stats.hp.shield == (approx(3000), 0, approx(900))
    assert api_tgt_ship_stats.hp.armor == (approx(2000), 0, 0)
    assert api_tgt_ship_stats.hp.hull == (approx(1000), 0, 0)
    # Action
    api_rasb.remove()
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(hp=True))
    assert api_tgt_fit_stats.hp.shield == (approx(3000), 0, 0)
    assert api_tgt_fit_stats.hp.armor == (approx(2000), 0, 0)
    assert api_tgt_fit_stats.hp.hull == (approx(1000), 0, 0)
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(hp=True))
    assert api_tgt_ship_stats.hp.shield == (approx(3000), 0, 0)
    assert api_tgt_ship_stats.hp.armor == (approx(2000), 0, 0)
    assert api_tgt_ship_stats.hp.hull == (approx(1000), 0, 0)


def test_remote_asb_ship_resist_and_rep_hp_limit(client, consts):
    # Also check projection addition/removal
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_ship_id = make_eve_tankable(
        client=client, basic_info=eve_basic_info, hps=(1000, 500, 250), rr_resist=1, maker=client.mk_eve_ship)
    eve_rep_item_id = make_eve_remote_asb(
        client=client, basic_info=eve_basic_info, rep_amount=1500, cycle_time=5000, capacity=112)
    eve_charge_item_id = client.mk_eve_item(attrs={eve_basic_info.volume_attr_id: 12})
    eve_mod_attr_id = client.mk_eve_attr()
    eve_resist_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_basic_info.rr_res_attr_id)
    eve_resist_mod_effect_id = client.mk_eve_effect(mod_info=[eve_resist_mod])
    eve_resist_rig = client.mk_eve_item(attrs={eve_mod_attr_id: -70}, eff_ids=[eve_resist_mod_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_rasb = api_src_fit.add_module(
        type_id=eve_rep_item_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_item_id)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(hp=True))
    assert api_tgt_fit_stats.hp.shield == (approx(1000), 0, 0)
    assert api_tgt_fit_stats.hp.armor == (approx(500), 0, 0)
    assert api_tgt_fit_stats.hp.hull == (approx(250), 0, 0)
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(hp=True))
    assert api_tgt_ship_stats.hp.shield == (approx(1000), 0, 0)
    assert api_tgt_ship_stats.hp.armor == (approx(500), 0, 0)
    assert api_tgt_ship_stats.hp.hull == (approx(250), 0, 0)
    # Action
    api_rasb.change_module(add_projs=[api_tgt_ship.id])
    # Verification - reps are limited from 1500 / cycle to 1000 / cycle
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(hp=True))
    assert api_tgt_fit_stats.hp.shield == (approx(1000), 0, approx(9000))
    assert api_tgt_fit_stats.hp.armor == (approx(500), 0, 0)
    assert api_tgt_fit_stats.hp.hull == (approx(250), 0, 0)
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(hp=True))
    assert api_tgt_ship_stats.hp.shield == (approx(1000), 0, approx(9000))
    assert api_tgt_ship_stats.hp.armor == (approx(500), 0, 0)
    assert api_tgt_ship_stats.hp.hull == (approx(250), 0, 0)
    # Action
    api_tgt_fit.add_rig(type_id=eve_resist_rig)
    # Verification - reps are reduced by RR resistance, and are no longer limited by HP:
    # 1500 (not 1000) * 9 * 0.3 = 4050
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(hp=True))
    assert api_tgt_fit_stats.hp.shield == (approx(1000), 0, approx(4050))
    assert api_tgt_fit_stats.hp.armor == (approx(500), 0, 0)
    assert api_tgt_fit_stats.hp.hull == (approx(250), 0, 0)
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(hp=True))
    assert api_tgt_ship_stats.hp.shield == (approx(1000), 0, approx(4050))
    assert api_tgt_ship_stats.hp.armor == (approx(500), 0, 0)
    assert api_tgt_ship_stats.hp.hull == (approx(250), 0, 0)
    # Action
    api_rasb.change_module(rm_projs=[api_tgt_ship.id])
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(hp=True))
    assert api_tgt_fit_stats.hp.shield == (approx(1000), 0, 0)
    assert api_tgt_fit_stats.hp.armor == (approx(500), 0, 0)
    assert api_tgt_fit_stats.hp.hull == (approx(250), 0, 0)
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(hp=True))
    assert api_tgt_ship_stats.hp.shield == (approx(1000), 0, 0)
    assert api_tgt_ship_stats.hp.armor == (approx(500), 0, 0)
    assert api_tgt_ship_stats.hp.hull == (approx(250), 0, 0)


def test_remote_asb_ship_proj_range_and_rep_hp_limit(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_ship_id = make_eve_tankable(
        client=client, basic_info=eve_basic_info, hps=(1000, 500, 250), maker=client.mk_eve_ship)
    eve_rep_item_id = make_eve_remote_asb(
        client=client,
        basic_info=eve_basic_info,
        rep_amount=1500,
        cycle_time=5000,
        capacity=112,
        optimal_range=10000,
        falloff_range=5000)
    eve_charge_item_id = client.mk_eve_item(attrs={eve_basic_info.volume_attr_id: 12})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_ship_id, coordinates=(0, 0, 0))
    api_rasb = api_src_fit.add_module(
        type_id=eve_rep_item_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_item_id)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id, coordinates=(0, 0, 0))
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(hp=True))
    assert api_tgt_fit_stats.hp.shield == (approx(1000), 0, 0)
    assert api_tgt_fit_stats.hp.armor == (approx(500), 0, 0)
    assert api_tgt_fit_stats.hp.hull == (approx(250), 0, 0)
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(hp=True))
    assert api_tgt_ship_stats.hp.shield == (approx(1000), 0, 0)
    assert api_tgt_ship_stats.hp.armor == (approx(500), 0, 0)
    assert api_tgt_ship_stats.hp.hull == (approx(250), 0, 0)
    # Action
    api_rasb.change_module(add_projs=[api_tgt_ship.id])
    # Verification - reps are limited from 1500 / cycle to 1000 / cycle
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(hp=True))
    assert api_tgt_fit_stats.hp.shield == (approx(1000), 0, approx(9000))
    assert api_tgt_fit_stats.hp.armor == (approx(500), 0, 0)
    assert api_tgt_fit_stats.hp.hull == (approx(250), 0, 0)
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(hp=True))
    assert api_tgt_ship_stats.hp.shield == (approx(1000), 0, approx(9000))
    assert api_tgt_ship_stats.hp.armor == (approx(500), 0, 0)
    assert api_tgt_ship_stats.hp.hull == (approx(250), 0, 0)
    # Action
    api_tgt_ship.change_ship(coordinates=(15000, 0, 0))
    # Verification - reps are reduced by RR resistance, and are no longer limited by HP:
    # 1500 (not 1000) * 9 * 0.5 = 6750
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(hp=True))
    assert api_tgt_fit_stats.hp.shield == (approx(1000), 0, approx(6750))
    assert api_tgt_fit_stats.hp.armor == (approx(500), 0, 0)
    assert api_tgt_fit_stats.hp.hull == (approx(250), 0, 0)
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(hp=True))
    assert api_tgt_ship_stats.hp.shield == (approx(1000), 0, approx(6750))
    assert api_tgt_ship_stats.hp.armor == (approx(500), 0, 0)
    assert api_tgt_ship_stats.hp.hull == (approx(250), 0, 0)


def test_remote_asb_drone_fighter(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_drone_id = make_eve_tankable(client=client, basic_info=eve_basic_info, hps=(1728, 672, 600))
    eve_fighter_id = make_eve_tankable(client=client, basic_info=eve_basic_info, hps=(2190, None, 100), fighter_count=9)
    eve_rep_item_id = make_eve_remote_asb(
        client=client, basic_info=eve_basic_info, rep_amount=300, cycle_time=5000, capacity=1)
    eve_charge_item_id = client.mk_eve_item(attrs={eve_basic_info.volume_attr_id: 0.1})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_rasb = api_src_fit.add_module(
        type_id=eve_rep_item_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_item_id)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_drone = api_tgt_fit.add_drone(type_id=eve_drone_id)
    api_tgt_fighter = api_tgt_fit.add_fighter(type_id=eve_fighter_id)
    api_rasb.change_module(add_projs=[api_tgt_drone.id, api_tgt_fighter.id])
    # Verification
    api_tgt_drone_stats = api_tgt_drone.get_stats(options=ItemStatsOptions(hp=True))
    assert api_tgt_drone_stats.hp.shield == (approx(1728), 0, approx(3000))
    assert api_tgt_drone_stats.hp.armor == (approx(672), 0, 0)
    assert api_tgt_drone_stats.hp.hull == (approx(600), 0, 0)
    api_tgt_fighter_stats = api_tgt_fighter.get_stats(options=ItemStatsOptions(hp=True))
    assert api_tgt_fighter_stats.hp.shield == (approx(2190), 0, approx(3000))
    assert api_tgt_fighter_stats.hp.armor == (0, 0, 0)
    assert api_tgt_fighter_stats.hp.hull == (approx(100), 0, 0)


def test_remote_aar_ship_accuracy_and_charge_switch(client, consts):
    # Accuracy = cases like 2.3 / 0.1 = 22.999999999999996
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_ship_id = make_eve_tankable(
        client=client, basic_info=eve_basic_info, hps=(3000, 2000, 1000), maker=client.mk_eve_ship)
    eve_rep_item_id = make_eve_remote_aar(
        client=client, basic_info=eve_basic_info, rep_amount=100, cycle_time=5000, capacity=2.3, charge_rate=1)
    eve_charge_item_id = client.mk_eve_item(
        id_=consts.EveItem.nanite_repair_paste, attrs={eve_basic_info.volume_attr_id: 0.1})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_raar = api_src_fit.add_module(
        type_id=eve_rep_item_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_item_id)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_raar.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(hp=True))
    assert api_tgt_fit_stats.hp.shield == (approx(3000), 0, 0)
    assert api_tgt_fit_stats.hp.armor == (approx(2000), 0, approx(6900))
    assert api_tgt_fit_stats.hp.hull == (approx(1000), 0, 0)
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(hp=True))
    assert api_tgt_ship_stats.hp.shield == (approx(3000), 0, 0)
    assert api_tgt_ship_stats.hp.armor == (approx(2000), 0, approx(6900))
    assert api_tgt_ship_stats.hp.hull == (approx(1000), 0, 0)
    # Action
    api_raar.change_module(charge_type_id=None)
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(hp=True))
    assert api_tgt_fit_stats.hp.shield == (approx(3000), 0, 0)
    assert api_tgt_fit_stats.hp.armor == (approx(2000), 0, 0)
    assert api_tgt_fit_stats.hp.hull == (approx(1000), 0, 0)
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(hp=True))
    assert api_tgt_ship_stats.hp.shield == (approx(3000), 0, 0)
    assert api_tgt_ship_stats.hp.armor == (approx(2000), 0, 0)
    assert api_tgt_ship_stats.hp.hull == (approx(1000), 0, 0)
    # Action
    api_raar.change_module(charge_type_id=eve_charge_item_id)
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(hp=True))
    assert api_tgt_fit_stats.hp.shield == (approx(3000), 0, 0)
    assert api_tgt_fit_stats.hp.armor == (approx(2000), 0, approx(6900))
    assert api_tgt_fit_stats.hp.hull == (approx(1000), 0, 0)
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(hp=True))
    assert api_tgt_ship_stats.hp.shield == (approx(3000), 0, 0)
    assert api_tgt_ship_stats.hp.armor == (approx(2000), 0, approx(6900))
    assert api_tgt_ship_stats.hp.hull == (approx(1000), 0, 0)
    # Action
    api_raar.remove()
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(hp=True))
    assert api_tgt_fit_stats.hp.shield == (approx(3000), 0, 0)
    assert api_tgt_fit_stats.hp.armor == (approx(2000), 0, 0)
    assert api_tgt_fit_stats.hp.hull == (approx(1000), 0, 0)
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(hp=True))
    assert api_tgt_ship_stats.hp.shield == (approx(3000), 0, 0)
    assert api_tgt_ship_stats.hp.armor == (approx(2000), 0, 0)
    assert api_tgt_ship_stats.hp.hull == (approx(1000), 0, 0)


def test_remote_aar_ship_charge_rate_rounding_and_state_switch(client, consts):
    # Rounding in this case means the way lib considers not-fully-charged-cycle
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_ship_id = make_eve_tankable(
        client=client, basic_info=eve_basic_info, hps=(3000, 2000, 1000), maker=client.mk_eve_ship)
    eve_rep_item_id = make_eve_remote_aar(
        client=client, basic_info=eve_basic_info, rep_amount=100, cycle_time=5000, capacity=15, charge_rate=4)
    eve_charge_item_id = client.mk_eve_item(
        id_=consts.EveItem.nanite_repair_paste, attrs={eve_basic_info.volume_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_raar = api_src_fit.add_module(
        type_id=eve_rep_item_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_item_id)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_raar.change_module(add_projs=[api_tgt_ship.id])
    # Verification - count of cycles is floored (i.e. forced to reload on partially charged cycles)
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(hp=True))
    assert api_tgt_fit_stats.hp.shield == (approx(3000), 0, 0)
    assert api_tgt_fit_stats.hp.armor == (approx(2000), 0, approx(900))
    assert api_tgt_fit_stats.hp.hull == (approx(1000), 0, 0)
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(hp=True))
    assert api_tgt_ship_stats.hp.shield == (approx(3000), 0, 0)
    assert api_tgt_ship_stats.hp.armor == (approx(2000), 0, approx(900))
    assert api_tgt_ship_stats.hp.hull == (approx(1000), 0, 0)
    # Action
    api_raar.change_module(state=consts.ApiModuleState.online)
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(hp=True))
    assert api_tgt_fit_stats.hp.shield == (approx(3000), 0, 0)
    assert api_tgt_fit_stats.hp.armor == (approx(2000), 0, 0)
    assert api_tgt_fit_stats.hp.hull == (approx(1000), 0, 0)
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(hp=True))
    assert api_tgt_ship_stats.hp.shield == (approx(3000), 0, 0)
    assert api_tgt_ship_stats.hp.armor == (approx(2000), 0, 0)
    assert api_tgt_ship_stats.hp.hull == (approx(1000), 0, 0)
    # Action
    api_raar.change_module(state=consts.ApiModuleState.active)
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(hp=True))
    assert api_tgt_fit_stats.hp.shield == (approx(3000), 0, 0)
    assert api_tgt_fit_stats.hp.armor == (approx(2000), 0, approx(900))
    assert api_tgt_fit_stats.hp.hull == (approx(1000), 0, 0)
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(hp=True))
    assert api_tgt_ship_stats.hp.shield == (approx(3000), 0, 0)
    assert api_tgt_ship_stats.hp.armor == (approx(2000), 0, approx(900))
    assert api_tgt_ship_stats.hp.hull == (approx(1000), 0, 0)
    # Action
    api_raar.remove()
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(hp=True))
    assert api_tgt_fit_stats.hp.shield == (approx(3000), 0, 0)
    assert api_tgt_fit_stats.hp.armor == (approx(2000), 0, 0)
    assert api_tgt_fit_stats.hp.hull == (approx(1000), 0, 0)
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(hp=True))
    assert api_tgt_ship_stats.hp.shield == (approx(3000), 0, 0)
    assert api_tgt_ship_stats.hp.armor == (approx(2000), 0, 0)
    assert api_tgt_ship_stats.hp.hull == (approx(1000), 0, 0)


def test_remote_aar_ship_resist_and_rep_hp_limit(client, consts):
    # Also check projection addition/removal
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_ship_id = make_eve_tankable(
        client=client, basic_info=eve_basic_info, hps=(2000, 1000, 500), rr_resist=1, maker=client.mk_eve_ship)
    eve_rep_item_id = make_eve_remote_aar(
        client=client, basic_info=eve_basic_info, rep_amount=500, cycle_time=5000, capacity=0.64, charge_rate=8)
    eve_charge_item_id = client.mk_eve_item(
        id_=consts.EveItem.nanite_repair_paste, attrs={eve_basic_info.volume_attr_id: 0.01})
    eve_mod_attr_id = client.mk_eve_attr()
    eve_resist_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_basic_info.rr_res_attr_id)
    eve_resist_mod_effect_id = client.mk_eve_effect(mod_info=[eve_resist_mod])
    eve_resist_rig = client.mk_eve_item(attrs={eve_mod_attr_id: -70}, eff_ids=[eve_resist_mod_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_raar = api_src_fit.add_module(
        type_id=eve_rep_item_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_item_id)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(hp=True))
    assert api_tgt_fit_stats.hp.shield == (approx(2000), 0, 0)
    assert api_tgt_fit_stats.hp.armor == (approx(1000), 0, 0)
    assert api_tgt_fit_stats.hp.hull == (approx(500), 0, 0)
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(hp=True))
    assert api_tgt_ship_stats.hp.shield == (approx(2000), 0, 0)
    assert api_tgt_ship_stats.hp.armor == (approx(1000), 0, 0)
    assert api_tgt_ship_stats.hp.hull == (approx(500), 0, 0)
    # Action
    api_raar.change_module(add_projs=[api_tgt_ship.id])
    # Verification - reps are limited from 1500 / cycle to 1000 / cycle
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(hp=True))
    assert api_tgt_fit_stats.hp.shield == (approx(2000), 0, 0)
    assert api_tgt_fit_stats.hp.armor == (approx(1000), 0, approx(8000))
    assert api_tgt_fit_stats.hp.hull == (approx(500), 0, 0)
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(hp=True))
    assert api_tgt_ship_stats.hp.shield == (approx(2000), 0, 0)
    assert api_tgt_ship_stats.hp.armor == (approx(1000), 0, approx(8000))
    assert api_tgt_ship_stats.hp.hull == (approx(500), 0, 0)
    # Action
    api_tgt_fit.add_rig(type_id=eve_resist_rig)
    # Verification - reps are reduced by RR resistance, and are no longer limited by HP:
    # 1500 (not 1000) * 8 * 0.3 = 3600
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(hp=True))
    assert api_tgt_fit_stats.hp.shield == (approx(2000), 0, 0)
    assert api_tgt_fit_stats.hp.armor == (approx(1000), 0, approx(3600))
    assert api_tgt_fit_stats.hp.hull == (approx(500), 0, 0)
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(hp=True))
    assert api_tgt_ship_stats.hp.shield == (approx(2000), 0, 0)
    assert api_tgt_ship_stats.hp.armor == (approx(1000), 0, approx(3600))
    assert api_tgt_ship_stats.hp.hull == (approx(500), 0, 0)
    # Action
    api_raar.change_module(rm_projs=[api_tgt_ship.id])
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(hp=True))
    assert api_tgt_fit_stats.hp.shield == (approx(2000), 0, 0)
    assert api_tgt_fit_stats.hp.armor == (approx(1000), 0, 0)
    assert api_tgt_fit_stats.hp.hull == (approx(500), 0, 0)
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(hp=True))
    assert api_tgt_ship_stats.hp.shield == (approx(2000), 0, 0)
    assert api_tgt_ship_stats.hp.armor == (approx(1000), 0, 0)
    assert api_tgt_ship_stats.hp.hull == (approx(500), 0, 0)


def test_remote_aar_ship_proj_range_and_rep_hp_limit(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_ship_id = make_eve_tankable(
        client=client, basic_info=eve_basic_info, hps=(2000, 1000, 500), maker=client.mk_eve_ship)
    eve_rep_item_id = make_eve_remote_aar(
        client=client,
        basic_info=eve_basic_info,
        rep_amount=500,
        cycle_time=5000,
        capacity=0.64,
        charge_rate=8,
        optimal_range=10000,
        falloff_range=5000)
    eve_charge_item_id = client.mk_eve_item(
        id_=consts.EveItem.nanite_repair_paste, attrs={eve_basic_info.volume_attr_id: 0.01})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_ship_id, coordinates=(0, 0, 0))
    api_raar = api_src_fit.add_module(
        type_id=eve_rep_item_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_item_id)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id, coordinates=(0, 0, 0))
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(hp=True))
    assert api_tgt_fit_stats.hp.shield == (approx(2000), 0, 0)
    assert api_tgt_fit_stats.hp.armor == (approx(1000), 0, 0)
    assert api_tgt_fit_stats.hp.hull == (approx(500), 0, 0)
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(hp=True))
    assert api_tgt_ship_stats.hp.shield == (approx(2000), 0, 0)
    assert api_tgt_ship_stats.hp.armor == (approx(1000), 0, 0)
    assert api_tgt_ship_stats.hp.hull == (approx(500), 0, 0)
    # Action
    api_raar.change_module(add_projs=[api_tgt_ship.id])
    # Verification - reps are limited from 1500 / cycle to 1000 / cycle
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(hp=True))
    assert api_tgt_fit_stats.hp.shield == (approx(2000), 0, 0)
    assert api_tgt_fit_stats.hp.armor == (approx(1000), 0, approx(8000))
    assert api_tgt_fit_stats.hp.hull == (approx(500), 0, 0)
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(hp=True))
    assert api_tgt_ship_stats.hp.shield == (approx(2000), 0, 0)
    assert api_tgt_ship_stats.hp.armor == (approx(1000), 0, approx(8000))
    assert api_tgt_ship_stats.hp.hull == (approx(500), 0, 0)
    # Action
    api_tgt_ship.change_ship(coordinates=(15000, 0, 0))
    # Verification - reps are reduced by RR resistance, and are no longer limited by HP:
    # 1500 (not 1000) * 8 * 0.5 = 6000
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(hp=True))
    assert api_tgt_fit_stats.hp.shield == (approx(2000), 0, 0)
    assert api_tgt_fit_stats.hp.armor == (approx(1000), 0, approx(6000))
    assert api_tgt_fit_stats.hp.hull == (approx(500), 0, 0)
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(hp=True))
    assert api_tgt_ship_stats.hp.shield == (approx(2000), 0, 0)
    assert api_tgt_ship_stats.hp.armor == (approx(1000), 0, approx(6000))
    assert api_tgt_ship_stats.hp.hull == (approx(500), 0, 0)


def test_remote_aar_drone_fighter(client, consts):
    # Accuracy = cases like 2.3 / 0.1 = 22.999999999999996
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_drone_id = make_eve_tankable(client=client, basic_info=eve_basic_info, hps=(1728, 672, 600))
    eve_fighter_id = make_eve_tankable(client=client, basic_info=eve_basic_info, hps=(2190, None, 100), fighter_count=9)
    eve_rep_item_id = make_eve_remote_aar(
        client=client, basic_info=eve_basic_info, rep_amount=100, cycle_time=5000, capacity=1, charge_rate=1)
    eve_charge_item_id = client.mk_eve_item(
        id_=consts.EveItem.nanite_repair_paste, attrs={eve_basic_info.volume_attr_id: 0.1})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_raar = api_src_fit.add_module(
        type_id=eve_rep_item_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_item_id)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_drone = api_tgt_fit.add_drone(type_id=eve_drone_id)
    api_tgt_fighter = api_tgt_fit.add_fighter(type_id=eve_fighter_id)
    api_raar.change_module(add_projs=[api_tgt_drone.id, api_tgt_fighter.id])
    # Verification - fighter receives 0 since it has no armor to apply reps to
    api_tgt_drone_stats = api_tgt_drone.get_stats(options=ItemStatsOptions(hp=True))
    assert api_tgt_drone_stats.hp.shield == (approx(1728), 0, 0)
    assert api_tgt_drone_stats.hp.armor == (approx(672), 0, approx(3000))
    assert api_tgt_drone_stats.hp.hull == (approx(600), 0, 0)
    api_tgt_fighter_stats = api_tgt_fighter.get_stats(options=ItemStatsOptions(hp=True))
    assert api_tgt_fighter_stats.hp.shield == (approx(2190), 0, 0)
    assert api_tgt_fighter_stats.hp.armor == (0, 0, 0)
    assert api_tgt_fighter_stats.hp.hull == (approx(100), 0, 0)


def test_no_ship(client, consts):
    setup_tank_basics(client=client, consts=consts)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(hp=True))
    assert api_stats.hp is None


def test_item_not_loaded(client, consts):
    setup_tank_basics(client=client, consts=consts)
    eve_item_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_item_id)
    api_drone = api_fit.add_drone(type_id=eve_item_id)
    api_fighter = api_fit.add_fighter(type_id=eve_item_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(hp=True))
    assert api_fit_stats.hp is None
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(hp=True))
    assert api_ship_stats.hp is None
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(hp=True))
    assert api_drone_stats.hp is None
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(hp=True))
    assert api_fighter_stats.hp is None


def test_not_requested(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_ship_id = make_eve_tankable(
        client=client, basic_info=eve_basic_info, hps=(3000, 2000, 1000), maker=client.mk_eve_ship)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(hp=False))
    with check_no_field():
        api_fit_stats.hp  # noqa: B018
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(hp=False))
    with check_no_field():
        api_ship_stats.hp  # noqa: B018
