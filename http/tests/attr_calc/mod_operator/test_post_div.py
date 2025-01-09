from tests import approx, check_no_field


def setup_penalization_test(*, client, consts, stackable):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr(stackable=stackable)
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_div,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_item_affector1_id = client.mk_eve_item(attrs={eve_affector_attr_id: 1.2}, eff_ids=[eve_effect_id])
    eve_item_affector2_id = client.mk_eve_item(attrs={eve_affector_attr_id: 1.5}, eff_ids=[eve_effect_id])
    eve_item_affector3_id = client.mk_eve_item(attrs={eve_affector_attr_id: 0.1}, eff_ids=[eve_effect_id])
    eve_item_affector4_id = client.mk_eve_item(attrs={eve_affector_attr_id: 0.75}, eff_ids=[eve_effect_id])
    eve_item_affector5_id = client.mk_eve_item(attrs={eve_affector_attr_id: 5}, eff_ids=[eve_effect_id])
    # Division by 1 is considered insignificant, and won't be exposed as modification
    eve_item_affector6_id = client.mk_eve_item(attrs={eve_affector_attr_id: 1}, eff_ids=[eve_effect_id])
    eve_item_affectee_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item_affector1 = api_fit.add_rig(type_id=eve_item_affector1_id)
    api_item_affector2 = api_fit.add_rig(type_id=eve_item_affector2_id)
    api_item_affector3 = api_fit.add_rig(type_id=eve_item_affector3_id)
    api_item_affector4 = api_fit.add_rig(type_id=eve_item_affector4_id)
    api_item_affector5 = api_fit.add_rig(type_id=eve_item_affector5_id)
    api_fit.add_rig(type_id=eve_item_affector6_id)
    api_item_affectee = api_fit.set_ship(type_id=eve_item_affectee_id)
    api_item_affectee.update()
    return (
        api_item_affectee.attrs[eve_affectee_attr_id].dogma,
        api_item_affectee.mods[eve_affectee_attr_id],
        api_item_affector1,
        api_item_affector2,
        api_item_affector3,
        api_item_affector4,
        api_item_affector5)


def test_non_penalized(client, consts):
    (attr_val,
     attr_mods,
     api_item_affector1,
     api_item_affector2,
     api_item_affector3,
     api_item_affector4,
     api_item_affector5) = setup_penalization_test(client=client, consts=consts, stackable=True)
    # Verification
    assert attr_val == approx(148.148148)
    assert len(attr_mods) == 5
    api_mod1 = attr_mods.find_by_affector_item(affector_item_id=api_item_affector1.id).one()
    assert api_mod1.op == consts.ApiModOp.post_div
    assert api_mod1.initial_val == approx(1.2)
    assert api_mod1.stacking_mult is None
    assert api_mod1.applied_val == approx(1.2)
    api_mod2 = attr_mods.find_by_affector_item(affector_item_id=api_item_affector2.id).one()
    assert api_mod2.op == consts.ApiModOp.post_div
    assert api_mod2.initial_val == approx(1.5)
    assert api_mod2.stacking_mult is None
    assert api_mod2.applied_val == approx(1.5)
    api_mod3 = attr_mods.find_by_affector_item(affector_item_id=api_item_affector3.id).one()
    assert api_mod3.op == consts.ApiModOp.post_div
    assert api_mod3.initial_val == approx(0.1)
    assert api_mod3.stacking_mult is None
    assert api_mod3.applied_val == approx(0.1)
    api_mod4 = attr_mods.find_by_affector_item(affector_item_id=api_item_affector4.id).one()
    assert api_mod4.op == consts.ApiModOp.post_div
    assert api_mod4.initial_val == approx(0.75)
    assert api_mod4.stacking_mult is None
    assert api_mod4.applied_val == approx(0.75)
    api_mod5 = attr_mods.find_by_affector_item(affector_item_id=api_item_affector5.id).one()
    assert api_mod5.op == consts.ApiModOp.post_div
    assert api_mod5.initial_val == approx(5)
    assert api_mod5.stacking_mult is None
    assert api_mod5.applied_val == approx(5)


def test_penalized(client, consts):
    (attr_val,
     attr_mods,
     api_item_affector1,
     api_item_affector2,
     api_item_affector3,
     api_item_affector4,
     api_item_affector5) = setup_penalization_test(client=client, consts=consts, stackable=False)
    # Verification
    assert attr_val == approx(165.790873)
    assert len(attr_mods) == 5
    api_mod1 = attr_mods.find_by_affector_item(affector_item_id=api_item_affector1.id).one()
    assert api_mod1.op == consts.ApiModOp.post_div
    assert api_mod1.initial_val == approx(1.2)
    assert api_mod1.stacking_mult == approx(consts.PenaltyStr.p3)
    assert api_mod1.applied_val == approx(1.105091)
    api_mod2 = attr_mods.find_by_affector_item(affector_item_id=api_item_affector2.id).one()
    assert api_mod2.op == consts.ApiModOp.post_div
    assert api_mod2.initial_val == approx(1.5)
    assert api_mod2.stacking_mult == approx(consts.PenaltyStr.p2)
    assert api_mod2.applied_val == approx(1.407869)
    api_mod3 = attr_mods.find_by_affector_item(affector_item_id=api_item_affector3.id).one()
    assert api_mod3.op == consts.ApiModOp.post_div
    assert api_mod3.initial_val == approx(0.1)
    assert api_mod3.stacking_mult == approx(consts.PenaltyStr.p1)
    assert api_mod3.applied_val == approx(0.1)
    api_mod4 = attr_mods.find_by_affector_item(affector_item_id=api_item_affector4.id).one()
    assert api_mod4.op == consts.ApiModOp.post_div
    assert api_mod4.initial_val == approx(0.75)
    assert api_mod4.stacking_mult == approx(consts.PenaltyStr.p2)
    assert api_mod4.applied_val == approx(0.7753701)
    api_mod5 = attr_mods.find_by_affector_item(affector_item_id=api_item_affector5.id).one()
    assert api_mod5.op == consts.ApiModOp.post_div
    assert api_mod5.initial_val == approx(5)
    assert api_mod5.stacking_mult == approx(consts.PenaltyStr.p1)
    assert api_mod5.applied_val == approx(5)


def test_deep_stacking(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr(stackable=False)
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_div,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_affector1_id = client.mk_eve_item(attrs={eve_affector_attr_id: 0.3}, eff_ids=[eve_effect_id])
    eve_affector2_id = client.mk_eve_item(attrs={eve_affector_attr_id: 0.31}, eff_ids=[eve_effect_id])
    eve_affector3_id = client.mk_eve_item(attrs={eve_affector_attr_id: 0.32}, eff_ids=[eve_effect_id])
    eve_affector4_id = client.mk_eve_item(attrs={eve_affector_attr_id: 0.33}, eff_ids=[eve_effect_id])
    eve_affector5_id = client.mk_eve_item(attrs={eve_affector_attr_id: 0.34}, eff_ids=[eve_effect_id])
    eve_affector6_id = client.mk_eve_item(attrs={eve_affector_attr_id: 0.35}, eff_ids=[eve_effect_id])
    eve_affector7_id = client.mk_eve_item(attrs={eve_affector_attr_id: 0.36}, eff_ids=[eve_effect_id])
    eve_affector8_id = client.mk_eve_item(attrs={eve_affector_attr_id: 0.37}, eff_ids=[eve_effect_id])
    eve_affector9_id = client.mk_eve_item(attrs={eve_affector_attr_id: 0.38}, eff_ids=[eve_effect_id])
    eve_affector10_id = client.mk_eve_item(attrs={eve_affector_attr_id: 0.39}, eff_ids=[eve_effect_id])
    eve_affector11_id = client.mk_eve_item(attrs={eve_affector_attr_id: 0.4}, eff_ids=[eve_effect_id])
    eve_affector12_id = client.mk_eve_item(attrs={eve_affector_attr_id: 0.41}, eff_ids=[eve_effect_id])
    eve_affectee_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_affector1 = api_fit.add_rig(type_id=eve_affector1_id)
    api_affector2 = api_fit.add_rig(type_id=eve_affector2_id)
    api_affector3 = api_fit.add_rig(type_id=eve_affector3_id)
    api_affector4 = api_fit.add_rig(type_id=eve_affector4_id)
    api_affector5 = api_fit.add_rig(type_id=eve_affector5_id)
    api_affector6 = api_fit.add_rig(type_id=eve_affector6_id)
    api_affector7 = api_fit.add_rig(type_id=eve_affector7_id)
    api_affector8 = api_fit.add_rig(type_id=eve_affector8_id)
    api_affector9 = api_fit.add_rig(type_id=eve_affector9_id)
    api_affector10 = api_fit.add_rig(type_id=eve_affector10_id)
    api_affector11 = api_fit.add_rig(type_id=eve_affector11_id)
    api_fit.add_rig(type_id=eve_affector12_id)
    api_affectee = api_fit.set_ship(type_id=eve_affectee_id)
    # Verification
    api_affectee.update()
    assert api_affectee.attrs[eve_affectee_attr_id].dogma == approx(4395.538972)
    api_mods = api_affectee.mods[eve_affectee_attr_id]
    # 12th affector is completely ignored both in calculation process and for modification listing
    assert len(api_mods) == 11
    api_mod1 = api_mods.find_by_affector_item(affector_item_id=api_affector1.id).one()
    assert api_mod1.op == consts.ApiModOp.post_div
    assert api_mod1.initial_val == approx(0.3)
    assert api_mod1.stacking_mult == approx(consts.PenaltyStr.p1)
    assert api_mod1.applied_val == approx(0.3)
    api_mod2 = api_mods.find_by_affector_item(affector_item_id=api_affector2.id).one()
    assert api_mod2.op == consts.ApiModOp.post_div
    assert api_mod2.initial_val == approx(0.31)
    assert api_mod2.stacking_mult == approx(consts.PenaltyStr.p2)
    assert api_mod2.applied_val == approx(0.3407744)
    api_mod3 = api_mods.find_by_affector_item(affector_item_id=api_affector3.id).one()
    assert api_mod3.op == consts.ApiModOp.post_div
    assert api_mod3.initial_val == approx(0.32)
    assert api_mod3.stacking_mult == approx(consts.PenaltyStr.p3)
    assert api_mod3.applied_val == approx(0.4519796)
    api_mod4 = api_mods.find_by_affector_item(affector_item_id=api_affector4.id).one()
    assert api_mod4.op == consts.ApiModOp.post_div
    assert api_mod4.initial_val == approx(0.33)
    assert api_mod4.stacking_mult == approx(consts.PenaltyStr.p4)
    assert api_mod4.applied_val == approx(0.6351284)
    api_mod5 = api_mods.find_by_affector_item(affector_item_id=api_affector5.id).one()
    assert api_mod5.op == consts.ApiModOp.post_div
    assert api_mod5.initial_val == approx(0.34)
    assert api_mod5.stacking_mult == approx(consts.PenaltyStr.p5)
    assert api_mod5.applied_val == approx(0.829359)
    api_mod6 = api_mods.find_by_affector_item(affector_item_id=api_affector6.id).one()
    assert api_mod6.op == consts.ApiModOp.post_div
    assert api_mod6.initial_val == approx(0.35)
    assert api_mod6.stacking_mult == approx(consts.PenaltyStr.p6)
    assert api_mod6.applied_val == approx(0.9472407)
    api_mod7 = api_mods.find_by_affector_item(affector_item_id=api_affector7.id).one()
    assert api_mod7.op == consts.ApiModOp.post_div
    assert api_mod7.initial_val == approx(0.36)
    assert api_mod7.stacking_mult == approx(consts.PenaltyStr.p7)
    assert api_mod7.applied_val == approx(0.9887325)
    api_mod8 = api_mods.find_by_affector_item(affector_item_id=api_affector8.id).one()
    assert api_mod8.op == consts.ApiModOp.post_div
    assert api_mod8.initial_val == approx(0.37)
    assert api_mod8.stacking_mult == approx(consts.PenaltyStr.p8)
    assert api_mod8.applied_val == approx(0.9982409)
    api_mod9 = api_mods.find_by_affector_item(affector_item_id=api_affector9.id).one()
    assert api_mod9.op == consts.ApiModOp.post_div
    assert api_mod9.initial_val == approx(0.38)
    assert api_mod9.stacking_mult == approx(consts.PenaltyStr.p9)
    assert api_mod9.applied_val == approx(0.9997941)
    api_mod10 = api_mods.find_by_affector_item(affector_item_id=api_affector10.id).one()
    assert api_mod10.op == consts.ApiModOp.post_div
    assert api_mod10.initial_val == approx(0.39)
    assert api_mod10.stacking_mult == approx(consts.PenaltyStr.p10)
    assert api_mod10.applied_val == approx(0.9999818)
    api_mod11 = api_mods.find_by_affector_item(affector_item_id=api_affector11.id).one()
    assert api_mod11.op == consts.ApiModOp.post_div
    assert api_mod11.initial_val == approx(0.40)
    assert api_mod11.stacking_mult == approx(consts.PenaltyStr.p11)
    assert api_mod11.applied_val == approx(0.9999988)


def test_insignificant_stacking(client, consts):
    # Here we check what happens if final result of stacking penalty chain is "neutral", its
    # modifications are not filtered out
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr(stackable=False)
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_div,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_affector1_id = client.mk_eve_item(attrs={eve_affector_attr_id: 0.5}, eff_ids=[eve_effect_id])
    eve_affector2_id = client.mk_eve_item(attrs={eve_affector_attr_id: 2}, eff_ids=[eve_effect_id])
    eve_affectee_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_affector1 = api_fit.add_rig(type_id=eve_affector1_id)
    api_affector2 = api_fit.add_rig(type_id=eve_affector2_id)
    api_affectee = api_fit.set_ship(type_id=eve_affectee_id)
    # Verification
    api_affectee.update()
    assert api_affectee.attrs[eve_affectee_attr_id].dogma == approx(100)
    api_mods = api_affectee.mods[eve_affectee_attr_id]
    assert len(api_mods) == 2
    api_mod1 = api_mods.find_by_affector_item(affector_item_id=api_affector1.id).one()
    assert api_mod1.op == consts.ApiModOp.post_div
    assert api_mod1.initial_val == approx(0.5)
    assert api_mod1.stacking_mult == approx(consts.PenaltyStr.p1)
    assert api_mod1.applied_val == approx(0.5)
    api_mod2 = api_mods.find_by_affector_item(affector_item_id=api_affector2.id).one()
    assert api_mod2.op == consts.ApiModOp.post_div
    assert api_mod2.initial_val == approx(2)
    assert api_mod2.stacking_mult == approx(consts.PenaltyStr.p1)
    assert api_mod2.applied_val == approx(2)


def test_insignificant_base(client, consts):
    # When value on top of which modifications should be applied is 0, all divisions are
    # insignificant and are not exposed as modifications
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_div,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_affector_id = client.mk_eve_item(attrs={eve_affector_attr_id: 4}, eff_ids=[eve_effect_id])
    eve_affectee_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_rig(type_id=eve_affector_id)
    api_affectee = api_fit.set_ship(type_id=eve_affectee_id)
    # Verification
    api_affectee.update()
    assert api_affectee.attrs[eve_affectee_attr_id].dogma == approx(0)
    with check_no_field():
        api_affectee.mods  # pylint: disable=W0104


def test_insignificant_modified_base(client, consts):
    # When value on top of which modifications should be applied is 0, all divisions are
    # insignificant and are not exposed as modifications
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod1 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect1_id = client.mk_eve_effect(mod_info=[eve_mod1])
    eve_affector1_id = client.mk_eve_item(attrs={eve_affector_attr_id: 0}, eff_ids=[eve_effect1_id])
    eve_mod2 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_div,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect2_id = client.mk_eve_effect(mod_info=[eve_mod2])
    eve_affector2_id = client.mk_eve_item(attrs={eve_affector_attr_id: 4}, eff_ids=[eve_effect2_id])
    eve_affectee_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_rig(type_id=eve_affector1_id)
    api_fit.add_rig(type_id=eve_affector2_id)
    api_affectee = api_fit.set_ship(type_id=eve_affectee_id)
    # Verification
    api_affectee.update()
    assert api_affectee.attrs[eve_affectee_attr_id].dogma == approx(0)
    api_mod = api_affectee.mods[eve_affectee_attr_id].one()
    assert api_mod.op == consts.ApiModOp.post_mul
    assert api_mod.initial_val == approx(0)
    assert api_mod.stacking_mult is None
    assert api_mod.applied_val == approx(0)


def test_zero(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_div,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_affector_id = client.mk_eve_item(attrs={eve_affector_attr_id: 0}, eff_ids=[eve_effect_id])
    eve_affectee_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_rig(type_id=eve_affector_id)
    api_affectee = api_fit.set_ship(type_id=eve_affectee_id)
    # Verification
    api_affectee.update()
    assert api_affectee.attrs[eve_affectee_attr_id].dogma == approx(100)
    with check_no_field():
        api_affectee.mods  # pylint: disable=W0104
