from __future__ import annotations

import typing
from dataclasses import dataclass

from tests.fw.eve.exception import TestDataConsistencyError
from tests.fw.util import Absent, conditional_insert

if typing.TYPE_CHECKING:
    from tests.fw.eve.containers.primitives import EvePrimitives
    from .item_ability import ItemAbilityData


@dataclass(kw_only=True)
class Item:

    id: int
    group_id: int | type[Absent]
    attributes: dict[int, float] | type[Absent]
    effect_ids: list[int] | type[Absent]
    default_effect_id: int | None
    ability_data: list[ItemAbilityData] | type[Absent]
    skill_reqs: dict[int, int] | type[Absent]
    capacity: float | type[Absent]
    mass: float | type[Absent]
    radius: float | type[Absent]
    volume: float | type[Absent]

    def to_primitives(self, *, primitive_data: EvePrimitives) -> None:
        item_entry = {'typeID': self.id}
        conditional_insert(container=item_entry, path=['groupID'], value=self.group_id)
        self.__add_primitive_item_attributes(primitive_data=primitive_data)
        self.__add_primitive_item_effects(primitive_data=primitive_data)
        self.__add_primitive_item_abilities(primitive_data=primitive_data)
        conditional_insert(container=primitive_data.requiredskillsfortypes, path=[self.id], value=self.skill_reqs)
        conditional_insert(container=item_entry, path=['capacity'], value=self.capacity)
        conditional_insert(container=item_entry, path=['mass'], value=self.mass)
        conditional_insert(container=item_entry, path=['radius'], value=self.radius)
        conditional_insert(container=item_entry, path=['volume'], value=self.volume)
        if self.id in primitive_data.types:
            msg = f'attempt to add item with duplicate ID {self.id}'
            raise TestDataConsistencyError(msg)
        primitive_data.types[self.id] = item_entry

    def __add_primitive_item_attributes(self, *, primitive_data: EvePrimitives) -> None:
        if self.attributes is Absent:
            return
        item_entry = primitive_data.typedogma.setdefault(self.id, {})
        if isinstance(self.attributes, dict):
            attrs_entry = item_entry['dogmaAttributes'] = []
            for attr_id, attr_val in self.attributes.items():
                attrs_entry.append({'attributeID': attr_id, 'value': attr_val})
        else:
            item_entry['dogmaAttributes'] = self.attributes

    def __add_primitive_item_effects(self, *, primitive_data: EvePrimitives) -> None:
        if self.effect_ids is Absent:
            return
        item_entry = primitive_data.typedogma.setdefault(self.id, {})
        if isinstance(self.effect_ids, list):
            item_entry['dogmaEffects'] = [
                {'effectID': e, 'isDefault': int(e == self.default_effect_id)}
                for e in self.effect_ids]
        else:
            item_entry['dogmaEffects'] = self.effect_ids

    def __add_primitive_item_abilities(self, *, primitive_data: EvePrimitives) -> None:
        if self.ability_data is Absent:
            return
        item_entry = primitive_data.fighterabilitiesbytype.setdefault(self.id, {})
        for (i, ability_data) in enumerate(self.ability_data):
            ability_entry = {'abilityID': ability_data.id}
            conditional_insert(container=ability_entry, path=['chargeCount'], value=ability_data.charge_count)
            conditional_insert(container=ability_entry, path=['cooldownSeconds'], value=ability_data.cooldown)
            conditional_insert(container=ability_entry, path=['rearmTimeSeconds'], value=ability_data.charge_rearm_time)
            item_entry[f'abilitySlot{i}'] = ability_entry
