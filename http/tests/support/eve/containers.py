from __future__ import annotations

import json
from collections import defaultdict
from typing import TYPE_CHECKING

from tests.support.consts import EveItemCat
from tests.support.util import Default
from .types import Item, Group, Attribute, Effect, Buff

if TYPE_CHECKING:
    from typing import Type, Union

    from tests.support.util import Absent
    from .types import BuffModifier, EffectModifier

ID_START = 1000000


class EveObjects:

    def __init__(self, alias: str):
        self.alias = alias
        self.items = []
        self.item_groups = []
        self.attributes = []
        self.effects = []
        self.buffs = []
        self.item_id = ID_START
        self.item_group_id = ID_START
        self.attr_id = ID_START
        self.effect_id = ID_START
        self.buff_id = ID_START
        # Helper map to ease management of groups/categories
        # Format: {group ID: [groups]}
        self.group_map = defaultdict(lambda: [])

    def alloc_item_id(self) -> int:
        id_ = self.item_id
        self.item_id += 1
        return id_

    def alloc_group_id(self) -> int:
        id_ = self.item_group_id
        self.item_group_id += 1
        return id_

    def alloc_attr_id(self) -> int:
        id_ = self.attr_id
        self.attr_id += 1
        return id_

    def alloc_effect_id(self) -> int:
        id_ = self.effect_id
        self.effect_id += 1
        return id_

    def alloc_buff_id(self) -> int:
        id_ = self.buff_id
        self.buff_id += 1
        return id_

    def mk_item(
            self,
            id_: Union[int, Type[Default]],
            group_id: Union[int, Type[Default]],
            category_id: Union[int, Type[Absent], Type[Default]],
            attributes: Union[dict[int, float], Type[Absent]],
            effect_ids: Union[list[int], tuple[int], Type[Absent]],
            default_effect_id: Union[int, None],
            skill_reqs: Union[dict[int, int], Type[Absent]],
            capacity: Union[float, Type[Absent]],
            mass: Union[float, Type[Absent]],
            radius: Union[float, Type[Absent]],
            volume: Union[float, Type[Absent]],
    ) -> Item:
        if id_ is Default:
            id_ = self.alloc_item_id()
        group = self.__fetch_or_mk_item_group(id_=group_id, category_id=category_id)
        item = Item(
            id_=id_,
            group_id=group.id,
            attributes=attributes,
            effect_ids=effect_ids,
            default_effect_id=default_effect_id,
            skill_reqs=skill_reqs,
            capacity=capacity,
            mass=mass,
            radius=radius,
            volume=volume)
        self.items.append(item)
        return item

    def mk_item_group(
            self,
            id_: Union[int, Type[Default]],
            category_id: Union[int, Type[Absent], Type[Default]],
    ) -> Group:
        if id_ is Default:
            id_ = self.alloc_group_id()
        if category_id is Default:
            category_id = EveItemCat.module
        group = Group(id_=id_, category_id=category_id)
        self.item_groups.append(group)
        self.group_map[group.id].append(group)
        return group

    def __fetch_or_mk_item_group(
            self,
            id_: Union[int, Type[Default]],
            category_id: Union[int, Type[Absent], Type[Default]],
    ) -> Group:
        # Fetch existing group if consistency is not broken:
        # - when requested category ID and group's category ID match
        # - when default category ID is requested, and we have just one group
        if id_ in self.group_map:
            groups = self.group_map[id_]
            if len(groups) == 1:
                group = groups[0]
                if category_id is Default or category_id == group.category_id:
                    return group
        return self.mk_item_group(id_=id_, category_id=category_id)

    def mk_attr(
            self,
            id_: Union[int, Type[Default]],
            stackable: Union[int, Type[Absent]],
            high_is_good: Union[int, Type[Absent]],
            default_value: Union[float, Type[Absent]],
            max_attribute_id: Union[int, Type[Absent]],
    ) -> Attribute:
        if id_ is Default:
            id_ = self.alloc_attr_id()
        attr = Attribute(
            id_=id_,
            stackable=stackable,
            high_is_good=high_is_good,
            default_value=default_value,
            max_attribute_id=max_attribute_id)
        self.attributes.append(attr)
        return attr

    def mk_effect(
            self,
            id_: Union[int, Type[Default]],
            category_id: Union[int, Type[Absent]],
            is_assistance: Union[int, Type[Absent]],
            is_offensive: Union[int, Type[Absent]],
            discharge_attribute_id: Union[int, Type[Absent]],
            duration_attribute_id: Union[int, Type[Absent]],
            range_attribute_id: Union[int, Type[Absent]],
            falloff_attribute_id: Union[int, Type[Absent]],
            tracking_attribute_id: Union[int, Type[Absent]],
            usage_chance_attribute_id: Union[int, Type[Absent]],
            resist_attribute_id: Union[int, Type[Absent]],
            modifier_info: Union[list[EffectModifier], tuple[EffectModifier], Type[Absent]],
    ) -> Effect:
        if id_ is Default:
            id_ = self.alloc_effect_id()
        effect = Effect(
            id_=id_,
            category_id=category_id,
            is_assistance=is_assistance,
            is_offensive=is_offensive,
            discharge_attribute_id=discharge_attribute_id,
            duration_attribute_id=duration_attribute_id,
            range_attribute_id=range_attribute_id,
            falloff_attribute_id=falloff_attribute_id,
            tracking_attribute_id=tracking_attribute_id,
            usage_chance_attribute_id=usage_chance_attribute_id,
            resist_attribute_id=resist_attribute_id,
            modifier_info=modifier_info)
        self.effects.append(effect)
        return effect

    def mk_buff(
            self,
            id_: Union[int, Type[Default]],
            aggregate_mode: Union[str, Type[Absent]],
            operation_name: Union[str, Type[Absent]],
            item_modifiers: Union[list[BuffModifier], tuple[BuffModifier], Type[Absent]],
            location_modifiers: Union[list[BuffModifier], tuple[BuffModifier], Type[Absent]],
            location_group_modifiers: Union[list[BuffModifier], tuple[BuffModifier], Type[Absent]],
            location_skillreq_modifiers: Union[list[BuffModifier], tuple[BuffModifier], Type[Absent]],
    ) -> Buff:
        if id_ is Default:
            id_ = self.alloc_buff_id()
        buff = Buff(
            id_=id_,
            aggregate_mode=aggregate_mode,
            operation_name=operation_name,
            item_modifiers=item_modifiers,
            location_modifiers=location_modifiers,
            location_group_modifiers=location_group_modifiers,
            location_skillreq_modifiers=location_skillreq_modifiers)
        self.buffs.append(buff)
        return buff

    def render(self) -> EveStrings:
        primitives = self.to_primitives()
        strings = primitives.to_strings()
        return strings

    def to_primitives(self) -> EvePrimitives:
        primitive_data = EvePrimitives(self.alias)
        for item in self.items:
            item.to_primitives(primitive_data)
        for item_group in self.item_groups:
            item_group.to_primitives(primitive_data)
        for attr in self.attributes:
            attr.to_primitives(primitive_data)
        for effect in self.effects:
            effect.to_primitives(primitive_data)
        for buff in self.buffs:
            buff.to_primitives(primitive_data)
        return primitive_data


class EvePrimitives:

    def __init__(self, alias: str):
        self.alias = alias
        self.types = {}
        self.groups = {}
        self.typedogma = {}
        self.dogmaattributes = {}
        self.dogmaeffects = {}
        self.fighterabilities = {}
        self.fighterabilitiesbytype = {}
        self.dbuffcollections = {}
        self.requiredskillsfortypes = {}
        self.dynamicitemattributes = {}

    def to_strings(self) -> EveStrings:
        string_data = EveStrings(self.alias)
        string_data.types = json.dumps(self.types)
        string_data.groups = json.dumps(self.groups)
        string_data.typedogma = json.dumps(self.typedogma)
        string_data.dogmaattributes = json.dumps(self.dogmaattributes)
        string_data.dogmaeffects = json.dumps(self.dogmaeffects)
        string_data.fighterabilities = json.dumps(self.fighterabilities)
        string_data.fighterabilitiesbytype = json.dumps(self.fighterabilitiesbytype)
        string_data.dbuffcollections = json.dumps(self.dbuffcollections)
        string_data.requiredskillsfortypes = json.dumps(self.requiredskillsfortypes)
        string_data.dynamicitemattributes = json.dumps(self.dynamicitemattributes)
        return string_data


class EveStrings:

    def __init__(self, alias: str):
        self.alias = alias
        self.types = ''
        self.groups = ''
        self.typedogma = ''
        self.dogmaattributes = ''
        self.dogmaeffects = ''
        self.fighterabilities = ''
        self.fighterabilitiesbytype = ''
        self.dbuffcollections = ''
        self.requiredskillsfortypes = ''
        self.dynamicitemattributes = ''
