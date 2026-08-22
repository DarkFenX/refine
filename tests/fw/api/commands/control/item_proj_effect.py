import dataclasses
import typing

from fw.api.commands import BaseCommand
from fw.api.commands.helpers import process_effect_map_request
from fw.util import conditional_insert

if typing.TYPE_CHECKING:
    from fw.consts import ApiEffMode
    from fw.util import Absent


@dataclasses.dataclass(kw_only=True)
class BaseCtlProjEffectCmd(BaseCommand):

    type_id: int | type[Absent]
    state: bool | type[Absent]
    effect_modes: dict[int | str, ApiEffMode] | type[Absent]

    def serialize(self) -> dict:
        body = super().serialize()
        conditional_insert(container=body, path=['type_id'], value=self.type_id)
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
class BaseCtlProjEffectAddCmd(BaseCtlProjEffectCmd):

    proj_item_ids: list[str] | type[Absent]

    def serialize(self) -> dict:
        body = super().serialize()
        conditional_insert(container=body, path=['proj_item_ids'], value=self.proj_item_ids)
        return body


@dataclasses.dataclass(kw_only=True)
class ItemCtlProjEffectAddCmd(BaseCtlProjEffectAddCmd):

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'proj_effect'
        return body


@dataclasses.dataclass(kw_only=True)
class SolCtlProjEffectAddCmd(BaseCtlProjEffectAddCmd):

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'proj_effect_add'
        return body


####################################################################################################
# Changing
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class BaseCtlProjEffectChangeCmd(BaseCtlProjEffectCmd):

    add_proj_item_ids: list[str] | type[Absent]
    rm_proj_item_ids: list[str] | type[Absent]

    def serialize(self) -> dict:
        body = super().serialize()
        conditional_insert(container=body, path=['add_proj_item_ids'], value=self.add_proj_item_ids)
        conditional_insert(container=body, path=['rm_proj_item_ids'], value=self.rm_proj_item_ids)
        return body


@dataclasses.dataclass(kw_only=True)
class ItemCtlProjEffectChangeCmd(BaseCtlProjEffectChangeCmd):

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'proj_effect'
        return body


@dataclasses.dataclass(kw_only=True)
class SolCtlProjEffectChangeCmd(BaseCtlProjEffectChangeCmd):

    item_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'proj_effect_change'
        body['item_id'] = self.item_id
        return body
