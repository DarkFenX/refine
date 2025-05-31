from tests import approx


def setup_root_test(*, client, consts):
    eve_skill_id = client.mk_eve_item()
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_srq,
        loc=consts.EveModLoc.tgt,
        srq=eve_skill_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: 20},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_root_ship_id = client.mk_eve_ship()
    eve_root_struct_id = client.mk_eve_struct()
    eve_root_not_loaded_id = client.alloc_item_id()
    eve_affectee_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 80}, srqs={eve_skill_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module = api_fit1.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_affectee = api_fit2.add_rig(type_id=eve_affectee_id)
    return (
        eve_affectee_attr_id,
        eve_root_ship_id,
        eve_root_struct_id,
        eve_root_not_loaded_id,
        api_fit2,
        api_affectee,
        api_module)


def test_root_ship_to_struct_remove(client, consts):
    (eve_affectee_attr_id,
     eve_root_ship_id,
     eve_root_struct_id,
     _,
     api_fit2,
     api_affectee,
     api_module) = setup_root_test(client=client, consts=consts)
    api_root = api_fit2.set_ship(type_id=eve_root_ship_id)
    api_module.change_module(add_projs=[api_root.id])
    # Verification
    assert api_affectee.update().attrs[eve_affectee_attr_id].dogma == approx(96)
    # Action
    api_root.change_ship(type_id=eve_root_struct_id)
    # Verification
    assert api_affectee.update().attrs[eve_affectee_attr_id].dogma == approx(96)
    # Action
    api_module.remove()
    # Verification
    assert api_affectee.update().attrs[eve_affectee_attr_id].dogma == approx(80)


def test_root_ship_to_not_loaded_remove(client, consts):
    (eve_affectee_attr_id,
     eve_root_ship_id,
     _,
     eve_root_not_loaded_id,
     api_fit2,
     api_affectee,
     api_module) = setup_root_test(client=client, consts=consts)
    api_root = api_fit2.set_ship(type_id=eve_root_ship_id)
    api_module.change_module(add_projs=[api_root.id])
    # Verification
    assert api_affectee.update().attrs[eve_affectee_attr_id].dogma == approx(96)
    # Action
    api_root.change_ship(type_id=eve_root_not_loaded_id)
    # Verification
    assert api_affectee.update().attrs[eve_affectee_attr_id].dogma == approx(80)
    # Action
    api_module.remove()
    # Verification
    assert api_affectee.update().attrs[eve_affectee_attr_id].dogma == approx(80)


def test_root_struct_to_ship_remove(client, consts):
    (eve_affectee_attr_id,
     eve_root_ship_id,
     eve_root_struct_id,
     _,
     api_fit2,
     api_affectee,
     api_module) = setup_root_test(client=client, consts=consts)
    api_root = api_fit2.set_ship(type_id=eve_root_struct_id)
    api_module.change_module(add_projs=[api_root.id])
    # Verification
    assert api_affectee.update().attrs[eve_affectee_attr_id].dogma == approx(96)
    # Action
    api_root.change_ship(type_id=eve_root_ship_id)
    # Verification
    assert api_affectee.update().attrs[eve_affectee_attr_id].dogma == approx(96)
    # Action
    api_module.remove()
    # Verification
    assert api_affectee.update().attrs[eve_affectee_attr_id].dogma == approx(80)


def test_root_not_loaded_to_ship_remove(client, consts):
    (eve_affectee_attr_id,
     eve_root_ship_id,
     _,
     eve_root_not_loaded_id,
     api_fit2,
     api_affectee,
     api_module) = setup_root_test(client=client, consts=consts)
    api_root = api_fit2.set_ship(type_id=eve_root_not_loaded_id)
    api_module.change_module(add_projs=[api_root.id])
    # Verification
    assert api_affectee.update().attrs[eve_affectee_attr_id].dogma == approx(80)
    # Action
    api_root.change_ship(type_id=eve_root_ship_id)
    # Verification
    assert api_affectee.update().attrs[eve_affectee_attr_id].dogma == approx(96)
    # Action
    api_module.remove()
    # Verification
    assert api_affectee.update().attrs[eve_affectee_attr_id].dogma == approx(80)
