import dataclasses
import typing

from fw.api.commands import BaseCommand
from fw.api.commands.helpers import process_effect_map_request
from fw.util import conditional_insert

if typing.TYPE_CHECKING:
    from fw.consts import ApiEffMode
    from fw.util import Absent


@dataclasses.dataclass(kw_only=True)
class BaseCtlCharacterCmd(BaseCommand):

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
# Setting
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class ItemCtlCharacterSetCmd(BaseCtlCharacterCmd):

    fit_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'character'
        body['fit_id'] = self.fit_id
        return body


@dataclasses.dataclass(kw_only=True)
class FitCtlCharacterSetCmd(BaseCtlCharacterCmd):

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'character_set'
        return body


@dataclasses.dataclass(kw_only=True)
class SolCtlCharacterSetCmd(BaseCtlCharacterCmd):

    fit_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'character_set'
        body['fit_id'] = self.fit_id
        return body


####################################################################################################
# Changing
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class ItemCtlCharacterChangeCmd(BaseCtlCharacterCmd):

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'character'
        return body


@dataclasses.dataclass(kw_only=True)
class FitCtlCharacterChangeCmd(BaseCtlCharacterCmd):

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'character_change'
        return body


@dataclasses.dataclass(kw_only=True)
class SolCtlCharacterChangeViaItemIdCmd(BaseCtlCharacterCmd):

    item_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'character_change'
        body['item_id'] = self.item_id
        return body


@dataclasses.dataclass(kw_only=True)
class SolCtlCharacterChangeViaFitIdCmd(BaseCtlCharacterCmd):

    fit_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'character_change'
        body['fit_id'] = self.fit_id
        return body


####################################################################################################
# Unsetting
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class BaseCtlCharacterUnsetCmd(BaseCommand):

    @typing.override
    def serialize(self) -> dict:
        return {'type': 'character_unset'}


@dataclasses.dataclass(kw_only=True)
class FitCtlCharacterUnsetCmd(BaseCtlCharacterUnsetCmd):
    ...


@dataclasses.dataclass(kw_only=True)
class SolCtlCharacterUnsetCmd(BaseCtlCharacterUnsetCmd):

    fit_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['fit_id'] = self.fit_id
        return body
