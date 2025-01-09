from tests import approx, check_no_field


def setup_penalization_test(*, client, consts, stackable):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr(stackable=stackable)
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_item_affector1_id = client.mk_eve_item(attrs={eve_affector_attr_id: 1.2}, eff_ids=[eve_effect_id])
    eve_item_affector2_id = client.mk_eve_item(attrs={eve_affector_attr_id: 1.5}, eff_ids=[eve_effect_id])
    eve_item_affector3_id = client.mk_eve_item(attrs={eve_affector_attr_id: 0.1}, eff_ids=[eve_effect_id])
    eve_item_affector4_id = client.mk_eve_item(attrs={eve_affector_attr_id: 0.75}, eff_ids=[eve_effect_id])
    eve_item_affector5_id = client.mk_eve_item(attrs={eve_affector_attr_id: 5}, eff_ids=[eve_effect_id])
    # Multiplication by 1 is considered insignificant, and won't be exposed as modification
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
    assert attr_val == approx(67.5)
    assert len(attr_mods) == 5
    api_mod1 = attr_mods.find_by_affector_item(affector_item_id=api_item_affector1.id).one()
    assert api_mod1.op == consts.ApiModOp.post_mul
    assert api_mod1.initial_val == approx(1.2)
    assert api_mod1.stacking_mult is None
    assert api_mod1.applied_val == approx(1.2)
    api_mod2 = attr_mods.find_by_affector_item(affector_item_id=api_item_affector2.id).one()
    assert api_mod2.op == consts.ApiModOp.post_mul
    assert api_mod2.initial_val == approx(1.5)
    assert api_mod2.stacking_mult is None
    assert api_mod2.applied_val == approx(1.5)
    api_mod3 = attr_mods.find_by_affector_item(affector_item_id=api_item_affector3.id).one()
    assert api_mod3.op == consts.ApiModOp.post_mul
    assert api_mod3.initial_val == approx(0.1)
    assert api_mod3.stacking_mult is None
    assert api_mod3.applied_val == approx(0.1)
    api_mod4 = attr_mods.find_by_affector_item(affector_item_id=api_item_affector4.id).one()
    assert api_mod4.op == consts.ApiModOp.post_mul
    assert api_mod4.initial_val == approx(0.75)
    assert api_mod4.stacking_mult is None
    assert api_mod4.applied_val == approx(0.75)
    api_mod5 = attr_mods.find_by_affector_item(affector_item_id=api_item_affector5.id).one()
    assert api_mod5.op == consts.ApiModOp.post_mul
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
    assert attr_val == approx(62.549783)
    assert len(attr_mods) == 5
    api_mod1 = attr_mods.find_by_affector_item(affector_item_id=api_item_affector1.id).one()
    assert api_mod1.op == consts.ApiModOp.post_mul
    assert api_mod1.initial_val == approx(1.2)
    assert api_mod1.stacking_mult == approx(consts.PenaltyStr.p3)
    assert api_mod1.applied_val == approx(1.114116)
    api_mod2 = attr_mods.find_by_affector_item(affector_item_id=api_item_affector2.id).one()
    assert api_mod2.op == consts.ApiModOp.post_mul
    assert api_mod2.initial_val == approx(1.5)
    assert api_mod2.stacking_mult == approx(consts.PenaltyStr.p2)
    assert api_mod2.applied_val == approx(1.43456)
    api_mod3 = attr_mods.find_by_affector_item(affector_item_id=api_item_affector3.id).one()
    assert api_mod3.op == consts.ApiModOp.post_mul
    assert api_mod3.initial_val == approx(0.1)
    assert api_mod3.stacking_mult == approx(consts.PenaltyStr.p1)
    assert api_mod3.applied_val == approx(0.1)
    api_mod4 = attr_mods.find_by_affector_item(affector_item_id=api_item_affector4.id).one()
    assert api_mod4.op == consts.ApiModOp.post_mul
    assert api_mod4.initial_val == approx(0.75)
    assert api_mod4.stacking_mult == approx(consts.PenaltyStr.p2)
    assert api_mod4.applied_val == approx(0.78272)
    api_mod5 = attr_mods.find_by_affector_item(affector_item_id=api_item_affector5.id).one()
    assert api_mod5.op == consts.ApiModOp.post_mul
    assert api_mod5.initial_val == approx(5)
    assert api_mod5.stacking_mult == approx(consts.PenaltyStr.p1)
    assert api_mod5.applied_val == approx(5)


def test_deep_stacking(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr(stackable=False)
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_affector1_id = client.mk_eve_item(attrs={eve_affector_attr_id: 1.5}, eff_ids=[eve_effect_id])
    eve_affector2_id = client.mk_eve_item(attrs={eve_affector_attr_id: 1.49}, eff_ids=[eve_effect_id])
    eve_affector3_id = client.mk_eve_item(attrs={eve_affector_attr_id: 1.48}, eff_ids=[eve_effect_id])
    eve_affector4_id = client.mk_eve_item(attrs={eve_affector_attr_id: 1.47}, eff_ids=[eve_effect_id])
    eve_affector5_id = client.mk_eve_item(attrs={eve_affector_attr_id: 1.46}, eff_ids=[eve_effect_id])
    eve_affector6_id = client.mk_eve_item(attrs={eve_affector_attr_id: 1.45}, eff_ids=[eve_effect_id])
    eve_affector7_id = client.mk_eve_item(attrs={eve_affector_attr_id: 1.44}, eff_ids=[eve_effect_id])
    eve_affector8_id = client.mk_eve_item(attrs={eve_affector_attr_id: 1.43}, eff_ids=[eve_effect_id])
    eve_affector9_id = client.mk_eve_item(attrs={eve_affector_attr_id: 1.42}, eff_ids=[eve_effect_id])
    eve_affector10_id = client.mk_eve_item(attrs={eve_affector_attr_id: 1.41}, eff_ids=[eve_effect_id])
    eve_affector11_id = client.mk_eve_item(attrs={eve_affector_attr_id: 1.40}, eff_ids=[eve_effect_id])
    eve_affector12_id = client.mk_eve_item(attrs={eve_affector_attr_id: 1.39}, eff_ids=[eve_effect_id])
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
    assert api_affectee.attrs[eve_affectee_attr_id].dogma == approx(329.202701)
    api_mods = api_affectee.mods[eve_affectee_attr_id]
    # 12th affector is completely ignored both in calculation process and for modification listing
    assert len(api_mods) == 11
    api_mod1 = api_mods.find_by_affector_item(affector_item_id=api_affector1.id).one()
    assert api_mod1.op == consts.ApiModOp.post_mul
    assert api_mod1.initial_val == approx(1.5)
    assert api_mod1.stacking_mult == approx(consts.PenaltyStr.p1)
    assert api_mod1.applied_val == approx(1.5)
    api_mod2 = api_mods.find_by_affector_item(affector_item_id=api_affector2.id).one()
    assert api_mod2.op == consts.ApiModOp.post_mul
    assert api_mod2.initial_val == approx(1.49)
    assert api_mod2.stacking_mult == approx(consts.PenaltyStr.p2)
    assert api_mod2.applied_val == approx(1.42586879)
    api_mod3 = api_mods.find_by_affector_item(affector_item_id=api_affector3.id).one()
    assert api_mod3.op == consts.ApiModOp.post_mul
    assert api_mod3.initial_val == approx(1.48)
    assert api_mod3.stacking_mult == approx(consts.PenaltyStr.p3)
    assert api_mod3.applied_val == approx(1.27387991)
    api_mod4 = api_mods.find_by_affector_item(affector_item_id=api_affector4.id).one()
    assert api_mod4.op == consts.ApiModOp.post_mul
    assert api_mod4.initial_val == approx(1.47)
    assert api_mod4.stacking_mult == approx(consts.PenaltyStr.p4)
    assert api_mod4.applied_val == approx(1.13298892)
    api_mod5 = api_mods.find_by_affector_item(affector_item_id=api_affector5.id).one()
    assert api_mod5.op == consts.ApiModOp.post_mul
    assert api_mod5.initial_val == approx(1.46)
    assert api_mod5.stacking_mult == approx(consts.PenaltyStr.p5)
    assert api_mod5.applied_val == approx(1.04875662)
    api_mod6 = api_mods.find_by_affector_item(affector_item_id=api_affector6.id).one()
    assert api_mod6.op == consts.ApiModOp.post_mul
    assert api_mod6.initial_val == approx(1.45)
    assert api_mod6.stacking_mult == approx(consts.PenaltyStr.p6)
    assert api_mod6.applied_val == approx(1.01349602)
    api_mod7 = api_mods.find_by_affector_item(affector_item_id=api_affector7.id).one()
    assert api_mod7.op == consts.ApiModOp.post_mul
    assert api_mod7.initial_val == approx(1.44)
    assert api_mod7.stacking_mult == approx(consts.PenaltyStr.p7)
    assert api_mod7.applied_val == approx(1.002820481)
    api_mod8 = api_mods.find_by_affector_item(affector_item_id=api_affector8.id).one()
    assert api_mod8.op == consts.ApiModOp.post_mul
    assert api_mod8.initial_val == approx(1.43)
    assert api_mod8.stacking_mult == approx(consts.PenaltyStr.p8)
    assert api_mod8.applied_val == approx(1.0004450158)
    api_mod9 = api_mods.find_by_affector_item(affector_item_id=api_affector9.id).one()
    assert api_mod9.op == consts.ApiModOp.post_mul
    assert api_mod9.initial_val == approx(1.42)
    assert api_mod9.stacking_mult == approx(consts.PenaltyStr.p9)
    assert api_mod9.applied_val == approx(1.00005300933)
    api_mod10 = api_mods.find_by_affector_item(affector_item_id=api_affector10.id).one()
    assert api_mod10.op == consts.ApiModOp.post_mul
    assert api_mod10.initial_val == approx(1.41)
    assert api_mod10.stacking_mult == approx(consts.PenaltyStr.p10)
    assert api_mod10.applied_val == approx(1.000004766969)
    api_mod11 = api_mods.find_by_affector_item(affector_item_id=api_affector11.id).one()
    assert api_mod11.op == consts.ApiModOp.post_mul
    assert api_mod11.initial_val == approx(1.40)
    assert api_mod11.stacking_mult == approx(consts.PenaltyStr.p11)
    assert api_mod11.applied_val == approx(1.0000003236185)


def test_insignificant_stacking(client, consts):
    # Here we check what happens if final result of stacking penalty chain is "neutral", its
    # modifications are not filtered out
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr(stackable=False)
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_mul,
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
    assert api_mod1.op == consts.ApiModOp.post_mul
    assert api_mod1.initial_val == approx(0.5)
    assert api_mod1.stacking_mult == approx(consts.PenaltyStr.p1)
    assert api_mod1.applied_val == approx(0.5)
    api_mod2 = api_mods.find_by_affector_item(affector_item_id=api_affector2.id).one()
    assert api_mod2.op == consts.ApiModOp.post_mul
    assert api_mod2.initial_val == approx(2)
    assert api_mod2.stacking_mult == approx(consts.PenaltyStr.p1)
    assert api_mod2.applied_val == approx(2)


def test_insignificant_base(client, consts):
    # When value on top of which modifications should be applied is 0, all multiplications are
    # insignificant and are not exposed as modifications
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_mul,
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
    # When value on top of which modifications should be applied is 0, all multiplications are
    # insignificant and are not exposed as modifications
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod1 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.mod_sub,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect1_id = client.mk_eve_effect(mod_info=[eve_mod1])
    eve_affector1_id = client.mk_eve_item(attrs={eve_affector_attr_id: 100}, eff_ids=[eve_effect1_id])
    eve_mod2 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_mul,
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
    assert api_mod.op == consts.ApiModOp.mod_sub
    assert api_mod.initial_val == approx(100)
    assert api_mod.stacking_mult is None
    assert api_mod.applied_val == approx(100)


def test_insignificant_earlier_ops(client, consts):
    # When a value is multiplied by 0, all earlier modifications are insignificant
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr(stackable=False)
    # Pre-assignment
    val_pre_ass = 5
    eve_mod_pre_ass = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.pre_assign,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_pre_ass_id = client.mk_eve_effect(mod_info=[eve_mod_pre_ass])
    eve_affector_pre_ass_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: val_pre_ass},
        eff_ids=[eve_effect_pre_ass_id])
    # Pre-multiplication
    val_pre_mul = 50
    eve_mod_pre_mul = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.pre_mul,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_pre_mul_id = client.mk_eve_effect(mod_info=[eve_mod_pre_mul])
    eve_affector_pre_mul_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: val_pre_mul},
        eff_ids=[eve_effect_pre_mul_id])
    # Pre-division
    val_pre_div = 0.5
    eve_mod_pre_div = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.pre_div,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_pre_div_id = client.mk_eve_effect(mod_info=[eve_mod_pre_div])
    eve_affector_pre_div_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: val_pre_div},
        eff_ids=[eve_effect_pre_div_id])
    # Addition
    val_mod_add = 10
    eve_mod_mod_add = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_mod_add_id = client.mk_eve_effect(mod_info=[eve_mod_mod_add])
    eve_affector_mod_add_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: val_mod_add},
        eff_ids=[eve_effect_mod_add_id])
    # Subtraction
    val_mod_sub = 63
    eve_mod_mod_sub = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.mod_sub,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_mod_sub_id = client.mk_eve_effect(mod_info=[eve_mod_mod_sub])
    eve_affector_mod_sub_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: val_mod_sub},
        eff_ids=[eve_effect_mod_sub_id])
    # Post-multiplication
    val_post_mul = 0
    eve_mod_post_mul = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_post_mul_id = client.mk_eve_effect(mod_info=[eve_mod_post_mul])
    eve_affector_post_mul_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: val_post_mul},
        eff_ids=[eve_effect_post_mul_id])
    # Post-division
    val_post_div = 2.7
    eve_mod_post_div = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_div,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_post_div_id = client.mk_eve_effect(mod_info=[eve_mod_post_div])
    eve_affector_post_div_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: val_post_div},
        eff_ids=[eve_effect_post_div_id])
    # Post-percent
    val_post_perc = 15
    eve_mod_post_perc = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_post_perc_id = client.mk_eve_effect(mod_info=[eve_mod_post_perc])
    eve_affector_post_perc_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: val_post_perc},
        eff_ids=[eve_effect_post_perc_id])
    eve_affectee_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_rig(type_id=eve_affector_pre_ass_id)
    api_fit.add_rig(type_id=eve_affector_pre_mul_id)
    api_fit.add_rig(type_id=eve_affector_pre_div_id)
    api_fit.add_rig(type_id=eve_affector_mod_add_id)
    api_fit.add_rig(type_id=eve_affector_mod_sub_id)
    api_fit.add_rig(type_id=eve_affector_post_mul_id)
    api_fit.add_rig(type_id=eve_affector_post_div_id)
    api_fit.add_rig(type_id=eve_affector_post_perc_id)
    api_affectee = api_fit.set_ship(type_id=eve_affectee_id)
    # Verification - post-multiplication by 0 should remove earlier modifiers, i.e. pre-assignment,
    # pre-division, addition and subtraction modifiers. It also removes post-division and
    # post-percent modifiers as a side effect (doesn't matter what right hand value is when left
    # hand value is 0), but this is not focus of the test.
    api_affectee.update()
    expected_value = (
            (val_pre_ass * val_pre_mul / val_pre_div + val_mod_add - val_mod_sub)
            * val_post_mul / val_post_div * (1 + val_post_perc / 100))
    assert api_affectee.attrs[eve_affectee_attr_id].dogma == approx(expected_value)
    api_mods = api_affectee.mods[eve_affectee_attr_id]
    assert len(api_mods) == 1
    api_post_mul_mod = api_mods.find_by_op(op=consts.ApiModOp.post_mul).one()
    assert api_post_mul_mod.initial_val == approx(val_post_mul)
    assert api_post_mul_mod.stacking_mult == approx(consts.PenaltyStr.p1)
    assert api_post_mul_mod.applied_val == approx(val_post_mul)


def setup_insignificant_chain_values_test(*, client, consts, stackable):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr(stackable=stackable)
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_affector1_id = client.mk_eve_item(attrs={eve_affector_attr_id: 0}, eff_ids=[eve_effect_id])
    eve_affector2_id = client.mk_eve_item(attrs={eve_affector_attr_id: 4}, eff_ids=[eve_effect_id])
    eve_affector3_id = client.mk_eve_item(attrs={eve_affector_attr_id: 0}, eff_ids=[eve_effect_id])
    eve_affector4_id = client.mk_eve_item(attrs={eve_affector_attr_id: 0.4}, eff_ids=[eve_effect_id])
    eve_affectee_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_affector1 = api_fit.add_rig(type_id=eve_affector1_id)
    api_fit.add_rig(type_id=eve_affector2_id)
    api_affector3 = api_fit.add_rig(type_id=eve_affector3_id)
    api_fit.add_rig(type_id=eve_affector4_id)
    api_affectee = api_fit.set_ship(type_id=eve_affectee_id)
    api_affectee.update()
    return (
        api_affectee.attrs[eve_affectee_attr_id].dogma,
        api_affectee.mods[eve_affectee_attr_id],
        api_affector1,
        api_affector3)


def test_insignificant_chain_values_non_penalized(client, consts):
    # When some values in chain result in final value of 0, only they should be exposed
    (attr_val,
     api_mods,
     api_affector1,
     api_affector3) = setup_insignificant_chain_values_test(client=client, consts=consts, stackable=True)
    # Verification
    assert attr_val == approx(0)
    # Without stacking penalty, all modifications which multiply by 0 are returned
    assert len(api_mods) == 2
    api_mod1 = api_mods.find_by_affector_item(affector_item_id=api_affector1.id).one()
    assert api_mod1.op == consts.ApiModOp.post_mul
    assert api_mod1.initial_val == approx(0)
    assert api_mod1.stacking_mult is None
    assert api_mod1.applied_val == approx(0)
    api_mod3 = api_mods.find_by_affector_item(affector_item_id=api_affector3.id).one()
    assert api_mod3.op == consts.ApiModOp.post_mul
    assert api_mod3.initial_val == approx(0)
    assert api_mod3.stacking_mult is None
    assert api_mod3.applied_val == approx(0)


def test_insignificant_chain_values_penalized(client, consts):
    # When some values in chain result in final value of 0, only they should be exposed
    (attr_val,
     api_mods,
     api_affector1,
     api_affector3) = setup_insignificant_chain_values_test(client=client, consts=consts, stackable=False)
    # Verification
    assert attr_val == approx(0)
    # With stacking penalty, only one of modifications is returned, while other one is getting
    # stacking penalized, thus making its final multiplier different from 0
    api_mod = api_mods.one()
    assert api_mod.op == consts.ApiModOp.post_mul
    assert api_mod.initial_val == approx(0)
    assert api_mod.stacking_mult == approx(consts.PenaltyStr.p1)
    assert api_mod.applied_val == approx(0)
    # Since both affector1 and affector3 can be exposed as significant modification, check for ID
    # of either
    assert api_mod.affectors.one().item_id in (api_affector1.id, api_affector3.id)
