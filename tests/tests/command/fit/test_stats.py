from fw import check_no_field
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
        api_item1 = api_fit_batch.add_drone(type_id=eve_drone_id)
        api_item2 = api_fit_batch.add_drone(type_id=eve_drone_id)
        api_fit_stats = api_fit_batch.get_fit_stats(
            item_options=(ItemStatsOptions(mass=True), [(ItemStatsOptions(speed=True), [api_item2.id])]))
    # Verification
    api_item1_stats = api_fit_stats.items[api_item1.id]
    assert api_item1_stats.mass.one() == 1000000
    with check_no_field():
        api_item1_stats.speed  # ruff:ignore[useless-expression]
    api_item2_stats = api_fit_stats.items[api_item2.id]
    with check_no_field():
        api_item2_stats.mass  # ruff:ignore[useless-expression]
    assert api_item2_stats.speed.one() == 500


def test_fit_item_override_backref_error(client, consts):
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_speed_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_ship_id = client.mk_eve_ship(attrs={eve_mass_attr_id: 1000000, eve_speed_attr_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    with api_fit.batch() as api_fit_batch:
        api_fit_batch.change_fit(sec_status=2.5)
        api_item = api_fit_batch.set_ship(type_id=eve_ship_id)
        api_fit_stats = api_fit_batch.get_fit_stats(
            item_options=(ItemStatsOptions(mass=True), [(ItemStatsOptions(speed=True), ['#0', '#5'])]))
    # Verification - #0 references existing command which does not return an item ID, #5 references
    # command which doesn't exist, so default is used
    api_item_stats = api_fit_stats.items[api_item.id]
    assert api_item_stats.mass.one() == 1000000
    with check_no_field():
        api_item_stats.speed  # ruff:ignore[useless-expression]


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
    # Verification
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
    # Verification
    with api_fit.batch(status_code=400, json_predicate={
            'code': 'BRF-001',
            'message': 'referenced command #0 exists, but does not have item ID info',
            'cmd_index': 2,
    }) as api_fit_batch:
        api_fit_batch.change_fit(sec_status=3.5)
        api_fit_batch.set_ship(type_id=eve_ship_id)
        api_fit_batch.get_item_stats(item_id='#0', item_options=ItemStatsOptions(mass=True))
