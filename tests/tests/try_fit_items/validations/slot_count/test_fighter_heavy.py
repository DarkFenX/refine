from fw.api import ValOptions


def test_known_failures(client, consts):
    eve_ftr_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_is_heavy)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_heavy_slots)
    eve_fighter_id = client.mk_eve_fighter(attrs={eve_ftr_type_attr_id: 1})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fighter1 = api_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.in_space)
    api_fighter2 = api_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.in_space)
    # Verification - fighters are tried in bay
    api_fit.check_try_fit_items(
        val_options=ValOptions(launched_heavy_fighter_count=[api_fighter1.id]),
        passing=[],
        failing=[eve_fighter_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(launched_heavy_fighter_count=[api_fighter2.id]),
        passing=[],
        failing=[eve_fighter_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(launched_heavy_fighter_count=[api_fighter1.id, api_fighter2.id]),
        passing=[eve_fighter_id],
        failing=[])
