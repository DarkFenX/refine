from fw import Muta, approx, check_no_field


def test_autocharge_remove(client, consts):
    eve_autocharge_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_launch_bomb_type)
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_autocharge_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.activates_autocharge,
        cat_id=consts.EveEffCat.active)
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.tgt_normal1,
        cat_id=consts.EveEffCat.target,
        mod_info=[eve_mod])
    eve_charge_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: 20},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_fighter_id = client.mk_eve_fighter(
        attrs={eve_autocharge_attr_id: eve_charge_id},
        eff_ids=[eve_autocharge_effect_id],
        defeff_id=eve_autocharge_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 1000})
    client.create_sources()
    api_sol = client.create_sol()
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        proj_item_ids=[api_tgt_ship.id])
    api_autocharge = api_fighter.autocharges[eve_autocharge_effect_id]
    # Action & verification
    api_autocharge.remove(
        status_code=403,
        json_predicate={'code': 'ACH-002', 'message': 'autocharge cannot be manually removed'})
    api_fighter.update()
    assert api_fighter.autocharges[eve_autocharge_effect_id].id == api_autocharge.id
    assert api_autocharge.update().state is True
    assert api_tgt_ship.update().attrs[eve_affectee_attr_id].modified == approx(1200)


def test_kind_mismatch(client, consts):
    eve_drone_id = client.mk_eve_drone()
    eve_module_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.engaging)
    # Action & verification
    api_drone.change_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.offline,
        status_code=400,
        json_predicate={
            'code': 'MOD-001',
            'message': f'failed to change module: item {api_drone.id} was requested as a module, but is a drone'})
    api_drone.update()
    assert api_drone.type_id == eve_drone_id
    assert api_drone.state == consts.ApiMinionState.engaging


def test_module_rolls_not_mutated(client):
    eve_attr_id = client.mk_eve_attr()
    eve_module_id = client.mk_eve_item(attrs={eve_attr_id: 20})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id)
    # Action & verification
    api_module.change_module(
        mutation={eve_attr_id: Muta.roll_to_api(val=0.1)},
        status_code=400,
        json_predicate={
            'code': 'MOD-005',
            'message': f'failed to change module: failed to mutate attributes: item {api_module.id} is not mutated'})
    api_module.update()
    with check_no_field():
        api_module.mutation  # ruff:ignore[useless-expression]
    assert api_module.attrs[eve_attr_id].modified == approx(20)


def test_drone_rolls_not_mutated(client):
    eve_attr_id = client.mk_eve_attr()
    eve_drone_id = client.mk_eve_drone(attrs={eve_attr_id: 20})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_drone = api_fit.add_drone(type_id=eve_drone_id)
    # Action & verification
    api_drone.change_drone(
        mutation={eve_attr_id: Muta.roll_to_api(val=0.1)},
        status_code=400,
        json_predicate={
            'code': 'DRN-005',
            'message': f'failed to change drone: failed to mutate attributes: item {api_drone.id} is not mutated'})
    api_drone.update()
    with check_no_field():
        api_drone.mutation  # ruff:ignore[useless-expression]
    assert api_drone.attrs[eve_attr_id].modified == approx(20)


def test_ship_change_no_ship(client):
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Action & verification
    with api_sol.batch(status_code=400, json_predicate={
            'code': 'SHP-002',
            'message': f'failed to change ship: fit {api_fit.id} has no ship set',
            'cmd_index': 0,
    }) as api_sol_batch:
        api_sol_batch.change_ship_via_fit_id(fit_id=api_fit.id, state=False)
    api_fit.update()
    with check_no_field():
        api_fit.ship  # ruff:ignore[useless-expression]


def test_skill_collision_on_add(client, consts):
    eve_skill_level_attr_id = client.mk_eve_attr(id_=consts.EveAttr.skill_level)
    eve_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_skill_level_attr_id,
        affectee_attr_id=eve_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_skill_id = client.mk_eve_item(attrs={eve_skill_level_attr_id: 0}, eff_ids=[eve_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_skill = api_fit.add_skill(type_id=eve_skill_id, level=3)
    # Action & verification
    api_fit.add_skill(
        type_id=eve_skill_id,
        level=5,
        state=False,
        effect_modes={eve_effect_id: consts.ApiEffMode.force_stop},
        status_code=400,
        json_predicate={
            'code': 'SKL-002',
            'message': (f'failed to add skill: skill {eve_skill_id} already exists on fit {api_fit.id}, '
                        f'item {api_skill.id} has the same type ID')})
    assert api_fit.update().skills.keys() == {api_skill.id}
    api_skill.update()
    assert api_skill.level == 3
    assert api_skill.state is True
    assert api_ship.update().attrs[eve_attr_id].modified == approx(103)


def test_skill_collision_on_change(client, consts):
    eve_skill_level_attr_id = client.mk_eve_attr(id_=consts.EveAttr.skill_level)
    eve_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_skill_level_attr_id,
        affectee_attr_id=eve_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_skill1_id = client.mk_eve_item(attrs={eve_skill_level_attr_id: 0}, eff_ids=[eve_effect_id])
    eve_skill2_id = client.mk_eve_item(attrs={eve_skill_level_attr_id: 0})
    eve_ship_id = client.mk_eve_ship(attrs={eve_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_skill1 = api_fit.add_skill(type_id=eve_skill1_id, level=3)
    api_skill2 = api_fit.add_skill(type_id=eve_skill2_id, level=5)
    # Action & verification - type is changed first, so level and state are not reached
    api_skill1.change_skill(
        type_id=eve_skill2_id,
        level=1,
        state=False,
        status_code=400,
        json_predicate={
            'code': 'SKL-003',
            'message': (f'failed to change skill: skill {eve_skill2_id} already exists on fit {api_fit.id}, '
                        f'item {api_skill2.id} has the same type ID')})
    api_skill1.update()
    assert api_skill1.type_id == eve_skill1_id
    assert api_skill1.level == 3
    assert api_skill1.state is True
    assert api_skill2.update().type_id == eve_skill2_id
    assert api_ship.update().attrs[eve_attr_id].modified == approx(103)
