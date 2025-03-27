from __future__ import annotations

import typing
from dataclasses import dataclass

from tests.fw.util import conditional_insert

if typing.TYPE_CHECKING:
    from tests.fw.eve.containers.primitives import EvePrimitives
    from tests.fw.util import Absent


@dataclass(kw_only=True)
class ItemList:

    id: int
    included_type_ids: list[int] | type[Absent]
    included_group_ids: list[int] | type[Absent]
    included_category_ids: list[int] | type[Absent]
    excluded_type_ids: list[int] | type[Absent]
    excluded_group_ids: list[int] | type[Absent]
    excluded_category_ids: list[int] | type[Absent]

    def to_primitives(self, *, primitive_data: EvePrimitives) -> None:
        item_list_entry = {}
        conditional_insert(container=item_list_entry, path=['includedTypeIDs'], value=self.included_type_ids)
        conditional_insert(container=item_list_entry, path=['includedGroupIDs'], value=self.included_group_ids)
        conditional_insert(container=item_list_entry, path=['includedCategoryIDs'], value=self.included_category_ids)
        conditional_insert(container=item_list_entry, path=['excludedTypeIDs'], value=self.excluded_type_ids)
        conditional_insert(container=item_list_entry, path=['excludedGroupIDs'], value=self.excluded_group_ids)
        conditional_insert(container=item_list_entry, path=['excludedCategoryIDs'], value=self.excluded_category_ids)
        primitive_data.typelist[self.id] = item_list_entry
