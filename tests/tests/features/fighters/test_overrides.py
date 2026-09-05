from fw import check_no_field


def test_count_set_not_loaded(client, consts):
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
    api_fighter.update()
    assert api_fighter.count == [12, 12]
    with check_no_field():
        api_fighter.count_override  # ruff:ignore[useless-expression]
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification
    api_fighter.update()
    with check_no_field():
        api_fighter.count  # ruff:ignore[useless-expression]
    with check_no_field():
        api_fighter.count_override  # ruff:ignore[useless-expression]
    # Action
    api_fighter.change_fighter(count_override=3)
    # Verification
    api_fighter.update()
    with check_no_field():
        api_fighter.count  # ruff:ignore[useless-expression]
    assert api_fighter.count_override == 3
    # Action
    api_sol.change_src(data=eve_d1)
    # Verification
    assert api_fighter.update().count == [3, 12]
    assert api_fighter.count_override == 3


def test_rearm_minion_set_not_loaded(client, consts):
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    eve_fighter_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    client.mk_eve_fighter(datas=[eve_d1], id_=eve_fighter_id)
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1, default_rearm_minions=consts.ApiRearmMinion.on_first_empty)
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id)
    # Verification
    api_fighter.update()
    assert api_fighter.rearm_minion == consts.ApiRearmMinion.on_first_empty
    with check_no_field():
        api_fighter.rearm_minion_override  # ruff:ignore[useless-expression]
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification
    api_fighter.update()
    assert api_fighter.rearm_minion == consts.ApiRearmMinion.on_first_empty
    with check_no_field():
        api_fighter.rearm_minion_override  # ruff:ignore[useless-expression]
    # Action
    api_fighter.change_fighter(rearm_minion_override=consts.ApiRearmMinion.disabled)
    # Verification
    api_fighter.update()
    assert api_fighter.rearm_minion == consts.ApiRearmMinion.disabled
    assert api_fighter.rearm_minion_override == consts.ApiRearmMinion.disabled
    # Action
    api_sol.change_src(data=eve_d1)
    # Verification
    assert api_fighter.update().rearm_minion == consts.ApiRearmMinion.disabled
    assert api_fighter.rearm_minion_override == consts.ApiRearmMinion.disabled
