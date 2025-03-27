from __future__ import annotations

import typing

from tests.fw.util import conditional_insert, make_repr_str

if typing.TYPE_CHECKING:
    from tests.fw.util import Absent


class EffectModifier:

    def __init__(
            self, *,
            func: str | type[Absent],
            location: str | type[Absent],
            group: int | type[Absent],
            skill_req: int | type[Absent],
            affector_attr_id: int | type[Absent],
            affectee_attr_id: int | type[Absent],
            operation: int | type[Absent],
    ) -> None:
        self.func = func
        self.location = location
        self.group = group
        self.skill_req = skill_req
        self.affector_attr_id = affector_attr_id
        self.affectee_attr_id = affectee_attr_id
        self.operation = operation

    def to_primitives(self) -> dict:
        mod_entry = {}
        conditional_insert(container=mod_entry, path=['func'], value=self.func)
        conditional_insert(container=mod_entry, path=['domain'], value=self.location)
        conditional_insert(container=mod_entry, path=['groupID'], value=self.group)
        conditional_insert(container=mod_entry, path=['skillTypeID'], value=self.skill_req)
        conditional_insert(container=mod_entry, path=['modifyingAttributeID'], value=self.affector_attr_id)
        conditional_insert(container=mod_entry, path=['modifiedAttributeID'], value=self.affectee_attr_id)
        conditional_insert(container=mod_entry, path=['operation'], value=self.operation)
        return mod_entry

    def __repr__(self) -> str:
        return make_repr_str(instance=self)
