import dataclasses
import typing

from fw.util import conditional_insert
from .base import BaseCommand

if typing.TYPE_CHECKING:
    from fw.consts import ApiEffMode
    from fw.util import Absent


@dataclasses.dataclass(kw_only=True)
class BaseShipCmd(BaseCommand):

    type_id: int | type[Absent]
    state: bool | type[Absent]
    coordinates: tuple[float, float, float] | type[Absent]
    movement: tuple[float, float, float] | type[Absent]
    effect_modes: dict[str, ApiEffMode] | type[Absent]

    def serialize(self) -> dict:
        body = {}
        conditional_insert(container=body, path=['type_id'], value=self.type_id)
        conditional_insert(container=body, path=['state'], value=self.state)
        conditional_insert(container=body, path=['coordinates'], value=self.coordinates)
        conditional_insert(container=body, path=['movement'], value=self.movement)
        conditional_insert(container=body, path=['effect_modes'], value=self.effect_modes)
        return body


####################################################################################################
# Setting
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class ItemShipSetCmd(BaseShipCmd):

    fit_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'ship'
        body['fit_id'] = self.fit_id
        return body


@dataclasses.dataclass(kw_only=True)
class FitShipSetCmd(BaseShipCmd):

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'set_ship'
        return body


@dataclasses.dataclass(kw_only=True)
class SolShipSetCmd(BaseShipCmd):

    fit_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'set_ship'
        body['fit_id'] = self.fit_id
        return body


####################################################################################################
# Changing
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class ItemShipChangeCmd(BaseShipCmd):

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'ship'
        return body


@dataclasses.dataclass(kw_only=True)
class FitShipChangeViaItemIdCmd(BaseShipCmd):

    item_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'set_ship'
        body['item_id'] = self.item_id
        return body


@dataclasses.dataclass(kw_only=True)
class FitShipChangeViaFitIdCmd(BaseShipCmd):

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'set_ship'
        return body


@dataclasses.dataclass(kw_only=True)
class SolShipChangeViaItemIdCmd(BaseShipCmd):

    item_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'set_ship'
        body['item_id'] = self.item_id
        return body


@dataclasses.dataclass(kw_only=True)
class SolShipChangeViaFitIdCmd(BaseShipCmd):

    fit_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'set_ship'
        body['fit_id'] = self.fit_id
        return body


####################################################################################################
# Unsetting
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class BaseShipUnsetCmd(BaseCommand):

    @typing.override
    def serialize(self) -> dict:
        return {'type': 'unset_ship'}


@dataclasses.dataclass(kw_only=True)
class FitShipUnsetCmd(BaseShipUnsetCmd):
    ...


@dataclasses.dataclass(kw_only=True)
class SolShipUnsetCmd(BaseShipUnsetCmd):

    fit_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['fit_id'] = self.fit_id
        return body
