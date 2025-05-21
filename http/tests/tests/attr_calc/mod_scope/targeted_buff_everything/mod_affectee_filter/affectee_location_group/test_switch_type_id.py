from tests import approx


def test_root_to_unaffected(client, consts):
    eve_grp_id = client.mk_eve_item_group()
    eve_affector_attr_id = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_affectee_attr_id = client.mk_eve_attr()
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        loc_grp_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id, group_id=eve_grp_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.doomsday_aoe_web, cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: -55},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_rig_id = client.mk_eve_item(grp_id=eve_grp_id, attrs={eve_affectee_attr_id: 200})
    eve_root1_id = client.mk_eve_ship()
    eve_root2_id = client.mk_eve_struct()
    eve_root3_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module = api_fit1.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_root = api_fit2.set_ship(type_id=eve_root1_id)
    api_rig = api_fit2.add_rig(type_id=eve_rig_id)
    api_module.change_module(add_projs=[api_root.id])
    # Verification
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(90)
    # Action
    api_root.change_ship(type_id=eve_root2_id)
    # Verification
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(200)
    # Action
    api_root.change_ship(type_id=eve_root1_id)
    # Verification
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(90)
    # Action
    api_root.change_ship(type_id=eve_root3_id)
    # Verification
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(200)
    # Action
    api_root.change_ship(type_id=eve_root1_id)
    # Verification
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(90)
