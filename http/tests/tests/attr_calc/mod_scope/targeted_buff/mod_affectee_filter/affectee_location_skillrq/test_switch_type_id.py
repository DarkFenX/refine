from fw import approx, check_no_field


def setup_root_test(*, client, consts):
    eve_skill_id = client.mk_eve_item()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_loaded_onlist_id = client.mk_eve_ship()
    eve_loaded_offlist_id = client.mk_eve_ship()
    eve_unloaded_onlist_id = client.alloc_item_id()
    eve_unloaded_offlist_id = client.alloc_item_id()
    eve_item_list_id = client.mk_eve_item_list(inc_type_ids=[eve_loaded_onlist_id, eve_unloaded_onlist_id])
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        loc_srq_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id, skill_id=eve_skill_id)])
    eve_module_id = client.mk_eve_item()
    client.mk_eve_space_comp(type_id=eve_module_id, sw_buffs=({eve_buff_id: -55}, eve_item_list_id))
    eve_rig_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 200}, srqs={eve_skill_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module = api_fit1.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_rig = api_fit2.add_rig(type_id=eve_rig_id)
    return (
        eve_affectee_attr_id,
        eve_loaded_onlist_id,
        eve_loaded_offlist_id,
        eve_unloaded_onlist_id,
        eve_unloaded_offlist_id,
        api_fit2,
        api_module,
        api_rig)


def test_root_loaded_onlist_to_loaded_offlist_remove(client, consts):
    (eve_affectee_attr_id,
     eve_loaded_onlist_id,
     eve_loaded_offlist_id,
     _,
     _,
     api_fit2,
     api_module,
     api_rig) = setup_root_test(client=client, consts=consts)
    api_root = api_fit2.set_ship(type_id=eve_loaded_onlist_id)
    api_module.change_module(add_projs=[api_root.id])
    # Verification
    assert api_rig.update().attrs[eve_affectee_attr_id].modified == approx(90)
    # Action
    api_root.change_ship(type_id=eve_loaded_offlist_id)
    # Verification
    assert api_rig.update().attrs[eve_affectee_attr_id].modified == approx(200)
    # Action
    api_module.remove()
    # Verification
    assert api_rig.update().attrs[eve_affectee_attr_id].modified == approx(200)


def test_root_loaded_onlist_to_unloaded_onlist_remove(client, consts):
    (eve_affectee_attr_id,
     eve_loaded_onlist_id,
     _,
     eve_unloaded_onlist_id,
     _,
     api_fit2,
     api_module,
     api_rig) = setup_root_test(client=client, consts=consts)
    api_root = api_fit2.set_ship(type_id=eve_loaded_onlist_id)
    api_module.change_module(add_projs=[api_root.id])
    # Verification
    assert api_rig.update().attrs[eve_affectee_attr_id].modified == approx(90)
    # Action
    api_root.change_ship(type_id=eve_unloaded_onlist_id)
    # Verification
    assert api_rig.update().attrs[eve_affectee_attr_id].modified == approx(200)
    # Action
    api_module.remove()
    # Verification
    assert api_rig.update().attrs[eve_affectee_attr_id].modified == approx(200)


def test_root_loaded_onlist_to_unloaded_offlist_remove(client, consts):
    (eve_affectee_attr_id,
     eve_loaded_onlist_id,
     _,
     _,
     eve_unloaded_offlist_id,
     api_fit2,
     api_module,
     api_rig) = setup_root_test(client=client, consts=consts)
    api_root = api_fit2.set_ship(type_id=eve_loaded_onlist_id)
    api_module.change_module(add_projs=[api_root.id])
    # Verification
    assert api_rig.update().attrs[eve_affectee_attr_id].modified == approx(90)
    # Action
    api_root.change_ship(type_id=eve_unloaded_offlist_id)
    # Verification
    assert api_rig.update().attrs[eve_affectee_attr_id].modified == approx(200)
    # Action
    api_module.remove()
    # Verification
    assert api_rig.update().attrs[eve_affectee_attr_id].modified == approx(200)


def test_root_loaded_offlist_to_loaded_onlist_remove(client, consts):
    (eve_affectee_attr_id,
     eve_loaded_onlist_id,
     eve_loaded_offlist_id,
     _,
     _,
     api_fit2,
     api_module,
     api_rig) = setup_root_test(client=client, consts=consts)
    api_root = api_fit2.set_ship(type_id=eve_loaded_offlist_id)
    api_module.change_module(add_projs=[api_root.id])
    # Verification
    assert api_rig.update().attrs[eve_affectee_attr_id].modified == approx(200)
    # Action
    api_root.change_ship(type_id=eve_loaded_onlist_id)
    # Verification
    assert api_rig.update().attrs[eve_affectee_attr_id].modified == approx(90)
    # Action
    api_module.remove()
    # Verification
    assert api_rig.update().attrs[eve_affectee_attr_id].modified == approx(200)


def test_root_unloaded_onlist_to_loaded_onlist_remove(client, consts):
    (eve_affectee_attr_id,
     eve_loaded_onlist_id,
     _,
     eve_unloaded_onlist_id,
     _,
     api_fit2,
     api_module,
     api_rig) = setup_root_test(client=client, consts=consts)
    api_root = api_fit2.set_ship(type_id=eve_unloaded_onlist_id)
    api_module.change_module(add_projs=[api_root.id])
    # Verification
    assert api_rig.update().attrs[eve_affectee_attr_id].modified == approx(200)
    # Action
    api_root.change_ship(type_id=eve_loaded_onlist_id)
    # Verification
    assert api_rig.update().attrs[eve_affectee_attr_id].modified == approx(90)
    # Action
    api_module.remove()
    # Verification
    assert api_rig.update().attrs[eve_affectee_attr_id].modified == approx(200)


def test_root_unloaded_offlist_to_loaded_onlist_remove(client, consts):
    (eve_affectee_attr_id,
     eve_loaded_onlist_id,
     _,
     _,
     eve_unloaded_offlist_id,
     api_fit2,
     api_module,
     api_rig) = setup_root_test(client=client, consts=consts)
    api_root = api_fit2.set_ship(type_id=eve_unloaded_offlist_id)
    api_module.change_module(add_projs=[api_root.id])
    # Verification
    assert api_rig.update().attrs[eve_affectee_attr_id].modified == approx(200)
    # Action
    api_root.change_ship(type_id=eve_loaded_onlist_id)
    # Verification
    assert api_rig.update().attrs[eve_affectee_attr_id].modified == approx(90)
    # Action
    api_module.remove()
    # Verification
    assert api_rig.update().attrs[eve_affectee_attr_id].modified == approx(200)


def test_child_rig(client, consts):
    eve_skill_id = client.mk_eve_item()
    eve_affector_attr_id = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_affectee_attr_id = client.mk_eve_attr()
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        loc_srq_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id, skill_id=eve_skill_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.doomsday_aoe_web, cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: -55},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_rig1_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 200}, srqs={eve_skill_id: 1})
    eve_rig2_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 100}, srqs={eve_skill_id: 1})
    eve_rig3_id = client.alloc_item_id()
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module = api_fit1.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_ship = api_fit2.set_ship(type_id=eve_ship_id)
    api_rig = api_fit2.add_rig(type_id=eve_rig1_id)
    api_module.change_module(add_projs=[api_ship.id])
    # Verification
    assert api_rig.update().attrs[eve_affectee_attr_id].modified == approx(90)
    # Action
    api_rig.change_rig(type_id=eve_rig2_id)
    # Verification
    assert api_rig.update().attrs[eve_affectee_attr_id].modified == approx(45)
    # Action
    api_rig.change_rig(type_id=eve_rig3_id)
    # Verification
    api_rig.update()
    with check_no_field():
        api_rig.attrs  # noqa: B018
    # Action
    api_rig.change_rig(type_id=eve_rig1_id)
    # Verification
    assert api_rig.update().attrs[eve_affectee_attr_id].modified == approx(90)
