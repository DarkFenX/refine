import dataclasses
import typing

from fw.api.commands import BaseCommand
from fw.api.commands.helpers import process_effect_map_request
from fw.util import conditional_insert

if typing.TYPE_CHECKING:
    from fw.consts import ApiEffMode
    from fw.util import Absent


@dataclasses.dataclass(kw_only=True)
class BaseCtlShipCmd(BaseCommand):

    type_id: int | type[Absent]
    state: bool | type[Absent]
    coordinates: tuple[float, float, float] | type[Absent]
    movement: tuple[float, float, float] | type[Absent]
    effect_modes: dict[int | str, ApiEffMode] | type[Absent]

    def serialize(self) -> dict:
        body = super().serialize()
        conditional_insert(container=body, path=['type_id'], value=self.type_id)
        conditional_insert(container=body, path=['state'], value=self.state)
        conditional_insert(container=body, path=['coordinates'], value=self.coordinates)
        conditional_insert(container=body, path=['movement'], value=self.movement)
        conditional_insert(
            container=body,
            path=['effect_modes'],
            value=process_effect_map_request(effect_map=self.effect_modes))
        return body


####################################################################################################
# Setting
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class ItemCtlShipSetCmd(BaseCtlShipCmd):

    fit_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'ship'
        body['fit_id'] = self.fit_id
        return body


@dataclasses.dataclass(kw_only=True)
class FitCtlShipSetCmd(BaseCtlShipCmd):

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'ship_set'
        return body


@dataclasses.dataclass(kw_only=True)
class SolCtlShipSetCmd(BaseCtlShipCmd):

    fit_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'ship_set'
        body['fit_id'] = self.fit_id
        return body


####################################################################################################
# Changing
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class ItemCtlShipChangeCmd(BaseCtlShipCmd):

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'ship'
        return body


@dataclasses.dataclass(kw_only=True)
class FitCtlShipChangeCmd(BaseCtlShipCmd):

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'ship_change'
        return body


@dataclasses.dataclass(kw_only=True)
class SolCtlShipChangeViaItemIdCmd(BaseCtlShipCmd):

    item_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'ship_change'
        body['item_id'] = self.item_id
        return body


@dataclasses.dataclass(kw_only=True)
class SolCtlShipChangeViaFitIdCmd(BaseCtlShipCmd):

    fit_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'ship_change'
        body['fit_id'] = self.fit_id
        return body


####################################################################################################
# Unsetting
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class BaseCtlShipUnsetCmd(BaseCommand):

    @typing.override
    def serialize(self) -> dict:
        return {'type': 'ship_unset'}


@dataclasses.dataclass(kw_only=True)
class FitCtlShipUnsetCmd(BaseCtlShipUnsetCmd):
    ...


@dataclasses.dataclass(kw_only=True)
class SolCtlShipUnsetCmd(BaseCtlShipUnsetCmd):

    fit_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['fit_id'] = self.fit_id
        return body
