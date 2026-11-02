from fw import Muta, approx, check_no_field
from tests.rollback.kinds import rollback_batch, setup_mod_basics


def setup_mutation(*, client, consts, basic_info):
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[basic_info.tgt_mod])
    eve_roll_attr_id = client.mk_eve_attr()
    eve_base_drone_id = client.mk_eve_drone(
        attrs={basic_info.affector_attr_id: 20, eve_roll_attr_id: 100},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_mutated_drone_id = client.mk_eve_drone(eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_mutator1_id = client.mk_eve_mutator(
        items=[([eve_base_drone_id], eve_mutated_drone_id)],
        attrs={basic_info.affector_attr_id: (0.5, 1.5), eve_roll_attr_id: (0.5, 1.5)})
    eve_mutator2_id = client.mk_eve_mutator(
        items=[([eve_base_drone_id], eve_mutated_drone_id)],
        attrs={basic_info.affector_attr_id: (0.9, 1.1), eve_roll_attr_id: (0.9, 1.1)})
    return eve_roll_attr_id, eve_base_drone_id, eve_mutated_drone_id, eve_mutator1_id, eve_mutator2_id


def test_add(client, consts):
    eve_basic_info = setup_mod_basics(client=client, consts=consts)
    eve_roll_attr_id, eve_base_drone_id, _, eve_mutator_id, _ = setup_mutation(
        client=client, consts=consts, basic_info=eve_basic_info)
    eve_ship_id = client.mk_eve_ship(attrs={eve_basic_info.affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_fit = api_sol.create_fit()
    # Action & verification
    with rollback_batch(api_sol=api_sol) as api_sol_batch:
        api_sol_batch.add_drone(
            fit_id=api_fit.id,
            type_id=eve_base_drone_id,
            state=consts.ApiMinionState.engaging,
            mutation=(eve_mutator_id, {eve_roll_attr_id: Muta.roll_to_api(val=0.8)}),
            proj_item_ids=[api_tgt_ship.id])
    with check_no_field():
        api_fit.update().drones  # ruff:ignore[useless-expression]
    assert api_tgt_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(100)


def test_remove(client, consts):
    eve_basic_info = setup_mod_basics(client=client, consts=consts)
    _, eve_base_drone_id, eve_mutated_drone_id, eve_mutator_id, _ = setup_mutation(
        client=client, consts=consts, basic_info=eve_basic_info)
    eve_ship_id = client.mk_eve_ship(attrs={eve_basic_info.affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_fit = api_sol.create_fit()
    api_drone = api_fit.add_drone(
        type_id=eve_base_drone_id,
        state=consts.ApiMinionState.engaging,
        mutation=(eve_mutator_id, {eve_basic_info.affector_attr_id: Muta.roll_to_api(val=0.8)}),
        proj_item_ids=[api_tgt_ship.id])
    # Verification
    assert api_tgt_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(126)
    # Action & verification
    with rollback_batch(api_sol=api_sol) as api_sol_batch:
        api_sol_batch.remove_item(item_id=api_drone.id)
    assert api_fit.update().drones.keys() == {api_drone.id}
    api_drone.update()
    assert api_drone.type_id == eve_mutated_drone_id
    assert api_drone.mutation.rolls == {eve_basic_info.affector_attr_id: approx(0.8)}
    assert api_drone.projs.keys() == {api_tgt_ship.id}
    assert api_tgt_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(126)


def test_change(client, consts):
    eve_basic_info = setup_mod_basics(client=client, consts=consts)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_basic_info.tgt_mod])
    eve_drone1_id = client.mk_eve_drone(
        attrs={eve_basic_info.affector_attr_id: 20},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_drone2_id = client.mk_eve_drone(
        attrs={eve_basic_info.affector_attr_id: 50},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_basic_info.affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id, coordinates=(0, 10000, 0))
    api_fit = api_sol.create_fit()
    api_drone = api_fit.add_drone(
        type_id=eve_drone1_id,
        state=consts.ApiMinionState.engaging,
        proj_item_ids=[api_tgt_ship.id])
    # Verification
    assert api_drone.update().projs[api_tgt_ship.id] == (approx(10000), approx(10000))
    assert api_tgt_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(120)
    # Action & verification
    with rollback_batch(api_sol=api_sol) as api_sol_batch:
        api_sol_batch.change_drone(
            item_id=api_drone.id,
            type_id=eve_drone2_id,
            state=consts.ApiMinionState.in_space,
            npc_prop_override=consts.ApiNpcProp.chase,
            coordinates=(0, 5000, 0),
            movement=(90, 0, 1),
            effect_modes={eve_effect_id: consts.ApiEffMode.force_stop})
    api_drone.update()
    assert api_drone.type_id == eve_drone1_id
    assert api_drone.state == consts.ApiMinionState.engaging
    with check_no_field():
        api_drone.npc_prop_override  # ruff:ignore[useless-expression]
    assert api_drone.coordinates == (0, 0, 0)
    assert api_drone.movement == (0, 0, 0)
    with check_no_field():
        api_drone.effect_mode_overrides  # ruff:ignore[useless-expression]
    assert api_drone.projs[api_tgt_ship.id] == (approx(10000), approx(10000))
    assert api_tgt_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(120)


def test_change_projs(client, consts):
    eve_basic_info = setup_mod_basics(client=client, consts=consts)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_basic_info.tgt_mod])
    eve_drone_id = client.mk_eve_drone(
        attrs={eve_basic_info.affector_attr_id: 20},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_basic_info.affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_tgt_fit1 = api_sol.create_fit()
    api_tgt_ship1 = api_tgt_fit1.set_ship(type_id=eve_ship_id)
    api_tgt_fit2 = api_sol.create_fit()
    api_tgt_ship2 = api_tgt_fit2.set_ship(type_id=eve_ship_id)
    api_fit = api_sol.create_fit()
    api_drone = api_fit.add_drone(
        type_id=eve_drone_id,
        state=consts.ApiMinionState.engaging,
        proj_item_ids=[api_tgt_ship1.id])
    # Verification
    assert api_tgt_ship1.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(120)
    # Action & verification
    with rollback_batch(api_sol=api_sol) as api_sol_batch:
        api_sol_batch.change_drone(
            item_id=api_drone.id,
            rm_proj_item_ids=[api_tgt_ship1.id],
            add_proj_item_ids=[api_tgt_ship2.id])
    assert api_drone.update().projs.keys() == {api_tgt_ship1.id}
    assert api_tgt_ship1.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(120)
    assert api_tgt_ship2.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(100)


def test_mutation_add(client, consts):
    eve_basic_info = setup_mod_basics(client=client, consts=consts)
    _, eve_base_drone_id, _, eve_mutator_id, _ = setup_mutation(client=client, consts=consts, basic_info=eve_basic_info)
    eve_ship_id = client.mk_eve_ship(attrs={eve_basic_info.affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_fit = api_sol.create_fit()
    api_drone = api_fit.add_drone(
        type_id=eve_base_drone_id,
        state=consts.ApiMinionState.engaging,
        proj_item_ids=[api_tgt_ship.id])
    # Verification
    assert api_tgt_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(120)
    # Action & verification
    with rollback_batch(api_sol=api_sol) as api_sol_batch:
        api_sol_batch.change_drone(
            item_id=api_drone.id,
            mutation=(eve_mutator_id, {eve_basic_info.affector_attr_id: Muta.roll_to_api(val=0.8)}))
    api_drone.update()
    assert api_drone.type_id == eve_base_drone_id
    with check_no_field():
        api_drone.mutation  # ruff:ignore[useless-expression]
    assert api_tgt_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(120)


def test_mutation_change(client, consts):
    eve_basic_info = setup_mod_basics(client=client, consts=consts)
    eve_roll_attr_id, eve_base_drone_id, _, eve_mutator1_id, eve_mutator2_id = setup_mutation(
        client=client, consts=consts, basic_info=eve_basic_info)
    eve_ship_id = client.mk_eve_ship(attrs={eve_basic_info.affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_fit = api_sol.create_fit()
    api_drone = api_fit.add_drone(
        type_id=eve_base_drone_id,
        state=consts.ApiMinionState.engaging,
        mutation=(eve_mutator1_id, {
            eve_basic_info.affector_attr_id: Muta.roll_to_api(val=0.8),
            eve_roll_attr_id: Muta.roll_to_api(val=0.3)}),
        proj_item_ids=[api_tgt_ship.id])
    # Verification
    assert api_drone.update().attrs[eve_roll_attr_id].modified == approx(80)
    assert api_tgt_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(126)
    # Action & verification - mutator is replaced, one roll is changed and another one is removed
    with rollback_batch(api_sol=api_sol) as api_sol_batch:
        api_sol_batch.change_drone(
            item_id=api_drone.id,
            mutation=(eve_mutator2_id, {
                eve_basic_info.affector_attr_id: Muta.roll_to_api(val=0.1),
                eve_roll_attr_id: None}))
    api_drone.update()
    assert api_drone.mutation.mutator_id == eve_mutator1_id
    assert api_drone.mutation.rolls == {eve_basic_info.affector_attr_id: approx(0.8), eve_roll_attr_id: approx(0.3)}
    assert api_drone.attrs[eve_roll_attr_id].modified == approx(80)
    assert api_tgt_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(126)


def test_mutation_remove_active(client, consts):
    eve_basic_info = setup_mod_basics(client=client, consts=consts)
    _, eve_base_drone_id, eve_mutated_drone_id, eve_mutator_id, _ = setup_mutation(
        client=client, consts=consts, basic_info=eve_basic_info)
    eve_ship_id = client.mk_eve_ship(attrs={eve_basic_info.affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_fit = api_sol.create_fit()
    api_drone = api_fit.add_drone(
        type_id=eve_base_drone_id,
        state=consts.ApiMinionState.engaging,
        mutation=(eve_mutator_id, {eve_basic_info.affector_attr_id: Muta.roll_to_api(val=0.8)}),
        proj_item_ids=[api_tgt_ship.id])
    # Verification
    assert api_tgt_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(126)
    # Action & verification
    with rollback_batch(api_sol=api_sol) as api_sol_batch:
        api_sol_batch.change_drone(item_id=api_drone.id, mutation=None)
    api_drone.update()
    assert api_drone.type_id == eve_mutated_drone_id
    assert api_drone.mutation.mutator_id == eve_mutator_id
    assert api_drone.mutation.rolls == {eve_basic_info.affector_attr_id: approx(0.8)}
    assert api_tgt_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(126)


def test_mutation_remove_dormant(client, consts):
    eve_basic_info = setup_mod_basics(client=client, consts=consts)
    _, eve_base_drone_id, _, _, _ = setup_mutation(client=client, consts=consts, basic_info=eve_basic_info)
    eve_mutator_id = client.alloc_item_id()
    eve_ship_id = client.mk_eve_ship(attrs={eve_basic_info.affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_fit = api_sol.create_fit()
    api_drone = api_fit.add_drone(
        type_id=eve_base_drone_id,
        state=consts.ApiMinionState.engaging,
        mutation=(eve_mutator_id, {eve_basic_info.affector_attr_id: Muta.roll_to_api(val=0.8)}),
        proj_item_ids=[api_tgt_ship.id])
    # Verification
    assert api_drone.update().mutation.type == consts.ApiMutationType.dormant
    assert api_tgt_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(120)
    # Action
    with rollback_batch(api_sol=api_sol) as api_sol_batch:
        api_sol_batch.change_drone(item_id=api_drone.id, mutation=None)
    # Verification
    api_drone.update()
    assert api_drone.type_id == eve_base_drone_id
    assert api_drone.mutation.type == consts.ApiMutationType.dormant
    assert api_drone.mutation.mutator_id == eve_mutator_id
    assert api_drone.mutation.rolls == {eve_basic_info.affector_attr_id: approx(0.8)}
    assert api_tgt_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(120)
