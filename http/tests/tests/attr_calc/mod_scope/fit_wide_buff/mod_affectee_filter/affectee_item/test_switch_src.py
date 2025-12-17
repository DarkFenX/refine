from fw import approx


def test_onlist_to_offlist_to_onlist(client, consts):
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    eve_buff_type_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2], id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2], id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_buff_id = client.mk_eve_buff(
        datas=[eve_d1, eve_d2],
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_effect_id = client.mk_eve_effect(
        datas=[eve_d1, eve_d2],
        id_=consts.UtilEffect.buff_everything,
        cat_id=consts.EveEffCat.active)
    eve_fw_effect_id = client.mk_eve_item(
        datas=[eve_d1, eve_d2],
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 5},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_root_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    client.mk_eve_ship(datas=[eve_d1], id_=eve_root_id, attrs={eve_affectee_attr_id: 7.5})
    client.mk_eve_struct(datas=[eve_d2], id_=eve_root_id, attrs={eve_affectee_attr_id: 7.5})
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1)
    api_fit = api_sol.create_fit()
    api_fit.add_fw_effect(type_id=eve_fw_effect_id)
    api_root = api_fit.set_ship(type_id=eve_root_id)
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)
    # Action
    api_sol.change_src(data=eve_d1)
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)
