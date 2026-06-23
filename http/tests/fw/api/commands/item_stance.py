import dataclasses
import typing

from fw.util import conditional_insert
from .base import BaseCommand

if typing.TYPE_CHECKING:
    from fw.consts import ApiEffMode
    from fw.util import Absent


@dataclasses.dataclass(kw_only=True)
class BaseStanceCmd(BaseCommand):

    type_id: int | type[Absent]
    state: bool | type[Absent]
    effect_modes: dict[str, ApiEffMode] | type[Absent]

    def serialize(self) -> dict:
        body = {}
        conditional_insert(container=body, path=['type_id'], value=self.type_id)
        conditional_insert(container=body, path=['state'], value=self.state)
        conditional_insert(container=body, path=['effect_modes'], value=self.effect_modes)
        return body


####################################################################################################
# Setting
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class ItemStanceSetCmd(BaseStanceCmd):

    fit_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'stance'
        body['fit_id'] = self.fit_id
        return body


@dataclasses.dataclass(kw_only=True)
class FitStanceSetCmd(BaseStanceCmd):

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'set_stance'
        return body


@dataclasses.dataclass(kw_only=True)
class SolStanceSetCmd(BaseStanceCmd):

    fit_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'set_stance'
        body['fit_id'] = self.fit_id
        return body


####################################################################################################
# Changing
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class ItemStanceChangeCmd(BaseStanceCmd):

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'stance'
        return body


@dataclasses.dataclass(kw_only=True)
class FitStanceChangeViaItemIdCmd(BaseStanceCmd):

    item_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'set_stance'
        body['item_id'] = self.item_id
        return body


@dataclasses.dataclass(kw_only=True)
class FitStanceChangeViaFitIdCmd(BaseStanceCmd):

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'set_stance'
        return body


@dataclasses.dataclass(kw_only=True)
class SolStanceChangeViaItemIdCmd(BaseStanceCmd):

    item_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'set_stance'
        body['item_id'] = self.item_id
        return body


@dataclasses.dataclass(kw_only=True)
class SolStanceChangeViaFitIdCmd(BaseStanceCmd):

    fit_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'set_stance'
        body['fit_id'] = self.fit_id
        return body


####################################################################################################
# Unsetting
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class BaseStanceUnsetCmd(BaseCommand):

    @typing.override
    def serialize(self) -> dict:
        return {'type': 'unset_stance'}


@dataclasses.dataclass(kw_only=True)
class FitStanceUnsetCmd(BaseStanceUnsetCmd):
    ...


@dataclasses.dataclass(kw_only=True)
class SolStanceUnsetCmd(BaseStanceUnsetCmd):

    fit_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['fit_id'] = self.fit_id
        return body
