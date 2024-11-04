from tests import approx


def test_to_struct(client, consts):
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
        id_=consts.EveEffect.weather_darkness,
        cat_id=consts.EveEffCat.active)
    eve_d2.mk_effect(
        id_=consts.EveEffect.weather_darkness,
        cat_id=consts.EveEffCat.active)
    eve_sw_effect_id = eve_d1.mk_item(
        attrs={consts.EveAttr.warfare_buff_1_id: eve_buff_id, consts.EveAttr.warfare_buff_1_value: 5},
        eff_ids=[consts.EveEffect.weather_darkness],
        defeff_id=consts.EveEffect.weather_darkness).id
    eve_d2.mk_item(
        id_=eve_sw_effect_id,
        attrs={consts.EveAttr.warfare_buff_1_id: eve_buff_id, consts.EveAttr.warfare_buff_1_value: 5},
        eff_ids=[consts.EveEffect.weather_darkness],
        defeff_id=consts.EveEffect.weather_darkness)
    eve_rig_id = eve_d1.mk_item(grp_id=eve_grp_id, attrs={eve_affectee_attr_id: 7.5}).id
    eve_d2.mk_item(id_=eve_rig_id, grp_id=eve_grp_id, attrs={eve_affectee_attr_id: 7.5})
    eve_root_id = eve_d1.mk_ship().id
    eve_d2.mk_struct(id_=eve_root_id)
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1)
    api_sol.add_sw_effect(type_id=eve_sw_effect_id)
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_root_id)
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
