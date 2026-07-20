from fw import approx
from fw.api import ItemStatsOptions


def test_sig_radius_stacking(client, consts):
    # This test checks that drone prop sig blow is not stacking penalized against PostMul
    # modifications. This was tested ingame on 2026-06-10 by bombing an MWD'ing drone in a c3
    # pulsar (which uses PostMul to apply sig blow). Here, in test, a buff used instead of a system
    # effect, because pulsar affecting drones seems to be a bug by itself. The bug is that ship
    # modifiers from system effects affect drones on first release, then do not after a scoop-deploy
    # cycle. This bug is not replicated in the lib, so using buff here instead.
    eve_sig_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.sig_radius)
    eve_cruise_speed_attr_id = client.mk_eve_attr(id_=consts.EveAttr.entity_cruise_speed)
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    client.mk_eve_attr(id_=consts.EveAttr.entity_max_velocity_sig_radius_mult, def_val=6)
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_sig_radius_attr_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.buff_everything, cat_id=consts.EveEffCat.active)
    eve_fw_effect_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 1.58},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_drone_id = client.mk_eve_drone(attrs={eve_sig_radius_attr_id: 25, eve_cruise_speed_attr_id: 894})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_fw_effect(type_id=eve_fw_effect_id)
    api_drone = api_fit.add_drone(type_id=eve_drone_id, npc_prop=consts.ApiNpcProp.chase)
    # Verification - multiplication happens in sig getter, no on dogma level, so non-MWD-blown attr
    # value is exposed in attributes
    assert api_drone.update().attrs[eve_sig_radius_attr_id].modified == approx(39.5)
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(sig_radius=True))
    assert api_drone_stats.sig_radius.one() == approx(237)
