import typing
from dataclasses import dataclass

from fw.eve.exception import TestDataConsistencyError
from fw.util import conditional_insert

if typing.TYPE_CHECKING:
    from fw.eve.containers.primitives import EvePrimitives
    from fw.util import Absent


@dataclass(kw_only=True)
class Attribute:

    id: int
    stackable: int | bool | type[Absent]
    high_is_good: int | bool | type[Absent]
    default_value: float | type[Absent]
    min_attribute_id: int | type[Absent]
    max_attribute_id: int | type[Absent]
    unit_id: int | type[Absent]

    def to_primitives(self, *, primitive_data: EvePrimitives) -> None:
        attr_entry = {'attributeID': self.id}
        conditional_insert(container=attr_entry, path=['stackable'], value=self.stackable, cast_to=int)
        conditional_insert(container=attr_entry, path=['highIsGood'], value=self.high_is_good, cast_to=int)
        conditional_insert(container=attr_entry, path=['defaultValue'], value=self.default_value, cast_to=float)
        conditional_insert(container=attr_entry, path=['minAttributeID'], value=self.min_attribute_id, cast_to=int)
        conditional_insert(container=attr_entry, path=['maxAttributeID'], value=self.max_attribute_id, cast_to=int)
        conditional_insert(container=attr_entry, path=['unitID'], value=self.unit_id, cast_to=int)
        if self.id in primitive_data.dogmaattributes:
            msg = f'attempt to add attribute with duplicate ID {self.id}'
            raise TestDataConsistencyError(msg)
        primitive_data.dogmaattributes[self.id] = attr_entry
