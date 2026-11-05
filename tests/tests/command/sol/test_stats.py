from fw import approx, check_no_field
from fw.api import FitStatsOptions, FleetStatsOptions, ItemStatsOptions


def test_sol_fleet_override(client, consts):
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_ship_id = client.mk_eve_ship(attrs={eve_mass_attr_id: 1000000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fleet1 = api_sol.create_fleet()
    api_fleet2 = api_sol.create_fleet()
    with api_sol.batch() as api_sol_batch:
        api_fit1 = api_sol_batch.create_fit(fleet_id=api_fleet1.id)
        api_sol_batch.set_ship(fit_id=api_fit1.id, type_id=eve_ship_id)
        api_fit2 = api_sol_batch.create_fit(fleet_id=api_fleet2.id)
        api_sol_batch.set_ship(fit_id=api_fit2.id, type_id=eve_ship_id)
        api_sol_stats = api_sol_batch.get_sol_stats(
            fleet_options=(FleetStatsOptions(mass=True), [(FleetStatsOptions(outgoing_nps=True), [api_fleet2.id])]))
    # Verification
    api_fleet1_stats = api_sol_stats.fleets[api_fleet1.id]
    assert api_fleet1_stats.mass.one() == 1000000
    with check_no_field():
        api_fleet1_stats.outgoing_nps  # ruff:ignore[useless-expression]
    api_fleet2_stats = api_sol_stats.fleets[api_fleet2.id]
    with check_no_field():
        api_fleet2_stats.mass  # ruff:ignore[useless-expression]
    assert api_fleet2_stats.outgoing_nps.one() == 0


def test_sol_fleet_override_backref(client, consts):
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_ship_id = client.mk_eve_ship(attrs={eve_mass_attr_id: 1000000})
    client.create_sources()
    api_sol = client.create_sol()
    with api_sol.batch() as api_sol_batch:
        api_fleet1 = api_sol_batch.create_fleet()
        api_fleet2 = api_sol_batch.create_fleet()
        api_fit1 = api_sol_batch.create_fit(fleet_id=api_fleet1.id)
        api_sol_batch.set_ship(fit_id=api_fit1.id, type_id=eve_ship_id)
        api_fit2 = api_sol_batch.create_fit(fleet_id=api_fleet2.id)
        api_sol_batch.set_ship(fit_id=api_fit2.id, type_id=eve_ship_id)
        api_sol_stats = api_sol_batch.get_sol_stats(fleet_options=(
            FleetStatsOptions(mass=True),
            [(FleetStatsOptions(outgoing_nps=True), ['#2', '#9', api_fleet2.id])]))
    # Verification
    api_fleet1_stats = api_sol_stats.fleets[api_fleet1.id]
    assert api_fleet1_stats.mass.one() == 1000000
    with check_no_field():
        api_fleet1_stats.outgoing_nps  # ruff:ignore[useless-expression]
    api_fleet2_stats = api_sol_stats.fleets[api_fleet2.id]
    with check_no_field():
        api_fleet2_stats.mass  # ruff:ignore[useless-expression]
    assert api_fleet2_stats.outgoing_nps.one() == 0


def test_sol_fit_override(client, consts):
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_speed_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_ship_id = client.mk_eve_ship(attrs={eve_mass_attr_id: 1000000, eve_speed_attr_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    with api_sol.batch() as api_sol_batch:
        api_sol_batch.set_ship(fit_id=api_fit1.id, type_id=eve_ship_id)
        api_sol_batch.set_ship(fit_id=api_fit2.id, type_id=eve_ship_id)
        api_sol_stats = api_sol_batch.get_sol_stats(
            fit_options=(FitStatsOptions(mass=True), [(FitStatsOptions(speed=True), [api_fit2.id])]))
    # Verification
    assert len(api_sol_stats.fits) == 2
    api_fit1_stats = api_sol_stats.fits[api_fit1.id]
    assert api_fit1_stats.mass.one() == 1000000
    with check_no_field():
        api_fit1_stats.speed  # ruff:ignore[useless-expression]
    api_fit2_stats = api_sol_stats.fits[api_fit2.id]
    with check_no_field():
        api_fit2_stats.mass  # ruff:ignore[useless-expression]
    assert api_fit2_stats.speed.one() == 500


def test_sol_fit_override_backref(client, consts):
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_speed_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_ship_id = client.mk_eve_ship(attrs={eve_mass_attr_id: 1000000, eve_speed_attr_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    with api_sol.batch() as api_sol_batch:
        api_fit1 = api_sol_batch.create_fit()
        api_sol_batch.set_ship(fit_id=api_fit1.id, type_id=eve_ship_id)
        api_fit2 = api_sol_batch.create_fit()
        api_sol_batch.set_ship(fit_id=api_fit2.id, type_id=eve_ship_id)
        api_sol_stats = api_sol_batch.get_sol_stats(
            fit_options=(FitStatsOptions(mass=True), [(FitStatsOptions(speed=True), ['#1', '#9', api_fit2.id])]))
    # Verification
    assert len(api_sol_stats.fits) == 2
    api_fit1_stats = api_sol_stats.fits[api_fit1.id]
    assert api_fit1_stats.mass.one() == 1000000
    with check_no_field():
        api_fit1_stats.speed  # ruff:ignore[useless-expression]
    api_fit2_stats = api_sol_stats.fits[api_fit2.id]
    with check_no_field():
        api_fit2_stats.mass  # ruff:ignore[useless-expression]
    assert api_fit2_stats.speed.one() == 500


def test_sol_fit_after_change(client, consts):
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
    with api_sol.batch() as api_sol_batch:
        api_sol_stats1 = api_sol_batch.get_sol_stats(fit_options=FitStatsOptions(outgoing_rps=True))
        api_sol_batch.change_module(item_id=api_module.id, type_id=eve_module2_id)
        api_sol_stats2 = api_sol_batch.get_sol_stats(fit_options=FitStatsOptions(outgoing_rps=True))
    # Verification
    assert api_sol_stats1.fits[api_fit.id].outgoing_rps.one().armor == approx(62.666667)
    assert api_sol_stats2.fits[api_fit.id].outgoing_rps.one().armor == approx(125.333333)


def test_sol_item_override(client, consts):
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_speed_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_drone_id = client.mk_eve_drone(attrs={eve_mass_attr_id: 1000000, eve_speed_attr_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item1 = api_fit.add_drone(type_id=eve_drone_id)
    api_item2 = api_fit.add_drone(type_id=eve_drone_id)
    with api_sol.batch() as api_sol_batch:
        api_sol_stats = api_sol_batch.get_sol_stats(
            item_options=(ItemStatsOptions(mass=True), [(ItemStatsOptions(speed=True), [api_item2.id])]))
    # Verification
    assert len(api_sol_stats.items) == 2
    api_item1_stats = api_sol_stats.items[api_item1.id]
    assert api_item1_stats.mass.one() == 1000000
    with check_no_field():
        api_item1_stats.speed  # ruff:ignore[useless-expression]
    api_item2_stats = api_sol_stats.items[api_item2.id]
    with check_no_field():
        api_item2_stats.mass  # ruff:ignore[useless-expression]
    assert api_item2_stats.speed.one() == 500


def test_sol_item_override_backref(client, consts):
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_speed_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_drone_id = client.mk_eve_drone(attrs={eve_mass_attr_id: 1000000, eve_speed_attr_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    with api_sol.batch() as api_sol_batch:
        api_fit = api_sol_batch.create_fit()
        api_item1 = api_sol_batch.add_drone(fit_id=api_fit.id, type_id=eve_drone_id)
        api_item2 = api_sol_batch.add_drone(fit_id=api_fit.id, type_id=eve_drone_id)
        api_sol_stats = api_sol_batch.get_sol_stats(
            item_options=(ItemStatsOptions(mass=True), [(ItemStatsOptions(speed=True), ['#0', '#9', api_item2.id])]))
    # Verification
    assert len(api_sol_stats.items) == 2
    api_item1_stats = api_sol_stats.items[api_item1.id]
    assert api_item1_stats.mass.one() == 1000000
    with check_no_field():
        api_item1_stats.speed  # ruff:ignore[useless-expression]
    api_item2_stats = api_sol_stats.items[api_item2.id]
    with check_no_field():
        api_item2_stats.mass  # ruff:ignore[useless-expression]
    assert api_item2_stats.speed.one() == 500


def test_fleet_fleet(client, consts):
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_ship_id = client.mk_eve_ship(attrs={eve_mass_attr_id: 1000000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fleet = api_sol.create_fleet()
    with api_sol.batch() as api_sol_batch:
        api_fit = api_sol_batch.create_fit(fleet_id=api_fleet.id)
        api_sol_batch.set_ship(fit_id=api_fit.id, type_id=eve_ship_id)
        api_fleet_stats1 = api_sol_batch.get_fleet_stats(
            fleet_id=api_fleet.id,
            fleet_options=FleetStatsOptions(mass=True))
        api_fleet_stats2 = api_sol_batch.get_fleet_stats(
            fleet_id=api_fleet.id,
            fleet_options=FleetStatsOptions(outgoing_nps=True))
    # Verification
    assert api_fleet_stats1.fleet.mass.one() == 1000000
    with check_no_field():
        api_fleet_stats1.fleet.outgoing_nps  # ruff:ignore[useless-expression]
    with check_no_field():
        api_fleet_stats2.fleet.mass  # ruff:ignore[useless-expression]
    assert api_fleet_stats2.fleet.outgoing_nps.one() == 0


def test_fleet_fleet_after_change(client, consts):
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
    api_fleet = api_sol.create_fleet()
    api_fit = api_sol.create_fit(fleet_id=api_fleet.id)
    api_module = api_fit.add_module(type_id=eve_module1_id, state=consts.ApiModuleState.active)
    with api_sol.batch() as api_sol_batch:
        api_fleet_stats1 = api_sol_batch.get_fleet_stats(
            fleet_id=api_fleet.id,
            fleet_options=FleetStatsOptions(outgoing_rps=True))
        api_sol_batch.change_module(item_id=api_module.id, type_id=eve_module2_id)
        api_fleet_stats2 = api_sol_batch.get_fleet_stats(
            fleet_id=api_fleet.id,
            fleet_options=FleetStatsOptions(outgoing_rps=True))
    # Verification
    assert api_fleet_stats1.fleet.outgoing_rps.one().armor == approx(62.666667)
    assert api_fleet_stats2.fleet.outgoing_rps.one().armor == approx(125.333333)


def test_fleet_fleet_backref(client, consts):
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_ship_id = client.mk_eve_ship(attrs={eve_mass_attr_id: 1000000})
    client.create_sources()
    api_sol = client.create_sol()
    with api_sol.batch() as api_sol_batch:
        api_fleet = api_sol_batch.create_fleet()
        api_fit = api_sol_batch.create_fit(fleet_id=api_fleet.id)
        api_sol_batch.set_ship(fit_id=api_fit.id, type_id=eve_ship_id)
        api_fleet_stats1 = api_sol_batch.get_fleet_stats(
            fleet_id=api_fleet.id,
            fleet_options=FleetStatsOptions(mass=True))
        api_fleet_stats2 = api_sol_batch.get_fleet_stats(
            fleet_id=api_fleet.id,
            fleet_options=FleetStatsOptions(outgoing_nps=True))
    # Verification
    assert api_fleet_stats1.fleet.mass.one() == 1000000
    with check_no_field():
        api_fleet_stats1.fleet.outgoing_nps  # ruff:ignore[useless-expression]
    with check_no_field():
        api_fleet_stats2.fleet.mass  # ruff:ignore[useless-expression]
    assert api_fleet_stats2.fleet.outgoing_nps.one() == 0


def test_fleet_fleet_backref_error_range(client):
    client.create_sources()
    api_sol = client.create_sol()
    # Action & verification
    with api_sol.batch(status_code=400, json_predicate={
            'code': 'BRF-001',
            'message': 'referenced command #2 does not have results recorded',
            'cmd_index': 1,
    }) as api_sol_batch:
        api_sol_batch.create_fleet()
        api_sol_batch.get_fleet_stats(fleet_id='#2', fleet_options=FleetStatsOptions(mass=True))


def test_fleet_fleet_backref_error_kind(client):
    client.create_sources()
    api_sol = client.create_sol()
    # Action & verification
    with api_sol.batch(status_code=400, json_predicate={
            'code': 'BRF-001',
            'message': 'referenced command #0 exists, but does not have fleet ID info',
            'cmd_index': 1,
    }) as api_sol_batch:
        api_sol_batch.create_fit()
        api_sol_batch.get_fleet_stats(fleet_id='#0', fleet_options=FleetStatsOptions(mass=True))


def test_fleet_fit_override(client, consts):
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_speed_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_ship_id = client.mk_eve_ship(attrs={eve_mass_attr_id: 1000000, eve_speed_attr_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    with api_sol.batch() as api_sol_batch:
        api_fleet = api_sol_batch.create_fleet(fit_ids=[api_fit1.id, api_fit2.id])
        api_sol_batch.set_ship(fit_id=api_fit1.id, type_id=eve_ship_id)
        api_sol_batch.set_ship(fit_id=api_fit2.id, type_id=eve_ship_id)
        api_fleet_stats = api_sol_batch.get_fleet_stats(
            fleet_id=api_fleet.id,
            fit_options=(FitStatsOptions(mass=True), [(FitStatsOptions(speed=True), [api_fit2.id])]))
    # Verification
    assert len(api_fleet_stats.fits) == 2
    api_fit1_stats = api_fleet_stats.fits[api_fit1.id]
    assert api_fit1_stats.mass.one() == 1000000
    with check_no_field():
        api_fit1_stats.speed  # ruff:ignore[useless-expression]
    api_fit2_stats = api_fleet_stats.fits[api_fit2.id]
    with check_no_field():
        api_fit2_stats.mass  # ruff:ignore[useless-expression]
    assert api_fit2_stats.speed.one() == 500


def test_fleet_fit_override_backref(client, consts):
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_speed_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_ship_id = client.mk_eve_ship(attrs={eve_mass_attr_id: 1000000, eve_speed_attr_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    with api_sol.batch() as api_sol_batch:
        api_fleet = api_sol_batch.create_fleet()
        api_fit1 = api_sol_batch.create_fit(fleet_id=api_fleet.id)
        api_sol_batch.set_ship(fit_id=api_fit1.id, type_id=eve_ship_id)
        api_fit2 = api_sol_batch.create_fit(fleet_id=api_fleet.id)
        api_sol_batch.set_ship(fit_id=api_fit2.id, type_id=eve_ship_id)
        api_fleet_stats = api_sol_batch.get_fleet_stats(
            fleet_id=api_fleet.id,
            fit_options=(FitStatsOptions(mass=True), [(FitStatsOptions(speed=True), ['#0', '#9', api_fit2.id])]))
    # Verification
    assert len(api_fleet_stats.fits) == 2
    api_fit1_stats = api_fleet_stats.fits[api_fit1.id]
    assert api_fit1_stats.mass.one() == 1000000
    with check_no_field():
        api_fit1_stats.speed  # ruff:ignore[useless-expression]
    api_fit2_stats = api_fleet_stats.fits[api_fit2.id]
    with check_no_field():
        api_fit2_stats.mass  # ruff:ignore[useless-expression]
    assert api_fit2_stats.speed.one() == 500


def test_fleet_item_override(client, consts):
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_speed_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_drone_id = client.mk_eve_drone(attrs={eve_mass_attr_id: 1000000, eve_speed_attr_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item1 = api_fit.add_drone(type_id=eve_drone_id)
    api_item2 = api_fit.add_drone(type_id=eve_drone_id)
    with api_sol.batch() as api_sol_batch:
        api_fleet = api_sol_batch.create_fleet(fit_ids=[api_fit.id])
        api_fleet_stats = api_sol_batch.get_fleet_stats(
            fleet_id=api_fleet.id,
            item_options=(ItemStatsOptions(mass=True), [(ItemStatsOptions(speed=True), [api_item2.id])]))
    # Verification
    assert len(api_fleet_stats.items) == 2
    api_item1_stats = api_fleet_stats.items[api_item1.id]
    assert api_item1_stats.mass.one() == 1000000
    with check_no_field():
        api_item1_stats.speed  # ruff:ignore[useless-expression]
    api_item2_stats = api_fleet_stats.items[api_item2.id]
    with check_no_field():
        api_item2_stats.mass  # ruff:ignore[useless-expression]
    assert api_item2_stats.speed.one() == 500


def test_fleet_item_override_backref(client, consts):
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_speed_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_drone_id = client.mk_eve_drone(attrs={eve_mass_attr_id: 1000000, eve_speed_attr_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    with api_sol.batch() as api_sol_batch:
        api_fleet = api_sol_batch.create_fleet()
        api_fit = api_sol_batch.create_fit(fleet_id=api_fleet.id)
        api_item1 = api_sol_batch.add_drone(fit_id=api_fit.id, type_id=eve_drone_id)
        api_item2 = api_sol_batch.add_drone(fit_id=api_fit.id, type_id=eve_drone_id)
        api_fleet_stats = api_sol_batch.get_fleet_stats(
            fleet_id=api_fleet.id,
            item_options=(ItemStatsOptions(mass=True), [(ItemStatsOptions(speed=True), ['#1', '#9', api_item2.id])]))
    # Verification
    assert len(api_fleet_stats.items) == 2
    api_item1_stats = api_fleet_stats.items[api_item1.id]
    assert api_item1_stats.mass.one() == 1000000
    with check_no_field():
        api_item1_stats.speed  # ruff:ignore[useless-expression]
    api_item2_stats = api_fleet_stats.items[api_item2.id]
    with check_no_field():
        api_item2_stats.mass  # ruff:ignore[useless-expression]
    assert api_item2_stats.speed.one() == 500


def test_fleet_fit_override_other_fleet(client, consts):
    eve_speed_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_ship_id = client.mk_eve_ship(attrs={eve_speed_attr_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_fleet = api_sol.create_fleet()
    api_fit = api_sol.create_fit(fleet_id=api_fleet.id)
    api_fit.set_ship(type_id=eve_ship_id)
    api_other_fleet = api_sol.create_fleet()
    api_other_fleet_fit = api_sol.create_fit(fleet_id=api_other_fleet.id)
    api_other_fleet_fit.set_ship(type_id=eve_ship_id)
    api_no_fleet_fit = api_sol.create_fit()
    api_no_fleet_fit.set_ship(type_id=eve_ship_id)
    with api_sol.batch() as api_sol_batch:
        api_fleet_stats = api_sol_batch.get_fleet_stats(fleet_id=api_fleet.id, fit_options=(FitStatsOptions(), [(
            FitStatsOptions(speed=True), [api_fit.id, api_other_fleet_fit.id, api_no_fleet_fit.id])]))
    # Verification - overrides for fits which are not in the fleet are ignored
    assert api_fleet_stats.fits.keys() == {api_fit.id}
    assert api_fleet_stats.fits[api_fit.id].speed.one() == 500


def test_fleet_item_override_other_fleet(client, consts):
    eve_speed_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_drone_id = client.mk_eve_drone(attrs={eve_speed_attr_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_fleet = api_sol.create_fleet()
    api_fit = api_sol.create_fit(fleet_id=api_fleet.id)
    api_item = api_fit.add_drone(type_id=eve_drone_id)
    api_other_fleet = api_sol.create_fleet()
    api_other_fleet_fit = api_sol.create_fit(fleet_id=api_other_fleet.id)
    api_other_fleet_item = api_other_fleet_fit.add_drone(type_id=eve_drone_id)
    api_no_fleet_fit = api_sol.create_fit()
    api_no_fleet_item = api_no_fleet_fit.add_drone(type_id=eve_drone_id)
    api_no_fit_item = api_sol.add_sw_effect(type_id=eve_drone_id)
    with api_sol.batch() as api_sol_batch:
        api_fleet_stats = api_sol_batch.get_fleet_stats(fleet_id=api_fleet.id, item_options=(ItemStatsOptions(), [(
            ItemStatsOptions(speed=True),
            [api_item.id, api_other_fleet_item.id, api_no_fleet_item.id, api_no_fit_item.id])]))
    # Verification - overrides for items which are not in the fleet are ignored
    assert api_fleet_stats.items.keys() == {api_item.id}
    assert api_fleet_stats.items[api_item.id].speed.one() == 500


def test_fit_fit(client, consts):
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_speed_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_ship_id = client.mk_eve_ship(attrs={eve_mass_attr_id: 1000000, eve_speed_attr_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    with api_sol.batch() as api_sol_batch:
        api_sol_batch.set_ship(fit_id=api_fit.id, type_id=eve_ship_id)
        api_fit_stats1 = api_sol_batch.get_fit_stats(fit_id=api_fit.id, fit_options=FitStatsOptions(mass=True))
        api_fit_stats2 = api_sol_batch.get_fit_stats(fit_id=api_fit.id, fit_options=FitStatsOptions(speed=True))
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
    with api_sol.batch() as api_sol_batch:
        api_fit_stats1 = api_sol_batch.get_fit_stats(fit_id=api_fit.id, fit_options=FitStatsOptions(outgoing_rps=True))
        api_sol_batch.change_module(item_id=api_module.id, type_id=eve_module2_id)
        api_fit_stats2 = api_sol_batch.get_fit_stats(fit_id=api_fit.id, fit_options=FitStatsOptions(outgoing_rps=True))
    # Verification
    assert api_fit_stats1.fit.outgoing_rps.one().armor == approx(62.666667)
    assert api_fit_stats2.fit.outgoing_rps.one().armor == approx(125.333333)


def test_fit_fit_backref(client, consts):
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_speed_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_ship_id = client.mk_eve_ship(attrs={eve_mass_attr_id: 1000000, eve_speed_attr_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    with api_sol.batch() as api_sol_batch:
        api_fit = api_sol_batch.create_fit()
        api_sol_batch.set_ship(fit_id=api_fit.id, type_id=eve_ship_id)
        api_fit_stats1 = api_sol_batch.get_fit_stats(fit_id=api_fit.id, fit_options=FitStatsOptions(mass=True))
        api_fit_stats2 = api_sol_batch.get_fit_stats(fit_id=api_fit.id, fit_options=FitStatsOptions(speed=True))
    # Verification
    assert api_fit_stats1.fit.mass.one() == 1000000
    with check_no_field():
        api_fit_stats1.fit.speed  # ruff:ignore[useless-expression]
    with check_no_field():
        api_fit_stats2.fit.mass  # ruff:ignore[useless-expression]
    assert api_fit_stats2.fit.speed.one() == 500


def test_fit_fit_backref_error_range(client):
    client.create_sources()
    api_sol = client.create_sol()
    # Action & verification
    with api_sol.batch(status_code=400, json_predicate={
            'code': 'BRF-001',
            'message': 'referenced command #2 does not have results recorded',
            'cmd_index': 1,
    }) as api_sol_batch:
        api_sol_batch.create_fit()
        api_sol_batch.get_fit_stats(fit_id='#2', fit_options=FitStatsOptions(mass=True))


def test_fit_fit_backref_error_kind(client):
    client.create_sources()
    api_sol = client.create_sol()
    # Action & verification
    with api_sol.batch(status_code=400, json_predicate={
            'code': 'BRF-001',
            'message': 'referenced command #0 exists, but does not have fit ID info',
            'cmd_index': 1,
    }) as api_sol_batch:
        api_sol_batch.create_fleet()
        api_sol_batch.get_fit_stats(fit_id='#0', fit_options=FitStatsOptions(mass=True))


def test_fit_fit_error_not_found(client):
    client.create_sources()
    api_sol = client.create_sol()
    api_other_fit = api_sol.create_fit()
    api_other_fit.remove()
    # Action & verification
    with api_sol.batch(status_code=400, json_predicate={
            'code': 'FIT-001',
            'message': f'fit {api_other_fit.id} not found',
            'cmd_index': 1,
    }) as api_sol_batch:
        api_sol_batch.create_fit()
        api_sol_batch.get_fit_stats(fit_id=api_other_fit.id, fit_options=FitStatsOptions(mass=True))


def test_fit_item_override(client, consts):
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_speed_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_drone_id = client.mk_eve_drone(attrs={eve_mass_attr_id: 1000000, eve_speed_attr_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item1 = api_fit.add_drone(type_id=eve_drone_id)
    api_item2 = api_fit.add_drone(type_id=eve_drone_id)
    with api_sol.batch() as api_sol_batch:
        api_fit_stats = api_sol_batch.get_fit_stats(
            fit_id=api_fit.id,
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
    with api_sol.batch() as api_sol_batch:
        api_fit = api_sol_batch.create_fit()
        api_item1 = api_sol_batch.add_drone(fit_id=api_fit.id, type_id=eve_drone_id)
        api_item2 = api_sol_batch.add_drone(fit_id=api_fit.id, type_id=eve_drone_id)
        api_fit_stats = api_sol_batch.get_fit_stats(
            fit_id=api_fit.id,
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


def test_item_item(client, consts):
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_speed_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_ship_id = client.mk_eve_ship(attrs={eve_mass_attr_id: 1000000, eve_speed_attr_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.set_ship(type_id=eve_ship_id)
    with api_sol.batch() as api_sol_batch:
        api_item_stats1 = api_sol_batch.get_item_stats(item_id=api_item.id, item_options=ItemStatsOptions(mass=True))
        api_item_stats2 = api_sol_batch.get_item_stats(item_id=api_item.id, item_options=ItemStatsOptions(speed=True))
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
    with api_sol.batch() as api_sol_batch:
        api_item_stats1 = api_sol_batch.get_item_stats(
            item_id=api_module.id,
            item_options=ItemStatsOptions(outgoing_rps=True))
        api_sol_batch.change_module(item_id=api_module.id, type_id=eve_module2_id)
        api_item_stats2 = api_sol_batch.get_item_stats(
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
    with api_sol.batch() as api_sol_batch:
        api_fit = api_sol_batch.create_fit()
        api_item = api_sol_batch.set_ship(fit_id=api_fit.id, type_id=eve_ship_id)
        api_item_stats1 = api_sol_batch.get_item_stats(item_id=api_item.id, item_options=ItemStatsOptions(mass=True))
        api_item_stats2 = api_sol_batch.get_item_stats(item_id=api_item.id, item_options=ItemStatsOptions(speed=True))
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
    with api_sol.batch(status_code=400, json_predicate={
            'code': 'BRF-001',
            'message': 'referenced command #2 does not have results recorded',
            'cmd_index': 1,
    }) as api_sol_batch:
        api_sol_batch.set_ship(fit_id=api_fit.id, type_id=eve_ship_id)
        api_sol_batch.get_item_stats(item_id='#2', item_options=ItemStatsOptions(mass=True))


def test_item_item_backref_error_kind(client):
    client.create_sources()
    api_sol = client.create_sol()
    # Action & verification
    with api_sol.batch(status_code=400, json_predicate={
            'code': 'BRF-001',
            'message': 'referenced command #0 exists, but does not have item ID info',
            'cmd_index': 1,
    }) as api_sol_batch:
        api_sol_batch.create_fit()
        api_sol_batch.get_item_stats(item_id='#0', item_options=ItemStatsOptions(mass=True))
