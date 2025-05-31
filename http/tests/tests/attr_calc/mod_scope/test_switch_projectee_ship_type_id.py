"""
Check that multiple types of projected modifiers are processed when switching ship type.
"""

from tests import approx


def setup_switch_type_id_test(*, client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    # System effect
    eve_system_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_system_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_system_mod])
    eve_system_proj_effect_id = client.mk_eve_item(attrs={eve_affector_attr_id: 3}, eff_ids=[eve_system_effect_id])
    # Buff effect
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.mod_add,
        loc_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_buff_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.weather_darkness,
        cat_id=consts.EveEffCat.active)
    eve_buff_proj_effect_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 11},
        eff_ids=[eve_buff_effect_id], defeff_id=eve_buff_effect_id)
    # Targeted effect
    eve_targeted_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_targeted_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_targeted_mod])
    eve_targeted_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: 19},
        eff_ids=[eve_targeted_effect_id],
        defeff_id=eve_targeted_effect_id)
    # Other setup
    eve_affectee_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 0})
    eve_root_ship_id = client.mk_eve_ship()
    eve_root_struct_id = client.mk_eve_struct()
    eve_root_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_affectee_item = api_fit.add_rig(type_id=eve_affectee_id)
    api_system_proj_effect = api_sol.add_proj_effect(type_id=eve_system_proj_effect_id)
    api_buff_proj_effect = api_sol.add_proj_effect(type_id=eve_buff_proj_effect_id)
    api_targeting_fit = api_sol.create_fit()
    api_targeted_module = api_targeting_fit.add_module(
        type_id=eve_targeted_module_id,
        state=consts.ApiModuleState.active)
    return (
        eve_affectee_attr_id,
        eve_root_ship_id,
        eve_root_struct_id,
        eve_root_not_loaded_id,
        api_fit,
        api_system_proj_effect,
        api_buff_proj_effect,
        api_targeted_module,
        api_affectee_item)


def test_switch_type_id_ship_to_struct_remove(client, consts):
    (eve_affectee_attr_id,
     eve_root_ship_id,
     eve_root_struct_id,
     _,
     api_fit,
     api_system_proj_effect,
     api_buff_proj_effect,
     api_targeted_module,
     api_affectee_item) = setup_switch_type_id_test(client=client, consts=consts)
    api_root = api_fit.set_ship(type_id=eve_root_ship_id)
    api_system_proj_effect.change_proj_effect(add_projs=[api_root.id])
    api_buff_proj_effect.change_proj_effect(add_projs=[api_root.id])
    api_targeted_module.change_module(add_projs=[api_root.id])
    # Verification
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(33)
    # Action
    api_root.change_ship(type_id=eve_root_struct_id)
    # Verification
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(19)
    # Action
    api_system_proj_effect.remove()
    api_buff_proj_effect.remove()
    api_targeted_module.remove()
    # Verification
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(0)


def test_switch_type_id_ship_to_not_loaded_remove(client, consts):
    (eve_affectee_attr_id,
     eve_root_ship_id,
     _,
     eve_root_not_loaded_id,
     api_fit,
     api_system_proj_effect,
     api_buff_proj_effect,
     api_targeted_module,
     api_affectee_item) = setup_switch_type_id_test(client=client, consts=consts)
    api_root = api_fit.set_ship(type_id=eve_root_ship_id)
    api_system_proj_effect.change_proj_effect(add_projs=[api_root.id])
    api_buff_proj_effect.change_proj_effect(add_projs=[api_root.id])
    api_targeted_module.change_module(add_projs=[api_root.id])
    # Verification
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(33)
    # Action
    api_root.change_ship(type_id=eve_root_not_loaded_id)
    # Verification
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(0)
    # Action
    api_system_proj_effect.remove()
    api_buff_proj_effect.remove()
    api_targeted_module.remove()
    # Verification
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(0)


def test_switch_type_id_struct_to_ship_remove(client, consts):
    (eve_affectee_attr_id,
     eve_root_ship_id,
     eve_root_struct_id,
     _,
     api_fit,
     api_system_proj_effect,
     api_buff_proj_effect,
     api_targeted_module,
     api_affectee_item) = setup_switch_type_id_test(client=client, consts=consts)
    api_root = api_fit.set_ship(type_id=eve_root_struct_id)
    api_system_proj_effect.change_proj_effect(add_projs=[api_root.id])
    api_buff_proj_effect.change_proj_effect(add_projs=[api_root.id])
    api_targeted_module.change_module(add_projs=[api_root.id])
    # Verification
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(19)
    # Action
    api_root.change_ship(type_id=eve_root_ship_id)
    # Verification
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(33)
    # Action
    api_system_proj_effect.remove()
    api_buff_proj_effect.remove()
    api_targeted_module.remove()
    # Verification
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(0)


def test_switch_type_id_not_loaded_to_ship_remove(client, consts):
    (eve_affectee_attr_id,
     eve_root_ship_id,
     _,
     eve_root_not_loaded_id,
     api_fit,
     api_system_proj_effect,
     api_buff_proj_effect,
     api_targeted_module,
     api_affectee_item) = setup_switch_type_id_test(client=client, consts=consts)
    api_root = api_fit.set_ship(type_id=eve_root_not_loaded_id)
    api_system_proj_effect.change_proj_effect(add_projs=[api_root.id])
    api_buff_proj_effect.change_proj_effect(add_projs=[api_root.id])
    api_targeted_module.change_module(add_projs=[api_root.id])
    # Verification
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(0)
    # Action
    api_root.change_ship(type_id=eve_root_ship_id)
    # Verification
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(33)
    # Action
    api_system_proj_effect.remove()
    api_buff_proj_effect.remove()
    api_targeted_module.remove()
    # Verification
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(0)
