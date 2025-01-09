from tests import approx


def setup_hig_test(*, client, consts, high_is_good):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr(stackable=False, high_is_good=high_is_good)
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_item_affector1_id = client.mk_eve_item(attrs={eve_affector_attr_id: 10}, eff_ids=[eve_effect_id])
    eve_item_affector2_id = client.mk_eve_item(attrs={eve_affector_attr_id: -20}, eff_ids=[eve_effect_id])
    eve_item_affector3_id = client.mk_eve_item(attrs={eve_affector_attr_id: 53.02}, eff_ids=[eve_effect_id])
    eve_item_affectee_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_rig(type_id=eve_item_affector1_id)
    api_item_affector2 = api_fit.add_rig(type_id=eve_item_affector2_id)
    api_item_affector3 = api_fit.add_rig(type_id=eve_item_affector3_id)
    api_item_affectee = api_fit.set_ship(type_id=eve_item_affectee_id)
    api_item_affectee.update()
    return (
        api_item_affectee.attrs[eve_affectee_attr_id].dogma,
        api_item_affectee.mods[eve_affectee_attr_id],
        api_item_affector2,
        api_item_affector3)


def test_high_is_good(client, consts):
    attr_val, attr_mods, _, api_item_affector3 = setup_hig_test(client=client, consts=consts, high_is_good=True)
    # Verification
    assert attr_val == approx(53.02)
    attr_mod = attr_mods.one()
    assert attr_mod.op == consts.ApiModOp.post_assign
    assert attr_mod.initial_val == approx(53.02)
    assert attr_mod.stacking_mult is None
    assert attr_mod.applied_val == approx(53.02)
    assert attr_mod.affectors.one().item_id == api_item_affector3.id


def test_high_is_bad(client, consts):
    attr_val, attr_mods, api_item_affector2, _ = setup_hig_test(client=client, consts=consts, high_is_good=False)
    # Verification
    assert attr_val == approx(-20)
    attr_mod = attr_mods.one()
    assert attr_mod.op == consts.ApiModOp.post_assign
    assert attr_mod.initial_val == approx(-20)
    assert attr_mod.stacking_mult is None
    assert attr_mod.applied_val == approx(-20)
    assert attr_mod.affectors.one().item_id == api_item_affector2.id


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
    val_post_mul = 1.35
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
    # Post-assignment
    val_post_ass = 68
    eve_mod_post_ass = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_post_ass_id = client.mk_eve_effect(mod_info=[eve_mod_post_ass])
    eve_affector_post_ass_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: val_post_ass},
        eff_ids=[eve_effect_post_ass_id])
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
    api_fit.add_rig(type_id=eve_affector_post_ass_id)
    api_affectee = api_fit.set_ship(type_id=eve_affectee_id)
    # Verification
    api_affectee.update()
    assert api_affectee.attrs[eve_affectee_attr_id].dogma == approx(val_post_ass)
    api_mods = api_affectee.mods[eve_affectee_attr_id]
    assert len(api_mods) == 1
    api_post_assign_mod = api_mods.find_by_op(op=consts.ApiModOp.post_assign).one()
    assert api_post_assign_mod.initial_val == approx(val_post_ass)
    assert api_post_assign_mod.stacking_mult is None
    assert api_post_assign_mod.applied_val == approx(val_post_ass)
