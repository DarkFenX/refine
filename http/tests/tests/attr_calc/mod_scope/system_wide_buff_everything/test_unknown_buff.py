from tests import approx


def test_static(client, consts):
    # Check that there is no strange side effects when buff isn't defined
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.weather_darkness,
        cat_id=consts.EveEffCat.active)
    eve_sw_effect_id = client.mk_eve_item(
        # Buff ID which we didn't create
        attrs={eve_buff_type_attr_id: 7, eve_buff_val_attr_id: 5},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_sw_effect = api_sol.add_sw_effect(type_id=eve_sw_effect_id)
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    assert api_ship.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)
    api_sw_effect.remove()
    assert api_ship.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)
