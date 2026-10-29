from fw.api import ValOptions


def test_charge_kfs_replaced(client, consts):
    # Known failures of a charge loaded into a module should not apply to charges tried in its place
    eve_ban_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_in_empire_space)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge1_id = client.mk_eve_charge()
    eve_charge2_id = client.mk_eve_charge(attrs={eve_ban_attr_id: 1})
    eve_charge3_id = client.mk_eve_charge(attrs={eve_ban_attr_id: 1})
    eve_module_id = client.mk_eve_item(attrs={eve_capacity_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.hisec)
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module_id,
        charge_type_id=eve_charge2_id,
        state=consts.ApiModuleState.disabled)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_unactivable=[api_module.charge.id]),
        passing=[eve_charge1_id],
        failing=[eve_charge2_id, eve_charge3_id])


def test_charge_kfs_restored(client, consts):
    # Charge which is put back after trying charges in its module should keep its known failures
    eve_grp1_id = client.mk_eve_charge_group()
    eve_grp2_id = client.mk_eve_charge_group()
    eve_group_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_group1, unit_id=consts.EveAttrUnit.group_id)
    eve_ban_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_in_empire_space)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge1_id = client.mk_eve_charge(grp_id=eve_grp1_id, attrs={eve_ban_attr_id: 1})
    eve_charge2_id = client.mk_eve_charge(grp_id=eve_grp2_id)
    eve_charge3_id = client.mk_eve_charge(grp_id=eve_grp1_id, attrs={eve_ban_attr_id: 1})
    eve_module1_id = client.mk_eve_item(attrs={eve_group_attr_id: eve_grp1_id, eve_capacity_attr_id: 1})
    eve_module2_id = client.mk_eve_item(attrs={eve_group_attr_id: eve_grp2_id, eve_capacity_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.hisec)
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_module(
        type_id=eve_module1_id,
        charge_type_id=eve_charge1_id,
        state=consts.ApiModuleState.disabled)
    api_fit.add_module(type_id=eve_module2_id, state=consts.ApiModuleState.disabled)
    # Verification - charge 2 can be loaded only into module 2, which is tried after charge 1 is
    # put back into module 1
    api_fit.check_try_fit_items(
        val_options=ValOptions(charge_group=True, sec_zone_unactivable=[api_module1.charge.id]),
        passing=[eve_charge2_id],
        failing=[eve_charge3_id])
