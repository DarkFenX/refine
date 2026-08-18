import dataclasses
import typing

from fw.api.commands import BaseCommand
from fw.util import conditional_insert

if typing.TYPE_CHECKING:
    from fw.consts import ApiEffMode
    from fw.util import Absent


@dataclasses.dataclass(kw_only=True)
class BaseCtlStanceCmd(BaseCommand):

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
class ItemCtlStanceSetCmd(BaseCtlStanceCmd):

    fit_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'stance'
        body['fit_id'] = self.fit_id
        return body


@dataclasses.dataclass(kw_only=True)
class FitCtlStanceSetCmd(BaseCtlStanceCmd):

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'stance_set'
        return body


@dataclasses.dataclass(kw_only=True)
class SolCtlStanceSetCmd(BaseCtlStanceCmd):

    fit_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'stance_set'
        body['fit_id'] = self.fit_id
        return body


####################################################################################################
# Changing
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class ItemCtlStanceChangeCmd(BaseCtlStanceCmd):

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'stance'
        return body


@dataclasses.dataclass(kw_only=True)
class FitCtlStanceChangeCmd(BaseCtlStanceCmd):

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'stance_change'
        return body


@dataclasses.dataclass(kw_only=True)
class SolCtlStanceChangeViaItemIdCmd(BaseCtlStanceCmd):

    item_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'stance_change'
        body['item_id'] = self.item_id
        return body


@dataclasses.dataclass(kw_only=True)
class SolCtlStanceChangeViaFitIdCmd(BaseCtlStanceCmd):

    fit_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'stance_change'
        body['fit_id'] = self.fit_id
        return body


####################################################################################################
# Unsetting
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class BaseCtlStanceUnsetCmd(BaseCommand):

    @typing.override
    def serialize(self) -> dict:
        return {'type': 'stance_unset'}


@dataclasses.dataclass(kw_only=True)
class FitCtlStanceUnsetCmd(BaseCtlStanceUnsetCmd):
    ...


@dataclasses.dataclass(kw_only=True)
class SolCtlStanceUnsetCmd(BaseCtlStanceUnsetCmd):

    fit_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['fit_id'] = self.fit_id
        return body
