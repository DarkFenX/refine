from __future__ import annotations

from typing import TYPE_CHECKING

from tests.support.util import conditional_insert, make_repr_str

if TYPE_CHECKING:
    from typing import Union

    from tests.support.util import Absent


class EffectModifier:

    def __init__(
            self, *,
            func: Union[str, type[Absent]],
            domain: Union[str, type[Absent]],
            group: Union[int, type[Absent]],
            skill_req: Union[int, type[Absent]],
            affector_attr_id: Union[int, type[Absent]],
            affectee_attr_id: Union[int, type[Absent]],
            operation: Union[int, type[Absent]],
    ):
        self.func = func
        self.domain = domain
        self.group = group
        self.skill_req = skill_req
        self.affector_attr_id = affector_attr_id
        self.affectee_attr_id = affectee_attr_id
        self.operation = operation

    def to_primitives(self) -> dict:
        mod_entry = {}
        conditional_insert(container=mod_entry, key='func', value=self.func)
        conditional_insert(container=mod_entry, key='domain', value=self.domain)
        conditional_insert(container=mod_entry, key='groupID', value=self.group)
        conditional_insert(container=mod_entry, key='skillTypeID', value=self.skill_req)
        conditional_insert(container=mod_entry, key='modifyingAttributeID', value=self.affector_attr_id)
        conditional_insert(container=mod_entry, key='modifiedAttributeID', value=self.affectee_attr_id)
        conditional_insert(container=mod_entry, key='operation', value=self.operation)
        return mod_entry

    def __repr__(self) -> str:
        return make_repr_str(instance=self)
