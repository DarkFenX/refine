from __future__ import annotations

import typing
from dataclasses import dataclass

from tests.fw.eve.exception import TestDataConsistencyError
from tests.fw.util import Absent, conditional_insert

if typing.TYPE_CHECKING:
    from tests.fw.eve.containers.primitives import EvePrimitives


@dataclass(kw_only=True)
class Group:

    id: int
    category_id: int | type[Absent]

    def to_primitives(self, *, primitive_data: EvePrimitives) -> None:
        # Groups are duplicated in test object data container. Here, we "deduplicate" them
        if self.id in primitive_data.groups:
            existing_entry = primitive_data.groups[self.id]
            if ((self.category_id is Absent and 'categoryID' in existing_entry) or
                    (self.category_id is not Absent and existing_entry.get('categoryID', Absent) != self.category_id)):
                msg = 'attempt to add group which already exists and has different category'
                raise TestDataConsistencyError(msg)
        group_entry = {'groupID': self.id}
        conditional_insert(container=group_entry, path=['categoryID'], value=self.category_id, cast_to=int)
        primitive_data.groups[self.id] = group_entry
