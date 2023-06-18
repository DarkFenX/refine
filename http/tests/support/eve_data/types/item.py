from typing import Union

from tests.support.util import Absent, conditional_insert, make_repr_str
from .exception import TestDataConsistencyError


class Item:

    def __init__(
            self,
            id_: int,
            group_id: Union[int, Absent],
            attributes: Union[dict[int, float], Absent],
            effect_ids: Union[list[int], tuple[int], Absent],
            default_effect_id: Union[int, None],
            skill_reqs: Union[dict[int, int], Absent],
            capacity: Union[float, Absent],
            mass: Union[float, Absent],
            radius: Union[float, Absent],
            volume: Union[float, Absent],
    ):
        self.id = id_
        self.group_id = group_id
        self.attributes = attributes
        self.effect_ids = effect_ids
        self.default_effect_id = default_effect_id
        self.skill_reqs = skill_reqs
        self.capacity = capacity
        self.mass = mass
        self.radius = radius
        self.volume = volume

    def to_primitives(self, primitive_data):
        item_entry = {'typeID': self.id}
        conditional_insert(item_entry, 'groupID', self.group_id)
        self.__add_primitive_item_attributes(primitive_data)
        self.__add_primitive_item_effects(primitive_data)
        conditional_insert(primitive_data.requiredskillsfortypes, self.id, self.skill_reqs)
        conditional_insert(item_entry, 'capacity', self.capacity)
        conditional_insert(item_entry, 'mass', self.mass)
        conditional_insert(item_entry, 'radius', self.radius)
        conditional_insert(item_entry, 'volume', self.volume)
        if self.id in primitive_data.types:
            raise TestDataConsistencyError(f'attempt to add item with duplicate ID {self.id}')
        primitive_data.types[self.id] = item_entry

    def __add_primitive_item_attributes(self, primitive_data):
        if self.attributes is Absent:
            return
        item_entry = primitive_data.typedogma.setdefault(self.id, {})
        if isinstance(self.attributes, dict):
            attrs_entry = item_entry['dogmaAttributes'] = []
            for attr_id, attr_val in self.attributes.items():
                attrs_entry.append({'attributeID': attr_id, 'value': attr_val})
        else:
            item_entry['dogmaAttributes'] = self.attributes

    def __add_primitive_item_effects(self, primitive_data):
        if self.effect_ids is Absent:
            return
        item_entry = primitive_data.typedogma.setdefault(self.id, {})
        if isinstance(self.effect_ids, (tuple, list, set)):
            effects_entry = item_entry['dogmaEffects'] = []
            for effect_id in self.effect_ids:
                effects_entry.append({'effectID': effect_id, 'isDefault': int(effect_id == self.default_effect_id)})
        else:
            item_entry['dogmaEffects'] = self.effect_ids

    def __repr__(self) -> str:
        return make_repr_str(self)
