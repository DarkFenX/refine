

def test_error_params_malformed(client):
    client.create_sources()
    client.create_sol(
        sol_info_mode='random',
        status_code=400,
        json_predicate={'code': 'PRM-001', 'message': 're:.+'})
    client.create_sol(
        fleet_info_mode='random',
        status_code=400,
        json_predicate={'code': 'PRM-001', 'message': 're:.+'})
    client.create_sol(
        fit_info_mode='random',
        status_code=400,
        json_predicate={'code': 'PRM-001', 'message': 're:.+'})
    client.create_sol(
        item_info_mode='random',
        status_code=400,
        json_predicate={'code': 'PRM-001', 'message': 're:.+'})


def test_default_incoming_dps(client):
    client.create_sources()
    api_sol1 = client.create_sol(default_incoming_dps=(2, 3, 1.5, 0.5))
    assert api_sol1.update().default_incoming_dps == (2, 3, 1.5, 0.5)
    api_sol2 = client.create_sol(default_incoming_dps=(2, 3, 1.5, 0.5, (0.01, 0.075)))
    assert api_sol2.update().default_incoming_dps == (2, 3, 1.5, 0.5, (0.01, 0.075))
