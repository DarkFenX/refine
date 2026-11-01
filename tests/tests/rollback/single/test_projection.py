from fw import approx


def test_module_add_not_found(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 100})
    eve_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: 20},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id, coordinates=(0, 1000, 0))
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        proj_item_ids=[api_tgt_ship.id])
    api_other_tgt = api_sol.add_sw_effect(type_id=eve_module_id)
    api_other_tgt.remove()
    # Action & verification
    api_module.change_module(
        add_proj_item_ids=[api_other_tgt.id],
        status_code=400,
        json_predicate={
            'code': 'MOD-003',
            'message': f'failed to change module: failed to add projection: item {api_other_tgt.id} not found'})
    api_module.update()
    assert api_module.projs.keys() == {api_tgt_ship.id}
    assert api_module.projs[api_tgt_ship.id] == (approx(1000), approx(1000))
    assert api_tgt_ship.update().attrs[eve_affectee_attr_id].modified == approx(120)


def test_module_rm_not_found(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 100})
    eve_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: 20},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        proj_item_ids=[api_tgt_ship.id])
    api_other_tgt = api_sol.add_sw_effect(type_id=eve_module_id)
    api_other_tgt.remove()
    # Action & verification
    api_module.change_module(
        rm_proj_item_ids=[api_other_tgt.id],
        status_code=400,
        json_predicate={
            'code': 'MOD-004',
            'message': f'failed to change module: failed to remove projection: item {api_other_tgt.id} not found'})
    assert api_module.update().projs.keys() == {api_tgt_ship.id}
    assert api_tgt_ship.update().attrs[eve_affectee_attr_id].modified == approx(120)


def test_drone_add_unreceptive(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 100})
    eve_drone_id = client.mk_eve_drone(
        attrs={eve_affector_attr_id: 20},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_rig_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_tgt_rig = api_tgt_fit.add_rig(type_id=eve_rig_id)
    api_fit = api_sol.create_fit()
    api_drone = api_fit.add_drone(
        type_id=eve_drone_id,
        state=consts.ApiMinionState.engaging,
        proj_item_ids=[api_tgt_ship.id])
    # Action & verification
    api_drone.change_drone(
        add_proj_item_ids=[api_tgt_rig.id],
        status_code=400,
        json_predicate={
            'code': 'DRN-003',
            'message': (f"failed to change drone: failed to add projection: "
                        f"rig {api_tgt_rig.id} can't receive projections")})
    assert api_drone.update().projs.keys() == {api_tgt_ship.id}
    assert api_tgt_ship.update().attrs[eve_affectee_attr_id].modified == approx(120)


def test_fighter_add_not_found(client):
    eve_ship_id = client.mk_eve_ship()
    eve_fighter_id = client.mk_eve_fighter()
    client.create_sources()
    api_sol = client.create_sol()
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id, coordinates=(0, 1000, 0))
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id, proj_item_ids=[api_tgt_ship.id])
    api_other_tgt = api_sol.add_sw_effect(type_id=eve_fighter_id)
    api_other_tgt.remove()
    # Action & verification
    api_fighter.change_fighter(
        add_proj_item_ids=[api_other_tgt.id],
        status_code=400,
        json_predicate={
            'code': 'FTR-003',
            'message': f'failed to change fighter: failed to add projection: item {api_other_tgt.id} not found'})
    api_fighter.update()
    assert api_fighter.projs.keys() == {api_tgt_ship.id}
    assert api_fighter.projs[api_tgt_ship.id] == (approx(1000), approx(1000))


def test_proj_effect_add_not_found(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_proj_effect_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect_id, proj_item_ids=[api_tgt_ship.id])
    api_other_tgt = api_sol.add_sw_effect(type_id=eve_proj_effect_id)
    api_other_tgt.remove()
    # Action & verification
    api_proj_effect.change_proj_effect(
        add_proj_item_ids=[api_other_tgt.id],
        status_code=400,
        json_predicate={
            'code': 'PJE-003',
            'message': (f'failed to change projected effect: '
                        f'failed to add projection: item {api_other_tgt.id} not found')})
    assert api_proj_effect.update().proj_item_ids == [api_tgt_ship.id]
    assert api_tgt_ship.update().attrs[eve_affectee_attr_id].modified == approx(120)
