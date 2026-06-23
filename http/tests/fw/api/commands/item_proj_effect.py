import dataclasses
import typing

from fw.util import conditional_insert
from .base import BaseCommand

if typing.TYPE_CHECKING:
    from fw.consts import ApiEffMode
    from fw.util import Absent


@dataclasses.dataclass(kw_only=True)
class BaseProjEffectCmd(BaseCommand):

    type_id: int | type[Absent]
    state: bool | type[Absent]
    effect_modes: dict[str, ApiEffMode] | type[Absent]

    def serialize(self) -> dict:
        body = super().serialize()
        conditional_insert(container=body, path=['type_id'], value=self.type_id)
        conditional_insert(container=body, path=['state'], value=self.state)
        conditional_insert(container=body, path=['effect_modes'], value=self.effect_modes)
        return body


####################################################################################################
# Addition
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class BaseProjEffectAddCmd(BaseProjEffectCmd):

    projs: list[str] | type[Absent]

    def serialize(self) -> dict:
        body = super().serialize()
        conditional_insert(container=body, path=['projs'], value=self.projs)
        return body


@dataclasses.dataclass(kw_only=True)
class ItemProjEffectAddCmd(BaseProjEffectAddCmd):

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'proj_effect'
        return body


@dataclasses.dataclass(kw_only=True)
class SolProjEffectAddCmd(BaseProjEffectAddCmd):

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'add_proj_effect'
        return body


####################################################################################################
# Changing
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class BaseProjEffectChangeCmd(BaseProjEffectCmd):

    add_projs: list[str] | type[Absent]
    rm_projs: list[str] | type[Absent]

    def serialize(self) -> dict:
        body = super().serialize()
        conditional_insert(container=body, path=['add_projs'], value=self.add_projs)
        conditional_insert(container=body, path=['rm_projs'], value=self.rm_projs)
        return body


@dataclasses.dataclass(kw_only=True)
class ItemProjEffectChangeCmd(BaseProjEffectChangeCmd):

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'proj_effect'
        return body


@dataclasses.dataclass(kw_only=True)
class SolProjEffectChangeCmd(BaseProjEffectChangeCmd):

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'change_proj_effect'
        return body
