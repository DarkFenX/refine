from __future__ import annotations

from typing import TYPE_CHECKING

from tests.support.util import conditional_insert, make_repr_str
from .exception import TestDataConsistencyError

if TYPE_CHECKING:
    from typing import Type, Union

    from tests.support.eve_data.containers import EvePrimitives
    from tests.support.util import Absent
    from .buff_modifier import BuffModifier


class Buff:

    def __init__(
            self,
            id_: int,
            aggregate_mode: Union[str, Type[Absent]],
            operation_name: Union[str, Type[Absent]],
            item_modifiers: Union[list[BuffModifier], tuple[BuffModifier], Type[Absent]],
            location_modifiers: Union[list[BuffModifier], tuple[BuffModifier], Type[Absent]],
            location_group_modifiers: Union[list[BuffModifier], tuple[BuffModifier], Type[Absent]],
            location_skillreq_modifiers: Union[list[BuffModifier], tuple[BuffModifier], Type[Absent]],
    ):
        self.id = id_
        self.aggregate_mode = aggregate_mode
        self.operation_name = operation_name
        self.item_modifiers = item_modifiers
        self.location_modifiers = location_modifiers
        self.location_group_modifiers = location_group_modifiers
        self.location_skillreq_modifiers = location_skillreq_modifiers

    def to_primitives(self, primitive_data: EvePrimitives) -> None:
        effect_entry = {}
        conditional_insert(effect_entry, 'aggregateMode', self.aggregate_mode, cast_to=str)
        conditional_insert(effect_entry, 'operationName', self.operation_name, cast_to=str)
        conditional_insert(effect_entry, 'itemModifiers', (
            [m.to_primitives() for m in self.item_modifiers]
            if isinstance(self.item_modifiers, (list, tuple))
            else self.item_modifiers))
        conditional_insert(effect_entry, 'locationModifiers', (
            [m.to_primitives() for m in self.location_modifiers]
            if isinstance(self.location_modifiers, (list, tuple))
            else self.location_modifiers))
        conditional_insert(effect_entry, 'locationGroupModifiers', (
            [m.to_primitives() for m in self.location_group_modifiers]
            if isinstance(self.location_group_modifiers, (list, tuple))
            else self.location_group_modifiers))
        conditional_insert(effect_entry, 'locationRequiredSkillModifiers', (
            [m.to_primitives() for m in self.location_skillreq_modifiers]
            if isinstance(self.location_skillreq_modifiers, (list, tuple))
            else self.location_skillreq_modifiers))
        if self.id in primitive_data.dbuffcollections:
            raise TestDataConsistencyError(f'attempt to add buff with duplicate ID {self.id}')
        primitive_data.dbuffcollections[self.id] = effect_entry

    def __repr__(self) -> str:
        return make_repr_str(self)
