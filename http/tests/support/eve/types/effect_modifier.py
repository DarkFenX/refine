from __future__ import annotations

from typing import TYPE_CHECKING

from tests.support.util import conditional_insert, make_repr_str

if TYPE_CHECKING:
    from typing import Type, Union

    from tests.support.util import Absent


class EffectModifier:

    def __init__(
            self,
            func: Union[str, Type[Absent]],
            domain: Union[str, Type[Absent]],
            group: Union[int, Type[Absent]],
            skill_req: Union[int, Type[Absent]],
            affector_attr_id: Union[int, Type[Absent]],
            affectee_attr_id: Union[int, Type[Absent]],
            operation: Union[int, Type[Absent]],
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
        conditional_insert(mod_entry, 'func', self.func)
        conditional_insert(mod_entry, 'domain', self.domain)
        conditional_insert(mod_entry, 'groupID', self.group)
        conditional_insert(mod_entry, 'skillTypeID', self.skill_req)
        conditional_insert(mod_entry, 'modifyingAttributeID', self.affector_attr_id)
        conditional_insert(mod_entry, 'modifiedAttributeID', self.affectee_attr_id)
        conditional_insert(mod_entry, 'operation', self.operation)
        return mod_entry

    def __repr__(self) -> str:
        return make_repr_str(self)
