from __future__ import annotations

from typing import TYPE_CHECKING

from tests.support.util import conditional_insert, make_repr_str

if TYPE_CHECKING:
    from typing import Type, Union

    from tests.support.util import Absent


class BuffModifier:

    def __init__(
            self,
            attr_id: Union[int, Type[Absent]],
            group_id: Union[int, Type[Absent]],
            skill_id: Union[int, Type[Absent]],
    ):
        self.attr_id = attr_id
        self.group_id = group_id
        self.skill_id = skill_id

    def to_primitives(self) -> dict:
        mod_entry = {}
        conditional_insert(mod_entry, 'dogmaAttributeID', self.attr_id)
        conditional_insert(mod_entry, 'groupID', self.group_id)
        conditional_insert(mod_entry, 'skillID', self.skill_id)
        return mod_entry

    def __repr__(self) -> str:
        return make_repr_str(self)
