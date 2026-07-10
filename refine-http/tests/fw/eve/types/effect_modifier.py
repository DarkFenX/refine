import typing
from dataclasses import dataclass

from fw.util import conditional_insert

if typing.TYPE_CHECKING:
    from fw.util import Absent


@dataclass(kw_only=True)
class EffectModifier:

    func: str | type[Absent]
    location: str | type[Absent]
    group: int | type[Absent]
    skill_req: int | type[Absent]
    affector_attr_id: int | type[Absent]
    affectee_attr_id: int | type[Absent]
    operation: int | type[Absent]
    effect_id: int | type[Absent]

    def to_primitives(self) -> dict:
        mod_entry = {}
        conditional_insert(container=mod_entry, path=['func'], value=self.func)
        conditional_insert(container=mod_entry, path=['domain'], value=self.location)
        conditional_insert(container=mod_entry, path=['groupID'], value=self.group)
        conditional_insert(container=mod_entry, path=['skillTypeID'], value=self.skill_req)
        conditional_insert(container=mod_entry, path=['modifyingAttributeID'], value=self.affector_attr_id)
        conditional_insert(container=mod_entry, path=['modifiedAttributeID'], value=self.affectee_attr_id)
        conditional_insert(container=mod_entry, path=['operation'], value=self.operation)
        conditional_insert(container=mod_entry, path=['effectID'], value=self.effect_id)
        return mod_entry
