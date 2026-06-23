import dataclasses
import typing

from fw.util import conditional_insert
from .base import BaseCommand

if typing.TYPE_CHECKING:
    from fw.consts import ApiEffMode
    from fw.util import Absent


@dataclasses.dataclass(kw_only=True)
class BaseSwEffectCmd(BaseCommand):

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
class ItemSwEffectAddCmd(BaseSwEffectCmd):

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'sw_effect'
        return body


@dataclasses.dataclass(kw_only=True)
class SolSwEffectAddCmd(BaseSwEffectCmd):

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'add_sw_effect'
        return body


####################################################################################################
# Changing
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class ItemSwEffectChangeCmd(BaseSwEffectCmd):

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'sw_effect'
        return body


@dataclasses.dataclass(kw_only=True)
class SolSwEffectChangeCmd(BaseSwEffectCmd):

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'change_sw_effect'
        return body
