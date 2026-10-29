from fw import approx


def test_switch_src_passive(client, consts):
    # Charge deactivation during source switch used to leave charge passive effects running with
    # stale data
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    # The same affectee attr ID
    eve_affectee_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    # Different affector attr IDs
    eve_d1_affector_attr_id = client.alloc_attr_id(datas=[eve_d1, eve_d2])
    client.mk_eve_attr(datas=[eve_d1], id_=eve_d1_affector_attr_id)
    eve_d2_affector_attr_id = client.alloc_attr_id(datas=[eve_d1, eve_d2])
    client.mk_eve_attr(datas=[eve_d2], id_=eve_d2_affector_attr_id)
    # Different effect IDs
    eve_mod1 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.other,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_d1_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_d1_effect_id = client.alloc_effect_id(datas=[eve_d1, eve_d2])
    client.mk_eve_effect(datas=[eve_d1], id_=eve_d1_effect_id, cat_id=consts.EveEffCat.passive, mod_info=[eve_mod1])
    eve_mod2 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.other,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_d2_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_d2_effect_id = client.alloc_effect_id(datas=[eve_d1, eve_d2])
    client.mk_eve_effect(datas=[eve_d2], id_=eve_d2_effect_id, cat_id=consts.EveEffCat.passive, mod_info=[eve_mod2])
    # The same charge ID
    eve_charge_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    client.mk_eve_item(
        datas=[eve_d1],
        id_=eve_charge_id,
        attrs={eve_d1_affector_attr_id: 20},
        eff_ids=[eve_d1_effect_id])
    client.mk_eve_item(
        datas=[eve_d2],
        id_=eve_charge_id,
        attrs={eve_d2_affector_attr_id: 1.5},
        eff_ids=[eve_d2_effect_id])
    # The same module ID
    eve_act_effect_id = client.mk_eve_effect(
        datas=[eve_d1, eve_d2],
        id_=consts.UtilEffect.activates_charge,
        cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_item(
        datas=[eve_d1, eve_d2],
        attrs={eve_affectee_attr_id: 100},
        eff_ids=[eve_act_effect_id],
        defeff_id=eve_act_effect_id)
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1)
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_id)
    # Verification
    assert api_module.update().attrs[eve_affectee_attr_id].modified == approx(120)
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification
    assert api_module.update().attrs[eve_affectee_attr_id].modified == approx(150)
    # Action
    api_sol.change_src(data=eve_d1)
    # Verification
    assert api_module.update().attrs[eve_affectee_attr_id].modified == approx(120)
