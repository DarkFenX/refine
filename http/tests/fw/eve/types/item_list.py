from __future__ import annotations

import typing

from tests.fw.util import conditional_insert, make_repr_str

if typing.TYPE_CHECKING:
    from tests.fw.eve.containers.primitives import EvePrimitives
    from tests.fw.util import Absent


class ItemList:

    def __init__(
            self, *,
            id_: int,
            included_type_ids: list[int] | type[Absent],
            included_group_ids: list[int] | type[Absent],
            included_category_ids: list[int] | type[Absent],
            excluded_type_ids: list[int] | type[Absent],
            excluded_group_ids: list[int] | type[Absent],
            excluded_category_ids: list[int] | type[Absent],
    ) -> None:
        self.id = id_
        self.included_type_ids = included_type_ids
        self.included_group_ids = included_group_ids
        self.included_category_ids = included_category_ids
        self.excluded_type_ids = excluded_type_ids
        self.excluded_group_ids = excluded_group_ids
        self.excluded_category_ids = excluded_category_ids

    def to_primitives(self, *, primitive_data: EvePrimitives) -> None:
        item_list_entry = {}
        conditional_insert(container=item_list_entry, key='includedTypeIDs', value=self.included_type_ids)
        conditional_insert(container=item_list_entry, key='includedGroupIDs', value=self.included_group_ids)
        conditional_insert(container=item_list_entry, key='includedCategoryIDs', value=self.included_category_ids)
        conditional_insert(container=item_list_entry, key='excludedTypeIDs', value=self.excluded_type_ids)
        conditional_insert(container=item_list_entry, key='excludedGroupIDs', value=self.excluded_group_ids)
        conditional_insert(container=item_list_entry, key='excludedCategoryIDs', value=self.excluded_category_ids)
        primitive_data.typelist[self.id] = item_list_entry

    def __repr__(self) -> str:
        return make_repr_str(instance=self)
