from fw import Muta, approx, check_no_field


def test_add_proj_not_found(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_tgt_attr_id = client.mk_eve_attr()
    eve_tgt_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_tgt_attr_id)
    eve_tgt_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_tgt_mod])
    eve_base_drone_id = client.mk_eve_drone(
        attrs={eve_affector_attr_id: 20},
        eff_ids=[eve_tgt_effect_id],
        defeff_id=eve_tgt_effect_id)
    eve_mutated_drone_id = client.mk_eve_drone(eff_ids=[eve_tgt_effect_id], defeff_id=eve_tgt_effect_id)
    eve_mutator_id = client.mk_eve_mutator(
        items=[([eve_base_drone_id], eve_mutated_drone_id)],
        attrs={eve_affector_attr_id: (0.5, 1.5)})
    eve_ship_id = client.mk_eve_ship(attrs={eve_tgt_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_other_tgt = api_sol.add_sw_effect(type_id=eve_ship_id)
    api_other_tgt.remove()
    # Action & verification - drone is fully set up and projected onto the first target before the
    # second projection fails
    api_fit.add_drone(
        type_id=eve_base_drone_id,
        state=consts.ApiMinionState.engaging,
        mutation=(eve_mutator_id, {eve_affector_attr_id: Muta.roll_to_api(val=1)}),
        npc_prop_override=consts.ApiNpcProp.chase,
        proj_item_ids=[api_tgt_ship.id, api_other_tgt.id],
        coordinates=(1000, 0, 0),
        movement=(90, 0, 1),
        effect_modes={eve_tgt_effect_id: consts.ApiEffMode.force_run},
        status_code=400,
        json_predicate={
            'code': 'DRN-002',
            'message': f'failed to add drone: failed to add projection: item {api_other_tgt.id} not found'})
    api_fit.update()
    with check_no_field():
        api_fit.drones  # ruff:ignore[useless-expression]
    assert api_tgt_ship.update().attrs[eve_tgt_attr_id].modified == approx(100)


def test_change_proj_add_fail(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_tgt_attr_id = client.mk_eve_attr()
    eve_tgt_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_tgt_attr_id)
    eve_tgt_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_tgt_mod])
    eve_drone_id = client.mk_eve_drone(
        attrs={eve_affector_attr_id: 20},
        eff_ids=[eve_tgt_effect_id],
        defeff_id=eve_tgt_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_tgt_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_tgt_fit1 = api_sol.create_fit()
    api_tgt_ship1 = api_tgt_fit1.set_ship(type_id=eve_ship_id, coordinates=(0, 10000, 0))
    api_tgt_fit2 = api_sol.create_fit()
    api_tgt_ship2 = api_tgt_fit2.set_ship(type_id=eve_ship_id)
    api_fit = api_sol.create_fit()
    api_drone = api_fit.add_drone(
        type_id=eve_drone_id,
        state=consts.ApiMinionState.engaging,
        proj_item_ids=[api_tgt_ship1.id],
        coordinates=(0, 0, 0),
        movement=(0, 0, 0))
    # Verification
    assert api_drone.update().projs[api_tgt_ship1.id] == (approx(10000), approx(10000))
    assert api_tgt_ship1.update().attrs[eve_tgt_attr_id].modified == approx(120)
    api_other_tgt = api_sol.add_sw_effect(type_id=eve_ship_id)
    api_other_tgt.remove()
    # Action & verification - every part of the change is applied before the second projection fails
    api_drone.change_drone(
        state=consts.ApiMinionState.in_space,
        npc_prop_override=consts.ApiNpcProp.chase,
        rm_proj_item_ids=[api_tgt_ship1.id],
        add_proj_item_ids=[api_tgt_ship2.id, api_other_tgt.id],
        coordinates=(0, 5000, 0),
        movement=(90, 0, 1),
        effect_modes={eve_tgt_effect_id: consts.ApiEffMode.force_stop},
        status_code=400,
        json_predicate={
            'code': 'DRN-003',
            'message': f'failed to change drone: failed to add projection: item {api_other_tgt.id} not found'})
    api_drone.update()
    assert api_drone.state == consts.ApiMinionState.engaging
    with check_no_field():
        api_drone.npc_prop_override  # ruff:ignore[useless-expression]
    assert api_drone.coordinates == (0, 0, 0)
    assert api_drone.movement.speed == approx(0)
    with check_no_field():
        api_drone.effect_mode_overrides  # ruff:ignore[useless-expression]
    assert list(api_drone.projs) == [api_tgt_ship1.id]
    assert api_drone.projs[api_tgt_ship1.id] == (approx(10000), approx(10000))
    assert api_tgt_ship1.update().attrs[eve_tgt_attr_id].modified == approx(120)
    assert api_tgt_ship2.update().attrs[eve_tgt_attr_id].modified == approx(100)


def test_change_type_proj_add_fail(client):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_drone1_id = client.mk_eve_drone(attrs={eve_affector_attr_id: 20})
    eve_drone2_id = client.mk_eve_drone(attrs={eve_affector_attr_id: 50})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_drone = api_fit.add_drone(type_id=eve_drone1_id)
    api_other_tgt = api_sol.add_sw_effect(type_id=eve_drone1_id)
    api_other_tgt.remove()
    # Action & verification
    api_drone.change_drone(
        type_id=eve_drone2_id,
        add_proj_item_ids=[api_other_tgt.id],
        status_code=400,
        json_predicate={
            'code': 'DRN-003',
            'message': f'failed to change drone: failed to add projection: item {api_other_tgt.id} not found'})
    api_drone.update()
    assert api_drone.type_id == eve_drone1_id
    assert api_drone.attrs[eve_affector_attr_id].modified == approx(20)


def test_change_mutation_proj_add_fail(client):
    eve_roll_attr_id = client.mk_eve_attr()
    eve_base_drone_id = client.mk_eve_drone(attrs={eve_roll_attr_id: 20})
    eve_mutated_drone_id = client.mk_eve_drone()
    eve_mutator1_id = client.mk_eve_mutator(
        items=[([eve_base_drone_id], eve_mutated_drone_id)],
        attrs={eve_roll_attr_id: (0.5, 1.5)})
    eve_mutator2_id = client.mk_eve_mutator(
        items=[([eve_base_drone_id], eve_mutated_drone_id)],
        attrs={eve_roll_attr_id: (0.9, 1.1)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_drone = api_fit.add_drone(
        type_id=eve_base_drone_id,
        mutation=(eve_mutator1_id, {eve_roll_attr_id: Muta.roll_to_api(val=0.8)}))
    # Verification
    assert api_drone.update().attrs[eve_roll_attr_id].modified == approx(26)
    api_other_tgt = api_sol.add_sw_effect(type_id=eve_base_drone_id)
    api_other_tgt.remove()
    # Action & verification - both mutator and roll are changed before projection fails
    api_drone.change_drone(
        mutation=(eve_mutator2_id, {eve_roll_attr_id: Muta.roll_to_api(val=0.1)}),
        add_proj_item_ids=[api_other_tgt.id],
        status_code=400,
        json_predicate={
            'code': 'DRN-003',
            'message': f'failed to change drone: failed to add projection: item {api_other_tgt.id} not found'})
    api_drone.update()
    assert api_drone.mutation.mutator_id == eve_mutator1_id
    assert api_drone.mutation.rolls[eve_roll_attr_id] == approx(0.8)
    assert api_drone.attrs[eve_roll_attr_id].modified == approx(26)


def test_change_not_mutated(client, consts):
    eve_attr_id = client.mk_eve_attr()
    eve_drone_id = client.mk_eve_drone(attrs={eve_attr_id: 20})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_drone = api_fit.add_drone(type_id=eve_drone_id)
    # Action & verification - state and coordinates are changed before rolls are found to have no
    # mutation to go to
    api_drone.change_drone(
        state=consts.ApiMinionState.engaging,
        mutation={eve_attr_id: Muta.roll_to_api(val=0.1)},
        coordinates=(1000, 0, 0),
        status_code=400,
        json_predicate={
            'code': 'DRN-005',
            'message': (f'failed to change drone: failed to mutate attributes: '
                        f'item {api_drone.id} is not mutated')})
    api_drone.update()
    assert api_drone.state == consts.ApiMinionState.in_bay
    assert api_drone.coordinates == (0, 0, 0)


def test_change_proj_rm_fail(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_tgt_attr_id = client.mk_eve_attr()
    eve_tgt_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_tgt_attr_id)
    eve_tgt_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_tgt_mod])
    eve_drone_id = client.mk_eve_drone(
        attrs={eve_affector_attr_id: 20},
        eff_ids=[eve_tgt_effect_id],
        defeff_id=eve_tgt_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_tgt_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_fit = api_sol.create_fit()
    api_drone = api_fit.add_drone(
        type_id=eve_drone_id,
        state=consts.ApiMinionState.engaging,
        proj_item_ids=[api_tgt_ship.id])
    api_other_tgt = api_sol.add_sw_effect(type_id=eve_ship_id)
    api_other_tgt.remove()
    # Action & verification - existing projection is removed before the next projectee is not found
    api_drone.change_drone(
        rm_proj_item_ids=[api_tgt_ship.id, api_other_tgt.id],
        status_code=400,
        json_predicate={
            'code': 'DRN-004',
            'message': f'failed to change drone: failed to remove projection: item {api_other_tgt.id} not found'})
    assert list(api_drone.update().projs) == [api_tgt_ship.id]
    assert api_tgt_ship.update().attrs[eve_tgt_attr_id].modified == approx(120)
