from fw.api import FitStatsOptions, ItemStatsOptions


def test_no_cap_changes(client, consts):
    eve_ship_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_ship_id = client.mk_eve_ship(attrs={eve_ship_amount_attr_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_balance=True))
    assert api_fit_stats.cap_balance.one() == 0
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_balance=True))
    assert api_ship_stats.cap_balance.one() == 0


def test_error_fatality(client, consts):
    eve_ship_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_nosf_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_transfer_amount)
    eve_override_attr_id = client.mk_eve_attr(id_=consts.EveAttr.nos_override)
    eve_use_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_need)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.radius)
    eve_nosf_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.energy_nosf_falloff,
        cat_id=consts.EveEffCat.target,
        discharge_attr_id=eve_use_amount_attr_id,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_nosf_id = client.mk_eve_item(
        attrs={
            eve_nosf_amount_attr_id: 120,
            eve_cycle_time_attr_id: 10000,
            eve_override_attr_id: 0},
        eff_ids=[eve_nosf_effect_id],
        defeff_id=eve_nosf_effect_id)
    eve_ship1_id = client.mk_eve_ship(attrs={eve_ship_amount_attr_id: 500, eve_radius_attr_id: 400})
    eve_ship2_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship1_id)
    api_nosf = api_fit.add_module(type_id=eve_nosf_id, state=consts.ApiModuleState.active)
    # Verification - attempt to get stats of item of incorrect kind fails whole batch
    api_nosf_stats = api_nosf.get_stats(options=ItemStatsOptions(cap_balance=True))
    assert api_nosf_stats.cap_balance is None
    # Action
    api_ship.change_ship(type_id=eve_ship2_id)
    # Verification - attempt to get stats of ship which is not loaded fails whole batch
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_balance=True))
    assert api_fit_stats.cap_balance is None
    api_src_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_balance=True))
    assert api_src_ship_stats.cap_balance is None
    # Action
    api_ship.remove()
    # Verification - attempt to get stats of a fit without ship fails whole batch
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_balance=True))
    assert api_fit_stats.cap_balance is None
