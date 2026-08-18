import dataclasses
import typing

from fw.api.commands import BaseCommand
from fw.util import conditional_insert

if typing.TYPE_CHECKING:
    from fw.util import Absent


####################################################################################################
# Adding
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class BaseCtlFleetAddCmd(BaseCommand):

    fit_ids: list[str] | type[Absent]

    def serialize(self) -> dict:
        body = super().serialize()
        conditional_insert(container=body, path=['fit_ids'], value=self.fit_ids)
        return body


@dataclasses.dataclass(kw_only=True)
class FleetCtlFleetAddCmd(BaseCtlFleetAddCmd):
    ...


@dataclasses.dataclass(kw_only=True)
class SolCtlFleetAddCmd(BaseCtlFleetAddCmd):

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'fleet_add'
        return body


####################################################################################################
# Changing
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class BaseCtlFleetChangeCmd(BaseCommand):

    add_fit_ids: list[str] | type[Absent]
    rm_fit_ids: list[str] | type[Absent]

    def serialize(self) -> dict:
        body = super().serialize()
        conditional_insert(container=body, path=['add_fit_ids'], value=self.add_fit_ids)
        conditional_insert(container=body, path=['rm_fit_ids'], value=self.rm_fit_ids)
        return body


@dataclasses.dataclass(kw_only=True)
class FleetCtlFleetChangeCmd(BaseCtlFleetChangeCmd):
    ...


@dataclasses.dataclass(kw_only=True)
class SolCtlFleetChangeCmd(BaseCtlFleetChangeCmd):

    fleet_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'fleet_change'
        body['fleet_id'] = self.fleet_id
        return body


####################################################################################################
# Removing
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class SolCtlFleetRemoveCmd(BaseCommand):

    fleet_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'fleet_remove'
        body['fleet_id'] = self.fleet_id
        return body
