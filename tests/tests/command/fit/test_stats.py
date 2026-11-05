from fw import approx, check_no_field
from fw.api import FitStatsOptions, ItemStatsOptions


def test_fit_fit(client, consts):
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_speed_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_ship_id = client.mk_eve_item(attrs={eve_mass_attr_id: 1000000, eve_speed_attr_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    with api_fit.batch() as api_fit_batch:
        api_fit_batch.set_ship(type_id=eve_ship_id)
        api_fit_stats1 = api_fit_batch.get_fit_stats(fit_options=FitStatsOptions(mass=True))
        api_fit_stats2 = api_fit_batch.get_fit_stats(fit_options=FitStatsOptions(speed=True))
    # Verification
    assert api_fit_stats1.fit.mass.one() == 1000000
    with check_no_field():
        api_fit_stats1.fit.speed  # ruff:ignore[useless-expression]
    with check_no_field():
        api_fit_stats2.fit.mass  # ruff:ignore[useless-expression]
    assert api_fit_stats2.fit.speed.one() == 500


def test_fit_fit_after_change(client, consts):
    # Cycle sequences are reused only within one stats command, so later ones see changes
    eve_rep_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_dmg_amount)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_remote_armor_repairer,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module1_id = client.mk_eve_item(
        attrs={eve_rep_amount_attr_id: 376, eve_cycle_time_attr_id: 6000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_module2_id = client.mk_eve_item(
        attrs={eve_rep_amount_attr_id: 376, eve_cycle_time_attr_id: 3000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module1_id, state=consts.ApiModuleState.active)
    with api_fit.batch() as api_fit_batch:
        api_fit_stats1 = api_fit_batch.get_fit_stats(fit_options=FitStatsOptions(outgoing_rps=True))
        api_fit_batch.change_module(item_id=api_module.id, type_id=eve_module2_id)
        api_fit_stats2 = api_fit_batch.get_fit_stats(fit_options=FitStatsOptions(outgoing_rps=True))
    # Verification
    assert api_fit_stats1.fit.outgoing_rps.one().armor == approx(62.666667)
    assert api_fit_stats2.fit.outgoing_rps.one().armor == approx(125.333333)


def test_fit_item_override(client, consts):
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_speed_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_drone_id = client.mk_eve_drone(attrs={eve_mass_attr_id: 1000000, eve_speed_attr_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item1 = api_fit.add_drone(type_id=eve_drone_id)
    api_item2 = api_fit.add_drone(type_id=eve_drone_id)
    with api_fit.batch() as api_fit_batch:
        api_fit_stats = api_fit_batch.get_fit_stats(
            item_options=(ItemStatsOptions(mass=True), [(ItemStatsOptions(speed=True), [api_item2.id])]))
    # Verification
    assert len(api_fit_stats.items) == 2
    api_item1_stats = api_fit_stats.items[api_item1.id]
    assert api_item1_stats.mass.one() == 1000000
    with check_no_field():
        api_item1_stats.speed  # ruff:ignore[useless-expression]
    api_item2_stats = api_fit_stats.items[api_item2.id]
    with check_no_field():
        api_item2_stats.mass  # ruff:ignore[useless-expression]
    assert api_item2_stats.speed.one() == 500


def test_fit_item_override_backref(client, consts):
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_speed_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_drone_id = client.mk_eve_drone(attrs={eve_mass_attr_id: 1000000, eve_speed_attr_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    with api_fit.batch() as api_fit_batch:
        api_fit_batch.change_fit(sec_status=2.5)
        api_item1 = api_fit_batch.add_drone(type_id=eve_drone_id)
        api_item2 = api_fit_batch.add_drone(type_id=eve_drone_id)
        api_fit_stats = api_fit_batch.get_fit_stats(
            item_options=(ItemStatsOptions(mass=True), [(ItemStatsOptions(speed=True), ['#0', '#9', api_item2.id])]))
    # Verification
    assert len(api_fit_stats.items) == 2
    api_item1_stats = api_fit_stats.items[api_item1.id]
    assert api_item1_stats.mass.one() == 1000000
    with check_no_field():
        api_item1_stats.speed  # ruff:ignore[useless-expression]
    api_item2_stats = api_fit_stats.items[api_item2.id]
    with check_no_field():
        api_item2_stats.mass  # ruff:ignore[useless-expression]
    assert api_item2_stats.speed.one() == 500


def test_fit_item_override_other_fit(client, consts):
    eve_speed_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_drone_id = client.mk_eve_drone(attrs={eve_speed_attr_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_drone(type_id=eve_drone_id)
    api_other_fit = api_sol.create_fit()
    api_other_fit_item = api_other_fit.add_drone(type_id=eve_drone_id)
    api_no_fit_item = api_sol.add_sw_effect(type_id=eve_drone_id)
    with api_fit.batch() as api_fit_batch:
        api_fit_stats = api_fit_batch.get_fit_stats(item_options=(ItemStatsOptions(), [(
            ItemStatsOptions(speed=True), [api_item.id, api_other_fit_item.id, api_no_fit_item.id])]))
    # Verification - overrides for items which are not on the fit are ignored
    assert api_fit_stats.items.keys() == {api_item.id}
    assert api_fit_stats.items[api_item.id].speed.one() == 500


def test_item_item(client, consts):
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_speed_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_ship_id = client.mk_eve_ship(attrs={eve_mass_attr_id: 1000000, eve_speed_attr_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.set_ship(type_id=eve_ship_id)
    with api_fit.batch() as api_fit_batch:
        api_item_stats1 = api_fit_batch.get_item_stats(item_id=api_item.id, item_options=ItemStatsOptions(mass=True))
        api_item_stats2 = api_fit_batch.get_item_stats(item_id=api_item.id, item_options=ItemStatsOptions(speed=True))
    # Verification
    assert api_item_stats1.item.mass.one() == 1000000
    with check_no_field():
        api_item_stats1.item.speed  # ruff:ignore[useless-expression]
    with check_no_field():
        api_item_stats2.item.mass  # ruff:ignore[useless-expression]
    assert api_item_stats2.item.speed.one() == 500


def test_item_item_after_change(client, consts):
    # Cycle sequences are reused only within one stats command, so later ones see changes
    eve_rep_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_dmg_amount)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_remote_armor_repairer,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module1_id = client.mk_eve_item(
        attrs={eve_rep_amount_attr_id: 376, eve_cycle_time_attr_id: 6000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_module2_id = client.mk_eve_item(
        attrs={eve_rep_amount_attr_id: 376, eve_cycle_time_attr_id: 3000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module1_id, state=consts.ApiModuleState.active)
    with api_fit.batch() as api_fit_batch:
        api_item_stats1 = api_fit_batch.get_item_stats(
            item_id=api_module.id,
            item_options=ItemStatsOptions(outgoing_rps=True))
        api_fit_batch.change_module(item_id=api_module.id, type_id=eve_module2_id)
        api_item_stats2 = api_fit_batch.get_item_stats(
            item_id=api_module.id,
            item_options=ItemStatsOptions(outgoing_rps=True))
    # Verification
    assert api_item_stats1.item.outgoing_rps.one().armor == approx(62.666667)
    assert api_item_stats2.item.outgoing_rps.one().armor == approx(125.333333)


def test_item_item_backref(client, consts):
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_speed_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_ship_id = client.mk_eve_ship(attrs={eve_mass_attr_id: 1000000, eve_speed_attr_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    with api_fit.batch() as api_fit_batch:
        api_item = api_fit_batch.set_ship(type_id=eve_ship_id)
        api_item_stats1 = api_fit_batch.get_item_stats(item_id=api_item.id, item_options=ItemStatsOptions(mass=True))
        api_item_stats2 = api_fit_batch.get_item_stats(item_id=api_item.id, item_options=ItemStatsOptions(speed=True))
    # Verification
    assert api_item_stats1.item.mass.one() == 1000000
    with check_no_field():
        api_item_stats1.item.speed  # ruff:ignore[useless-expression]
    with check_no_field():
        api_item_stats2.item.mass  # ruff:ignore[useless-expression]
    assert api_item_stats2.item.speed.one() == 500


def test_item_item_backref_error_range(client):
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Action & verification
    with api_fit.batch(status_code=400, json_predicate={
            'code': 'BRF-001',
            'message': 'referenced command #2 does not have results recorded',
            'cmd_index': 1,
    }) as api_fit_batch:
        api_fit_batch.set_ship(type_id=eve_ship_id)
        api_fit_batch.get_item_stats(item_id='#2', item_options=ItemStatsOptions(mass=True))


def test_item_item_backref_error_kind(client):
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Action & verification
    with api_fit.batch(status_code=400, json_predicate={
            'code': 'BRF-001',
            'message': 'referenced command #0 exists, but does not have item ID info',
            'cmd_index': 2,
    }) as api_fit_batch:
        api_fit_batch.change_fit(sec_status=3.5)
        api_fit_batch.set_ship(type_id=eve_ship_id)
        api_fit_batch.get_item_stats(item_id='#0', item_options=ItemStatsOptions(mass=True))


def test_item_item_error_not_found(client):
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_other_item = api_sol.add_sw_effect(type_id=eve_ship_id)
    api_other_item.remove()
    # Action & verification
    with api_fit.batch(status_code=400, json_predicate={
            'code': 'ITM-001',
            'message': f'item {api_other_item.id} not found',
            'cmd_index': 1,
    }) as api_fit_batch:
        api_fit_batch.set_ship(type_id=eve_ship_id)
        api_fit_batch.get_item_stats(item_id=api_other_item.id, item_options=ItemStatsOptions(mass=True))


def test_item_item_error_other_fit(client):
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_ship = api_fit2.set_ship(type_id=eve_ship_id)
    # Action & verification
    with api_fit1.batch(status_code=400, json_predicate={
            'code': 'ITM-003',
            'message': f'item {api_ship.id} is not on fit {api_fit1.id}',
            'cmd_index': 0,
    }) as api_fit_batch:
        api_fit_batch.get_item_stats(item_id=api_ship.id, item_options=ItemStatsOptions(mass=True))
