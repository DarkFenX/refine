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


def test_booster_add(client):
    eve_booster_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    with api_sol.commands() as api_sol_cmds:
        api_fit = api_sol_cmds.create_fit()
        api_booster = api_sol_cmds.add_booster(fit_id=api_fit.id, type_id=eve_booster_id)
    # Verification
    assert api_booster.update().type_id == eve_booster_id


def test_booster_change(client):
    eve_booster_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    with api_sol.commands() as api_sol_cmds:
        api_booster = api_sol_cmds.add_booster(fit_id=api_fit.id, type_id=eve_booster_id, state=True)
        api_sol_cmds.change_booster(item_id=api_booster.id, state=False)
    # Verification
    assert api_booster.update().state is False


def test_character_set(client):
    eve_character_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    with api_sol.commands() as api_sol_cmds:
        api_fit = api_sol_cmds.create_fit()
        api_character = api_sol_cmds.set_character(fit_id=api_fit.id, type_id=eve_character_id)
    # Verification
    assert api_character.update().type_id == eve_character_id


def test_character_change_via_fit_id(client):
    eve_character_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    with api_sol.commands() as api_sol_cmds:
        api_fit = api_sol_cmds.create_fit()
        api_character = api_sol_cmds.set_character(fit_id=api_fit.id, type_id=eve_character_id, state=True)
        api_sol_cmds.change_character_via_fit_id(fit_id=api_fit.id, state=False)
    # Verification
    assert api_character.update().state is False


def test_character_change_via_item_id(client):
    eve_character_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    with api_sol.commands() as api_sol_cmds:
        api_character = api_sol_cmds.set_character(fit_id=api_fit.id, type_id=eve_character_id, state=True)
        api_sol_cmds.change_character_via_item_id(item_id=api_character.id, state=False)
    # Verification
    assert api_character.update().state is False


def test_character_unset(client):
    eve_character_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    with api_sol.commands() as api_sol_cmds:
        api_fit = api_sol_cmds.create_fit()
        api_sol_cmds.set_character(fit_id=api_fit.id, type_id=eve_character_id)
        api_sol_cmds.unset_character(fit_id=api_fit.id)
    # Verification
    api_fit.update()
    with check_no_field():
        api_fit.character  # noqa: B018


def test_charge_change_after_module_add(client):
    eve_module_id = client.mk_eve_item()
    eve_charge_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    with api_sol.commands() as api_sol_cmds:
        api_module = api_sol_cmds.add_module(fit_id=api_fit.id, type_id=eve_module_id, charge_type_id=eve_charge_id)
        api_sol_cmds.change_charge(item_id=api_module.charge.id, state=False)
    # Verification
    assert api_module.update().charge.state is False


def test_charge_change_after_module_change(client):
    eve_module_id = client.mk_eve_item()
    eve_charge_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id)
    with api_sol.commands() as api_sol_cmds:
        api_charge = api_sol_cmds.change_module(item_id=api_module.id, charge_type_id=eve_charge_id)
        api_sol_cmds.change_charge(item_id=api_charge.id, state=False)
    # Verification
    assert api_charge.update().state is False


def test_drone_add(client):
    eve_drone_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_tgt_fit = api_sol.create_fit()
    with api_sol.commands() as api_sol_cmds:
        api_tgt_ship = api_sol_cmds.set_ship(fit_id=api_tgt_fit.id, type_id=eve_ship_id)
        api_src_fit = api_sol_cmds.create_fit()
        api_src_drone = api_sol_cmds.add_drone(
            fit_id=api_src_fit.id,
            type_id=eve_drone_id,
            proj_item_ids=[api_tgt_ship.id])
    # Verification
    assert api_tgt_ship.id in api_src_drone.update().projs


def test_drone_change(client):
    eve_drone_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_tgt_fit = api_sol.create_fit()
    with api_sol.commands() as api_sol_cmds:
        api_tgt_ship = api_sol_cmds.set_ship(fit_id=api_tgt_fit.id, type_id=eve_ship_id)
        api_src_drone = api_sol_cmds.add_drone(fit_id=api_src_fit.id, type_id=eve_drone_id)
        api_sol_cmds.change_drone(item_id=api_src_drone.id, add_proj_item_ids=[api_tgt_ship.id])
    # Verification
    assert api_tgt_ship.id in api_src_drone.update().projs


def test_fighter_add(client):
    eve_fighter_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_tgt_fit = api_sol.create_fit()
    with api_sol.commands() as api_sol_cmds:
        api_tgt_ship = api_sol_cmds.set_ship(fit_id=api_tgt_fit.id, type_id=eve_ship_id)
        api_src_fit = api_sol_cmds.create_fit()
        api_src_fighter = api_sol_cmds.add_fighter(
            fit_id=api_src_fit.id,
            type_id=eve_fighter_id,
            proj_item_ids=[api_tgt_ship.id])
    # Verification
    assert api_tgt_ship.id in api_src_fighter.update().projs


def test_fighter_change(client):
    eve_fighter_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_tgt_fit = api_sol.create_fit()
    with api_sol.commands() as api_sol_cmds:
        api_tgt_ship = api_sol_cmds.set_ship(fit_id=api_tgt_fit.id, type_id=eve_ship_id)
        api_src_fighter = api_sol_cmds.add_fighter(fit_id=api_src_fit.id, type_id=eve_fighter_id)
        api_sol_cmds.change_fighter(item_id=api_src_fighter.id, add_proj_item_ids=[api_tgt_ship.id])
    # Verification
    assert api_tgt_ship.id in api_src_fighter.update().projs


def test_fw_effect_add(client):
    eve_fw_effect_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    with api_sol.commands() as api_sol_cmds:
        api_fit = api_sol_cmds.create_fit()
        api_fw_effect = api_sol_cmds.add_fw_effect(fit_id=api_fit.id, type_id=eve_fw_effect_id)
    # Verification
    assert api_fw_effect.update().type_id == eve_fw_effect_id


def test_fw_effect_change(client):
    eve_fw_effect_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    with api_sol.commands() as api_sol_cmds:
        api_fw_effect = api_sol_cmds.add_fw_effect(fit_id=api_fit.id, type_id=eve_fw_effect_id, state=True)
        api_sol_cmds.change_fw_effect(item_id=api_fw_effect.id, state=False)
    # Verification
    assert api_fw_effect.update().state is False


def test_implant_add(client):
    eve_implant_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    with api_sol.commands() as api_sol_cmds:
        api_fit = api_sol_cmds.create_fit()
        api_implant = api_sol_cmds.add_implant(fit_id=api_fit.id, type_id=eve_implant_id)
    # Verification
    assert api_implant.update().type_id == eve_implant_id


def test_implant_change(client):
    eve_implant_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    with api_sol.commands() as api_sol_cmds:
        api_implant = api_sol_cmds.add_implant(fit_id=api_fit.id, type_id=eve_implant_id, state=True)
        api_sol_cmds.change_implant(item_id=api_implant.id, state=False)
    # Verification
    assert api_implant.update().state is False


def test_module_add(client):
    eve_module_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_tgt_fit = api_sol.create_fit()
    with api_sol.commands() as api_sol_cmds:
        api_tgt_ship = api_sol_cmds.set_ship(fit_id=api_tgt_fit.id, type_id=eve_ship_id)
        api_src_fit = api_sol_cmds.create_fit()
        api_src_module = api_sol_cmds.add_module(
            fit_id=api_src_fit.id,
            type_id=eve_module_id,
            proj_item_ids=[api_tgt_ship.id])
    # Verification
    assert api_tgt_ship.id in api_src_module.update().projs


def test_module_change(client):
    eve_module_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_tgt_fit = api_sol.create_fit()
    with api_sol.commands() as api_sol_cmds:
        api_tgt_ship = api_sol_cmds.set_ship(fit_id=api_tgt_fit.id, type_id=eve_ship_id)
        api_src_module = api_sol_cmds.add_module(fit_id=api_src_fit.id, type_id=eve_module_id)
        api_sol_cmds.change_module(item_id=api_src_module.id, add_proj_item_ids=[api_tgt_ship.id])
    # Verification
    assert api_tgt_ship.id in api_src_module.update().projs


def test_proj_effect_add(client):
    eve_proj_effect_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    with api_sol.commands() as api_sol_cmds:
        api_tgt_ship = api_sol_cmds.set_ship(fit_id=api_fit.id, type_id=eve_ship_id)
        api_src_proj_effect = api_sol_cmds.add_proj_effect(type_id=eve_proj_effect_id, proj_item_ids=[api_tgt_ship.id])
    # Verification
    assert api_tgt_ship.id in api_src_proj_effect.update().proj_item_ids


def test_proj_effect_change(client):
    eve_proj_effect_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    with api_sol.commands() as api_sol_cmds:
        api_tgt_ship = api_sol_cmds.set_ship(fit_id=api_fit.id, type_id=eve_ship_id)
        api_src_proj_effect = api_sol_cmds.add_proj_effect(type_id=eve_proj_effect_id)
        api_sol_cmds.change_proj_effect(item_id=api_src_proj_effect.id, add_proj_item_ids=[api_tgt_ship.id])
    # Verification
    assert api_tgt_ship.id in api_src_proj_effect.update().proj_item_ids


def test_rig_add(client):
    eve_rig_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    with api_sol.commands() as api_sol_cmds:
        api_fit = api_sol_cmds.create_fit()
        api_rig = api_sol_cmds.add_rig(fit_id=api_fit.id, type_id=eve_rig_id)
    # Verification
    assert api_rig.update().type_id == eve_rig_id


def test_rig_change(client):
    eve_rig_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    with api_sol.commands() as api_sol_cmds:
        api_rig = api_sol_cmds.add_rig(fit_id=api_fit.id, type_id=eve_rig_id, state=True)
        api_sol_cmds.change_rig(item_id=api_rig.id, state=False)
    # Verification
    assert api_rig.update().state is False


def test_service_add(client):
    eve_service_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    with api_sol.commands() as api_sol_cmds:
        api_fit = api_sol_cmds.create_fit()
        api_service = api_sol_cmds.add_service(fit_id=api_fit.id, type_id=eve_service_id)
    # Verification
    assert api_service.update().type_id == eve_service_id


def test_service_change(client, consts):
    eve_service_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    with api_sol.commands() as api_sol_cmds:
        api_service = api_sol_cmds.add_service(
            fit_id=api_fit.id,
            type_id=eve_service_id,
            state=consts.ApiServiceState.online)
        api_sol_cmds.change_service(item_id=api_service.id, state=consts.ApiServiceState.disabled)
    # Verification
    assert api_service.update().state == consts.ApiServiceState.disabled


def test_ship_set(client):
    eve_ship_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    with api_sol.commands() as api_sol_cmds:
        api_fit = api_sol_cmds.create_fit()
        api_ship = api_sol_cmds.set_ship(fit_id=api_fit.id, type_id=eve_ship_id)
    # Verification
    assert api_ship.update().type_id == eve_ship_id


def test_ship_change_via_fit_id(client):
    eve_ship_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    with api_sol.commands() as api_sol_cmds:
        api_fit = api_sol_cmds.create_fit()
        api_ship = api_sol_cmds.set_ship(fit_id=api_fit.id, type_id=eve_ship_id, state=True)
        api_sol_cmds.change_ship_via_fit_id(fit_id=api_fit.id, state=False)
    # Verification
    assert api_ship.update().state is False


def test_ship_change_via_item_id(client):
    eve_ship_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    with api_sol.commands() as api_sol_cmds:
        api_ship = api_sol_cmds.set_ship(fit_id=api_fit.id, type_id=eve_ship_id, state=True)
        api_sol_cmds.change_ship_via_item_id(item_id=api_ship.id, state=False)
    # Verification
    assert api_ship.update().state is False


def test_ship_unset(client):
    eve_ship_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    with api_sol.commands() as api_sol_cmds:
        api_fit = api_sol_cmds.create_fit()
        api_sol_cmds.set_ship(fit_id=api_fit.id, type_id=eve_ship_id)
        api_sol_cmds.unset_ship(fit_id=api_fit.id)
    # Verification
    api_fit.update()
    with check_no_field():
        api_fit.ship  # noqa: B018


def test_skill_add(client):
    eve_skill_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    with api_sol.commands() as api_sol_cmds:
        api_fit = api_sol_cmds.create_fit()
        api_skill = api_sol_cmds.add_skill(fit_id=api_fit.id, type_id=eve_skill_id, level=3)
    # Verification
    api_skill.update()
    assert api_skill.type_id == eve_skill_id
    assert api_skill.level == 3


def test_skill_change(client):
    eve_skill_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    with api_sol.commands() as api_sol_cmds:
        api_skill = api_sol_cmds.add_skill(fit_id=api_fit.id, type_id=eve_skill_id, level=1)
        api_sol_cmds.change_skill(item_id=api_skill.id, level=5)
    # Verification
    assert api_skill.update().level == 5


def test_stance_set(client):
    eve_stance_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    with api_sol.commands() as api_sol_cmds:
        api_fit = api_sol_cmds.create_fit()
        api_stance = api_sol_cmds.set_stance(fit_id=api_fit.id, type_id=eve_stance_id)
    # Verification
    assert api_stance.update().type_id == eve_stance_id


def test_stance_change_via_fit_id(client):
    eve_stance_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    with api_sol.commands() as api_sol_cmds:
        api_fit = api_sol_cmds.create_fit()
        api_stance = api_sol_cmds.set_stance(fit_id=api_fit.id, type_id=eve_stance_id, state=True)
        api_sol_cmds.change_stance_via_fit_id(fit_id=api_fit.id, state=False)
    # Verification
    assert api_stance.update().state is False


def test_stance_change_via_item_id(client):
    eve_stance_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    with api_sol.commands() as api_sol_cmds:
        api_stance = api_sol_cmds.set_stance(fit_id=api_fit.id, type_id=eve_stance_id, state=True)
        api_sol_cmds.change_stance_via_item_id(item_id=api_stance.id, state=False)
    # Verification
    assert api_stance.update().state is False


def test_stance_unset(client):
    eve_stance_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    with api_sol.commands() as api_sol_cmds:
        api_fit = api_sol_cmds.create_fit()
        api_sol_cmds.set_stance(fit_id=api_fit.id, type_id=eve_stance_id)
        api_sol_cmds.unset_stance(fit_id=api_fit.id)
    # Verification
    api_fit.update()
    with check_no_field():
        api_fit.stance  # noqa: B018


def test_subsystem_add(client):
    eve_subsystem_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    with api_sol.commands() as api_sol_cmds:
        api_fit = api_sol_cmds.create_fit()
        api_subsystem = api_sol_cmds.add_subsystem(fit_id=api_fit.id, type_id=eve_subsystem_id)
    # Verification
    assert api_subsystem.update().type_id == eve_subsystem_id


def test_subsystem_change(client):
    eve_subsystem_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    with api_sol.commands() as api_sol_cmds:
        api_subsystem = api_sol_cmds.add_subsystem(fit_id=api_fit.id, type_id=eve_subsystem_id, state=True)
        api_sol_cmds.change_subsystem(item_id=api_subsystem.id, state=False)
    # Verification
    assert api_subsystem.update().state is False


def test_sw_effect_change(client):
    eve_sw_effect_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    with api_sol.commands() as api_sol_cmds:
        api_sw_effect = api_sol_cmds.add_sw_effect(type_id=eve_sw_effect_id, state=True)
        api_sol_cmds.change_sw_effect(item_id=api_sw_effect.id, state=False)
    # Verification
    assert api_sw_effect.update().state is False
