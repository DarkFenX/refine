"""
Check which damage types RAH chooses when receiving equal damage.
"""

from tests import approx
from tests.tests.features.reactive_armor_hardener import make_eve_rah, make_eve_ship, setup_rah_basics


def test_multi(client, consts):
    eve_basic_info = setup_rah_basics(client=client, consts=consts)
    eve_rah_id = make_eve_rah(client=client, basic_info=eve_basic_info, resos=(0.85, 0.85, 0.85, 0.85), shift_amount=6)
    eve_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, resos=(0.675, 0.675, 0.675, 0.675))
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit(rah_incoming_dps=(1, 1, 1, 1))
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah = api_fit.add_module(type_id=eve_rah_id, state=consts.ApiModuleState.active)
    # Verification - from real tests (2017-02-20), gecko vs gnosis
    # ---loop---
    # 0 0.850 0.850 0.850 0.850
    # 1 0.910 0.790 0.790 0.910 (kin therm > em expl)
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.88)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.82)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.82)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.88)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.594)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.5535)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.5535)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.594)


def test_therm_kin_expl(client, consts):
    eve_basic_info = setup_rah_basics(client=client, consts=consts)
    eve_rah_id = make_eve_rah(client=client, basic_info=eve_basic_info, resos=(0.85, 0.85, 0.85, 0.85), shift_amount=6)
    eve_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, resos=(0.675, 0.675, 0.675, 0.675))
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit(rah_incoming_dps=(0, 1, 1, 1))
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah = api_fit.add_module(type_id=eve_rah_id, state=consts.ApiModuleState.active)
    # Verification - from real tests (2017-02-20), gecko vs gnosis with 2 EM hardeners
    # 0 0.850 0.850 0.850 0.850
    # 1 0.910 0.790 0.790 0.910 (kin therm > expl)
    # 2 0.970 0.730 0.850 0.850 (therm > kin)
    # ---loop---
    # 3 1.000 0.790 0.805 0.805
    # 4 1.000 0.850 0.775 0.775
    # 5 1.000 0.820 0.745 0.835 (kin > expl)
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].dogma == approx(1)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.82)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.775)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.805)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.675)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.5535)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.523125)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.543375)


def test_em_kin_expl(client, consts):
    eve_basic_info = setup_rah_basics(client=client, consts=consts)
    eve_rah_id = make_eve_rah(client=client, basic_info=eve_basic_info, resos=(0.85, 0.85, 0.85, 0.85), shift_amount=6)
    eve_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, resos=(0.675, 0.675, 0.675, 0.675))
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit(rah_incoming_dps=(1, 0, 1, 1))
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah = api_fit.add_module(type_id=eve_rah_id, state=consts.ApiModuleState.active)
    # Verification - from real tests (2017-02-20), gecko vs gnosis with 2 thermal hardeners
    # 0 0.850 0.850 0.850 0.850
    # 1 0.910 0.910 0.790 0.790 (kin expl > em)
    # 2 0.850 0.970 0.730 0.850 (kin > expl)
    # ---loop---
    # 3 0.805 1.000 0.790 0.805
    # 4 0.775 1.000 0.850 0.775
    # 5 0.835 1.000 0.820 0.745 (expl > em)
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.805)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(1)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.82)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.775)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.543375)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.675)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.5535)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.523125)


def test_em_therm_expl(client, consts):
    eve_basic_info = setup_rah_basics(client=client, consts=consts)
    eve_rah_id = make_eve_rah(client=client, basic_info=eve_basic_info, resos=(0.85, 0.85, 0.85, 0.85), shift_amount=6)
    eve_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, resos=(0.675, 0.675, 0.675, 0.675))
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit(rah_incoming_dps=(1, 1, 0, 1))
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah = api_fit.add_module(type_id=eve_rah_id, state=consts.ApiModuleState.active)
    # Verification - from real tests (2017-02-20), gecko vs gnosis with 2 kinetic hardeners
    # 0 0.850 0.850 0.850 0.850
    # 1 0.910 0.790 0.910 0.790 (expl therm > em)
    # 2 0.850 0.730 0.970 0.850 (therm > expl)
    # ---loop---
    # 3 0.805 0.790 1.000 0.805
    # 4 0.775 0.850 1.000 0.775
    # 5 0.835 0.820 1.000 0.745 (expl > em)
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.805)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.82)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(1)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.775)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.543375)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.5535)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.675)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.523125)


def test_em_therm_kin(client, consts):
    eve_basic_info = setup_rah_basics(client=client, consts=consts)
    eve_rah_id = make_eve_rah(client=client, basic_info=eve_basic_info, resos=(0.85, 0.85, 0.85, 0.85), shift_amount=6)
    eve_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, resos=(0.675, 0.675, 0.675, 0.675))
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit(rah_incoming_dps=(1, 1, 1, 0))
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah = api_fit.add_module(type_id=eve_rah_id, state=consts.ApiModuleState.active)
    # Verification - from real tests (2017-02-20), gecko vs gnosis with 2 explosive hardeners
    # 0 0.850 0.850 0.850 0.850
    # 1 0.910 0.790 0.790 0.910 (kin therm > em)
    # 2 0.850 0.730 0.850 0.970 (therm > kin)
    # ---loop---
    # 3 0.805 0.790 0.805 1.000
    # 4 0.775 0.850 0.775 1.000
    # 5 0.835 0.820 0.745 1.000 (kin > em)
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.805)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.82)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.775)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(1)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.543375)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.5535)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.523125)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.675)
