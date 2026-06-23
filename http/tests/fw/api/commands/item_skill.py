import dataclasses
import typing

from fw.util import conditional_insert
from .base import BaseCommand

if typing.TYPE_CHECKING:
    from fw.consts import ApiEffMode
    from fw.util import Absent


@dataclasses.dataclass(kw_only=True)
class BaseSkillCmd(BaseCommand):

    type_id: int | type[Absent]
    level: int | type[Absent]
    state: bool | type[Absent]
    effect_modes: dict[str, ApiEffMode] | type[Absent]

    def serialize(self) -> dict:
        body = {'type': 'skill'}
        conditional_insert(container=body, path=['type_id'], value=self.type_id)
        conditional_insert(container=body, path=['level'], value=self.level)
        conditional_insert(container=body, path=['state'], value=self.state)
        conditional_insert(container=body, path=['effect_modes'], value=self.effect_modes)
        return body


####################################################################################################
# Addition
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class ItemSkillAddCmd(BaseSkillCmd):

    fit_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'skill'
        body['fit_id'] = self.fit_id
        return body


@dataclasses.dataclass(kw_only=True)
class FitSkillAddCmd(BaseSkillCmd):

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'add_skill'
        return body


@dataclasses.dataclass(kw_only=True)
class SolSkillAddCmd(BaseSkillCmd):

    fit_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'add_skill'
        body['fit_id'] = self.fit_id
        return body


####################################################################################################
# Changing
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class ItemSkillChangeCmd(BaseSkillCmd):

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'skill'
        return body


@dataclasses.dataclass(kw_only=True)
class FitSkillChangeCmd(BaseSkillCmd):

    item_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'change_skill'
        body['item_id'] = self.item_id
        return body


@dataclasses.dataclass(kw_only=True)
class SolSkillChangeCmd(BaseSkillCmd):

    item_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'change_skill'
        body['item_id'] = self.item_id
        return body
