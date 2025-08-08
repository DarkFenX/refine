from tests import check_no_field


def test_basic(client, consts):
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.cycle_none)
    eve_charge_id = client.mk_eve_item(attrs={eve_volume_attr_id: 1})
    eve_module_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 1},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge_id)
    # Verification - can cycle infinitely regardless of anything
    assert api_module.update().cycles_until_empty is None


def test_no_charge(client, consts):
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.cycle_none)
    eve_module_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 1},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id)
    # Verification - can cycle infinitely w/o charge, but info is not exposed when charge is not set
    api_module.update()
    with check_no_field():
        api_module.cycles_until_empty  # noqa: B018


def test_not_loaded_charge(client, consts):
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.cycle_none)
    eve_charge_id = client.alloc_item_id()
    eve_module_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 1},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge_id)
    # Verification
    assert api_module.update().cycles_until_empty is None
