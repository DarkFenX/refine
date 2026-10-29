from fw.api import ValOptions


def test_known_failures(client, consts):
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_active_drones)
    eve_drone_id = client.mk_eve_drone()
    eve_char_id = client.mk_eve_item(attrs={eve_max_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_character(type_id=eve_char_id)
    api_drone1 = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.in_space)
    api_drone2 = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.in_space)
    # Verification - drones are tried in bay
    api_fit.check_try_fit_items(
        val_options=ValOptions(launched_drone_count=[api_drone1.id]),
        passing=[],
        failing=[eve_drone_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(launched_drone_count=[api_drone2.id]),
        passing=[],
        failing=[eve_drone_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(launched_drone_count=[api_drone1.id, api_drone2.id]),
        passing=[eve_drone_id],
        failing=[])
