from fw import approx


def test_module_add_existing(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: 20},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id, coordinates=(0, 1000, 0))
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        proj_item_ids=[api_tgt_ship.id])
    # Action
    api_module.change_module(add_proj_item_ids=[api_tgt_ship.id])
    # Verification
    api_module.update()
    assert list(api_module.projs) == [api_tgt_ship.id]
    assert api_module.projs[api_tgt_ship.id] == (approx(1000), approx(1000))
    assert api_tgt_ship.update().attrs[eve_affectee_attr_id].modified == approx(120)


def test_module_add_duplicate(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: 20},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_fit = api_sol.create_fit()
    # Action
    api_module = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        proj_item_ids=[api_tgt_ship.id, api_tgt_ship.id])
    # Verification
    assert list(api_module.update().projs) == [api_tgt_ship.id]
    assert api_tgt_ship.update().attrs[eve_affectee_attr_id].modified == approx(120)


def test_module_rm_absent(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: 20},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_tgt_fit1 = api_sol.create_fit()
    api_tgt_ship1 = api_tgt_fit1.set_ship(type_id=eve_ship_id)
    api_tgt_fit2 = api_sol.create_fit()
    api_tgt_ship2 = api_tgt_fit2.set_ship(type_id=eve_ship_id)
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        proj_item_ids=[api_tgt_ship1.id])
    # Action
    api_module.change_module(rm_proj_item_ids=[api_tgt_ship2.id])
    # Verification
    assert list(api_module.update().projs) == [api_tgt_ship1.id]
    assert api_tgt_ship1.update().attrs[eve_affectee_attr_id].modified == approx(120)
    assert api_tgt_ship2.update().attrs[eve_affectee_attr_id].modified == approx(100)


def test_drone_add_existing(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_drone_id = client.mk_eve_drone(
        attrs={eve_affector_attr_id: 20},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id, coordinates=(0, 1000, 0))
    api_fit = api_sol.create_fit()
    api_drone = api_fit.add_drone(
        type_id=eve_drone_id,
        state=consts.ApiMinionState.engaging,
        proj_item_ids=[api_tgt_ship.id])
    # Action
    api_drone.change_drone(add_proj_item_ids=[api_tgt_ship.id])
    # Verification
    api_drone.update()
    assert list(api_drone.projs) == [api_tgt_ship.id]
    assert api_drone.projs[api_tgt_ship.id] == (approx(1000), approx(1000))
    assert api_tgt_ship.update().attrs[eve_affectee_attr_id].modified == approx(120)


def test_drone_add_duplicate(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_drone_id = client.mk_eve_drone(
        attrs={eve_affector_attr_id: 20},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_fit = api_sol.create_fit()
    # Action
    api_drone = api_fit.add_drone(
        type_id=eve_drone_id,
        state=consts.ApiMinionState.engaging,
        proj_item_ids=[api_tgt_ship.id, api_tgt_ship.id])
    # Verification
    assert list(api_drone.update().projs) == [api_tgt_ship.id]
    assert api_tgt_ship.update().attrs[eve_affectee_attr_id].modified == approx(120)


def test_drone_rm_absent(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_drone_id = client.mk_eve_drone(
        attrs={eve_affector_attr_id: 20},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 100})
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
    # Action
    api_drone.change_drone(rm_proj_item_ids=[api_tgt_ship2.id])
    # Verification
    assert list(api_drone.update().projs) == [api_tgt_ship1.id]
    assert api_tgt_ship1.update().attrs[eve_affectee_attr_id].modified == approx(120)
    assert api_tgt_ship2.update().attrs[eve_affectee_attr_id].modified == approx(100)


def test_fighter_add_existing(client):
    eve_ship_id = client.mk_eve_ship()
    eve_fighter_id = client.mk_eve_fighter()
    client.create_sources()
    api_sol = client.create_sol()
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id, coordinates=(0, 1000, 0))
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id, proj_item_ids=[api_tgt_ship.id])
    # Action
    api_fighter.change_fighter(add_proj_item_ids=[api_tgt_ship.id])
    # Verification
    api_fighter.update()
    assert list(api_fighter.projs) == [api_tgt_ship.id]
    assert api_fighter.projs[api_tgt_ship.id] == (approx(1000), approx(1000))


def test_fighter_add_duplicate(client):
    eve_ship_id = client.mk_eve_ship()
    eve_fighter_id = client.mk_eve_fighter()
    client.create_sources()
    api_sol = client.create_sol()
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_fit = api_sol.create_fit()
    # Action
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id, proj_item_ids=[api_tgt_ship.id, api_tgt_ship.id])
    # Verification
    assert list(api_fighter.update().projs) == [api_tgt_ship.id]


def test_fighter_rm_absent(client):
    eve_ship_id = client.mk_eve_ship()
    eve_fighter_id = client.mk_eve_fighter()
    client.create_sources()
    api_sol = client.create_sol()
    api_tgt_fit1 = api_sol.create_fit()
    api_tgt_ship1 = api_tgt_fit1.set_ship(type_id=eve_ship_id)
    api_tgt_fit2 = api_sol.create_fit()
    api_tgt_ship2 = api_tgt_fit2.set_ship(type_id=eve_ship_id)
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id, proj_item_ids=[api_tgt_ship1.id])
    # Action
    api_fighter.change_fighter(rm_proj_item_ids=[api_tgt_ship2.id])
    # Verification
    assert list(api_fighter.update().projs) == [api_tgt_ship1.id]


def test_proj_effect_add_existing(client, consts):
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
    # Action
    api_proj_effect.change_proj_effect(add_proj_item_ids=[api_tgt_ship.id])
    # Verification
    assert api_proj_effect.update().proj_item_ids == [api_tgt_ship.id]
    assert api_tgt_ship.update().attrs[eve_affectee_attr_id].modified == approx(120)


def test_proj_effect_add_duplicate(client, consts):
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
    # Action
    api_proj_effect = api_sol.add_proj_effect(
        type_id=eve_proj_effect_id,
        proj_item_ids=[api_tgt_ship.id, api_tgt_ship.id])
    # Verification
    assert api_proj_effect.update().proj_item_ids == [api_tgt_ship.id]
    assert api_tgt_ship.update().attrs[eve_affectee_attr_id].modified == approx(120)


def test_proj_effect_rm_absent(client, consts):
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
    api_tgt_fit1 = api_sol.create_fit()
    api_tgt_ship1 = api_tgt_fit1.set_ship(type_id=eve_ship_id)
    api_tgt_fit2 = api_sol.create_fit()
    api_tgt_ship2 = api_tgt_fit2.set_ship(type_id=eve_ship_id)
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect_id, proj_item_ids=[api_tgt_ship1.id])
    # Action
    api_proj_effect.change_proj_effect(rm_proj_item_ids=[api_tgt_ship2.id])
    # Verification
    assert api_proj_effect.update().proj_item_ids == [api_tgt_ship1.id]
    assert api_tgt_ship1.update().attrs[eve_affectee_attr_id].modified == approx(120)
    assert api_tgt_ship2.update().attrs[eve_affectee_attr_id].modified == approx(100)
