from tests import approx, check_no_field
from tests.fw.api import ValOptions


def test_main(client, consts):
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_use_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active, discharge_attr_id=eve_use_attr_id)
    eve_module1_id = client.mk_eve_item(attrs={eve_use_attr_id: 1000}, eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_module2_id = client.mk_eve_item(attrs={eve_use_attr_id: 700}, eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_module3_id = client.mk_eve_item(attrs={eve_use_attr_id: 500}, eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 750})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module1 = api_fit.add_module(type_id=eve_module1_id, state=consts.ApiModuleState.disabled)
    api_fit.add_module(type_id=eve_module2_id, state=consts.ApiModuleState.disabled)
    api_fit.add_module(type_id=eve_module3_id, state=consts.ApiModuleState.disabled)
    # Verification
    api_val = api_fit.validate(options=ValOptions(unusable_cap=True))
    assert api_val.passed is False
    assert api_val.details.unusable_cap == (approx(750), {api_module1.id: approx(1000)})
    # Action
    api_module1.remove()
    # Verification
    api_val = api_fit.validate(options=ValOptions(unusable_cap=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
