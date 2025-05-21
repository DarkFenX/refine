from tests import approx


def test_root_to_unaffected(client, consts):
    eve_skill_id = client.mk_eve_item()
    eve_attr1_id = client.mk_eve_attr()
    eve_attr2_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_srq,
        loc=consts.EveModLoc.tgt,
        srq=eve_skill_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_attr1_id,
        affectee_attr_id=eve_attr2_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module_id = client.mk_eve_item(attrs={eve_attr1_id: 20}, eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_rig_id = client.mk_eve_item(attrs={eve_attr2_id: 80}, srqs={eve_skill_id: 1})
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
    assert api_rig.update().attrs[eve_attr2_id].dogma == approx(96)
    # Action
    api_root.change_ship(type_id=eve_root2_id)
    # Verification
    assert api_rig.update().attrs[eve_attr2_id].dogma == approx(80)
    # Action
    api_root.change_ship(type_id=eve_root1_id)
    # Verification
    assert api_rig.update().attrs[eve_attr2_id].dogma == approx(96)
    # Action
    api_root.change_ship(type_id=eve_root3_id)
    # Verification
    assert api_rig.update().attrs[eve_attr2_id].dogma == approx(80)
    # Action
    api_root.change_ship(type_id=eve_root1_id)
    # Verification
    assert api_rig.update().attrs[eve_attr2_id].dogma == approx(96)
