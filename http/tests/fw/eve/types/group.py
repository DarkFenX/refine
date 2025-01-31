from __future__ import annotations

from typing import TYPE_CHECKING

from tests.fw.util import Absent, conditional_insert, make_repr_str
from .exception import TestDataConsistencyError

if TYPE_CHECKING:
    from tests.fw.eve.containers.primitives import EvePrimitives


class Group:

    def __init__(
            self, *,
            id_: int,
            category_id: int | type[Absent],
    ):
        self.id = id_
        self.category_id = category_id

    def to_primitives(self, *, primitive_data: EvePrimitives) -> None:
        # Groups are duplicated in test object data container. Here, we "deduplicate" them
        if self.id in primitive_data.groups:
            existing_entry = primitive_data.groups[self.id]
            if ((self.category_id is Absent and 'categoryID' in existing_entry) or
                    (self.category_id is not Absent and existing_entry.get('categoryID', Absent) != self.category_id)):
                raise TestDataConsistencyError('attempt to add group which already exists and has different category')
        group_entry = {'groupID': self.id}
        conditional_insert(container=group_entry, key='categoryID', value=self.category_id, cast_to=int)
        primitive_data.groups[self.id] = group_entry

    def __repr__(self) -> str:
        return make_repr_str(instance=self)
