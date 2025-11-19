from tests import check_no_field


def test_set_not_loaded(client, consts):
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    eve_max_count_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2], id_=consts.EveAttr.ftr_sq_max_size)
    eve_fighter_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    client.mk_eve_fighter(datas=[eve_d1], id_=eve_fighter_id, attrs={eve_max_count_attr_id: 12})
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1)
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id)
    # Verification
    assert api_fighter.update().count == [12, 12, False]
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification
    api_fighter.update()
    with check_no_field():
        api_fighter.count  # noqa: B018
    # Action
    api_fighter.change_fighter(count=3)
    # Verification
    api_fighter.update()
    with check_no_field():
        api_fighter.count  # noqa: B018
    # Action
    api_sol.change_src(data=eve_d1)
    # Verification
    assert api_fighter.update().count == [3, 12, True]
