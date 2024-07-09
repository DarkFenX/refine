from pytest import approx


def setup_penalization_test(client, consts, stackable):
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr(stackable=stackable)
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.pre_mul,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(mod_info=[eve_mod])
    eve_item_affector1 = client.mk_eve_item(attrs={eve_affector_attr.id: 1.2}, eff_ids=[eve_effect.id])
    eve_item_affector2 = client.mk_eve_item(attrs={eve_affector_attr.id: 1.5}, eff_ids=[eve_effect.id])
    eve_item_affector3 = client.mk_eve_item(attrs={eve_affector_attr.id: 0.1}, eff_ids=[eve_effect.id])
    eve_item_affector4 = client.mk_eve_item(attrs={eve_affector_attr.id: 0.75}, eff_ids=[eve_effect.id])
    eve_item_affector5 = client.mk_eve_item(attrs={eve_affector_attr.id: 5}, eff_ids=[eve_effect.id])
    # Multiplication by 1 is considered insignificant, and won't be exposed as modification
    eve_item_affector6 = client.mk_eve_item(attrs={eve_affector_attr.id: 1}, eff_ids=[eve_effect.id])
    eve_item_affectee = client.mk_eve_ship(attrs={eve_affectee_attr.id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item_affector1 = api_fit.add_rig(type_id=eve_item_affector1.id)
    api_item_affector2 = api_fit.add_rig(type_id=eve_item_affector2.id)
    api_item_affector3 = api_fit.add_rig(type_id=eve_item_affector3.id)
    api_item_affector4 = api_fit.add_rig(type_id=eve_item_affector4.id)
    api_item_affector5 = api_fit.add_rig(type_id=eve_item_affector5.id)
    api_fit.add_rig(type_id=eve_item_affector6.id)
    api_item_affectee = api_fit.set_ship(type_id=eve_item_affectee.id)
    api_item_affectee.update()
    return (
        api_item_affectee.attrs[eve_affectee_attr.id].dogma,
        api_item_affectee.mods[eve_affectee_attr.id],
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
     api_item_affector5) = setup_penalization_test(client, consts, stackable=True)
    assert attr_val == approx(67.5)
    assert len(attr_mods) == 5
    api_mod1 = attr_mods.find_by_affector_item(affector_item_id=api_item_affector1.id).one()
    assert api_mod1.op == consts.ApiModOp.pre_mul
    assert api_mod1.initial_val == approx(1.2)
    assert api_mod1.stacking_mult is None
    assert api_mod1.applied_val == approx(1.2)
    api_mod2 = attr_mods.find_by_affector_item(affector_item_id=api_item_affector2.id).one()
    assert api_mod2.op == consts.ApiModOp.pre_mul
    assert api_mod2.initial_val == approx(1.5)
    assert api_mod2.stacking_mult is None
    assert api_mod2.applied_val == approx(1.5)
    api_mod3 = attr_mods.find_by_affector_item(affector_item_id=api_item_affector3.id).one()
    assert api_mod3.op == consts.ApiModOp.pre_mul
    assert api_mod3.initial_val == approx(0.1)
    assert api_mod3.stacking_mult is None
    assert api_mod3.applied_val == approx(0.1)
    api_mod4 = attr_mods.find_by_affector_item(affector_item_id=api_item_affector4.id).one()
    assert api_mod4.op == consts.ApiModOp.pre_mul
    assert api_mod4.initial_val == approx(0.75)
    assert api_mod4.stacking_mult is None
    assert api_mod4.applied_val == approx(0.75)
    api_mod5 = attr_mods.find_by_affector_item(affector_item_id=api_item_affector5.id).one()
    assert api_mod5.op == consts.ApiModOp.pre_mul
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
     api_item_affector5) = setup_penalization_test(client, consts, stackable=False)
    assert attr_val == approx(62.549783)
    assert len(attr_mods) == 5
    api_mod1 = attr_mods.find_by_affector_item(affector_item_id=api_item_affector1.id).one()
    assert api_mod1.op == consts.ApiModOp.pre_mul
    assert api_mod1.initial_val == approx(1.2)
    assert api_mod1.stacking_mult == approx(consts.PenaltyStr.p3)
    assert api_mod1.applied_val == approx(1.114116)
    api_mod2 = attr_mods.find_by_affector_item(affector_item_id=api_item_affector2.id).one()
    assert api_mod2.op == consts.ApiModOp.pre_mul
    assert api_mod2.initial_val == approx(1.5)
    assert api_mod2.stacking_mult == approx(consts.PenaltyStr.p2)
    assert api_mod2.applied_val == approx(1.43456)
    api_mod3 = attr_mods.find_by_affector_item(affector_item_id=api_item_affector3.id).one()
    assert api_mod3.op == consts.ApiModOp.pre_mul
    assert api_mod3.initial_val == approx(0.1)
    assert api_mod3.stacking_mult == approx(consts.PenaltyStr.p1)
    assert api_mod3.applied_val == approx(0.1)
    api_mod4 = attr_mods.find_by_affector_item(affector_item_id=api_item_affector4.id).one()
    assert api_mod4.op == consts.ApiModOp.pre_mul
    assert api_mod4.initial_val == approx(0.75)
    assert api_mod4.stacking_mult == approx(consts.PenaltyStr.p2)
    assert api_mod4.applied_val == approx(0.78272)
    api_mod5 = attr_mods.find_by_affector_item(affector_item_id=api_item_affector5.id).one()
    assert api_mod5.op == consts.ApiModOp.pre_mul
    assert api_mod5.initial_val == approx(5)
    assert api_mod5.stacking_mult == approx(consts.PenaltyStr.p1)
    assert api_mod5.applied_val == approx(5)


def test_deep_stacking(client, consts):
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr(stackable=False)
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.pre_mul,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(mod_info=[eve_mod])
    eve_affector1 = client.mk_eve_item(attrs={eve_affector_attr.id: 1.5}, eff_ids=[eve_effect.id])
    eve_affector2 = client.mk_eve_item(attrs={eve_affector_attr.id: 1.49}, eff_ids=[eve_effect.id])
    eve_affector3 = client.mk_eve_item(attrs={eve_affector_attr.id: 1.48}, eff_ids=[eve_effect.id])
    eve_affector4 = client.mk_eve_item(attrs={eve_affector_attr.id: 1.47}, eff_ids=[eve_effect.id])
    eve_affector5 = client.mk_eve_item(attrs={eve_affector_attr.id: 1.46}, eff_ids=[eve_effect.id])
    eve_affector6 = client.mk_eve_item(attrs={eve_affector_attr.id: 1.45}, eff_ids=[eve_effect.id])
    eve_affector7 = client.mk_eve_item(attrs={eve_affector_attr.id: 1.44}, eff_ids=[eve_effect.id])
    eve_affector8 = client.mk_eve_item(attrs={eve_affector_attr.id: 1.43}, eff_ids=[eve_effect.id])
    eve_affector9 = client.mk_eve_item(attrs={eve_affector_attr.id: 1.42}, eff_ids=[eve_effect.id])
    eve_affector10 = client.mk_eve_item(attrs={eve_affector_attr.id: 1.41}, eff_ids=[eve_effect.id])
    eve_affector11 = client.mk_eve_item(attrs={eve_affector_attr.id: 1.40}, eff_ids=[eve_effect.id])
    eve_affector12 = client.mk_eve_item(attrs={eve_affector_attr.id: 1.39}, eff_ids=[eve_effect.id])
    eve_affectee = client.mk_eve_ship(attrs={eve_affectee_attr.id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_affector1 = api_fit.add_rig(type_id=eve_affector1.id)
    api_affector2 = api_fit.add_rig(type_id=eve_affector2.id)
    api_affector3 = api_fit.add_rig(type_id=eve_affector3.id)
    api_affector4 = api_fit.add_rig(type_id=eve_affector4.id)
    api_affector5 = api_fit.add_rig(type_id=eve_affector5.id)
    api_affector6 = api_fit.add_rig(type_id=eve_affector6.id)
    api_affector7 = api_fit.add_rig(type_id=eve_affector7.id)
    api_affector8 = api_fit.add_rig(type_id=eve_affector8.id)
    api_affector9 = api_fit.add_rig(type_id=eve_affector9.id)
    api_affector10 = api_fit.add_rig(type_id=eve_affector10.id)
    api_affector11 = api_fit.add_rig(type_id=eve_affector11.id)
    api_fit.add_rig(type_id=eve_affector12.id)
    api_affectee = api_fit.set_ship(type_id=eve_affectee.id)
    api_affectee.update()
    assert api_affectee.attrs[eve_affectee_attr.id].dogma == approx(329.202701)
    api_mods = api_affectee.mods[eve_affectee_attr.id]
    # 12th affector is completely ignored both in calculation process and for modification listing
    assert len(api_mods) == 11
    api_mod1 = api_mods.find_by_affector_item(affector_item_id=api_affector1.id).one()
    assert api_mod1.op == consts.ApiModOp.pre_mul
    assert api_mod1.initial_val == approx(1.5)
    assert api_mod1.stacking_mult == approx(consts.PenaltyStr.p1)
    assert api_mod1.applied_val == approx(1.5)
    api_mod2 = api_mods.find_by_affector_item(affector_item_id=api_affector2.id).one()
    assert api_mod2.op == consts.ApiModOp.pre_mul
    assert api_mod2.initial_val == approx(1.49)
    assert api_mod2.stacking_mult == approx(consts.PenaltyStr.p2)
    assert api_mod2.applied_val == approx(1.42586879)
    api_mod3 = api_mods.find_by_affector_item(affector_item_id=api_affector3.id).one()
    assert api_mod3.op == consts.ApiModOp.pre_mul
    assert api_mod3.initial_val == approx(1.48)
    assert api_mod3.stacking_mult == approx(consts.PenaltyStr.p3)
    assert api_mod3.applied_val == approx(1.27387991)
    api_mod4 = api_mods.find_by_affector_item(affector_item_id=api_affector4.id).one()
    assert api_mod4.op == consts.ApiModOp.pre_mul
    assert api_mod4.initial_val == approx(1.47)
    assert api_mod4.stacking_mult == approx(consts.PenaltyStr.p4)
    assert api_mod4.applied_val == approx(1.13298892)
    api_mod5 = api_mods.find_by_affector_item(affector_item_id=api_affector5.id).one()
    assert api_mod5.op == consts.ApiModOp.pre_mul
    assert api_mod5.initial_val == approx(1.46)
    assert api_mod5.stacking_mult == approx(consts.PenaltyStr.p5)
    assert api_mod5.applied_val == approx(1.04875662)
    api_mod6 = api_mods.find_by_affector_item(affector_item_id=api_affector6.id).one()
    assert api_mod6.op == consts.ApiModOp.pre_mul
    assert api_mod6.initial_val == approx(1.45)
    assert api_mod6.stacking_mult == approx(consts.PenaltyStr.p6)
    assert api_mod6.applied_val == approx(1.01349602)
    api_mod7 = api_mods.find_by_affector_item(affector_item_id=api_affector7.id).one()
    assert api_mod7.op == consts.ApiModOp.pre_mul
    assert api_mod7.initial_val == approx(1.44)
    assert api_mod7.stacking_mult == approx(consts.PenaltyStr.p7)
    assert api_mod7.applied_val == approx(1.002820481)
    api_mod8 = api_mods.find_by_affector_item(affector_item_id=api_affector8.id).one()
    assert api_mod8.op == consts.ApiModOp.pre_mul
    assert api_mod8.initial_val == approx(1.43)
    assert api_mod8.stacking_mult == approx(consts.PenaltyStr.p8)
    assert api_mod8.applied_val == approx(1.0004450158)
    api_mod9 = api_mods.find_by_affector_item(affector_item_id=api_affector9.id).one()
    assert api_mod9.op == consts.ApiModOp.pre_mul
    assert api_mod9.initial_val == approx(1.42)
    assert api_mod9.stacking_mult == approx(consts.PenaltyStr.p9)
    assert api_mod9.applied_val == approx(1.00005300933)
    api_mod10 = api_mods.find_by_affector_item(affector_item_id=api_affector10.id).one()
    assert api_mod10.op == consts.ApiModOp.pre_mul
    assert api_mod10.initial_val == approx(1.41)
    assert api_mod10.stacking_mult == approx(consts.PenaltyStr.p10)
    assert api_mod10.applied_val == approx(1.000004766969)
    api_mod11 = api_mods.find_by_affector_item(affector_item_id=api_affector11.id).one()
    assert api_mod11.op == consts.ApiModOp.pre_mul
    assert api_mod11.initial_val == approx(1.40)
    assert api_mod11.stacking_mult == approx(consts.PenaltyStr.p11)
    assert api_mod11.applied_val == approx(1.0000003236185)


def test_insignificant_stacking(client, consts):
    # Here we check what happens if final result of stacking penalty chain is "neutral", its
    # modifications are not filtered out
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr(stackable=False)
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.pre_mul,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(mod_info=[eve_mod])
    eve_affector1 = client.mk_eve_item(attrs={eve_affector_attr.id: 0.5}, eff_ids=[eve_effect.id])
    eve_affector2 = client.mk_eve_item(attrs={eve_affector_attr.id: 2}, eff_ids=[eve_effect.id])
    eve_affectee = client.mk_eve_ship(attrs={eve_affectee_attr.id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_affector1 = api_fit.add_rig(type_id=eve_affector1.id)
    api_affector2 = api_fit.add_rig(type_id=eve_affector2.id)
    api_affectee = api_fit.set_ship(type_id=eve_affectee.id)
    api_affectee.update()
    assert api_affectee.attrs[eve_affectee_attr.id].dogma == approx(100)
    api_mods = api_affectee.mods[eve_affectee_attr.id]
    assert len(api_mods) == 2
    api_mod1 = api_mods.find_by_affector_item(affector_item_id=api_affector1.id).one()
    assert api_mod1.op == consts.ApiModOp.pre_mul
    assert api_mod1.initial_val == approx(0.5)
    assert api_mod1.stacking_mult == approx(consts.PenaltyStr.p1)
    assert api_mod1.applied_val == approx(0.5)
    api_mod2 = api_mods.find_by_affector_item(affector_item_id=api_affector2.id).one()
    assert api_mod2.op == consts.ApiModOp.pre_mul
    assert api_mod2.initial_val == approx(2)
    assert api_mod2.stacking_mult == approx(consts.PenaltyStr.p1)
    assert api_mod2.applied_val == approx(2)
