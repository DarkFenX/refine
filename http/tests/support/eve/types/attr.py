from __future__ import annotations

from typing import TYPE_CHECKING

from tests.support.util import conditional_insert, make_repr_str
from .exception import TestDataConsistencyError

if TYPE_CHECKING:
    from typing import Union

    from tests.support.eve.containers.primitives import EvePrimitives
    from tests.support.util import Absent


class Attribute:

    def __init__(
            self, *,
            id_: int,
            stackable: Union[int, bool, type[Absent]],
            high_is_good: Union[int, bool, type[Absent]],
            default_value: Union[float, type[Absent]],
            min_attribute_id: Union[int, type[Absent]],
            max_attribute_id: Union[int, type[Absent]],
    ):
        self.id = id_
        self.stackable = stackable
        self.high_is_good = high_is_good
        self.default_value = default_value
        self.min_attribute_id = min_attribute_id
        self.max_attribute_id = max_attribute_id

    def to_primitives(self, *, primitive_data: EvePrimitives) -> None:
        attr_entry = {'attributeID': self.id}
        conditional_insert(container=attr_entry, key='stackable', value=self.stackable, cast_to=int)
        conditional_insert(container=attr_entry, key='highIsGood', value=self.high_is_good, cast_to=int)
        conditional_insert(container=attr_entry, key='defaultValue', value=self.default_value, cast_to=float)
        conditional_insert(container=attr_entry, key='minAttributeID', value=self.min_attribute_id, cast_to=int)
        conditional_insert(container=attr_entry, key='maxAttributeID', value=self.max_attribute_id, cast_to=int)
        if self.id in primitive_data.dogmaattributes:
            raise TestDataConsistencyError(f'attempt to add attribute with duplicate ID {self.id}')
        primitive_data.dogmaattributes[self.id] = attr_entry

    def __repr__(self) -> str:
        return make_repr_str(instance=self)
