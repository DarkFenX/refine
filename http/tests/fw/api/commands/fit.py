import dataclasses
import typing

from fw.util import conditional_insert
from .base import BaseCommand

if typing.TYPE_CHECKING:
    from fw.api.aliases import DpsProfile
    from fw.util import Absent


@dataclasses.dataclass(kw_only=True)
class BaseFitCmd(BaseCommand):

    fleet_id: str | type[Absent] | None
    sec_status: float | type[Absent]
    rah_incoming_dps: DpsProfile | type[Absent] | None

    def serialize(self) -> dict:
        body = {}
        conditional_insert(container=body, path=['fleet_id'], value=self.fleet_id)
        conditional_insert(container=body, path=['sec_status'], value=self.sec_status)
        conditional_insert(container=body, path=['rah_incoming_dps'], value=self.rah_incoming_dps)
        return body


####################################################################################################
# Adding
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class SolFitAddCmd(BaseFitCmd):

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'add_fit'
        return body


@dataclasses.dataclass(kw_only=True)
class RootFitAddCmd(BaseFitCmd):
    ...


####################################################################################################
# Changing
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class FitFitChangeCmd(BaseFitCmd):

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'change_fit'
        return body


@dataclasses.dataclass(kw_only=True)
class SolFitChangeCmd(BaseFitCmd):

    fit_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'change_fit'
        body['fit_id'] = self.fit_id
        return body


####################################################################################################
# Removing
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class SolFitRemoveCmd(BaseCommand):

    fit_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'remove_fit'
        body['fit_id'] = self.fit_id
        return body
