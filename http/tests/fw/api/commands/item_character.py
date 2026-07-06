import dataclasses
import typing

from fw.util import conditional_insert
from .base import BaseCommand

if typing.TYPE_CHECKING:
    from fw.consts import ApiEffMode
    from fw.util import Absent


@dataclasses.dataclass(kw_only=True)
class BaseCharacterCmd(BaseCommand):

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
# Setting
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class ItemCharacterSetCmd(BaseCharacterCmd):

    fit_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'character'
        body['fit_id'] = self.fit_id
        return body


@dataclasses.dataclass(kw_only=True)
class FitCharacterSetCmd(BaseCharacterCmd):

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'set_character'
        return body


@dataclasses.dataclass(kw_only=True)
class SolCharacterSetCmd(BaseCharacterCmd):

    fit_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'set_character'
        body['fit_id'] = self.fit_id
        return body


####################################################################################################
# Changing
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class ItemCharacterChangeCmd(BaseCharacterCmd):

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'character'
        return body


@dataclasses.dataclass(kw_only=True)
class FitCharacterChangeCmd(BaseCharacterCmd):

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'change_character'
        return body


@dataclasses.dataclass(kw_only=True)
class SolCharacterChangeViaItemIdCmd(BaseCharacterCmd):

    item_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'change_character'
        body['item_id'] = self.item_id
        return body


@dataclasses.dataclass(kw_only=True)
class SolCharacterChangeViaFitIdCmd(BaseCharacterCmd):

    fit_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'change_character'
        body['fit_id'] = self.fit_id
        return body


####################################################################################################
# Unsetting
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class BaseCharacterUnsetCmd(BaseCommand):

    @typing.override
    def serialize(self) -> dict:
        return {'type': 'unset_character'}


@dataclasses.dataclass(kw_only=True)
class FitCharacterUnsetCmd(BaseCharacterUnsetCmd):
    ...


@dataclasses.dataclass(kw_only=True)
class SolCharacterUnsetCmd(BaseCharacterUnsetCmd):

    fit_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['fit_id'] = self.fit_id
        return body
