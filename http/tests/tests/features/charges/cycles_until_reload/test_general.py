
def test_no_default_effect(client, consts):
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge_rate_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_rate)
    eve_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.cycle_charge_rate)
    eve_charge_id = client.mk_eve_item(attrs={eve_volume_attr_id: 0.05})
    eve_module_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 0.50, eve_charge_rate_attr_id: 1},
        eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge_id)
    # Verification
    assert api_module.update().cycles_until_empty is None


def test_module_not_loaded(client, consts):
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_charge_id = client.mk_eve_item(attrs={eve_volume_attr_id: 0.05})
    eve_module_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge_id)
    # Verification
    assert api_module.update().cycles_until_empty is None
