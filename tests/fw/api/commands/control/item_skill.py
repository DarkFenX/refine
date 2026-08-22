import dataclasses
import typing

from fw.api.commands import BaseCommand
from fw.api.commands.helpers import process_effect_map_request
from fw.util import conditional_insert

if typing.TYPE_CHECKING:
    from fw.consts import ApiEffMode
    from fw.util import Absent


@dataclasses.dataclass(kw_only=True)
class BaseCtlSkillCmd(BaseCommand):

    type_id: int | type[Absent]
    level: int | type[Absent]
    state: bool | type[Absent]
    effect_modes: dict[int | str, ApiEffMode] | type[Absent]

    def serialize(self) -> dict:
        body = super().serialize()
        conditional_insert(container=body, path=['type_id'], value=self.type_id)
        conditional_insert(container=body, path=['level'], value=self.level)
        conditional_insert(container=body, path=['state'], value=self.state)
        conditional_insert(
            container=body,
            path=['effect_modes'],
            value=process_effect_map_request(effect_map=self.effect_modes))
        return body


####################################################################################################
# Addition
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class ItemCtlSkillAddCmd(BaseCtlSkillCmd):

    fit_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'skill'
        body['fit_id'] = self.fit_id
        return body


@dataclasses.dataclass(kw_only=True)
class FitCtlSkillAddCmd(BaseCtlSkillCmd):

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'skill_add'
        return body


@dataclasses.dataclass(kw_only=True)
class SolCtlSkillAddCmd(BaseCtlSkillCmd):

    fit_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'skill_add'
        body['fit_id'] = self.fit_id
        return body


####################################################################################################
# Changing
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class ItemCtlSkillChangeCmd(BaseCtlSkillCmd):

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'skill'
        return body


@dataclasses.dataclass(kw_only=True)
class FitCtlSkillChangeCmd(BaseCtlSkillCmd):

    item_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'skill_change'
        body['item_id'] = self.item_id
        return body


@dataclasses.dataclass(kw_only=True)
class SolCtlSkillChangeCmd(BaseCtlSkillCmd):

    item_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'skill_change'
        body['item_id'] = self.item_id
        return body
