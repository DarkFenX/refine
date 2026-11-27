from fw import approx
from fw.api import FitStatsOptions


def test_state(client, consts):
    # Launched fighters take no in-bay volume, but still count towards volume of all fighters
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_capacity)
    eve_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_fighter1_id = client.mk_eve_item(attrs={eve_use_attr_id: 1000, eve_count_attr_id: 9})
    eve_fighter2_id = client.mk_eve_item(attrs={eve_use_attr_id: 800, eve_count_attr_id: 12})
    eve_fighter3_id = client.mk_eve_item(attrs={eve_use_attr_id: 1800, eve_count_attr_id: 3})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 15000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fighter1 = api_fit.add_fighter(type_id=eve_fighter1_id, state=consts.ApiMinionState.in_bay)
    api_fighter2 = api_fit.add_fighter(type_id=eve_fighter2_id, state=consts.ApiMinionState.in_space)
    api_fighter3 = api_fit.add_fighter(type_id=eve_fighter3_id, state=consts.ApiMinionState.engaging)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(fighter_bay=True))
    assert api_stats.fighter_bay.one() == (approx(9000), approx(24000), approx(15000))
    # Action
    api_fighter1.change_fighter(state=consts.ApiMinionState.in_space)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(fighter_bay=True))
    assert api_stats.fighter_bay.one() == (approx(0), approx(24000), approx(15000))
    # Action
    api_fighter2.change_fighter(state=consts.ApiMinionState.in_bay)
    api_fighter3.change_fighter(state=consts.ApiMinionState.in_bay)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(fighter_bay=True))
    assert api_stats.fighter_bay.one() == (approx(15000), approx(24000), approx(15000))
    # Action
    api_fighter3.change_fighter(state=consts.ApiMinionState.engaging)
    api_fighter2.remove()
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(fighter_bay=True))
    assert api_stats.fighter_bay.one() == (approx(0), approx(14400), approx(15000))
