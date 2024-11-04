from __future__ import annotations

from typing import TYPE_CHECKING

from tests.support.util import Absent, conditional_insert, make_repr_str
from .exception import TestDataConsistencyError

if TYPE_CHECKING:
    from typing import Type, Union

    from tests.support.eve.containers.primitives import EvePrimitives


class Item:

    def __init__(
            self, *,
            id_: int,
            group_id: Union[int, Type[Absent]],
            attributes: Union[dict[int, float], Type[Absent]],
            effect_ids: Union[list[int], tuple[int], Type[Absent]],
            default_effect_id: Union[int, None],
            skill_reqs: Union[dict[int, int], Type[Absent]],
            capacity: Union[float, Type[Absent]],
            mass: Union[float, Type[Absent]],
            radius: Union[float, Type[Absent]],
            volume: Union[float, Type[Absent]],
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

    def to_primitives(self, *, primitive_data: EvePrimitives) -> None:
        item_entry = {'typeID': self.id}
        conditional_insert(container=item_entry, key='groupID', value=self.group_id)
        self.__add_primitive_item_attributes(primitive_data=primitive_data)
        self.__add_primitive_item_effects(primitive_data=primitive_data)
        conditional_insert(container=primitive_data.requiredskillsfortypes, key=self.id, value=self.skill_reqs)
        conditional_insert(container=item_entry, key='capacity', value=self.capacity)
        conditional_insert(container=item_entry, key='mass', value=self.mass)
        conditional_insert(container=item_entry, key='radius', value=self.radius)
        conditional_insert(container=item_entry, key='volume', value=self.volume)
        if self.id in primitive_data.types:
            raise TestDataConsistencyError(f'attempt to add item with duplicate ID {self.id}')
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
        if isinstance(self.effect_ids, (tuple, list, set)):
            effects_entry = item_entry['dogmaEffects'] = []
            for effect_id in self.effect_ids:
                effects_entry.append({'effectID': effect_id, 'isDefault': int(effect_id == self.default_effect_id)})
        else:
            item_entry['dogmaEffects'] = self.effect_ids

    def __repr__(self) -> str:
        return make_repr_str(instance=self)
