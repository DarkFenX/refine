from tests import approx


def test_affected_charge_bundled(client, consts):
    # Check that charge is affected by module if they were added simultaneously
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.other,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(mod_info=[eve_mod])
    eve_affector_item = client.mk_eve_item(attrs={eve_affector_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_affectee_item = client.mk_eve_item(attrs={eve_affectee_attr.id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_mod(
        type_id=eve_affector_item.id,
        charge_type_id=eve_affectee_item.id)
    assert api_module.update().charge.attrs[eve_affectee_attr.id].dogma == approx(120)


def test_affected_charge_separate(client, consts):
    # Check that charge is affected by module if charge is added after module
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.other,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(mod_info=[eve_mod])
    eve_affector_item = client.mk_eve_item(attrs={eve_affector_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_affectee_item = client.mk_eve_item(attrs={eve_affectee_attr.id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_mod(type_id=eve_affector_item.id)
    api_module.change_mod(charge=eve_affectee_item.id)
    assert api_module.update().charge.attrs[eve_affectee_attr.id].dogma == approx(120)


def test_affected_module_bundled(client, consts):
    # Check that module is affected by charge if they were added simultaneously
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.other,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(mod_info=[eve_mod])
    eve_affector_item = client.mk_eve_item(attrs={eve_affector_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_affectee_item = client.mk_eve_item(attrs={eve_affectee_attr.id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_mod(type_id=eve_affectee_item.id, charge_type_id=eve_affector_item.id)
    assert api_module.update().attrs[eve_affectee_attr.id].dogma == approx(120)


def test_affected_module_separate(client, consts):
    # Check that module is affected by charge if charge is added/removed without touching the module
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.other,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(mod_info=[eve_mod])
    eve_affector_item = client.mk_eve_item(attrs={eve_affector_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_affectee_item = client.mk_eve_item(attrs={eve_affectee_attr.id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_mod(type_id=eve_affectee_item.id)
    assert api_module.update().attrs[eve_affectee_attr.id].dogma == approx(100)
    api_module.change_mod(charge=eve_affector_item.id)
    assert api_module.update().attrs[eve_affectee_attr.id].dogma == approx(120)
    api_module.change_mod(charge=None)
    assert api_module.update().attrs[eve_affectee_attr.id].dogma == approx(100)


def test_unaffected_fighter_to_autocharge(client, consts):
    # There is no such scenario in EVE, but the lib assumes autocharge is isolated - can't receive
    # any modifications via other reference
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_autocharge_attr = client.mk_eve_attr(id_=consts.EveAttr.fighter_ability_launch_bomb_type)
    eve_autocharge_effect = client.mk_eve_effect(
        id_=consts.EveEffect.fighter_ability_launch_bomb,
        cat_id=consts.EveEffCat.active)
    eve_other_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.other,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_other_effect = client.mk_eve_effect(mod_info=[eve_other_mod])
    eve_autocharge = client.mk_eve_item(attrs={eve_affectee_attr.id: 100})
    eve_fighter = client.mk_eve_item(
        attrs={eve_autocharge_attr.id: eve_autocharge.id, eve_affector_attr.id: 20},
        eff_ids=[eve_other_effect.id, eve_autocharge_effect.id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(type_id=eve_fighter.id)
    api_fighter.update()
    assert len(api_fighter.autocharges) == 1
    api_autocharge = api_fighter.autocharges[eve_autocharge_effect.id]
    assert api_autocharge.attrs[eve_affectee_attr.id].dogma == approx(100)


def test_unaffected_autocharge_to_fighter(client, consts):
    # There is no such scenario in EVE, but the lib assumes autocharge is isolated - can't emit
    # any modifications via other reference
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_autocharge_attr = client.mk_eve_attr(id_=consts.EveAttr.fighter_ability_launch_bomb_type)
    eve_autocharge_effect = client.mk_eve_effect(
        id_=consts.EveEffect.fighter_ability_launch_bomb,
        cat_id=consts.EveEffCat.active)
    eve_other_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.other,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_other_effect = client.mk_eve_effect(mod_info=[eve_other_mod])
    eve_autocharge = client.mk_eve_item(attrs={eve_affector_attr.id: 20}, eff_ids=[eve_other_effect.id])
    eve_fighter = client.mk_eve_item(
        attrs={eve_autocharge_attr.id: eve_autocharge.id, eve_affectee_attr.id: 100},
        eff_ids=[eve_autocharge_effect.id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(type_id=eve_fighter.id)
    api_fighter.update()
    assert api_fighter.attrs[eve_affectee_attr.id].dogma == approx(100)


def test_propagation_charge(client, consts):
    # Check that changes to attribute value which is source of modification are propagated to target
    eve_affector_attr = client.mk_eve_attr()
    eve_middle_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_affector_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_middle_attr.id)
    eve_affector_effect = client.mk_eve_effect(mod_info=[eve_affector_mod])
    eve_affector_item = client.mk_eve_item(attrs={eve_affector_attr.id: 2}, eff_ids=[eve_affector_effect.id])
    eve_middle_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.other,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_middle_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_middle_effect = client.mk_eve_effect(mod_info=[eve_middle_mod])
    eve_middle_item = client.mk_eve_item(attrs={eve_middle_attr.id: 20}, eff_ids=[eve_middle_effect.id])
    eve_affectee_item = client.mk_eve_item(attrs={eve_affectee_attr.id: 100})
    eve_ship = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship.id)
    api_middle_item = api_fit.add_mod(type_id=eve_middle_item.id, charge_type_id=eve_affectee_item.id)
    assert api_middle_item.update().charge.attrs[eve_affectee_attr.id].dogma == approx(120)
    api_affector_item = api_fit.add_rig(type_id=eve_affector_item.id)
    assert api_middle_item.update().charge.attrs[eve_affectee_attr.id].dogma == approx(140)
    api_affector_item.remove()
    assert api_middle_item.update().charge.attrs[eve_affectee_attr.id].dogma == approx(120)


def test_propagation_module(client, consts):
    # Check that changes to attribute value which is source of modification are propagated to target
    eve_skill = client.mk_eve_item()
    eve_affector_attr = client.mk_eve_attr()
    eve_middle_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_affector_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        dom=consts.EveModDom.char,
        srq=eve_skill.id,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_middle_attr.id)
    eve_affector_effect = client.mk_eve_effect(mod_info=[eve_affector_mod])
    eve_affector_item = client.mk_eve_item(attrs={eve_affector_attr.id: 2}, eff_ids=[eve_affector_effect.id])
    eve_middle_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.other,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_middle_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_middle_effect = client.mk_eve_effect(mod_info=[eve_middle_mod])
    eve_middle_item = client.mk_eve_item(
        attrs={eve_middle_attr.id: 20},
        eff_ids=[eve_middle_effect.id],
        srqs={eve_skill.id: 1})
    eve_affectee_item = client.mk_eve_item(attrs={eve_affectee_attr.id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_affectee_item = api_fit.add_mod(type_id=eve_affectee_item.id, charge_type_id=eve_middle_item.id)
    assert api_affectee_item.update().attrs[eve_affectee_attr.id].dogma == approx(120)
    api_affector_item = api_fit.add_rig(type_id=eve_affector_item.id)
    assert api_affectee_item.update().attrs[eve_affectee_attr.id].dogma == approx(140)
    api_affector_item.remove()
    assert api_affectee_item.update().attrs[eve_affectee_attr.id].dogma == approx(120)
