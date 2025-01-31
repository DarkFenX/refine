from __future__ import annotations

from typing import TYPE_CHECKING

from tests.fw.util import conditional_insert, make_repr_str

if TYPE_CHECKING:
    from tests.fw.util import Absent


class BuffModifier:

    def __init__(
            self, *,
            attr_id: int | type[Absent],
            group_id: int | type[Absent],
            skill_id: int | type[Absent],
    ):
        self.attr_id = attr_id
        self.group_id = group_id
        self.skill_id = skill_id

    def to_primitives(self) -> dict:
        mod_entry = {}
        conditional_insert(container=mod_entry, key='dogmaAttributeID', value=self.attr_id)
        conditional_insert(container=mod_entry, key='groupID', value=self.group_id)
        conditional_insert(container=mod_entry, key='skillID', value=self.skill_id)
        return mod_entry

    def __repr__(self) -> str:
        return make_repr_str(instance=self)
