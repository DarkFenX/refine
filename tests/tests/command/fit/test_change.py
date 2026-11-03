import typing

from fw import check_no_field

if typing.TYPE_CHECKING:
    from fw.request import Request


def test_error_params_malformed(client):
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit(sec_status=1.5)
    # Action & verification
    api_fit.change(
        sec_status=-5,
        fit_info_mode='random',
        status_code=400,
        json_predicate={'code': 'PRM-001', 'message': 're:.+'})
    api_fit.change(
        sec_status=-5,
        item_info_mode='random',
        status_code=400,
        json_predicate={'code': 'PRM-001', 'message': 're:.+'})
    assert api_fit.update().sec_status == 1.5


def test_execution(client):
    eve_ship_id = client.mk_eve_item()
    eve_drone_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    with api_fit.batch() as api_fit_batch:
        api_ship = api_fit_batch.set_ship(type_id=eve_ship_id)
        api_drone = api_fit_batch.add_drone(type_id=eve_drone_id)
    # Verification
    assert api_ship.update().type_id == eve_ship_id
    assert api_drone.update().type_id == eve_drone_id


def test_rollback_error_execution(client):
    eve_drone_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_src_fit = api_sol.create_fit()
    api_src_drone = api_src_fit.add_drone(type_id=eve_drone_id, proj_item_ids=[api_tgt_ship.id])
    with api_src_fit.batch(status_code=400, json_predicate={
            'code': 'ITM-001',
            'message': f'failed to change drone: item {api_src_drone.id} not found',
            'cmd_index': 1,
    }) as api_src_fit_batch:
        api_src_fit_batch.remove_item(item_id=api_src_drone.id)
        api_src_fit_batch.change_drone(item_id=api_src_drone.id, rm_proj_item_ids=[api_tgt_ship.id])
    # Verification - failing 2nd command should've reverted all the prior commands, including drone
    # removal
    assert api_src_drone.update().projs[api_tgt_ship.id] == [0, 0]


def test_rollback_error_parsing(client):

    def hook_req(req: Request):
        data = req.get_json()
        del data[1]['item_id']
        req.set_json(data=data)

    eve_drone_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_src_fit = api_sol.create_fit()
    api_src_drone = api_src_fit.add_drone(type_id=eve_drone_id, proj_item_ids=[api_tgt_ship.id])
    with api_src_fit.batch(
            hook_req=hook_req,
            status_code=400,
            json_predicate={'code': 'JSN-002', 'message': 're:.+', 'cmd_index': 1},
    ) as api_src_fit_batch:
        api_src_fit_batch.remove_item(item_id=api_src_drone.id)
        api_src_fit_batch.change_drone(item_id=api_src_drone.id, rm_proj_item_ids=[api_tgt_ship.id])
    # Verification - failing 2nd command should've reverted all the prior commands, including drone
    # removal
    assert api_src_drone.update().projs[api_tgt_ship.id] == [0, 0]


def test_error_item_change_other_fit(client):
    eve_booster_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_booster = api_fit2.add_booster(type_id=eve_booster_id, state=True)
    # Action & verification
    with api_fit1.batch(status_code=400, json_predicate={
            'code': 'ITM-003',
            'message': f'failed to change booster: item {api_booster.id} is not on fit {api_fit1.id}',
            'cmd_index': 0,
    }) as api_fit_batch:
        api_fit_batch.change_booster(item_id=api_booster.id, state=False)
    assert api_booster.update().state is True


def test_error_item_remove_other_fit(client):
    eve_booster_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_booster = api_fit2.add_booster(type_id=eve_booster_id)
    # Action & verification
    with api_fit1.batch(status_code=400, json_predicate={
            'code': 'ITM-003',
            'message': f'failed to remove item: item {api_booster.id} is not on fit {api_fit1.id}',
            'cmd_index': 0,
    }) as api_fit_batch:
        api_fit_batch.remove_item(item_id=api_booster.id)
    assert api_fit2.update().boosters.keys() == {api_booster.id}


def test_fleet_same(client):
    client.create_sources()
    api_sol = client.create_sol()
    api_fleet = api_sol.create_fleet()
    api_fit = api_sol.create_fit(fleet_id=api_fleet.id)
    # Action
    api_fit.change(fleet_id=api_fleet.id)
    # Verification
    assert api_fit.update().fleet_id == api_fleet.id
    assert api_fleet.update().fits.keys() == {api_fit.id}


def test_fleet_switch(client):
    client.create_sources()
    api_sol = client.create_sol()
    api_fleet1 = api_sol.create_fleet()
    api_fleet2 = api_sol.create_fleet()
    api_fit = api_sol.create_fit(fleet_id=api_fleet1.id)
    # Action
    api_fit.change(fleet_id=api_fleet2.id)
    # Verification - fit leaves its old fleet
    assert api_fit.update().fleet_id == api_fleet2.id
    assert api_fleet2.update().fits.keys() == {api_fit.id}
    api_fleet1.update()
    with check_no_field():
        api_fleet1.fits  # ruff:ignore[useless-expression]


def test_fleet_unset_absent(client):
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Action
    api_fit.change(fleet_id=None)
    # Verification
    api_fit.update()
    with check_no_field():
        api_fit.fleet_id  # ruff:ignore[useless-expression]


def test_rah_incoming_dps_unset_absent(client):
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Action
    api_fit.change(rah_incoming_dps=None)
    # Verification
    api_fit.update()
    with check_no_field():
        api_fit.rah_incoming_dps  # ruff:ignore[useless-expression]
