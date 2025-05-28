from tests import approx, check_no_field, effect_dogma_to_api


def test_affected_charge_bundled(client, consts):
    # Check that charge is affected by module if they were added simultaneously
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.other,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_affector_item_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_affectee_item_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_affector_item_id,
        charge_type_id=eve_affectee_item_id)
    assert api_module.update().charge.attrs[eve_affectee_attr_id].dogma == approx(120)


def test_affected_charge_separate(client, consts):
    # Check that charge is affected by module if charge is added after module
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.other,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_affector_item_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_affectee_item_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_affector_item_id)
    api_module.change_module(charge_type_id=eve_affectee_item_id)
    assert api_module.update().charge.attrs[eve_affectee_attr_id].dogma == approx(120)


def test_affected_module_bundled(client, consts):
    # Check that module is affected by charge if they were added simultaneously
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.other,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_affector_item_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_affectee_item_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_affectee_item_id, charge_type_id=eve_affector_item_id)
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(120)


def test_affected_module_separate(client, consts):
    # Check that module is affected by charge if charge is added/removed without touching the module
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.other,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_affector_item_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_affectee_item_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_affectee_item_id)
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(100)
    api_module.change_module(charge_type_id=eve_affector_item_id)
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    api_module.change_module(charge_type_id=None)
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(100)


def test_unaffected_fighter_to_autocharge(client, consts):
    # There is no such scenario in EVE, but the lib assumes autocharge cannot receive any
    # modifications from its carrier
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_autocharge_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_launch_bomb_type)
    eve_autocharge_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.fighter_ability_launch_bomb,
        cat_id=consts.EveEffCat.active)
    eve_other_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.other,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_other_effect_id = client.mk_eve_effect(mod_info=[eve_other_mod])
    eve_autocharge_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 100})
    eve_fighter_id = client.mk_eve_item(
        attrs={eve_autocharge_attr_id: eve_autocharge_id, eve_affector_attr_id: 20},
        eff_ids=[eve_other_effect_id, eve_autocharge_effect_id])
    client.create_sources()
    api_autocharge_effect_id = effect_dogma_to_api(dogma_effect_id=eve_autocharge_effect_id)
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id)
    api_fighter.update()
    assert len(api_fighter.autocharges) == 1
    api_autocharge = api_fighter.autocharges[api_autocharge_effect_id]
    assert api_autocharge.attrs[eve_affectee_attr_id].dogma == approx(100)


def test_unaffected_autocharge_to_fighter(client, consts):
    # There is no such scenario in EVE, but the lib assumes autocharge carrier cannot receive any
    # modifications from autocharge
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_autocharge_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_launch_bomb_type)
    eve_autocharge_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.fighter_ability_launch_bomb,
        cat_id=consts.EveEffCat.active)
    eve_other_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.other,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_other_effect_id = client.mk_eve_effect(mod_info=[eve_other_mod])
    eve_autocharge_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_other_effect_id])
    eve_fighter_id = client.mk_eve_item(
        attrs={eve_autocharge_attr_id: eve_autocharge_id, eve_affectee_attr_id: 100},
        eff_ids=[eve_autocharge_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id)
    api_fighter.update()
    assert api_fighter.attrs[eve_affectee_attr_id].dogma == approx(100)


def test_propagation_charge(client, consts):
    # Check that changes to attribute value which is source of modification are propagated to target
    eve_affector_attr_id = client.mk_eve_attr()
    eve_middle_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_affector_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_middle_attr_id)
    eve_affector_effect_id = client.mk_eve_effect(mod_info=[eve_affector_mod])
    eve_affector_item_id = client.mk_eve_item(attrs={eve_affector_attr_id: 2}, eff_ids=[eve_affector_effect_id])
    eve_middle_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.other,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_middle_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_middle_effect_id = client.mk_eve_effect(mod_info=[eve_middle_mod])
    eve_middle_item_id = client.mk_eve_item(attrs={eve_middle_attr_id: 20}, eff_ids=[eve_middle_effect_id])
    eve_affectee_item_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 100})
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_middle_item = api_fit.add_module(type_id=eve_middle_item_id, charge_type_id=eve_affectee_item_id)
    assert api_middle_item.update().charge.attrs[eve_affectee_attr_id].dogma == approx(120)
    api_affector_item = api_fit.add_rig(type_id=eve_affector_item_id)
    assert api_middle_item.update().charge.attrs[eve_affectee_attr_id].dogma == approx(140)
    api_affector_item.remove()
    assert api_middle_item.update().charge.attrs[eve_affectee_attr_id].dogma == approx(120)


def test_propagation_module(client, consts):
    # Check that changes to attribute value which is source of modification are propagated to target
    eve_skill_id = client.mk_eve_item()
    eve_affector_attr_id = client.mk_eve_attr()
    eve_middle_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_affector_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        loc=consts.EveModLoc.char,
        srq=eve_skill_id,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_middle_attr_id)
    eve_affector_effect_id = client.mk_eve_effect(mod_info=[eve_affector_mod])
    eve_affector_item_id = client.mk_eve_item(attrs={eve_affector_attr_id: 2}, eff_ids=[eve_affector_effect_id])
    eve_middle_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.other,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_middle_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_middle_effect_id = client.mk_eve_effect(mod_info=[eve_middle_mod])
    eve_middle_item_id = client.mk_eve_item(
        attrs={eve_middle_attr_id: 20},
        eff_ids=[eve_middle_effect_id],
        srqs={eve_skill_id: 1})
    eve_affectee_item_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_affectee_item = api_fit.add_module(type_id=eve_affectee_item_id, charge_type_id=eve_middle_item_id)
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    api_affector_item = api_fit.add_rig(type_id=eve_affector_item_id)
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(140)
    api_affector_item.remove()
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(120)


def setup_switch_type_id_test(*, client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.other,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_affector_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_affectee_loaded_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 100})
    eve_affectee_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    return eve_affectee_attr_id, eve_affector_id, eve_affectee_loaded_id, eve_affectee_not_loaded_id


def test_switch_type_id_module_affected_to_not_loaded_remove(client, consts):
    (eve_affectee_attr_id,
     eve_affector_id,
     eve_affectee_loaded_id,
     eve_affectee_not_loaded_id) = setup_switch_type_id_test(client=client, consts=consts)
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_affectee_loaded_id, charge_type_id=eve_affector_id)
    # Verification
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    # Action
    api_module.change_module(type_id=eve_affectee_not_loaded_id)
    # Verification
    api_module.update()
    with check_no_field():
        api_module.attrs  # noqa: B018
    # Action
    api_module.charge.remove()
    # Verification
    api_module.update()
    with check_no_field():
        api_module.attrs  # noqa: B018


def test_switch_type_id_module_not_loaded_to_affected_remove(client, consts):
    (eve_affectee_attr_id,
     eve_affector_id,
     eve_affectee_loaded_id,
     eve_affectee_not_loaded_id) = setup_switch_type_id_test(client=client, consts=consts)
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_affectee_not_loaded_id, charge_type_id=eve_affector_id)
    # Verification
    api_module.update()
    with check_no_field():
        api_module.attrs  # noqa: B018
    # Action
    api_module.change_module(type_id=eve_affectee_loaded_id)
    # Verification
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    # Action
    api_module.charge.remove()
    # Verification
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(100)


def test_switch_type_id_module_affected_to_not_loaded_to_affected(client, consts):
    (eve_affectee_attr_id,
     eve_affector_id,
     eve_affectee_loaded_id,
     eve_affectee_not_loaded_id) = setup_switch_type_id_test(client=client, consts=consts)
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_affector_id, charge_type_id=eve_affectee_loaded_id)
    # Verification
    assert api_module.charge.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    # Action
    api_module.charge.change_charge(type_id=eve_affectee_not_loaded_id)
    # Verification
    api_charge = api_module.charge.update()
    with check_no_field():
        api_charge.attrs  # noqa: B018
    # Action
    api_module.charge.change_charge(type_id=eve_affectee_loaded_id)
    # Verification
    assert api_module.charge.update().attrs[eve_affectee_attr_id].dogma == approx(120)
