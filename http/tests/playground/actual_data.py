"""
Just collection of methods to test with real data from time to time.
"""

import json
from pathlib import Path
from time import time

from tests import approx
from tests.fw.api import ValOptions
from tests.fw.api.types.item import Item

SCRIPT_FOLDER_PATH = Path(__file__).resolve().absolute().parent
PHOBOS_BASE_PATH = Path('~', 'Desktop', 'phobos_tq_en-us').expanduser()


def test_benchmark_attr_calc(client, consts):  # noqa: ANN001, ANN201
    setup_eve_data(client=client, data=client._get_default_eve_data())  # noqa: SLF001
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.hisec)
    api_fit = api_sol.create_fit()
    api_fit.set_character(type_id=1373)
    for eve_skill_id in get_skill_type_ids():
        api_fit.add_skill(type_id=eve_skill_id, level=5)
    api_fit.set_ship(type_id=11184)  # Crusader
    iterations = 1000000
    print('starting attr-calc benchmark')  # noqa: T201
    before = time()
    api_sol.benchmark(command={'type': 'attr_calc', 'fit_id': api_fit.id, 'type_id': 1306, 'iterations': iterations})
    after = time()
    print('done with attr-calc benchmark')  # noqa: T201
    delta = after - before
    ips = iterations / delta
    print(f'{iterations} iterations done in {delta:.3f} seconds, {ips:.2f} iterations per second')  # noqa: T201


def test_benchmark_try_fit_items(client, consts):  # noqa: ANN001, ANN201
    setup_eve_data(client=client, data=client._get_default_eve_data())  # noqa: SLF001
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.hisec)
    api_fit = api_sol.create_fit()
    api_fit.set_character(type_id=1373)
    for eve_skill_id in get_skill_type_ids():
        api_fit.add_skill(type_id=eve_skill_id, level=5)
    api_fit.add_implant(type_id=13231)  # TD-603
    api_fit.add_implant(type_id=10228)  # SM-703
    api_fit.add_implant(type_id=24663)  # Zor hyperlink
    api_fit.add_implant(type_id=13244)  # SS-903
    api_fit.add_implant(type_id=13219)  # LP-1003
    api_fit.add_booster(type_id=28674)  # Synth drop
    api_fit.add_booster(type_id=28672)  # Synth crash
    api_fit.add_booster(type_id=45999)  # Pyro 2
    api_fit.set_ship(type_id=32311)  # NTyphoon
    # T2 800mms with hail
    for _ in range(3):
        api_fit.add_module(
            type_id=2929,
            rack=consts.ApiRack.high,
            state=consts.ApiModuleState.overload,
            charge_type_id=12779)
    # T2 torpedo launchers with thermal rages
    for _ in range(3):
        api_fit.add_module(
            type_id=2420,
            rack=consts.ApiRack.high,
            state=consts.ApiModuleState.overload,
            charge_type_id=2811)
    api_fit.add_module(type_id=5945, rack=consts.ApiRack.mid, state=consts.ApiModuleState.active)  # Enduring 500MN
    # T2 med cap booster with navy 800
    api_fit.add_module(type_id=2024, rack=consts.ApiRack.mid, state=consts.ApiModuleState.active, charge_type_id=32014)
    api_fit.add_module(type_id=2301, rack=consts.ApiRack.mid, state=consts.ApiModuleState.active)  # T2 EM hardener
    api_fit.add_module(type_id=448, rack=consts.ApiRack.mid, state=consts.ApiModuleState.active)  # T2 scram
    api_fit.add_module(type_id=2048, rack=consts.ApiRack.low, state=consts.ApiModuleState.online)  # T2 DC
    for _ in range(2):
        api_fit.add_module(type_id=519, rack=consts.ApiRack.low, state=consts.ApiModuleState.online)  # T2 gyrostab
    for _ in range(2):
        api_fit.add_module(type_id=22291, rack=consts.ApiRack.low, state=consts.ApiModuleState.online)  # T2 BCS
    api_fit.add_module(type_id=4405, rack=consts.ApiRack.low, state=consts.ApiModuleState.online)  # T2 DDA
    api_fit.add_rig(type_id=26436)  # T2 therm rig
    # T1 CDFEs
    for _ in range(2):
        api_fit.add_rig(type_id=26088)
    # T2 ogres
    for _ in range(5):
        api_fit.add_drone(type_id=2446, state=consts.ApiMinionState.engaging)
    # T2 ogres
    for _ in range(2):
        api_fit.add_drone(type_id=2446, state=consts.ApiMinionState.in_bay)
    api_val = api_fit.validate(options=ValOptions(default=True))
    assert api_val.passed is True
    iterations = 1000
    try_fit_type_ids = get_try_fit_type_ids()
    options = ValOptions(default=True).to_dict()
    print(f'starting try-fit-items benchmark, trying {len(try_fit_type_ids)} items per iteration')  # noqa: T201
    before = time()
    api_sol.benchmark(command={
        'type': 'try_fit_items',
        'fit_id': api_fit.id,
        'type_ids': try_fit_type_ids,
        'validation_options': options,
        'iterations': iterations})
    after = time()
    print('done with try-fit-items benchmark')  # noqa: T201
    delta = after - before
    ips = iterations / delta
    print(f'{iterations} iterations done in {delta:.3f} seconds, {ips:.2f} iterations per second')  # noqa: T201


def test_try_fit_items_nphoon(client, consts):  # noqa: ANN001, ANN201
    setup_eve_data(client=client, data=client._get_default_eve_data())  # noqa: SLF001
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.hisec)
    api_fit = api_sol.create_fit()
    api_fit.set_character(type_id=1373)
    for eve_skill_id in get_skill_type_ids():
        api_fit.add_skill(type_id=eve_skill_id, level=5)
    api_fit.add_implant(type_id=13231)  # TD-603
    api_fit.add_implant(type_id=10228)  # SM-703
    api_fit.add_implant(type_id=24663)  # Zor hyperlink
    api_fit.add_implant(type_id=13244)  # SS-903
    api_fit.add_implant(type_id=13219)  # LP-1003
    api_fit.add_booster(type_id=28674)  # Synth drop
    api_fit.add_booster(type_id=28672)  # Synth crash
    api_fit.add_booster(type_id=45999)  # Pyro 2
    api_fit.set_ship(type_id=32311)  # NTyphoon
    # T2 800mms with hail
    for _ in range(3):
        api_fit.add_module(
            type_id=2929,
            rack=consts.ApiRack.high,
            state=consts.ApiModuleState.overload,
            charge_type_id=12779)
    # T2 torpedo launchers with thermal rages
    for _ in range(3):
        api_fit.add_module(
            type_id=2420,
            rack=consts.ApiRack.high,
            state=consts.ApiModuleState.overload,
            charge_type_id=2811)
    api_fit.add_module(type_id=5945, rack=consts.ApiRack.mid, state=consts.ApiModuleState.active)  # Enduring 500MN
    # T2 med cap booster with navy 800
    api_fit.add_module(type_id=2024, rack=consts.ApiRack.mid, state=consts.ApiModuleState.active, charge_type_id=32014)
    api_fit.add_module(type_id=2301, rack=consts.ApiRack.mid, state=consts.ApiModuleState.active)  # T2 EM hardener
    api_fit.add_module(type_id=448, rack=consts.ApiRack.mid, state=consts.ApiModuleState.active)  # T2 scram
    api_fit.add_module(type_id=2048, rack=consts.ApiRack.low, state=consts.ApiModuleState.online)  # T2 DC
    for _ in range(2):
        api_fit.add_module(type_id=519, rack=consts.ApiRack.low, state=consts.ApiModuleState.online)  # T2 gyrostab
    for _ in range(2):
        api_fit.add_module(type_id=22291, rack=consts.ApiRack.low, state=consts.ApiModuleState.online)  # T2 BCS
    api_fit.add_module(type_id=4405, rack=consts.ApiRack.low, state=consts.ApiModuleState.online)  # T2 DDA
    api_fit.add_rig(type_id=26436)  # T2 therm rig
    # T1 CDFEs
    for _ in range(2):
        api_fit.add_rig(type_id=26088)
    # T2 ogres
    for _ in range(5):
        api_fit.add_drone(type_id=2446, state=consts.ApiMinionState.engaging)
    # T2 ogres
    for _ in range(2):
        api_fit.add_drone(type_id=2446, state=consts.ApiMinionState.in_bay)
    api_val = api_fit.validate(options=ValOptions(default=True))
    assert api_val.passed is True
    try_fit_type_ids = get_try_fit_type_ids()
    type_ids = api_fit.try_fit_items(type_ids=try_fit_type_ids, options=ValOptions(default=True))
    print('---')  # noqa: T201
    print(f'Sent {len(try_fit_type_ids)} items, received {len(type_ids)} eligible items')  # noqa: T201
    print_items(type_ids=type_ids, print_types=False)


def test_stacking(client, consts):  # noqa: ANN001, ANN201
    setup_eve_data(client=client, data=client._get_default_eve_data())  # noqa: SLF001
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.hisec)
    api_fit_m = api_sol.create_fit()
    api_fit_m.set_character(type_id=1373)
    for eve_skill_id in get_skill_type_ids():
        api_fit_m.add_skill(type_id=eve_skill_id, level=5)
    api_male = api_fit_m.set_ship(type_id=11186)  # Malediction
    for _ in range(2):
        api_fit_s = api_sol.create_fit()
        api_fit_s.set_character(type_id=1373)
        for eve_skill_id in get_skill_type_ids():
            api_fit_s.add_skill(type_id=eve_skill_id, level=5)
        api_fit_s.set_ship(type_id=640)  # Scorpion
        for _ in range(8):
            api_rsb = api_fit_s.add_module(
                type_id=1964,
                rack=consts.ApiRack.mid,
                state=consts.ApiModuleState.active,
                charge_type_id=29011)
            api_rsb.change_module(add_projs=[api_male.id])
    # Actual value captured ingame
    assert api_male.update().attrs[564].dogma == approx(5264.18055777, accuracy=9)


def test_item_attrs(client, consts):  # noqa: ANN001, ANN201
    setup_eve_data(client=client, data=client._get_default_eve_data())  # noqa: SLF001
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.hisec)
    api_fit = api_sol.create_fit()
    api_fit.set_character(type_id=1373)
    for eve_skill_id in get_skill_type_ids():
        api_fit.add_skill(type_id=eve_skill_id, level=5)
    api_ship = api_fit.set_ship(type_id=11184)  # Crusader
    api_ship.update()
    attrs1 = api_ship.update().attrs
    api_fit.add_fw_effect(type_id=87949)  # Plasma stability generator
    attrs2 = api_ship.update().attrs
    print_attr_diff(attrs1=attrs1, attrs2=attrs2)


def setup_eve_data(*, client, data) -> None:  # noqa: ANN001
    files = [
        'fsd_built/types.json',
        'fsd_built/groups.json',
        'fsd_built/typelist.json',
        'fsd_built/dogmaattributes.json',
        'fsd_built/typedogma.json',
        'fsd_built/dogmaeffects.json',
        'fsd_lite/fighterabilities.json',
        'fsd_lite/fighterabilitiesbytype.json',
        'fsd_lite/dbuffcollections.json',
        'fsd_built/spacecomponentsbytype.json',
        'fsd_built/requiredskillsfortypes.json',
        'fsd_built/dynamicitemattributes.json']
    for file in files:
        with (PHOBOS_BASE_PATH / file).open() as f:
            client._EveDataServer__setup_handler(url=f'/{data.alias}/{file}', data=f.read())  # noqa: SLF001
    client.create_source(data=data, cleanup_check=False)


def get_skill_type_ids() -> list[int]:
    with (SCRIPT_FOLDER_PATH / 'skill_type_ids.txt').open() as f:
        return [int(line) for line in f.readlines() if line]


def get_try_fit_type_ids() -> list[int]:
    with (SCRIPT_FOLDER_PATH / 'try_fit_type_ids.txt').open() as f:
        return [int(line) for line in f.readlines() if line]


def print_items(*, type_ids: list[int], print_types: bool = False) -> None:
    with (PHOBOS_BASE_PATH / 'fsd_built' / 'types.json').open() as f:
        item_id_item_name_map = {}
        item_id_group_id_map = {}
        for entry in json.load(f).values():
            item_id_item_name_map[entry['typeID']] = entry['typeName']
            item_id_group_id_map[entry['typeID']] = entry['groupID']
    with (PHOBOS_BASE_PATH / 'fsd_built' / 'groups.json').open() as f:
        group_id_group_name_map = {}
        group_id_category_id_map = {}
        for entry in json.load(f).values():
            group_id_group_name_map[entry['groupID']] = entry['groupName']
            group_id_category_id_map[entry['groupID']] = entry['categoryID']
    with (PHOBOS_BASE_PATH / 'fsd_built' / 'categories.json').open() as f:
        category_id_category_name_map = {}
        for entry in json.load(f).values():
            category_id_category_name_map[entry['categoryID']] = entry['categoryName']
    item_id_group_name_map = {
        item_id: group_id_group_name_map[group_id]
        for item_id, group_id in item_id_group_id_map.items()}
    item_id_category_name_map = {
        item_id: category_id_category_name_map[group_id_category_id_map[group_id]]
        for item_id, group_id in item_id_group_id_map.items()}

    def sorter(item_id: int) -> tuple[str, str, str]:
        return item_id_category_name_map[item_id], item_id_group_name_map[item_id], item_id_item_name_map[item_id]

    if print_types:
        for item_id in sorted(type_ids, key=sorter):
            line = (f'  {item_id_category_name_map[item_id]}'
                    f' / {item_id_group_name_map[item_id]}'
                    f' / {item_id_item_name_map[item_id]}')
            print(line)  # noqa: T201
    else:
        seen_lines = set()
        for item_id in sorted(type_ids, key=sorter):
            line = f'  {item_id_category_name_map[item_id]} / {item_id_group_name_map[item_id]}'
            if line in seen_lines:
                continue
            seen_lines.add(line)
            print(line)  # noqa: T201


def print_attrs(*, api_item: Item) -> None:
    item_id_item_name_map = {}
    with (PHOBOS_BASE_PATH / 'fsd_built' / 'types.json').open() as f:
        for entry in json.load(f).values():
            item_id_item_name_map[entry['typeID']] = entry['typeName']
    attr_id_attr_name_map = {}
    with (PHOBOS_BASE_PATH / 'fsd_built' / 'dogmaattributes.json').open() as f:
        for entry in json.load(f).values():
            attr_id_attr_name_map[entry['attributeID']] = entry['name']
    print('---')  # noqa: T201
    print(f'{item_id_item_name_map[api_item.type_id]}:')  # noqa: T201
    for attr_id in sorted(api_item.attrs, key=lambda i: attr_id_attr_name_map[i]):
        attr_name = attr_id_attr_name_map[attr_id]
        attr_val = api_item.attrs[attr_id].extra
        print(f'  {attr_name}: {attr_val}')  # noqa: T201


def print_attr_diff(*, attrs1: dict, attrs2: dict) -> None:
    item_id_item_name_map = {}
    with (PHOBOS_BASE_PATH / 'fsd_built' / 'types.json').open() as f:
        for entry in json.load(f).values():
            item_id_item_name_map[entry['typeID']] = entry['typeName']
    attr_id_attr_name_map = {}
    with (PHOBOS_BASE_PATH / 'fsd_built' / 'dogmaattributes.json').open() as f:
        for entry in json.load(f).values():
            attr_id_attr_name_map[entry['attributeID']] = entry['name']
    print('---')  # noqa: T201
    attr_ids = set(attrs1.keys()) | set(attrs2.keys())
    for attr_id in sorted(attr_ids, key=lambda i: attr_id_attr_name_map[i]):
        try:
            attr1_val = attrs1[attr_id].extra
        except KeyError:
            attr1_val = None
        try:
            attr2_val = attrs2[attr_id].extra
        except KeyError:
            attr2_val = None
        if attr1_val != attr2_val:
            attr_name = attr_id_attr_name_map[attr_id]
            print(f'  {attr_name}: {attr1_val} -> {attr2_val}')  # noqa: T201
