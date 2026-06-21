import dataclasses

from fw.util import Absent, conditional_insert
from .base import Command


@dataclasses.dataclass(kw_only=True)
class BaseBoosterCmd(Command):

    type_id: int | type[Absent] = Absent
    state: bool | type[Absent] = Absent
    side_effects: dict[str, bool] | type[Absent] = Absent

    def serialize(self) -> dict:
        body = {'type': 'booster'}
        conditional_insert(container=body, path=['type_id'], value=self.type_id)
        conditional_insert(container=body, path=['state'], value=self.state)
        conditional_insert(container=body, path=['side_effects'], value=self.side_effects)
        return body


####################################################################################################
# Addition
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class FitAddBoosterCmd(BaseBoosterCmd):
    ...


@dataclasses.dataclass(kw_only=True)
class ItemAddBoosterCmd(FitAddBoosterCmd):

    fit_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['fit_id'] = self.fit_id
        return body


@dataclasses.dataclass(kw_only=True)
class SolAddBoosterCmd(ItemAddBoosterCmd):
    ...


####################################################################################################
# Changing
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class ItemChangeBoosterCmd(BaseBoosterCmd):
    ...


@dataclasses.dataclass(kw_only=True)
class FitChangeBoosterCmd(ItemChangeBoosterCmd):

    item_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['item_id'] = self.item_id
        return body


@dataclasses.dataclass(kw_only=True)
class SolChangeBoosterCmd(FitChangeBoosterCmd):
    ...
