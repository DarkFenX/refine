from __future__ import annotations

import json
from typing import Any, Union, TYPE_CHECKING

from tests.support.util import Default
from .types import Item, Group, Attribute, Effect, Buff

if TYPE_CHECKING:
    from tests.support.util import Absent
    from .types import Modifier

ID_START = 1000000


class TestObjects:

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

    def mk_item(
            self,
            id_: Union[int, Default],
            group_id: Union[int, Default],
            category_id: Union[int, Absent, Default],
            attributes: Union[dict[int, float], Absent],
            effect_ids: Union[list[int], tuple[int], Absent],
            default_effect_id: Union[int, None],
            skill_reqs: Union[dict[int, int], Absent],
            capacity: Union[float, Absent],
            mass: Union[float, Absent],
            radius: Union[float, Absent],
            volume: Union[float, Absent],
    ) -> Item:
        if id_ is Default:
            id_ = self.item_id
            self.item_id += 1
        group = self.mk_item_group(id_=group_id, category_id=category_id)
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
            id_: Union[int, Default],
            category_id: Union[int, Absent, Default],
    ) -> Group:
        if id_ is Default:
            id_ = self.item_group_id
            self.item_group_id += 1
        group = Group(id_=id_, category_id=category_id)
        self.items.append(group)
        return group

    def mk_attr(
            self,
            id_: Union[int, Default],
            stackable: Union[int, Absent],
            high_is_good: Union[int, Absent],
            default_value: Union[float, Absent],
            max_attribute_id: Union[int, Absent],
    ) -> Attribute:
        if id_ is Default:
            id_ = self.attr_id
            self.attr_id += 1
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
            id_: Union[int, Default],
            category_id: Union[int, Absent],
            is_assistance: Union[int, Absent],
            is_offensive: Union[int, Absent],
            discharge_attribute_id: Union[int, Absent],
            duration_attribute_id: Union[int, Absent],
            range_attribute_id: Union[int, Absent],
            falloff_attribute_id: Union[int, Absent],
            tracking_attribute_id: Union[int, Absent],
            usage_chance_attribute_id: Union[int, Absent],
            resist_attribute_id: Union[int, Absent],
            modifier_info: Union[list[Modifier], tuple[Modifier], Absent],
    ) -> Effect:
        if id_ is Default:
            id_ = self.effect_id
            self.effect_id += 1
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
            id_: Union[int, Default],
            aggregate_mode: Any,
            operation_name: Any,
            item_modifiers: Any,
            location_modifiers: Any,
            location_group_modifiers: Any,
            location_skillreq_modifiers: Any,
    ) -> Buff:
        if id_ is Default:
            id_ = self.buff_id
            self.buff_id += 1
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

    def render(self) -> TestStrings:
        primitives = self.to_primitives()
        strings = primitives.to_strings()
        return strings

    def to_primitives(self) -> TestPrimitives:
        primitive_data = TestPrimitives(self.alias)
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


class TestPrimitives:

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

    def to_strings(self) -> TestStrings:
        string_data = TestStrings(self.alias)
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


class TestStrings:

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
