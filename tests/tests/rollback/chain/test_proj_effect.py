from fw import approx, check_no_field


def test_add_proj_not_found(client, consts):
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
    api_other_tgt = api_sol.add_sw_effect(type_id=eve_ship_id)
    api_other_tgt.remove()
    # Action & verification - projected effect is projected onto the first target before the second
    # projection fails
    api_sol.add_proj_effect(
        type_id=eve_proj_effect_id,
        proj_item_ids=[api_tgt_ship.id, api_other_tgt.id],
        status_code=400,
        json_predicate={
            'code': 'PJE-002',
            'message': f'failed to add projected effect: failed to add projection: item {api_other_tgt.id} not found'})
    api_sol.update()
    with check_no_field():
        api_sol.proj_effects  # ruff:ignore[useless-expression]
    assert api_tgt_ship.update().attrs[eve_affectee_attr_id].modified == approx(100)


def test_change_proj_add_fail(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_proj_effect1_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_proj_effect2_id = client.mk_eve_item(attrs={eve_affector_attr_id: 50}, eff_ids=[eve_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_tgt_fit1 = api_sol.create_fit()
    api_tgt_ship1 = api_tgt_fit1.set_ship(type_id=eve_ship_id)
    api_tgt_fit2 = api_sol.create_fit()
    api_tgt_ship2 = api_tgt_fit2.set_ship(type_id=eve_ship_id)
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect1_id, proj_item_ids=[api_tgt_ship1.id])
    # Verification
    assert api_tgt_ship1.update().attrs[eve_affectee_attr_id].modified == approx(120)
    api_other_tgt = api_sol.add_sw_effect(type_id=eve_ship_id)
    api_other_tgt.remove()
    # Action & verification - every part of the change is applied before the second projection fails
    api_proj_effect.change_proj_effect(
        type_id=eve_proj_effect2_id,
        state=False,
        rm_proj_item_ids=[api_tgt_ship1.id],
        add_proj_item_ids=[api_tgt_ship2.id, api_other_tgt.id],
        effect_modes={eve_effect_id: consts.ApiEffMode.force_stop},
        status_code=400,
        json_predicate={
            'code': 'PJE-003',
            'message': (f'failed to change projected effect: '
                        f'failed to add projection: item {api_other_tgt.id} not found')})
    api_proj_effect.update()
    assert api_proj_effect.type_id == eve_proj_effect1_id
    assert api_proj_effect.state is True
    with check_no_field():
        api_proj_effect.effect_mode_overrides  # ruff:ignore[useless-expression]
    assert api_proj_effect.proj_item_ids == [api_tgt_ship1.id]
    assert api_tgt_ship1.update().attrs[eve_affectee_attr_id].modified == approx(120)
    assert api_tgt_ship2.update().attrs[eve_affectee_attr_id].modified == approx(100)


def test_change_proj_rm_fail(client, consts):
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
    api_other_tgt = api_sol.add_sw_effect(type_id=eve_ship_id)
    api_other_tgt.remove()
    # Action & verification - existing projection is removed before the next projectee is not found
    api_proj_effect.change_proj_effect(
        rm_proj_item_ids=[api_tgt_ship.id, api_other_tgt.id],
        status_code=400,
        json_predicate={
            'code': 'PJE-004',
            'message': (f'failed to change projected effect: '
                        f'failed to remove projection: item {api_other_tgt.id} not found')})
    assert api_proj_effect.update().proj_item_ids == [api_tgt_ship.id]
    assert api_tgt_ship.update().attrs[eve_affectee_attr_id].modified == approx(120)
