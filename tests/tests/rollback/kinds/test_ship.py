from fw import approx, check_no_field
from tests.rollback.kinds import rollback_batch, setup_mod_basics


def test_set(client, consts):
    eve_basic_info = setup_mod_basics(client=client, consts=consts)
    eve_ship_effect_id = client.mk_eve_effect(mod_info=[eve_basic_info.ship_mod])
    eve_tgt_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_basic_info.tgt_mod])
    eve_rig_id = client.mk_eve_item(attrs={eve_basic_info.affector_attr_id: 50}, eff_ids=[eve_ship_effect_id])
    eve_module_id = client.mk_eve_item(
        attrs={eve_basic_info.affector_attr_id: 20},
        eff_ids=[eve_tgt_effect_id],
        defeff_id=eve_tgt_effect_id)
    eve_ship1_id = client.mk_eve_ship(attrs={eve_basic_info.affectee_attr_id: 100})
    eve_ship2_id = client.mk_eve_ship(attrs={eve_basic_info.affectee_attr_id: 200})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship1_id)
    api_fit.add_rig(type_id=eve_rig_id)
    api_other_fit = api_sol.create_fit()
    api_module = api_other_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        proj_item_ids=[api_ship.id])
    # Verification
    assert api_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(180)
    # Action & verification - new ship replaces the old one, which takes the module projection with
    # it
    with rollback_batch(api_sol=api_sol) as api_sol_batch:
        api_sol_batch.set_ship(fit_id=api_fit.id, type_id=eve_ship2_id)
    assert api_fit.update().ship.id == api_ship.id
    assert api_module.update().projs.keys() == {api_ship.id}
    api_ship.update()
    assert api_ship.type_id == eve_ship1_id
    assert api_ship.attrs[eve_basic_info.affectee_attr_id].modified == approx(180)


def test_unset(client, consts):
    eve_basic_info = setup_mod_basics(client=client, consts=consts)
    eve_ship_effect_id = client.mk_eve_effect(mod_info=[eve_basic_info.ship_mod])
    eve_tgt_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_basic_info.tgt_mod])
    eve_system_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_basic_info.ship_mod])
    eve_rig_id = client.mk_eve_item(attrs={eve_basic_info.affector_attr_id: 50}, eff_ids=[eve_ship_effect_id])
    eve_module_id = client.mk_eve_item(
        attrs={eve_basic_info.affector_attr_id: 20},
        eff_ids=[eve_tgt_effect_id],
        defeff_id=eve_tgt_effect_id)
    eve_proj_effect_id = client.mk_eve_item(attrs={eve_basic_info.affector_attr_id: 10}, eff_ids=[eve_system_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_basic_info.affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_rig(type_id=eve_rig_id)
    api_other_fit = api_sol.create_fit()
    api_module = api_other_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        proj_item_ids=[api_ship.id])
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect_id, proj_item_ids=[api_ship.id])
    # Verification
    assert api_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(198)
    # Action & verification
    with rollback_batch(api_sol=api_sol) as api_sol_batch:
        api_sol_batch.unset_ship(fit_id=api_fit.id)
    assert api_fit.update().ship.id == api_ship.id
    assert api_module.update().projs.keys() == {api_ship.id}
    assert api_proj_effect.update().proj_item_ids == [api_ship.id]
    assert api_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(198)


def test_change(client, consts):
    eve_basic_info = setup_mod_basics(client=client, consts=consts)
    eve_ship_effect_id = client.mk_eve_effect(mod_info=[eve_basic_info.ship_mod])
    eve_tgt_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_basic_info.tgt_mod])
    eve_module_id = client.mk_eve_item(
        attrs={eve_basic_info.affector_attr_id: 20},
        eff_ids=[eve_tgt_effect_id],
        defeff_id=eve_tgt_effect_id)
    eve_ship1_id = client.mk_eve_ship(
        attrs={eve_basic_info.affector_attr_id: 20, eve_basic_info.affectee_attr_id: 100},
        eff_ids=[eve_ship_effect_id])
    eve_ship2_id = client.mk_eve_ship(
        attrs={eve_basic_info.affector_attr_id: 50, eve_basic_info.affectee_attr_id: 200},
        eff_ids=[eve_ship_effect_id])
    eve_other_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship1_id, coordinates=(0, 1000, 0))
    api_other_fit = api_sol.create_fit()
    api_other_ship = api_other_fit.set_ship(type_id=eve_other_ship_id)
    api_module = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        proj_item_ids=[api_other_ship.id])
    api_other_module = api_other_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        proj_item_ids=[api_ship.id])
    # Verification
    assert api_module.update().projs[api_other_ship.id] == (approx(1000), approx(1000))
    assert api_other_module.update().projs[api_ship.id] == (approx(1000), approx(1000))
    assert api_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(144)
    # Action & verification - range of projections from and to the ship changes with its movement
    with rollback_batch(api_sol=api_sol) as api_sol_batch:
        api_sol_batch.change_ship_via_item_id(
            item_id=api_ship.id,
            type_id=eve_ship2_id,
            state=False,
            coordinates=(0, 5000, 0),
            movement=(90, 0, 1),
            effect_modes={eve_ship_effect_id: consts.ApiEffMode.force_stop})
    api_ship.update()
    assert api_ship.type_id == eve_ship1_id
    assert api_ship.state is True
    assert api_ship.coordinates == (0, 1000, 0)
    assert api_ship.movement == (0, 0, 0)
    with check_no_field():
        api_ship.effect_mode_overrides  # ruff:ignore[useless-expression]
    assert api_ship.attrs[eve_basic_info.affectee_attr_id].modified == approx(144)
    assert api_module.update().projs[api_other_ship.id] == (approx(1000), approx(1000))
    assert api_other_module.update().projs[api_ship.id] == (approx(1000), approx(1000))
