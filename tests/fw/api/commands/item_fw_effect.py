import dataclasses
import typing

from fw.util import conditional_insert
from .base import BaseCommand

if typing.TYPE_CHECKING:
    from fw.consts import ApiEffMode
    from fw.util import Absent


@dataclasses.dataclass(kw_only=True)
class BaseFwEffectCmd(BaseCommand):

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
# Addition
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class ItemFwEffectAddCmd(BaseFwEffectCmd):

    fit_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'fw_effect'
        body['fit_id'] = self.fit_id
        return body


@dataclasses.dataclass(kw_only=True)
class FitFwEffectAddCmd(BaseFwEffectCmd):

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'add_fw_effect'
        return body


@dataclasses.dataclass(kw_only=True)
class SolFwEffectAddCmd(BaseFwEffectCmd):

    fit_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'add_fw_effect'
        body['fit_id'] = self.fit_id
        return body


####################################################################################################
# Changing
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class ItemFwEffectChangeCmd(BaseFwEffectCmd):

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'fw_effect'
        return body


@dataclasses.dataclass(kw_only=True)
class FitFwEffectChangeCmd(BaseFwEffectCmd):

    item_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'change_fw_effect'
        body['item_id'] = self.item_id
        return body


@dataclasses.dataclass(kw_only=True)
class SolFwEffectChangeCmd(BaseFwEffectCmd):

    item_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'change_fw_effect'
        body['item_id'] = self.item_id
        return body
