from tests import check_no_field


def test_booster(client, consts):
    eve_slot_attr_id = client.mk_eve_attr(id_=consts.EveAttr.boosterness)
    eve_booster_id = client.mk_eve_item(cat_id=consts.EveItemCat.implant, attrs={eve_slot_attr_id: 1})
    eve_other_id = client.mk_eve_item(cat_id=consts.EveItemCat.drone)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_booster(type_id=eve_booster_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.item_type])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_other1 = api_fit.add_booster(type_id=eve_other_id)
    api_other2 = api_fit.add_implant(type_id=eve_booster_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.item_type])
    assert api_val.passed is False
    assert api_val.details.item_type == {
        api_other1.id: (consts.ApiValItemType.drone, consts.ApiValItemType.booster),
        api_other2.id: (consts.ApiValItemType.booster, consts.ApiValItemType.implant)}
