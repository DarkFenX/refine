from fw import check_no_field


def test_fleet_create(client):
    client.create_sources()
    api_sol = client.create_sol()
    with api_sol.commands() as api_sol_cmds:
        api_fit = api_sol_cmds.create_fit()
        api_fleet = api_sol_cmds.create_fleet(fit_ids=[api_fit.id])
    # Verification
    assert api_fleet.update().fit_ids == [api_fit.id]
    assert api_fit.update().fleet_id == api_fleet.id


def test_fleet_change(client):
    client.create_sources()
    api_sol = client.create_sol()
    with api_sol.commands() as api_sol_cmds:
        api_fit1 = api_sol_cmds.create_fit()
        api_fleet = api_sol_cmds.create_fleet(fit_ids=[api_fit1.id])
        api_fit2 = api_sol_cmds.create_fit()
        api_sol_cmds.change_fleet(fleet_id=api_fleet.id, add_fit_ids=[api_fit2.id], rm_fit_ids=[api_fit1.id])
    # Verification
    assert api_fleet.update().fit_ids == [api_fit2.id]
    api_fit1.update()
    with check_no_field():
        api_fit1.fleet_id  # noqa: B018
    assert api_fit2.update().fleet_id == api_fleet.id


def test_fleet_remove(client):
    client.create_sources()
    api_sol = client.create_sol()
    with api_sol.commands() as api_sol_cmds:
        api_fleet = api_sol_cmds.create_fleet()
        api_fit = api_sol_cmds.create_fit(fleet_id=api_fleet.id)
        api_sol_cmds.remove_fleet(fleet_id=api_fleet.id)
    # Verification
    api_fleet.update(status_code=404)
    api_fit.update()
    with check_no_field():
        api_fit.fleet_id  # noqa: B018


def test_fit_create(client):
    client.create_sources()
    api_sol = client.create_sol()
    with api_sol.commands() as api_sol_cmds:
        api_fleet = api_sol_cmds.create_fleet()
        api_fit = api_sol_cmds.create_fit(fleet_id=api_fleet.id)
    # Verification
    assert api_fit.update().fleet_id == api_fleet.id
    assert api_fleet.update().fit_ids == [api_fit.id]


def test_fit_change(client):
    client.create_sources()
    api_sol = client.create_sol()
    with api_sol.commands() as api_sol_cmds:
        api_fleet1 = api_sol_cmds.create_fleet()
        api_fit = api_sol_cmds.create_fit(fleet_id=api_fleet1.id)
        api_fleet2 = api_sol_cmds.create_fleet()
        api_sol_cmds.change_fit(fit_id=api_fit.id, fleet_id=api_fleet2.id)
    # Verification
    assert api_fit.update().fleet_id == api_fleet2.id
    assert api_fleet1.update()
    with check_no_field():
        api_fleet1.fit_ids  # noqa: B018
    assert api_fleet2.update().fit_ids == [api_fit.id]


def test_fit_remove(client):
    client.create_sources()
    api_sol = client.create_sol()
    with api_sol.commands() as api_sol_cmds:
        api_fit = api_sol_cmds.create_fit()
        api_fleet = api_sol_cmds.create_fleet(fit_ids=[api_fit.id])
        api_sol_cmds.remove_fit(fit_id=api_fit.id)
    # Verification
    api_fit.update(status_code=404)
    api_fleet.update()
    with check_no_field():
        api_fleet.fit_ids  # noqa: B018


def test_item_remove(client):
    eve_module_id = client.mk_eve_item()
    eve_charge_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    with api_sol.commands() as api_sol_cmds:
        api_module = api_sol_cmds.add_module(fit_id=api_fit.id, type_id=eve_module_id, charge_type_id=eve_charge_id)
        api_sol_cmds.remove_item(item_id=api_module.charge.id)
    # Verification
    api_module.update()
    with check_no_field():
        api_module.charge  # noqa: B018
