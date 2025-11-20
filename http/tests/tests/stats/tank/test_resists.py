from tests import Muta, approx, check_no_field
from tests.fw.api import FitStatsOptions, ItemStatsOptions
from tests.tests.stats.tank import make_eve_tankable, setup_tank_basics


def test_ship_modified(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_ship_id = make_eve_tankable(
        client=client,
        basic_info=eve_basic_info,
        resos_shield=(1, 0.8, 0.6, 0.4),
        resos_armor=(0.5, 0.65, 0.75, 0.7),
        resos_hull=(0.67, 0.67, 0.67, 0.67),
        maker=client.mk_eve_ship)
    eve_shield_mod_attr_id = client.mk_eve_attr()
    eve_armor_mod_attr_id = client.mk_eve_attr()
    eve_hull_em_mod_attr_id = client.mk_eve_attr()
    eve_hull_therm_mod_attr_id = client.mk_eve_attr()
    eve_hull_kin_mod_attr_id = client.mk_eve_attr()
    eve_hull_expl_mod_attr_id = client.mk_eve_attr()
    eve_mods = [
        client.mk_eve_effect_mod(
            func=consts.EveModFunc.item,
            loc=consts.EveModLoc.ship,
            op=consts.EveModOp.pre_mul,
            affector_attr_id=eve_affector_attr_id,
            affectee_attr_id=eve_affectee_attr_id)
        for eve_affector_attr_id, eve_affectee_attr_id in (
            (eve_shield_mod_attr_id, eve_basic_info.shield_res_em_attr_id),
            (eve_shield_mod_attr_id, eve_basic_info.shield_res_therm_attr_id),
            (eve_shield_mod_attr_id, eve_basic_info.shield_res_kin_attr_id),
            (eve_shield_mod_attr_id, eve_basic_info.shield_res_expl_attr_id),
            (eve_armor_mod_attr_id, eve_basic_info.armor_res_em_attr_id),
            (eve_armor_mod_attr_id, eve_basic_info.armor_res_therm_attr_id),
            (eve_armor_mod_attr_id, eve_basic_info.armor_res_kin_attr_id),
            (eve_armor_mod_attr_id, eve_basic_info.armor_res_expl_attr_id),
            (eve_hull_em_mod_attr_id, eve_basic_info.hull_res_em_attr_id),
            (eve_hull_therm_mod_attr_id, eve_basic_info.hull_res_therm_attr_id),
            (eve_hull_kin_mod_attr_id, eve_basic_info.hull_res_kin_attr_id),
            (eve_hull_expl_mod_attr_id, eve_basic_info.hull_res_expl_attr_id))]
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active, mod_info=eve_mods)
    eve_base_module_id = client.mk_eve_item(
        attrs={
            eve_shield_mod_attr_id: 0.875,
            eve_armor_mod_attr_id: 0.85,
            eve_hull_em_mod_attr_id: 0.6,
            eve_hull_therm_mod_attr_id: 0.6,
            eve_hull_kin_mod_attr_id: 0.6,
            eve_hull_expl_mod_attr_id: 0.6},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_mutated_module_id = client.mk_eve_item(eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_mutator_id = client.mk_eve_mutator(
        items=[([eve_base_module_id], eve_mutated_module_id)],
        attrs={
            eve_hull_em_mod_attr_id: (0.9, 1.05),
            eve_hull_therm_mod_attr_id: (0.9, 1.05),
            eve_hull_kin_mod_attr_id: (0.9, 1.05),
            eve_hull_expl_mod_attr_id: (0.9, 1.05)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(resists=True))
    assert api_fit_stats.resists.shield == (approx(0), approx(0.2), approx(0.4), approx(0.6))
    assert api_fit_stats.resists.armor == (approx(0.5), approx(0.35), approx(0.25), approx(0.3))
    assert api_fit_stats.resists.hull == (approx(0.33), approx(0.33), approx(0.33), approx(0.33))
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(resists=True))
    assert api_ship_stats.resists.shield == (approx(0), approx(0.2), approx(0.4), approx(0.6))
    assert api_ship_stats.resists.armor == (approx(0.5), approx(0.35), approx(0.25), approx(0.3))
    assert api_ship_stats.resists.hull == (approx(0.33), approx(0.33), approx(0.33), approx(0.33))
    # Action
    api_module = api_fit.add_module(type_id=eve_base_module_id, state=consts.ApiModuleState.active)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(resists=True))
    assert api_fit_stats.resists.shield == (approx(0.125), approx(0.3), approx(0.475), approx(0.65))
    assert api_fit_stats.resists.armor == (approx(0.575), approx(0.4475), approx(0.3625), approx(0.405))
    assert api_fit_stats.resists.hull == (approx(0.598), approx(0.598), approx(0.598), approx(0.598))
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(resists=True))
    assert api_ship_stats.resists.shield == (approx(0.125), approx(0.3), approx(0.475), approx(0.65))
    assert api_ship_stats.resists.armor == (approx(0.575), approx(0.4475), approx(0.3625), approx(0.405))
    assert api_ship_stats.resists.hull == (approx(0.598), approx(0.598), approx(0.598), approx(0.598))
    # Action
    api_module.change_module(mutation=(eve_mutator_id, {
        eve_hull_em_mod_attr_id: Muta.roll_to_api(val=0.22),
        eve_hull_therm_mod_attr_id: Muta.roll_to_api(val=0.87),
        eve_hull_kin_mod_attr_id: Muta.roll_to_api(val=0.64),
        eve_hull_expl_mod_attr_id: Muta.roll_to_api(val=0.43)}))
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(resists=True))
    assert api_fit_stats.resists.shield == (approx(0.125), approx(0.3), approx(0.475), approx(0.65))
    assert api_fit_stats.resists.armor == (approx(0.575), approx(0.4475), approx(0.3625), approx(0.405))
    assert api_fit_stats.resists.hull == (approx(0.624934), approx(0.585739), approx(0.599608), approx(0.612271))
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(resists=True))
    assert api_ship_stats.resists.shield == (approx(0.125), approx(0.3), approx(0.475), approx(0.65))
    assert api_ship_stats.resists.armor == (approx(0.575), approx(0.4475), approx(0.3625), approx(0.405))
    assert api_ship_stats.resists.hull == (approx(0.624934), approx(0.585739), approx(0.599608), approx(0.612271))


def test_drone_modified(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_drone_id = make_eve_tankable(
        client=client,
        basic_info=eve_basic_info,
        resos_shield=(1, 0.8, 0.6, 0.5),
        resos_armor=(0.5, 0.55, 0.75, 0.9),
        resos_hull=(1, 1, 1, 1),
        maker=client.mk_eve_drone)
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        item_mods=[
            client.mk_eve_buff_mod(attr_id=eve_basic_info.shield_res_therm_attr_id),
            client.mk_eve_buff_mod(attr_id=eve_basic_info.armor_res_therm_attr_id),
            client.mk_eve_buff_mod(attr_id=eve_basic_info.hull_res_therm_attr_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.buff_everything, cat_id=consts.EveEffCat.active)
    eve_fw_effect_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 70},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_drone = api_fit.add_drone(type_id=eve_drone_id)
    # Verification
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(resists=True))
    assert api_drone_stats.resists.shield == (approx(0), approx(0.2), approx(0.4), approx(0.5))
    assert api_drone_stats.resists.armor == (approx(0.5), approx(0.45), approx(0.25), approx(0.1))
    assert api_drone_stats.resists.hull == (approx(0), approx(0), approx(0), approx(0))
    # Action
    api_fw_effect = api_fit.add_fw_effect(type_id=eve_fw_effect_id)
    # Verification
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(resists=True))
    assert api_drone_stats.resists.shield == (approx(0), approx(0), approx(0.4), approx(0.5))
    assert api_drone_stats.resists.armor == (approx(0.5), approx(0.065), approx(0.25), approx(0.1))
    assert api_drone_stats.resists.hull == (approx(0), approx(0), approx(0), approx(0))
    # Action
    api_fw_effect.remove()
    # Verification
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(resists=True))
    assert api_drone_stats.resists.shield == (approx(0), approx(0.2), approx(0.4), approx(0.5))
    assert api_drone_stats.resists.armor == (approx(0.5), approx(0.45), approx(0.25), approx(0.1))
    assert api_drone_stats.resists.hull == (approx(0), approx(0), approx(0), approx(0))


def test_fighter_modified(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_fighter_id = make_eve_tankable(
        client=client,
        basic_info=eve_basic_info,
        resos_shield=(0.7, 0.85, 1, 1),
        fighter_count=9,
        maker=client.mk_eve_fighter)
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        item_mods=[
            client.mk_eve_buff_mod(attr_id=eve_basic_info.shield_res_therm_attr_id),
            client.mk_eve_buff_mod(attr_id=eve_basic_info.armor_res_therm_attr_id),
            client.mk_eve_buff_mod(attr_id=eve_basic_info.hull_res_therm_attr_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.buff_everything, cat_id=consts.EveEffCat.active)
    eve_fw_effect_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 70},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id)
    # Verification
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(resists=True))
    assert api_fighter_stats.resists.shield == (approx(0.3), approx(0.15), approx(0), approx(0))
    assert api_fighter_stats.resists.armor == (approx(0), approx(0), approx(0), approx(0))
    assert api_fighter_stats.resists.hull == (approx(0), approx(0), approx(0), approx(0))
    # Action
    api_fw_effect = api_fit.add_fw_effect(type_id=eve_fw_effect_id)
    # Verification
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(resists=True))
    assert api_fighter_stats.resists.shield == (approx(0.3), approx(0), approx(0), approx(0))
    assert api_fighter_stats.resists.armor == (approx(0), approx(0), approx(0), approx(0))
    assert api_fighter_stats.resists.hull == (approx(0), approx(0), approx(0), approx(0))
    # Action
    api_fw_effect.remove()
    # Verification
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(resists=True))
    assert api_fighter_stats.resists.shield == (approx(0.3), approx(0.15), approx(0), approx(0))
    assert api_fighter_stats.resists.armor == (approx(0), approx(0), approx(0), approx(0))
    assert api_fighter_stats.resists.hull == (approx(0), approx(0), approx(0), approx(0))


def test_no_ship(client, consts):
    setup_tank_basics(client=client, consts=consts)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(resists=True))
    assert api_fit_stats.resists is None


def test_not_loaded_item(client, consts):
    setup_tank_basics(client=client, consts=consts)
    eve_item_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_item_id)
    api_drone = api_fit.add_drone(type_id=eve_item_id)
    api_fighter = api_fit.add_fighter(type_id=eve_item_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(resists=True))
    assert api_fit_stats.resists is None
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(resists=True))
    assert api_ship_stats.resists is None
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(resists=True))
    assert api_drone_stats.resists is None
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(resists=True))
    assert api_fighter_stats.resists is None


def test_not_requested(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_ship_id = make_eve_tankable(
        client=client,
        basic_info=eve_basic_info,
        resos_shield=(1, 0.8, 0.6, 0.4),
        resos_armor=(0.5, 0.65, 0.75, 0.7),
        resos_hull=(0.67, 0.67, 0.67, 0.67),
        maker=client.mk_eve_ship)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(resists=False))
    with check_no_field():
        api_fit_stats.resists  # noqa: B018
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(resists=False))
    with check_no_field():
        api_ship_stats.resists  # noqa: B018
