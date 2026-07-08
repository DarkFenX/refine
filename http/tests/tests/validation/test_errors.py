from fw.api import ValOptions


def test_sol_param_malformed(client, consts):
    client.create_sources()
    api_sol = client.create_sol()
    # Verification
    api_sol.validate_direct(
        options=ValOptions(cpu=True),
        val_info_mode='random',
        status_code=400,
        json_predicate={'code': 'PRM-001', 'message': 're:.+'})


def test_fit_param_malformed(client, consts):
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Verification
    api_fit.validate_direct(
        options=ValOptions(cpu=True),
        val_info_mode='random',
        status_code=400,
        json_predicate={'code': 'PRM-001', 'message': 're:.+'})
