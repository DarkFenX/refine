from fw import check_no_field


def test_item_remove(client):
    eve_module_id = client.mk_eve_item()
    eve_charge_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    with api_fit.commands() as api_fit_cmds:
        api_module = api_fit_cmds.add_module(type_id=eve_module_id, charge_type_id=eve_charge_id)
        api_fit_cmds.remove_item(item_id=api_module.charge.id)
    # Verification
    api_module.update()
    with check_no_field():
        api_module.charge  # noqa: B018


def test_drone_change(client):
    eve_drone_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_src_fit = api_sol.create_fit()
    with api_src_fit.commands() as api_src_fit_cmds:
        api_src_drone = api_src_fit_cmds.add_drone(type_id=eve_drone_id)
        api_src_fit_cmds.change_drone(item_id=api_src_drone.id, add_proj_item_ids=[api_tgt_ship.id])
    # Verification
    assert api_src_drone.update().projs[api_tgt_ship.id] == [0, 0]
