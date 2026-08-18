import dataclasses
import typing

from fw.api.commands import BaseCommand
from fw.util import conditional_insert

if typing.TYPE_CHECKING:
    from fw.api.aliases import DpsProfileAlias
    from fw.util import Absent


@dataclasses.dataclass(kw_only=True)
class BaseCtlFitCmd(BaseCommand):

    fleet_id: str | type[Absent] | None
    sec_status: float | type[Absent]
    rah_incoming_dps: DpsProfileAlias | type[Absent] | None

    def serialize(self) -> dict:
        body = super().serialize()
        conditional_insert(container=body, path=['fleet_id'], value=self.fleet_id)
        conditional_insert(container=body, path=['sec_status'], value=self.sec_status)
        conditional_insert(container=body, path=['rah_incoming_dps'], value=self.rah_incoming_dps)
        return body


####################################################################################################
# Adding
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class SolCtlFitAddCmd(BaseCtlFitCmd):

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'fit_add'
        return body


@dataclasses.dataclass(kw_only=True)
class FitCtlFitAddCmd(BaseCtlFitCmd):
    ...


####################################################################################################
# Changing
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class FitCtlFitChangeCmd(BaseCtlFitCmd):

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'fit_change'
        return body


@dataclasses.dataclass(kw_only=True)
class SolCtlFitChangeCmd(BaseCtlFitCmd):

    fit_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'fit_change'
        body['fit_id'] = self.fit_id
        return body


####################################################################################################
# Removing
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class SolCtlFitRemoveCmd(BaseCommand):

    fit_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'fit_remove'
        body['fit_id'] = self.fit_id
        return body
