from fw import check_no_field


def test_item_booster(client):
    eve_booster_id = client.mk_eve_booster()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Action
    api_booster = api_fit.autodetect_add_item(type_id=eve_booster_id)
    # Verification
    assert api_fit.update().boosters.keys() == {api_booster.id}
    api_booster.update()
    assert api_booster.type_id == eve_booster_id
    assert api_booster.state is True


def test_item_charge_error(client):
    eve_charge_id = client.mk_eve_charge()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Action & verification
    api_fit.autodetect_add_item(
        type_id=eve_charge_id,
        status_code=400,
        json_predicate={
            'code': 'ITM-006',
            'message': 'failed to add autodetected item: item of kind "charge" cannot be auto-added to the fit'})
    assert api_fit.update().get_raw().keys() == {'id', 'sec_status'}


def test_item_character_error(client, consts):
    eve_character_id = client.mk_eve_item(grp_id=consts.EveItemGrp.character)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Action & verification
    api_fit.autodetect_add_item(
        type_id=eve_character_id,
        status_code=400,
        json_predicate={
            'code': 'ITM-006',
            'message': 'failed to add autodetected item: item of kind "character" cannot be auto-added to the fit'})
    assert api_fit.update().get_raw().keys() == {'id', 'sec_status'}


def test_item_drone(client, consts):
    eve_drone_id = client.mk_eve_drone()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Action
    api_drone = api_fit.autodetect_add_item(type_id=eve_drone_id)
    # Verification
    assert api_fit.update().drones.keys() == {api_drone.id}
    api_drone.update()
    assert api_drone.type_id == eve_drone_id
    assert api_drone.state == consts.ApiMinionState.engaging


def test_item_fighter(client, consts):
    eve_fighter_id = client.mk_eve_fighter()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Action
    api_fighter = api_fit.autodetect_add_item(type_id=eve_fighter_id)
    # Verification
    assert api_fit.update().fighters.keys() == {api_fighter.id}
    api_fighter.update()
    assert api_fighter.type_id == eve_fighter_id
    assert api_fighter.state == consts.ApiMinionState.engaging


def test_item_implant(client):
    eve_implant_id = client.mk_eve_implant()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Action
    api_implant = api_fit.autodetect_add_item(type_id=eve_implant_id)
    # Verification
    assert api_fit.update().implants.keys() == {api_implant.id}
    api_implant.update()
    assert api_implant.type_id == eve_implant_id
    assert api_implant.state is True


def test_item_module_high(client, consts):
    eve_module_id = client.mk_eve_module(rack=consts.Rack.high)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Action
    api_module = api_fit.autodetect_add_item(type_id=eve_module_id)
    # Verification
    api_fit.update()
    assert api_fit.modules.high.map(lambda m: m.id) == [api_module.id]
    with check_no_field():
        api_fit.modules.mid  # ruff:ignore[useless-expression]
    with check_no_field():
        api_fit.modules.low  # ruff:ignore[useless-expression]
    assert api_module.update().type_id == eve_module_id


def test_item_module_mid(client, consts):
    eve_module_id = client.mk_eve_module(rack=consts.Rack.mid)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Action
    api_module = api_fit.autodetect_add_item(type_id=eve_module_id)
    # Verification
    api_fit.update()
    assert api_fit.modules.mid.map(lambda m: m.id) == [api_module.id]
    with check_no_field():
        api_fit.modules.high  # ruff:ignore[useless-expression]
    with check_no_field():
        api_fit.modules.low  # ruff:ignore[useless-expression]
    assert api_module.update().type_id == eve_module_id


def test_item_module_low(client, consts):
    eve_module_id = client.mk_eve_module(rack=consts.Rack.low)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Action
    api_module = api_fit.autodetect_add_item(type_id=eve_module_id)
    # Verification
    api_fit.update()
    assert api_fit.modules.low.map(lambda m: m.id) == [api_module.id]
    with check_no_field():
        api_fit.modules.high  # ruff:ignore[useless-expression]
    with check_no_field():
        api_fit.modules.mid  # ruff:ignore[useless-expression]
    assert api_module.update().type_id == eve_module_id


def test_item_module_limit(client, consts):
    eve_module_id = client.mk_eve_module(rack=consts.Rack.low)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    with api_fit.batch() as api_fit_batch:
        api_modules = [api_fit_batch.autodetect_add_item(type_id=eve_module_id) for _ in range(1000)]
    # Verification
    assert api_fit.update().modules.low.map(lambda m: m.id) == [m.id for m in api_modules]
    # Action & verification
    api_fit.autodetect_add_item(
        type_id=eve_module_id,
        status_code=400,
        json_predicate={
            'code': 'MOD-006',
            'message': (f"failed to add autodetected item: "
                        f"low rack of fit {api_fit.id} can't have modules past position 999")})
    assert api_fit.update().modules.low.map(lambda m: m.id) == [m.id for m in api_modules]


def test_item_module_state_offline(client, consts):
    eve_slot_effect_id = client.mk_eve_effect(id_=consts.EveEffect.hi_power, cat_id=consts.EveEffCat.passive)
    eve_module_id = client.mk_eve_item(eff_ids=[eve_slot_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Action
    api_module = api_fit.autodetect_add_item(type_id=eve_module_id)
    # Verification
    assert api_module.update().state == consts.ApiModuleState.offline


def test_item_module_state_online(client, consts):
    eve_slot_effect_id = client.mk_eve_effect(id_=consts.EveEffect.hi_power, cat_id=consts.EveEffCat.passive)
    eve_online_effect_id = client.mk_eve_effect(id_=consts.EveEffect.online, cat_id=consts.EveEffCat.online)
    eve_module_id = client.mk_eve_item(eff_ids=[eve_slot_effect_id, eve_online_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Action
    api_module = api_fit.autodetect_add_item(type_id=eve_module_id)
    # Verification
    assert api_module.update().state == consts.ApiModuleState.online


def test_item_module_state_active(client, consts):
    eve_slot_effect_id = client.mk_eve_effect(id_=consts.EveEffect.hi_power, cat_id=consts.EveEffCat.passive)
    eve_online_effect_id = client.mk_eve_effect(id_=consts.EveEffect.online, cat_id=consts.EveEffCat.online)
    eve_active_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_item(
        eff_ids=[eve_slot_effect_id, eve_online_effect_id, eve_active_effect_id],
        defeff_id=eve_active_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Action
    api_module = api_fit.autodetect_add_item(type_id=eve_module_id)
    # Verification
    assert api_module.update().state == consts.ApiModuleState.active


def test_item_module_state_overload(client, consts):
    # Modules which can be overloaded are added active
    eve_slot_effect_id = client.mk_eve_effect(id_=consts.EveEffect.hi_power, cat_id=consts.EveEffCat.passive)
    eve_online_effect_id = client.mk_eve_effect(id_=consts.EveEffect.online, cat_id=consts.EveEffCat.online)
    eve_active_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active)
    eve_overload_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.overload)
    eve_module_id = client.mk_eve_item(
        eff_ids=[eve_slot_effect_id, eve_online_effect_id, eve_active_effect_id, eve_overload_effect_id],
        defeff_id=eve_active_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Action
    api_module = api_fit.autodetect_add_item(type_id=eve_module_id)
    # Verification
    assert api_module.update().state == consts.ApiModuleState.active


def test_item_module_equip(client, consts):
    eve_module_id = client.mk_eve_module(rack=consts.Rack.high)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_module(type_id=eve_module_id, add_mode={consts.ApiModAddMode.replace: 0})
    api_module3 = api_fit.add_module(type_id=eve_module_id, add_mode={consts.ApiModAddMode.replace: 2})
    # Action
    api_module2 = api_fit.autodetect_add_item(type_id=eve_module_id)
    # Verification - module takes the free slot
    assert api_fit.update().modules.high.map(lambda m: m.id) == [api_module1.id, api_module2.id, api_module3.id]
    assert api_module2.update().pos == 1
    # Action
    api_module4 = api_fit.autodetect_add_item(type_id=eve_module_id)
    # Verification - with no free slots, module goes after the last one
    assert api_fit.update().modules.high.map(lambda m: m.id) == [
        api_module1.id, api_module2.id, api_module3.id, api_module4.id]
    assert api_module4.update().pos == 3


def test_item_rig(client):
    eve_rig_id = client.mk_eve_rig()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Action
    api_rig = api_fit.autodetect_add_item(type_id=eve_rig_id)
    # Verification
    assert api_fit.update().rigs.keys() == {api_rig.id}
    api_rig.update()
    assert api_rig.type_id == eve_rig_id
    assert api_rig.state is True


def test_item_service(client, consts):
    eve_service_id = client.mk_eve_service()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Action
    api_service = api_fit.autodetect_add_item(type_id=eve_service_id)
    # Verification
    assert api_fit.update().services.keys() == {api_service.id}
    api_service.update()
    assert api_service.type_id == eve_service_id
    assert api_service.state == consts.ApiServiceState.online


def test_item_ship_error(client):
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Action & verification
    api_fit.autodetect_add_item(
        type_id=eve_ship_id,
        status_code=400,
        json_predicate={
            'code': 'ITM-006',
            'message': 'failed to add autodetected item: item of kind "ship" cannot be auto-added to the fit'})
    assert api_fit.update().get_raw().keys() == {'id', 'sec_status'}


def test_item_skill_error(client, consts):
    eve_skill_id = client.mk_eve_item(cat_id=consts.EveItemCat.skill)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Action & verification
    api_fit.autodetect_add_item(
        type_id=eve_skill_id,
        status_code=400,
        json_predicate={
            'code': 'ITM-006',
            'message': 'failed to add autodetected item: item of kind "skill" cannot be auto-added to the fit'})
    assert api_fit.update().get_raw().keys() == {'id', 'sec_status'}


def test_item_stance_error(client, consts):
    eve_stance_id = client.mk_eve_item(grp_id=consts.EveItemGrp.ship_modifier)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Action & verification
    api_fit.autodetect_add_item(
        type_id=eve_stance_id,
        status_code=400,
        json_predicate={
            'code': 'ITM-006',
            'message': 'failed to add autodetected item: item of kind "stance" cannot be auto-added to the fit'})
    assert api_fit.update().get_raw().keys() == {'id', 'sec_status'}


def test_item_subsystem(client):
    eve_subsystem_id = client.mk_eve_subsystem()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Action
    api_subsystem = api_fit.autodetect_add_item(type_id=eve_subsystem_id)
    # Verification
    assert api_fit.update().subsystems.keys() == {api_subsystem.id}
    api_subsystem.update()
    assert api_subsystem.type_id == eve_subsystem_id
    assert api_subsystem.state is True


def test_batch_fit(client, consts):
    eve_drone_id = client.mk_eve_drone()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Action
    with api_fit.batch() as api_fit_batch:
        api_drone = api_fit_batch.autodetect_add_item(type_id=eve_drone_id)
        api_fit_batch.change_drone(item_id=api_drone.id, state=consts.ApiMinionState.in_bay)
    # Verification
    assert api_fit.update().drones.keys() == {api_drone.id}
    assert api_drone.update().state == consts.ApiMinionState.in_bay


def test_batch_sol(client, consts):
    eve_drone_id = client.mk_eve_drone()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Action
    with api_sol.batch() as api_sol_batch:
        api_drone = api_sol_batch.autodetect_add_item(fit_id=api_fit.id, type_id=eve_drone_id)
        api_sol_batch.change_drone(item_id=api_drone.id, state=consts.ApiMinionState.in_bay)
    # Verification
    assert api_fit.update().drones.keys() == {api_drone.id}
    assert api_drone.update().state == consts.ApiMinionState.in_bay


def test_error_type_not_found(client):
    eve_type_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Action & verification
    api_fit.autodetect_add_item(
        type_id=eve_type_id,
        status_code=400,
        json_predicate={
            'code': 'ITM-004',
            'message': f'failed to add autodetected item: type ID {eve_type_id} not found'})
    assert api_fit.update().get_raw().keys() == {'id', 'sec_status'}


def test_error_kind_undetectable(client):
    eve_item_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Action & verification
    api_fit.autodetect_add_item(
        type_id=eve_item_id,
        status_code=400,
        json_predicate={
            'code': 'ITM-005',
            'message': 'failed to add autodetected item: item kind could not be detected'})
    assert api_fit.update().get_raw().keys() == {'id', 'sec_status'}


def test_error_kind_ambiguous(client, consts):
    eve_high_effect_id = client.mk_eve_effect(id_=consts.EveEffect.hi_power, cat_id=consts.EveEffCat.passive)
    eve_mid_effect_id = client.mk_eve_effect(id_=consts.EveEffect.med_power, cat_id=consts.EveEffCat.passive)
    eve_module_id = client.mk_eve_item(eff_ids=[eve_high_effect_id, eve_mid_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Action & verification
    api_fit.autodetect_add_item(
        type_id=eve_module_id,
        status_code=400,
        json_predicate={
            'code': 'ITM-005',
            'message': 'failed to add autodetected item: item kind could not be detected'})
    assert api_fit.update().get_raw().keys() == {'id', 'sec_status'}
