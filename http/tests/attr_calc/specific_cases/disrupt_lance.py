from tests import approx


def test_debuff_rr(client, consts):
    eve_affectee_attr = client.mk_eve_attr(stackable=True)
    client.mk_eve_buff(
        id_=consts.EveBuff.remote_repair_impedance,
        aggr_mode=consts.EveBuffAggrMode.min,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr.id)])
    eve_effect = client.mk_eve_effect(id_=consts.EveEffect.debuff_lance, cat_id=consts.EveEffCat.active)
    eve_affector_module = client.mk_eve_item(eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_affectee_ship = client.mk_eve_ship(attrs={eve_affectee_attr.id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship.id)
    api_affector_fit1 = api_sol.create_fit()
    api_affector_module1 = api_affector_fit1.add_mod(type_id=eve_affector_module.id, state=consts.ApiState.active)
    api_affector_module1.change_mod(add_projs=[api_affectee_ship.id])
    # Verification
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr.id].dogma == approx(0.5)
    api_mod = api_affectee_ship.mods[eve_affectee_attr.id].one()
    assert api_mod.op == consts.ApiModOp.post_percent
    assert api_mod.initial_val == approx(-50)
    assert api_mod.stacking_mult is None
    assert api_mod.initial_val == approx(-50)
    assert api_mod.affectors.one().item_id == api_affector_module1.id
    assert api_mod.affectors.one().attr_id is None
    # Action
    api_affector_fit2 = api_sol.create_fit()
    api_affector_module2 = api_affector_fit2.add_mod(type_id=eve_affector_module.id, state=consts.ApiState.active)
    api_affector_module2.change_mod(add_projs=[api_affectee_ship.id])
    # Verification - no stacking, lances are applied via debuff
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr.id].dogma == approx(0.5)
    api_mod = api_affectee_ship.mods[eve_affectee_attr.id].one()
    assert api_mod.op == consts.ApiModOp.post_percent
    assert api_mod.initial_val == approx(-50)
    assert api_mod.stacking_mult is None
    assert api_mod.initial_val == approx(-50)
    assert api_mod.affectors.one().item_id in (api_affector_module1.id, api_affector_module2.id)
    assert api_mod.affectors.one().attr_id is None


def test_debuff_warp(client, consts):
    eve_affectee_attr = client.mk_eve_attr(stackable=True)
    client.mk_eve_buff(
        id_=consts.EveBuff.warp_penalty,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.mod_add,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr.id)])
    eve_effect = client.mk_eve_effect(id_=consts.EveEffect.debuff_lance, cat_id=consts.EveEffCat.active)
    eve_affector_module = client.mk_eve_item(eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_affectee_ship = client.mk_eve_ship(attrs={eve_affectee_attr.id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship.id)
    api_affector_fit1 = api_sol.create_fit()
    api_affector_module1 = api_affector_fit1.add_mod(type_id=eve_affector_module.id, state=consts.ApiState.active)
    api_affector_module1.change_mod(add_projs=[api_affectee_ship.id])
    # Verification
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr.id].dogma == approx(100)
    api_mod = api_affectee_ship.mods[eve_affectee_attr.id].one()
    assert api_mod.op == consts.ApiModOp.mod_add
    assert api_mod.initial_val == approx(100)
    assert api_mod.stacking_mult is None
    assert api_mod.initial_val == approx(100)
    assert api_mod.affectors.one().item_id == api_affector_module1.id
    assert api_mod.affectors.one().attr_id is None
    # Action
    api_affector_fit2 = api_sol.create_fit()
    api_affector_module2 = api_affector_fit2.add_mod(type_id=eve_affector_module.id, state=consts.ApiState.active)
    api_affector_module2.change_mod(add_projs=[api_affectee_ship.id])
    # Verification - no stacking, lances are applied via debuff
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr.id].dogma == approx(100)
    api_mod = api_affectee_ship.mods[eve_affectee_attr.id].one()
    assert api_mod.op == consts.ApiModOp.mod_add
    assert api_mod.initial_val == approx(100)
    assert api_mod.stacking_mult is None
    assert api_mod.initial_val == approx(100)
    assert api_mod.affectors.one().item_id in (api_affector_module1.id, api_affector_module2.id)
    assert api_mod.affectors.one().attr_id is None


def test_debuff_dock_jump(client, consts):
    eve_affectee_dock_attr = client.mk_eve_attr(stackable=True)
    eve_affectee_jump_attr = client.mk_eve_attr(stackable=True)
    client.mk_eve_buff(
        id_=consts.EveBuff.disallow_dock_jump,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.mod_add,
        item_mods=[
            client.mk_eve_buff_mod(attr_id=eve_affectee_dock_attr.id),
            client.mk_eve_buff_mod(attr_id=eve_affectee_jump_attr.id)])
    eve_effect = client.mk_eve_effect(id_=consts.EveEffect.debuff_lance, cat_id=consts.EveEffCat.active)
    eve_affector_module = client.mk_eve_item(eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_affectee_ship = client.mk_eve_ship(attrs={eve_affectee_dock_attr.id: 0, eve_affectee_jump_attr.id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship.id)
    api_affector_fit1 = api_sol.create_fit()
    api_affector_module1 = api_affector_fit1.add_mod(type_id=eve_affector_module.id, state=consts.ApiState.active)
    api_affector_module1.change_mod(add_projs=[api_affectee_ship.id])
    # Verification
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_dock_attr.id].dogma == approx(1)
    assert api_affectee_ship.attrs[eve_affectee_jump_attr.id].dogma == approx(1)
    api_dock_mod = api_affectee_ship.mods[eve_affectee_dock_attr.id].one()
    assert api_dock_mod.op == consts.ApiModOp.mod_add
    assert api_dock_mod.initial_val == approx(1)
    assert api_dock_mod.stacking_mult is None
    assert api_dock_mod.initial_val == approx(1)
    assert api_dock_mod.affectors.one().item_id == api_affector_module1.id
    assert api_dock_mod.affectors.one().attr_id is None
    api_jump_mod = api_affectee_ship.mods[eve_affectee_jump_attr.id].one()
    assert api_jump_mod.op == consts.ApiModOp.mod_add
    assert api_jump_mod.initial_val == approx(1)
    assert api_jump_mod.stacking_mult is None
    assert api_jump_mod.initial_val == approx(1)
    assert api_jump_mod.affectors.one().item_id == api_affector_module1.id
    assert api_jump_mod.affectors.one().attr_id is None
    # Action
    api_affector_fit2 = api_sol.create_fit()
    api_affector_module2 = api_affector_fit2.add_mod(type_id=eve_affector_module.id, state=consts.ApiState.active)
    api_affector_module2.change_mod(add_projs=[api_affectee_ship.id])
    # Verification - no stacking, lances are applied via debuff
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_dock_attr.id].dogma == approx(1)
    assert api_affectee_ship.attrs[eve_affectee_jump_attr.id].dogma == approx(1)
    api_dock_mod = api_affectee_ship.mods[eve_affectee_dock_attr.id].one()
    assert api_dock_mod.op == consts.ApiModOp.mod_add
    assert api_dock_mod.initial_val == approx(1)
    assert api_dock_mod.stacking_mult is None
    assert api_dock_mod.initial_val == approx(1)
    assert api_dock_mod.affectors.one().item_id in (api_affector_module1.id, api_affector_module2.id)
    assert api_dock_mod.affectors.one().attr_id is None
    api_jump_mod = api_affectee_ship.mods[eve_affectee_jump_attr.id].one()
    assert api_jump_mod.op == consts.ApiModOp.mod_add
    assert api_jump_mod.initial_val == approx(1)
    assert api_jump_mod.stacking_mult is None
    assert api_jump_mod.initial_val == approx(1)
    assert api_jump_mod.affectors.one().item_id in (api_affector_module1.id, api_affector_module2.id)
    assert api_jump_mod.affectors.one().attr_id is None


def test_debuff_tether(client, consts):
    eve_affectee_attr = client.mk_eve_attr(stackable=True)
    client.mk_eve_buff(
        id_=consts.EveBuff.disallow_tether,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.mod_add,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr.id)])
    eve_effect = client.mk_eve_effect(id_=consts.EveEffect.debuff_lance, cat_id=consts.EveEffCat.active)
    eve_affector_module = client.mk_eve_item(eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_affectee_ship = client.mk_eve_ship(attrs={eve_affectee_attr.id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship.id)
    api_affector_fit1 = api_sol.create_fit()
    api_affector_module1 = api_affector_fit1.add_mod(type_id=eve_affector_module.id, state=consts.ApiState.active)
    api_affector_module1.change_mod(add_projs=[api_affectee_ship.id])
    # Verification
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr.id].dogma == approx(1)
    api_mod = api_affectee_ship.mods[eve_affectee_attr.id].one()
    assert api_mod.op == consts.ApiModOp.mod_add
    assert api_mod.initial_val == approx(1)
    assert api_mod.stacking_mult is None
    assert api_mod.initial_val == approx(1)
    assert api_mod.affectors.one().item_id == api_affector_module1.id
    assert api_mod.affectors.one().attr_id is None
    # Action
    api_affector_fit2 = api_sol.create_fit()
    api_affector_module2 = api_affector_fit2.add_mod(type_id=eve_affector_module.id, state=consts.ApiState.active)
    api_affector_module2.change_mod(add_projs=[api_affectee_ship.id])
    # Verification - no stacking, lances are applied via debuff
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr.id].dogma == approx(1)
    api_mod = api_affectee_ship.mods[eve_affectee_attr.id].one()
    assert api_mod.op == consts.ApiModOp.mod_add
    assert api_mod.initial_val == approx(1)
    assert api_mod.stacking_mult is None
    assert api_mod.initial_val == approx(1)
    assert api_mod.affectors.one().item_id in (api_affector_module1.id, api_affector_module2.id)
    assert api_mod.affectors.one().attr_id is None


def test_drone(client, consts):
    eve_affectee_attr = client.mk_eve_attr(stackable=True)
    client.mk_eve_buff(
        id_=consts.EveBuff.remote_repair_impedance,
        aggr_mode=consts.EveBuffAggrMode.min,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr.id)])
    eve_effect = client.mk_eve_effect(id_=consts.EveEffect.debuff_lance, cat_id=consts.EveEffCat.active)
    eve_affector_module = client.mk_eve_item(eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_affectee_drone = client.mk_eve_item(attrs={eve_affectee_attr.id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_drone = api_affectee_fit.add_drone(type_id=eve_affectee_drone.id)
    api_affector_fit = api_sol.create_fit()
    api_affector_module = api_affector_fit.add_mod(type_id=eve_affector_module.id, state=consts.ApiState.active)
    api_affector_module.change_mod(add_projs=[api_affectee_drone.id])
    # Verification
    api_affectee_drone.update()
    assert api_affectee_drone.attrs[eve_affectee_attr.id].dogma == approx(0.5)
    api_mod = api_affectee_drone.mods[eve_affectee_attr.id].one()
    assert api_mod.op == consts.ApiModOp.post_percent
    assert api_mod.initial_val == approx(-50)
    assert api_mod.stacking_mult is None
    assert api_mod.initial_val == approx(-50)
    assert api_mod.affectors.one().item_id == api_affector_module.id
    assert api_mod.affectors.one().attr_id is None
