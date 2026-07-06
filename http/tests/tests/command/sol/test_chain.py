

def test_execution(client):
    eve_ship_id = client.mk_eve_item()
    eve_drone_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    with api_sol.commands() as api_sol_cmds:
        api_ship = api_sol_cmds.set_ship(fit_id=api_fit.id, type_id=eve_ship_id)
        api_drone = api_sol_cmds.add_drone(fit_id=api_fit.id, type_id=eve_drone_id)
    # Verification
    assert api_ship.update().type_id == eve_ship_id
    assert api_drone.update().type_id == eve_drone_id


def test_rollback(client):
    eve_drone_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_src_fit = api_sol.create_fit()
    api_src_drone = api_src_fit.add_drone(type_id=eve_drone_id, proj_item_ids=[api_tgt_ship.id])
    with api_sol.commands(
            status_code=400,
            json_predicate={'code': 'ITM-001', 'message': f'command #1 failed: item {api_src_drone.id} not found'},
    ) as api_sol_cmds:
        api_sol_cmds.remove_item(item_id=api_src_drone.id)
        api_sol_cmds.change_drone(item_id=api_src_drone.id, rm_proj_item_ids=[api_tgt_ship.id])
    # Verification - failing 2nd command should've reverted all the prior commands, including drone
    # removal
    assert api_src_drone.update().projs[api_tgt_ship.id] == [0, 0]
