from tests import Spool, approx, check_no_field
from tests.fw.api import ItemStatsOptions


def test_cycles_basic(client, consts):
    eve_rep_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_dmg_amount)
    eve_spool_step_id = client.mk_eve_attr(id_=consts.EveAttr.repair_mult_bonus_per_cycle)
    eve_spool_max_id = client.mk_eve_attr(id_=consts.EveAttr.repair_mult_bonus_max)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_module_spool_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_remote_armor_mutadaptive_repairer,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_spool_id = client.mk_eve_item(
        attrs={
            eve_rep_amount_attr_id: 512,
            eve_spool_step_id: 0.12,
            eve_spool_max_id: 1.8,
            eve_cycle_time_attr_id: 6000},
        eff_ids=[eve_module_spool_effect_id],
        defeff_id=eve_module_spool_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module_spool_id,
        state=consts.ApiModuleState.active,
        spool=Spool.cycles_to_api(count=0))
    # Verification
    assert api_module.update().spool_cycles == (0, 15, True)
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(rr_armor=True))
    assert api_module_stats.rr_armor == [approx(85.333333)]
    # Action
    api_module.change_module(spool=Spool.cycles_to_api(count=20))
    # Verification
    assert api_module.update().spool_cycles == (15, 15, True)
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(rr_armor=True))
    assert api_module_stats.rr_armor == [approx(238.933333)]
    # Action
    api_module.change_module(spool=Spool.cycles_to_api(count=8))
    # Verification
    assert api_module.update().spool_cycles == (8, 15, True)
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(rr_armor=True))
    assert api_module_stats.rr_armor == [approx(167.253333)]


def test_time_basic(client, consts):
    eve_rep_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_dmg_amount)
    eve_spool_step_id = client.mk_eve_attr(id_=consts.EveAttr.repair_mult_bonus_per_cycle)
    eve_spool_max_id = client.mk_eve_attr(id_=consts.EveAttr.repair_mult_bonus_max)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_module_spool_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_remote_armor_mutadaptive_repairer,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_spool_id = client.mk_eve_item(
        attrs={
            eve_rep_amount_attr_id: 512,
            eve_spool_step_id: 0.12,
            eve_spool_max_id: 1.8,
            eve_cycle_time_attr_id: 6000},
        eff_ids=[eve_module_spool_effect_id],
        defeff_id=eve_module_spool_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module_spool_id,
        state=consts.ApiModuleState.active,
        spool=Spool.time_to_api(time=0))
    # Verification
    assert api_module.update().spool_cycles == (0, 15, True)
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(rr_armor=True))
    assert api_module_stats.rr_armor == [approx(85.333333)]
    # Action
    api_module.change_module(spool=Spool.time_to_api(time=5.99))
    # Verification
    assert api_module.update().spool_cycles == (0, 15, True)
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(rr_armor=True))
    assert api_module_stats.rr_armor == [approx(85.333333)]
    # Action
    api_module.change_module(spool=Spool.time_to_api(time=6))
    # Verification
    assert api_module.update().spool_cycles == (1, 15, True)
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(rr_armor=True))
    assert api_module_stats.rr_armor == [approx(95.573333)]
    # Action
    api_module.change_module(spool=Spool.time_to_api(time=25))
    # Verification
    assert api_module.update().spool_cycles == (4, 15, True)
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(rr_armor=True))
    assert api_module_stats.rr_armor == [approx(126.293333)]
    # Action
    api_module.change_module(spool=Spool.time_to_api(time=250))
    # Verification
    assert api_module.update().spool_cycles == (15, 15, True)
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(rr_armor=True))
    assert api_module_stats.rr_armor == [approx(238.933333)]
    # Action
    api_module.change_module(spool=Spool.time_to_api(time=89.99))
    # Verification
    assert api_module.update().spool_cycles == (14, 15, True)
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(rr_armor=True))
    assert api_module_stats.rr_armor == [approx(228.693333)]


def test_time_cycle_time_zero(client, consts):
    eve_rep_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_dmg_amount)
    eve_spool_step_id = client.mk_eve_attr(id_=consts.EveAttr.repair_mult_bonus_per_cycle)
    eve_spool_max_id = client.mk_eve_attr(id_=consts.EveAttr.repair_mult_bonus_max)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_module_spool_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_remote_armor_mutadaptive_repairer,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_spool_id = client.mk_eve_item(
        attrs={
            eve_rep_amount_attr_id: 512,
            eve_spool_step_id: 0.12,
            eve_spool_max_id: 1.8,
            eve_cycle_time_attr_id: 0},
        eff_ids=[eve_module_spool_effect_id],
        defeff_id=eve_module_spool_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module_spool_id,
        state=consts.ApiModuleState.active,
        spool=Spool.time_to_api(time=0))
    # Verification
    api_module.update()
    with check_no_field():
        api_module.spool_cycles  # noqa: B018
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(rr_armor=True))
    assert api_module_stats.rr_armor == [0]


def test_spool_scale_basic(client, consts):
    eve_rep_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_dmg_amount)
    eve_spool_step_id = client.mk_eve_attr(id_=consts.EveAttr.repair_mult_bonus_per_cycle)
    eve_spool_max_id = client.mk_eve_attr(id_=consts.EveAttr.repair_mult_bonus_max)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_module_spool_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_remote_armor_mutadaptive_repairer,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_spool_id = client.mk_eve_item(
        attrs={
            eve_rep_amount_attr_id: 512,
            eve_spool_step_id: 0.1,
            eve_spool_max_id: 0.455,
            eve_cycle_time_attr_id: 6000},
        eff_ids=[eve_module_spool_effect_id],
        defeff_id=eve_module_spool_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module_spool_id,
        state=consts.ApiModuleState.active,
        spool=Spool.spool_scale_to_api(val=0))
    # Verification
    assert api_module.update().spool_cycles == (0, 5, True)
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(rr_armor=True))
    assert api_module_stats.rr_armor == [approx(85.333333)]
    # Action
    api_module.change_module(spool=Spool.spool_scale_to_api(val=0.42))
    # Verification
    assert api_module.update().spool_cycles == (2, 5, True)
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(rr_armor=True))
    assert api_module_stats.rr_armor == [approx(102.4)]
    # Action
    api_module.change_module(spool=Spool.spool_scale_to_api(val=1))
    # Verification
    assert api_module.update().spool_cycles == (5, 5, True)
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(rr_armor=True))
    assert api_module_stats.rr_armor == [approx(124.16)]


def test_cycle_scale_basic(client, consts):
    eve_rep_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_dmg_amount)
    eve_spool_step_id = client.mk_eve_attr(id_=consts.EveAttr.repair_mult_bonus_per_cycle)
    eve_spool_max_id = client.mk_eve_attr(id_=consts.EveAttr.repair_mult_bonus_max)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_module_spool_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_remote_armor_mutadaptive_repairer,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_spool_id = client.mk_eve_item(
        attrs={
            eve_rep_amount_attr_id: 512,
            eve_spool_step_id: 0.1,
            eve_spool_max_id: 0.455,
            eve_cycle_time_attr_id: 6000},
        eff_ids=[eve_module_spool_effect_id],
        defeff_id=eve_module_spool_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module_spool_id,
        state=consts.ApiModuleState.active,
        spool=Spool.cycle_scale_to_api(val=0))
    # Verification
    assert api_module.update().spool_cycles == (0, 5, True)
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(rr_armor=True))
    assert api_module_stats.rr_armor == [approx(85.333333)]
    # Action
    api_module.change_module(spool=Spool.cycle_scale_to_api(val=0.42))
    # Verification
    assert api_module.update().spool_cycles == (3, 5, True)
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(rr_armor=True))
    assert api_module_stats.rr_armor == [approx(110.933333)]
    # Action
    api_module.change_module(spool=Spool.cycle_scale_to_api(val=1))
    # Verification
    assert api_module.update().spool_cycles == (5, 5, True)
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(rr_armor=True))
    assert api_module_stats.rr_armor == [approx(124.16)]


def test_step_zero(client, consts):
    eve_rep_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_dmg_amount)
    eve_spool_step_id = client.mk_eve_attr(id_=consts.EveAttr.repair_mult_bonus_per_cycle)
    eve_spool_max_id = client.mk_eve_attr(id_=consts.EveAttr.repair_mult_bonus_max)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_module_spool_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_remote_armor_mutadaptive_repairer,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_spool_id = client.mk_eve_item(
        attrs={
            eve_rep_amount_attr_id: 512,
            eve_spool_step_id: 0,
            eve_spool_max_id: 1.8,
            eve_cycle_time_attr_id: 6000},
        eff_ids=[eve_module_spool_effect_id],
        defeff_id=eve_module_spool_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module_spool_id,
        state=consts.ApiModuleState.active,
        spool=Spool.cycles_to_api(count=5))
    # Verification
    api_module.update()
    with check_no_field():
        api_module.spool_cycles  # noqa: B018
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(rr_armor=True))
    assert api_module_stats.rr_armor == [approx(85.333333)]
    # Action
    api_module.change_module(spool=Spool.time_to_api(time=25))
    # Verification
    api_module.update()
    with check_no_field():
        api_module.spool_cycles  # noqa: B018
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(rr_armor=True))
    assert api_module_stats.rr_armor == [approx(85.333333)]
    # Action
    api_module.change_module(spool=Spool.spool_scale_to_api(val=0.42))
    # Verification
    api_module.update()
    with check_no_field():
        api_module.spool_cycles  # noqa: B018
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(rr_armor=True))
    assert api_module_stats.rr_armor == [approx(85.333333)]
    # Action
    api_module.change_module(spool=Spool.cycle_scale_to_api(val=1))
    # Verification
    api_module.update()
    with check_no_field():
        api_module.spool_cycles  # noqa: B018
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(rr_armor=True))
    assert api_module_stats.rr_armor == [approx(85.333333)]
