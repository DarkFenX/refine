from tests import approx


def setup_test(client, consts):
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    eve_grp_id = eve_d1.mk_item_group().id
    eve_d2.mk_item_group(id_=eve_grp_id)
    eve_d1.mk_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_d2.mk_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_d1.mk_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_d2.mk_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr_id = eve_d1.mk_attr().id
    eve_d2.mk_attr(id_=eve_affectee_attr_id)
    eve_buff_id = eve_d1.mk_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        loc_grp_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id, group_id=eve_grp_id)]).id
    eve_d2.mk_buff(
        id_=eve_buff_id,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        loc_grp_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id, group_id=eve_grp_id)])
    eve_d1.mk_effect(
        id_=consts.EveEffect.mod_bonus_warfare_link_armor,
        cat_id=consts.EveEffCat.active)
    eve_d2.mk_effect(
        id_=consts.EveEffect.mod_bonus_warfare_link_armor,
        cat_id=consts.EveEffCat.active)
    eve_module_id = eve_d1.mk_item(
        attrs={consts.EveAttr.warfare_buff_1_id: eve_buff_id, consts.EveAttr.warfare_buff_1_value: 5},
        eff_ids=[consts.EveEffect.mod_bonus_warfare_link_armor],
        defeff_id=consts.EveEffect.mod_bonus_warfare_link_armor).id
    eve_d2.mk_item(
        id_=eve_module_id,
        attrs={consts.EveAttr.warfare_buff_1_id: eve_buff_id, consts.EveAttr.warfare_buff_1_value: 5},
        eff_ids=[consts.EveEffect.mod_bonus_warfare_link_armor],
        defeff_id=consts.EveEffect.mod_bonus_warfare_link_armor)
    eve_rig_id = eve_d1.mk_item(grp_id=eve_grp_id, attrs={eve_affectee_attr_id: 7.5}).id
    eve_d2.mk_item(id_=eve_rig_id, grp_id=eve_grp_id, attrs={eve_affectee_attr_id: 7.5})
    eve_root_id = eve_d1.mk_ship().id
    eve_d2.mk_struct(id_=eve_root_id)
    client.create_sources()
    return eve_d1, eve_d2, eve_affectee_attr_id, eve_root_id, eve_module_id, eve_rig_id


def test_self_to_struct(client, consts):
    eve_d1, eve_d2, eve_affectee_attr_id, eve_root_id, eve_module_id, eve_rig_id = setup_test(
        client=client, consts=consts)
    api_sol = client.create_sol(data=eve_d1)
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_root_id)
    api_fit.add_mod(type_id=eve_module_id, state=consts.ApiState.active)
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)
    # Action
    api_sol.change_src(data=eve_d1)
    # Verification
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)


def test_fleeted_to_struct(client, consts):
    eve_d1, eve_d2, eve_affectee_attr_id, eve_root_id, eve_module_id, eve_rig_id = setup_test(
        client=client, consts=consts)
    api_sol = client.create_sol(data=eve_d1)
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit1.id, api_fit2.id])
    api_fit1.add_mod(type_id=eve_module_id, state=consts.ApiState.active)
    api_fit2.set_ship(type_id=eve_root_id)
    api_rig = api_fit2.add_rig(type_id=eve_rig_id)
    # Verification
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)
    # Action
    api_sol.change_src(data=eve_d1)
    # Verification
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)
