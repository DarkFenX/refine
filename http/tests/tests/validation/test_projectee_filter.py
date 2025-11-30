from tests import check_no_field
from tests.fw.api import ValOptions


def test_src_module_tgt_ship_project_unproject(client, consts):
    # Also test that only validation of source fit is affected
    eve_tgt_list_attr_id = client.mk_eve_attr(id_=consts.EveAttr.valid_tgt_whitelist)
    eve_src_effect_id = client.mk_eve_effect(id_=consts.EveEffect.use_missiles, cat_id=consts.EveEffCat.active)
    eve_tgt_item_id = client.mk_eve_ship()
    eve_item_list_id = client.mk_eve_item_list()
    eve_src_item_id = client.mk_eve_item(
        attrs={eve_tgt_list_attr_id: eve_item_list_id},
        eff_ids=[eve_src_effect_id],
        defeff_id=eve_src_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_item = api_src_fit.add_module(type_id=eve_src_item_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_item = api_tgt_fit.set_ship(type_id=eve_tgt_item_id)
    # Verification
    api_val = api_src_fit.validate(options=ValOptions(projectee_filter=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    api_val = api_tgt_fit.validate(options=ValOptions(projectee_filter=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_src_item.change_module(add_projs=[api_tgt_item.id])
    # Verification
    api_val = api_src_fit.validate(options=ValOptions(projectee_filter=True))
    assert api_val.passed is False
    assert api_val.details.projectee_filter == {api_src_item.id: [api_tgt_item.id]}
    api_val = api_tgt_fit.validate(options=ValOptions(projectee_filter=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_src_item.change_module(rm_projs=[api_tgt_item.id])
    # Verification
    api_val = api_src_fit.validate(options=ValOptions(projectee_filter=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    api_val = api_tgt_fit.validate(options=ValOptions(projectee_filter=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
