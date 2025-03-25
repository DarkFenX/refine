from __future__ import annotations

import typing

from tests.fw.eve.exception import TestDataConsistencyError
from tests.fw.util import conditional_insert, make_repr_str

if typing.TYPE_CHECKING:
    from tests.fw.eve.containers.primitives import EvePrimitives
    from tests.fw.util import Absent


class Attribute:

    def __init__(
            self, *,
            id_: int,
            stackable: int | bool | type[Absent],
            high_is_good: int | bool | type[Absent],
            default_value: float | type[Absent],
            min_attribute_id: int | type[Absent],
            max_attribute_id: int | type[Absent],
            unit_id: int | type[Absent],
    ) -> None:
        self.id = id_
        self.stackable = stackable
        self.high_is_good = high_is_good
        self.default_value = default_value
        self.min_attribute_id = min_attribute_id
        self.max_attribute_id = max_attribute_id
        self.unit_id = unit_id

    def to_primitives(self, *, primitive_data: EvePrimitives) -> None:
        attr_entry = {'attributeID': self.id}
        conditional_insert(container=attr_entry, key='stackable', value=self.stackable, cast_to=int)
        conditional_insert(container=attr_entry, key='highIsGood', value=self.high_is_good, cast_to=int)
        conditional_insert(container=attr_entry, key='defaultValue', value=self.default_value, cast_to=float)
        conditional_insert(container=attr_entry, key='minAttributeID', value=self.min_attribute_id, cast_to=int)
        conditional_insert(container=attr_entry, key='maxAttributeID', value=self.max_attribute_id, cast_to=int)
        conditional_insert(container=attr_entry, key='unitID', value=self.unit_id, cast_to=int)
        if self.id in primitive_data.dogmaattributes:
            msg = f'attempt to add attribute with duplicate ID {self.id}'
            raise TestDataConsistencyError(msg)
        primitive_data.dogmaattributes[self.id] = attr_entry

    def __repr__(self) -> str:
        return make_repr_str(instance=self)
