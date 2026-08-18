import dataclasses
import typing

from fw.api.commands import BaseCommand
from fw.util import conditional_insert

if typing.TYPE_CHECKING:
    from fw.consts import ApiEffMode
    from fw.util import Absent


@dataclasses.dataclass(kw_only=True)
class BaseCtlImplantCmd(BaseCommand):

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
class ItemCtlImplantAddCmd(BaseCtlImplantCmd):

    fit_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'implant'
        body['fit_id'] = self.fit_id
        return body


@dataclasses.dataclass(kw_only=True)
class FitCtlImplantAddCmd(BaseCtlImplantCmd):

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'implant_add'
        return body


@dataclasses.dataclass(kw_only=True)
class SolCtlImplantAddCmd(BaseCtlImplantCmd):

    fit_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'implant_add'
        body['fit_id'] = self.fit_id
        return body


####################################################################################################
# Changing
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class ItemCtlImplantChangeCmd(BaseCtlImplantCmd):

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'implant'
        return body


@dataclasses.dataclass(kw_only=True)
class FitCtlImplantChangeCmd(BaseCtlImplantCmd):

    item_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'implant_change'
        body['item_id'] = self.item_id
        return body


@dataclasses.dataclass(kw_only=True)
class SolCtlImplantChangeCmd(BaseCtlImplantCmd):

    item_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'implant_change'
        body['item_id'] = self.item_id
        return body
