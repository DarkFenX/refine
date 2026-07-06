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


def test_booster_change(client):
    eve_booster_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    with api_fit.commands() as api_fit_cmds:
        api_booster = api_fit_cmds.add_booster(type_id=eve_booster_id, state=True)
        api_fit_cmds.change_booster(item_id=api_booster.id, state=False)
    # Verification
    assert api_booster.update().enabled is False


def test_charge_change_after_module_add(client):
    eve_module_id = client.mk_eve_item()
    eve_charge_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    with api_fit.commands() as api_fit_cmds:
        api_module = api_fit_cmds.add_module(type_id=eve_module_id, charge_type_id=eve_charge_id)
        api_fit_cmds.change_charge(item_id=api_module.charge.id, state=False)
    # Verification
    assert api_module.update().charge.enabled is False


def test_charge_change_after_module_change(client):
    eve_module_id = client.mk_eve_item()
    eve_charge_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id)
    with api_fit.commands() as api_fit_cmds:
        api_charge = api_fit_cmds.change_module(item_id=api_module.id, charge_type_id=eve_charge_id)
        api_fit_cmds.change_charge(item_id=api_charge.id, state=False)
    # Verification
    assert api_charge.update().enabled is False


def test_drone_add(client):
    eve_drone_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    with api_fit.commands() as api_fit_cmds:
        api_tgt_drone = api_fit_cmds.add_drone(type_id=eve_drone_id)
        api_src_drone = api_fit_cmds.add_drone(type_id=eve_drone_id, proj_item_ids=[api_tgt_drone.id])
    # Verification
    assert api_tgt_drone.id in api_src_drone.update().projs


def test_drone_change(client):
    eve_drone_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    with api_fit.commands() as api_fit_cmds:
        api_tgt_drone = api_fit_cmds.add_drone(type_id=eve_drone_id)
        api_src_drone = api_fit_cmds.add_drone(type_id=eve_drone_id)
        api_fit_cmds.change_drone(item_id=api_src_drone.id, add_proj_item_ids=[api_tgt_drone.id])
    # Verification
    assert api_tgt_drone.id in api_src_drone.update().projs


def test_fighter_add(client):
    eve_fighter_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    with api_fit.commands() as api_fit_cmds:
        api_tgt_fighter = api_fit_cmds.add_fighter(type_id=eve_fighter_id)
        api_src_fighter = api_fit_cmds.add_fighter(type_id=eve_fighter_id, proj_item_ids=[api_tgt_fighter.id])
    # Verification
    assert api_tgt_fighter.id in api_src_fighter.update().projs


def test_fighter_change(client):
    eve_fighter_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    with api_fit.commands() as api_fit_cmds:
        api_tgt_fighter = api_fit_cmds.add_fighter(type_id=eve_fighter_id)
        api_src_fighter = api_fit_cmds.add_fighter(type_id=eve_fighter_id)
        api_fit_cmds.change_fighter(item_id=api_src_fighter.id, add_proj_item_ids=[api_tgt_fighter.id])
    # Verification
    assert api_tgt_fighter.id in api_src_fighter.update().projs


def test_fw_effect_change(client):
    eve_fw_effect_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    with api_fit.commands() as api_fit_cmds:
        api_fw_effect = api_fit_cmds.add_fw_effect(type_id=eve_fw_effect_id, state=True)
        api_fit_cmds.change_fw_effect(item_id=api_fw_effect.id, state=False)
    # Verification
    assert api_fw_effect.update().enabled is False


def test_implant_change(client):
    eve_implant_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    with api_fit.commands() as api_fit_cmds:
        api_implant = api_fit_cmds.add_implant(type_id=eve_implant_id, state=True)
        api_fit_cmds.change_implant(item_id=api_implant.id, state=False)
    # Verification
    assert api_implant.update().enabled is False


def test_module_add(client):
    eve_module_id = client.mk_eve_item()
    eve_drone_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    with api_fit.commands() as api_fit_cmds:
        api_tgt_drone = api_fit_cmds.add_drone(type_id=eve_drone_id)
        api_src_module = api_fit_cmds.add_module(type_id=eve_module_id, proj_item_ids=[api_tgt_drone.id])
    # Verification
    assert api_tgt_drone.id in api_src_module.update().projs


def test_module_change(client):
    eve_module_id = client.mk_eve_item()
    eve_drone_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    with api_fit.commands() as api_fit_cmds:
        api_tgt_drone = api_fit_cmds.add_drone(type_id=eve_drone_id)
        api_src_module = api_fit_cmds.add_module(type_id=eve_module_id)
        api_fit_cmds.change_module(item_id=api_src_module.id, add_proj_item_ids=[api_tgt_drone.id])
    # Verification
    assert api_tgt_drone.id in api_src_module.update().projs


def test_rig_change(client):
    eve_rig_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    with api_fit.commands() as api_fit_cmds:
        api_rig = api_fit_cmds.add_rig(type_id=eve_rig_id, state=True)
        api_fit_cmds.change_rig(item_id=api_rig.id, state=False)
    # Verification
    assert api_rig.update().enabled is False


def test_service_change(client, consts):
    eve_service_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    with api_fit.commands() as api_fit_cmds:
        api_service = api_fit_cmds.add_service(type_id=eve_service_id, state=consts.ApiServiceState.online)
        api_fit_cmds.change_service(item_id=api_service.id, state=consts.ApiServiceState.disabled)
    # Verification
    assert api_service.update().state == consts.ApiServiceState.disabled


def test_skill_change(client):
    eve_skill_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    with api_fit.commands() as api_fit_cmds:
        api_skill = api_fit_cmds.add_skill(type_id=eve_skill_id, level=2)
        api_fit_cmds.change_skill(item_id=api_skill.id, level=4)
    # Verification
    assert api_skill.update().level == 4


def test_subsystem_change(client):
    eve_subsystem_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    with api_fit.commands() as api_fit_cmds:
        api_subsystem = api_fit_cmds.add_subsystem(type_id=eve_subsystem_id, state=True)
        api_fit_cmds.change_subsystem(item_id=api_subsystem.id, state=False)
    # Verification
    assert api_subsystem.update().enabled is False
