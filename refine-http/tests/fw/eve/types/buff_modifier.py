import typing
from dataclasses import dataclass

from fw.util import conditional_insert

if typing.TYPE_CHECKING:
    from fw.util import Absent


@dataclass(kw_only=True)
class BuffModifier:

    attr_id: int | type[Absent]
    group_id: int | type[Absent]
    skill_id: int | type[Absent]

    def to_primitives(self) -> dict:
        mod_entry = {}
        conditional_insert(container=mod_entry, path=['dogmaAttributeID'], value=self.attr_id)
        conditional_insert(container=mod_entry, path=['groupID'], value=self.group_id)
        conditional_insert(container=mod_entry, path=['skillID'], value=self.skill_id)
        return mod_entry
