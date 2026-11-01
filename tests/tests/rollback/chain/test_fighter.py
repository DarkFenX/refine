from fw import approx, check_no_field


def test_add_proj_not_found(client, consts):
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
    api_other_tgt = api_sol.add_sw_effect(type_id=eve_ship_id)
    api_other_tgt.remove()
    # Action & verification - fighter is fully set up and projected onto the first target before the
    # second projection fails
    api_fit.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        count_override=3,
        rearm_minion_override=consts.ApiRearmMinion.disabled,
        proj_item_ids=[api_tgt_ship.id, api_other_tgt.id],
        coordinates=(1000, 0, 0),
        movement=(90, 0, 1),
        effect_modes={eve_autocharge_effect_id: consts.ApiEffMode.force_run},
        status_code=400,
        json_predicate={
            'code': 'FTR-002',
            'message': f'failed to add fighter: failed to add projection: item {api_other_tgt.id} not found'})
    api_fit.update()
    with check_no_field():
        api_fit.fighters  # ruff:ignore[useless-expression]
    assert api_tgt_ship.update().attrs[eve_affectee_attr_id].modified == approx(1000)


def test_change_type_proj_add_fail(client, consts):
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
    eve_charge1_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: 20},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_charge2_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: 50},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_fighter1_id = client.mk_eve_fighter(
        attrs={eve_autocharge_attr_id: eve_charge1_id},
        eff_ids=[eve_autocharge_effect_id],
        defeff_id=eve_autocharge_effect_id)
    eve_fighter2_id = client.mk_eve_fighter(
        attrs={eve_autocharge_attr_id: eve_charge2_id},
        eff_ids=[eve_autocharge_effect_id],
        defeff_id=eve_autocharge_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 1000})
    client.create_sources()
    api_sol = client.create_sol()
    api_tgt_fit1 = api_sol.create_fit()
    api_tgt_ship1 = api_tgt_fit1.set_ship(type_id=eve_ship_id)
    api_tgt_fit2 = api_sol.create_fit()
    api_tgt_ship2 = api_tgt_fit2.set_ship(type_id=eve_ship_id)
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(
        type_id=eve_fighter1_id,
        state=consts.ApiMinionState.engaging,
        count_override=3,
        proj_item_ids=[api_tgt_ship1.id],
        coordinates=(0, 0, 0))
    api_autocharge_id = api_fighter.autocharges[eve_autocharge_effect_id].id
    # Verification
    assert api_tgt_ship1.update().attrs[eve_affectee_attr_id].modified == approx(1200)
    api_other_tgt = api_sol.add_sw_effect(type_id=eve_ship_id)
    api_other_tgt.remove()
    # Action & verification - type change replaces the autocharge, and everything else is applied
    # before the second projection fails
    api_fighter.change_fighter(
        type_id=eve_fighter2_id,
        state=consts.ApiMinionState.in_space,
        count_override=None,
        rearm_minion_override=consts.ApiRearmMinion.disabled,
        rm_proj_item_ids=[api_tgt_ship1.id],
        add_proj_item_ids=[api_tgt_ship2.id, api_other_tgt.id],
        coordinates=(0, 5000, 0),
        status_code=400,
        json_predicate={
            'code': 'FTR-003',
            'message': f'failed to change fighter: failed to add projection: item {api_other_tgt.id} not found'})
    api_fighter.update()
    assert api_fighter.type_id == eve_fighter1_id
    assert api_fighter.state == consts.ApiMinionState.engaging
    assert api_fighter.count_override == 3
    with check_no_field():
        api_fighter.rearm_minion_override  # ruff:ignore[useless-expression]
    assert api_fighter.coordinates == (0, 0, 0)
    assert api_fighter.projs.keys() == {api_tgt_ship1.id}
    assert len(api_fighter.autocharges) == 1
    api_autocharge = api_fighter.autocharges[eve_autocharge_effect_id]
    assert api_autocharge.id == api_autocharge_id
    assert api_autocharge.update().type_id == eve_charge1_id
    assert api_autocharge.attrs[eve_affector_attr_id].modified == approx(20)
    assert api_tgt_ship1.update().attrs[eve_affectee_attr_id].modified == approx(1200)
    assert api_tgt_ship2.update().attrs[eve_affectee_attr_id].modified == approx(1000)


def test_change_proj_rm_fail(client):
    eve_ship_id = client.mk_eve_ship()
    eve_fighter_id = client.mk_eve_fighter()
    client.create_sources()
    api_sol = client.create_sol()
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id, proj_item_ids=[api_tgt_ship.id])
    api_other_tgt = api_sol.add_sw_effect(type_id=eve_ship_id)
    api_other_tgt.remove()
    # Action & verification - existing projection is removed before the next projectee is not found
    api_fighter.change_fighter(
        rm_proj_item_ids=[api_tgt_ship.id, api_other_tgt.id],
        status_code=400,
        json_predicate={
            'code': 'FTR-004',
            'message': f'failed to change fighter: failed to remove projection: item {api_other_tgt.id} not found'})
    assert api_fighter.update().projs.keys() == {api_tgt_ship.id}
