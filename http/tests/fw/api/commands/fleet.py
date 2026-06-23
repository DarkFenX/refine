import dataclasses
import typing

from fw.util import conditional_insert
from .base import BaseCommand

if typing.TYPE_CHECKING:
    from fw.util import Absent


####################################################################################################
# Adding
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class BaseFleetAddCmd(BaseCommand):

    fits: list[str] | type[Absent]

    def serialize(self) -> dict:
        body = super().serialize()
        conditional_insert(container=body, path=['fits'], value=self.fits)
        return body


@dataclasses.dataclass(kw_only=True)
class FleetFleetAddCmd(BaseFleetAddCmd):
    ...


@dataclasses.dataclass(kw_only=True)
class SolFleetAddCmd(BaseFleetAddCmd):

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'add_fleet'
        return body


####################################################################################################
# Changing
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class BaseFleetChangeCmd(BaseCommand):

    add_fits: list[str] | type[Absent]
    rm_fits: list[str] | type[Absent]

    def serialize(self) -> dict:
        body = super().serialize()
        conditional_insert(container=body, path=['add_fits'], value=self.add_fits)
        conditional_insert(container=body, path=['rm_fits'], value=self.rm_fits)
        return body


@dataclasses.dataclass(kw_only=True)
class FleetFleetChangeCmd(BaseFleetChangeCmd):
    ...


@dataclasses.dataclass(kw_only=True)
class SolFleetChangeCmd(BaseFleetChangeCmd):

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'change_fleet'
        return body


####################################################################################################
# Removing
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class SolFleetRemoveCmd(BaseCommand):

    fleet_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'remove_fleet'
        body['fleet_id'] = self.fleet_id
        return body
