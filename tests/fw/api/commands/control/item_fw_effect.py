import dataclasses
import typing

from fw.api.commands import BaseCommand
from fw.api.commands.helpers import process_effect_map_request
from fw.util import conditional_insert

if typing.TYPE_CHECKING:
    from fw.consts import ApiEffMode
    from fw.util import Absent


@dataclasses.dataclass(kw_only=True)
class BaseCtlFwEffectCmd(BaseCommand):

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
class ItemCtlFwEffectAddCmd(BaseCtlFwEffectCmd):

    fit_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'fw_effect'
        body['fit_id'] = self.fit_id
        return body


@dataclasses.dataclass(kw_only=True)
class FitCtlFwEffectAddCmd(BaseCtlFwEffectCmd):

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'fw_effect_add'
        return body


@dataclasses.dataclass(kw_only=True)
class SolCtlFwEffectAddCmd(BaseCtlFwEffectCmd):

    fit_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'fw_effect_add'
        body['fit_id'] = self.fit_id
        return body


####################################################################################################
# Changing
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class ItemCtlFwEffectChangeCmd(BaseCtlFwEffectCmd):

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'fw_effect'
        return body


@dataclasses.dataclass(kw_only=True)
class FitCtlFwEffectChangeCmd(BaseCtlFwEffectCmd):

    item_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'fw_effect_change'
        body['item_id'] = self.item_id
        return body


@dataclasses.dataclass(kw_only=True)
class SolCtlFwEffectChangeCmd(BaseCtlFwEffectCmd):

    item_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'fw_effect_change'
        body['item_id'] = self.item_id
        return body
