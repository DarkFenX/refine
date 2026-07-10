import typing
from dataclasses import dataclass

from fw.eve.exception import TestDataConsistencyError
from fw.util import conditional_insert

if typing.TYPE_CHECKING:
    from fw.eve.containers.primitives import EvePrimitives
    from fw.util import Absent
    from .buff_modifier import BuffModifier


@dataclass(kw_only=True)
class Buff:

    id: int
    aggregate_mode: str | type[Absent]
    operation_name: str | type[Absent]
    item_modifiers: list[BuffModifier] | type[Absent]
    location_modifiers: list[BuffModifier] | type[Absent]
    location_group_modifiers: list[BuffModifier] | type[Absent]
    location_skillreq_modifiers: list[BuffModifier] | type[Absent]

    def to_primitives(self, *, primitive_data: EvePrimitives) -> None:
        effect_entry = {}
        conditional_insert(container=effect_entry, path=['aggregateMode'], value=self.aggregate_mode, cast_to=str)
        conditional_insert(container=effect_entry, path=['operationName'], value=self.operation_name, cast_to=str)
        conditional_insert(container=effect_entry, path=['itemModifiers'], value=(
            [m.to_primitives() for m in self.item_modifiers]
            if isinstance(self.item_modifiers, list)
            else self.item_modifiers))
        conditional_insert(container=effect_entry, path=['locationModifiers'], value=(
            [m.to_primitives() for m in self.location_modifiers]
            if isinstance(self.location_modifiers, list)
            else self.location_modifiers))
        conditional_insert(container=effect_entry, path=['locationGroupModifiers'], value=(
            [m.to_primitives() for m in self.location_group_modifiers]
            if isinstance(self.location_group_modifiers, list)
            else self.location_group_modifiers))
        conditional_insert(container=effect_entry, path=['locationRequiredSkillModifiers'], value=(
            [m.to_primitives() for m in self.location_skillreq_modifiers]
            if isinstance(self.location_skillreq_modifiers, list)
            else self.location_skillreq_modifiers))
        if self.id in primitive_data.dbuffcollections:
            msg = f'attempt to add buff with duplicate ID {self.id}'
            raise TestDataConsistencyError(msg)
        primitive_data.dbuffcollections[self.id] = effect_entry
