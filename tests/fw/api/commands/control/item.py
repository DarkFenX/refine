import dataclasses
import typing

from fw.api.commands import BaseCommand
from fw.util import Absent, conditional_insert

if typing.TYPE_CHECKING:
    from fw.consts import ApiModRmMode


####################################################################################################
# Addition
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class BaseCtlItemAutodetectAddCmd(BaseCommand):

    type_id: int

    def serialize(self) -> dict:
        body = super().serialize()
        body['type_id'] = self.type_id
        return body


@dataclasses.dataclass(kw_only=True)
class ItemCtlItemAutodetectAddCmd(BaseCtlItemAutodetectAddCmd):

    fit_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'autodetect'
        body['fit_id'] = self.fit_id
        return body


@dataclasses.dataclass(kw_only=True)
class FitCtlItemAutodetectAddCmd(BaseCtlItemAutodetectAddCmd):

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'item_autodetect_add'
        return body


@dataclasses.dataclass(kw_only=True)
class SolCtlItemAutodetectAddCmd(BaseCtlItemAutodetectAddCmd):

    fit_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'item_autodetect_add'
        body['fit_id'] = self.fit_id
        return body


####################################################################################################
# Removal
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class BaseCtlItemRemoveCmd(BaseCommand):

    rm_mode: ApiModRmMode | type[Absent]

    def serialize(self) -> dict:
        body = super().serialize()
        conditional_insert(container=body, path=['rm_mode'], value=self.rm_mode)
        return body


@dataclasses.dataclass(kw_only=True)
class ItemCtlItemRemoveCmd(BaseCtlItemRemoveCmd):
    ...


@dataclasses.dataclass(kw_only=True)
class FitCtlItemRemoveCmd(BaseCtlItemRemoveCmd):

    item_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'item_remove'
        body['item_id'] = self.item_id
        return body


@dataclasses.dataclass(kw_only=True)
class SolCtlItemRemoveCmd(BaseCtlItemRemoveCmd):

    item_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'item_remove'
        body['item_id'] = self.item_id
        return body
