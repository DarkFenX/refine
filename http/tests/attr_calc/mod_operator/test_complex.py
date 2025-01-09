from tests import approx


def test_almost_all_in(client, consts):
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
    # Verification
    api_affectee.update()
    expected_value = (
            (val_pre_ass * val_pre_mul / val_pre_div + val_mod_add - val_mod_sub)
            * val_post_mul / val_post_div * (1 + val_post_perc / 100))
    assert api_affectee.attrs[eve_affectee_attr_id].dogma == approx(expected_value)
    api_mods = api_affectee.mods[eve_affectee_attr_id]
    api_pre_assign_mod = api_mods.find_by_op(op=consts.ApiModOp.pre_assign).one()
    assert api_pre_assign_mod.initial_val == approx(val_pre_ass)
    assert api_pre_assign_mod.stacking_mult is None
    assert api_pre_assign_mod.applied_val == approx(val_pre_ass)
    api_pre_mul_mod = api_mods.find_by_op(op=consts.ApiModOp.pre_mul).one()
    assert api_pre_mul_mod.initial_val == approx(val_pre_mul)
    assert api_pre_mul_mod.stacking_mult == approx(consts.PenaltyStr.p1)
    assert api_pre_mul_mod.applied_val == approx(val_pre_mul)
    api_pre_div_mod = api_mods.find_by_op(op=consts.ApiModOp.pre_div).one()
    assert api_pre_div_mod.initial_val == approx(val_pre_div)
    assert api_pre_div_mod.stacking_mult == approx(consts.PenaltyStr.p1)
    assert api_pre_div_mod.applied_val == approx(val_pre_div)
    api_add_mod = api_mods.find_by_op(op=consts.ApiModOp.mod_add).one()
    assert api_add_mod.initial_val == approx(val_mod_add)
    assert api_add_mod.stacking_mult is None
    assert api_add_mod.applied_val == approx(val_mod_add)
    api_sub_mod = api_mods.find_by_op(op=consts.ApiModOp.mod_sub).one()
    assert api_sub_mod.initial_val == approx(val_mod_sub)
    assert api_sub_mod.stacking_mult is None
    assert api_sub_mod.applied_val == approx(val_mod_sub)
    api_post_mul_mod = api_mods.find_by_op(op=consts.ApiModOp.post_mul).one()
    assert api_post_mul_mod.initial_val == approx(val_post_mul)
    assert api_post_mul_mod.stacking_mult == approx(consts.PenaltyStr.p1)
    assert api_post_mul_mod.applied_val == approx(val_post_mul)
    api_post_div_mod = api_mods.find_by_op(op=consts.ApiModOp.post_div).one()
    assert api_post_div_mod.initial_val == approx(val_post_div)
    assert api_post_div_mod.stacking_mult == approx(consts.PenaltyStr.p1)
    assert api_post_div_mod.applied_val == approx(val_post_div)
    api_post_perc_mod = api_mods.find_by_op(op=consts.ApiModOp.post_percent).one()
    assert api_post_perc_mod.initial_val == approx(val_post_perc)
    assert api_post_perc_mod.stacking_mult == approx(consts.PenaltyStr.p1)
    assert api_post_perc_mod.applied_val == approx(val_post_perc)
