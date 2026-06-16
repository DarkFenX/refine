"""
Finds effect info in a raw static data dump, and prints it, adding entity names next to their IDs.
"""

# Python 3.13 and earlier compatibility
from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

PHOBOS_BASE_PATH = Path('~', 'Desktop', 'phobos_tq_en-us').expanduser()


OP_MAP = {
    -1: 'PreAssign',
    0: 'PreMul',
    1: 'PreDiv',
    2: 'Add',
    3: 'Sub',
    4: 'PostMul',
    5: 'PostDiv',
    6: 'PostPercent',
    7: 'PostAssign'}

class StaticData:

    _type_id_name_map = None
    _group_id_name_map = None
    _attr_id_name_map = None
    _effect_id_name_map = None
    _effect_name_id_map = None

    @classmethod
    def check_effect_id(cls, effect_id: int) -> bool:
        cls.__ensure_effects_loaded()
        return effect_id in cls._effect_id_name_map

    @classmethod
    def get_effect_id_by_name(cls, effect_name: str) -> int | None:
        cls.__ensure_effects_loaded()
        return cls._effect_name_id_map.get(effect_name)

    @classmethod
    def get_type_name_by_id(cls, type_id: int) -> str | None:
        cls.__ensure_types_loaded()
        return cls._type_id_name_map.get(type_id)

    @classmethod
    def get_group_name_by_id(cls, group_id: int) -> str | None:
        cls.__ensure_groups_loaded()
        return cls._group_id_name_map.get(group_id)

    @classmethod
    def get_attr_name_by_id(cls, attr_id: int) -> str | None:
        cls.__ensure_attrs_loaded()
        return cls._attr_id_name_map.get(attr_id)

    @classmethod
    def get_effect_name_by_id(cls, effect_id: int) -> str | None:
        cls.__ensure_effects_loaded()
        return cls._effect_id_name_map.get(effect_id)

    @staticmethod
    def get_effect_entry(effect_id: int) -> dict:
        with (PHOBOS_BASE_PATH / 'fsd_built' / 'dogmaeffects.json').open() as f:
            return json.load(f)[str(effect_id)]

    @classmethod
    def __ensure_types_loaded(cls) -> None:
        type_id_name_map = {}
        with (PHOBOS_BASE_PATH / 'fsd_built' / 'types.json').open() as f:
            for entry in json.load(f).values():
                type_id_name_map[entry['typeID']] = entry['typeName']
        cls._type_id_name_map = type_id_name_map

    @classmethod
    def __ensure_groups_loaded(cls) -> None:
        group_id_name_map = {}
        with (PHOBOS_BASE_PATH / 'fsd_built' / 'groups.json').open() as f:
            for entry in json.load(f).values():
                group_id_name_map[entry['groupID']] = entry['groupName']
        cls._group_id_name_map = group_id_name_map

    @classmethod
    def __ensure_attrs_loaded(cls) -> None:
        attr_id_name_map = {}
        with (PHOBOS_BASE_PATH / 'fsd_built' / 'dogmaattributes.json').open() as f:
            for entry in json.load(f).values():
                attr_id_name_map[entry['attributeID']] = entry['name']
        cls._attr_id_name_map = attr_id_name_map

    @classmethod
    def __ensure_effects_loaded(cls) -> None:
        effect_id_name_map = {}
        effect_name_id_map = {}
        with (PHOBOS_BASE_PATH / 'fsd_built' / 'dogmaeffects.json').open() as f:
            for entry in json.load(f).values():
                effect_id_name_map[entry['effectID']] = entry['effectName']
                effect_name_id_map[entry['effectName']] = entry['effectID']
        cls._effect_id_name_map = effect_id_name_map
        cls._effect_name_id_map = effect_name_id_map


def get_effect_id(effect_arg: str) -> int:
    if effect_arg.isdigit():
        effect_id = int(effect_arg)
        if not StaticData.check_effect_id(effect_id):
            effect_id = None
    else:
        effect_id = StaticData.get_effect_id_by_name(effect_arg)
    if effect_id is None:
        print(f'unable to find effect "{effect_arg}"')
        sys.exit()
    return effect_id


def expand_mod_ids(modifier_info: dict):
    if group_id := modifier_info.get('groupID'):
        if group_name := StaticData.get_group_name_by_id(group_id=group_id):
            modifier_info['groupID'] = f'{group_id} {group_name}'
    if effect_id := modifier_info.get('effectID'):
        if effect_name := StaticData.get_effect_name_by_id(effect_id=effect_id):
            modifier_info['effectID'] = f'{effect_id} {effect_name}'
    if attr_id := modifier_info.get('modifiedAttributeID'):
        if attr_name := StaticData.get_attr_name_by_id(attr_id=attr_id):
            modifier_info['modifiedAttributeID'] = f'{attr_id} {attr_name}'
    if attr_id := modifier_info.get('modifyingAttributeID'):
        if attr_name := StaticData.get_attr_name_by_id(attr_id=attr_id):
            modifier_info['modifyingAttributeID'] = f'{attr_id} {attr_name}'
    if op := modifier_info.get('operation'):
        if op in OP_MAP:
            modifier_info['operation'] = f'{op} {OP_MAP[op]}'
    if type_id := modifier_info.get('skillTypeID'):
        if type_name := StaticData.get_type_name_by_id(type_id=type_id):
            modifier_info['skillTypeID'] = f'{type_id} {type_name}'



if __name__ == '__main__':
    parser = argparse.ArgumentParser(description='prints effect data in a human-readable format')
    parser.add_argument('effect', type=str, help='effect ID or name')
    args = parser.parse_args()

    effect_id = get_effect_id(args.effect)
    effect_data = StaticData.get_effect_entry(effect_id)
    for modifier_info in effect_data.get('modifierInfo', ()):
        expand_mod_ids(modifier_info=modifier_info)

    print(json.dumps(effect_data, indent=2))
