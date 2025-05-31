from tests import approx


def setup_root_test(*, client, consts):
    eve_grp_id = client.mk_eve_item_group()
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        loc_grp_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id, group_id=eve_grp_id)])
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.weather_darkness,
        cat_id=consts.EveEffCat.active)
    eve_proj_effect_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 5},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_root_ship_id = client.mk_eve_ship()
    eve_root_struct_id = client.mk_eve_struct()
    eve_root_not_loaded_id = client.alloc_item_id()
    eve_module_id = client.mk_eve_item(grp_id=eve_grp_id, attrs={eve_affectee_attr_id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect_id)
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id)
    return (
        eve_affectee_attr_id,
        eve_root_ship_id,
        eve_root_struct_id,
        eve_root_not_loaded_id,
        api_fit,
        api_proj_effect,
        api_module)


def test_root_affected_to_unaffected_remove(client, consts):
    (eve_affectee_attr_id,
     eve_root_ship_id,
     eve_root_struct_id,
     _,
     api_fit,
     api_proj_effect,
     api_module) = setup_root_test(client=client, consts=consts)
    api_root = api_fit.set_ship(type_id=eve_root_ship_id)
    api_proj_effect.change_proj_effect(add_projs=[api_root.id])
    # Verification
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)
    # Action
    api_root.change_ship(type_id=eve_root_struct_id)
    # Verification
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)
    # Action
    api_proj_effect.remove()
    # Verification
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)


def test_root_affected_to_not_loaded_remove(client, consts):
    (eve_affectee_attr_id,
     eve_root_ship_id,
     _,
     eve_root_not_loaded_id,
     api_fit,
     api_proj_effect,
     api_module) = setup_root_test(client=client, consts=consts)
    api_root = api_fit.set_ship(type_id=eve_root_ship_id)
    api_proj_effect.change_proj_effect(add_projs=[api_root.id])
    # Verification
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)
    # Action
    api_root.change_ship(type_id=eve_root_not_loaded_id)
    # Verification
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)
    # Action
    api_proj_effect.remove()
    # Verification
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)


def test_root_unaffected_to_affected_remove(client, consts):
    (eve_affectee_attr_id,
     eve_root_ship_id,
     eve_root_struct_id,
     _,
     api_fit,
     api_proj_effect,
     api_module) = setup_root_test(client=client, consts=consts)
    api_root = api_fit.set_ship(type_id=eve_root_struct_id)
    api_proj_effect.change_proj_effect(add_projs=[api_root.id])
    # Verification
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)
    # Action
    api_root.change_ship(type_id=eve_root_ship_id)
    # Verification
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)
    # Action
    api_proj_effect.remove()
    # Verification
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)


def test_root_not_loaded_to_affected_remove(client, consts):
    (eve_affectee_attr_id,
     eve_root_ship_id,
     _,
     eve_root_not_loaded_id,
     api_fit,
     api_proj_effect,
     api_module) = setup_root_test(client=client, consts=consts)
    api_root = api_fit.set_ship(type_id=eve_root_not_loaded_id)
    api_proj_effect.change_proj_effect(add_projs=[api_root.id])
    # Verification
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)
    # Action
    api_root.change_ship(type_id=eve_root_ship_id)
    # Verification
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)
    # Action
    api_proj_effect.remove()
    # Verification
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)
