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

    fit_ids: list[str] | type[Absent]

    def serialize(self) -> dict:
        body = super().serialize()
        conditional_insert(container=body, path=['fit_ids'], value=self.fit_ids)
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

    add_fit_ids: list[str] | type[Absent]
    rm_fit_ids: list[str] | type[Absent]

    def serialize(self) -> dict:
        body = super().serialize()
        conditional_insert(container=body, path=['add_fit_ids'], value=self.add_fit_ids)
        conditional_insert(container=body, path=['rm_fit_ids'], value=self.rm_fit_ids)
        return body


@dataclasses.dataclass(kw_only=True)
class FleetFleetChangeCmd(BaseFleetChangeCmd):
    ...


@dataclasses.dataclass(kw_only=True)
class SolFleetChangeCmd(BaseFleetChangeCmd):

    fleet_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'change_fleet'
        body['fleet_id'] = self.fleet_id
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
