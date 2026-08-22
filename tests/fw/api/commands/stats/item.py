import dataclasses
import typing

from fw.api.commands import BaseCommand
from fw.util import conditional_insert

if typing.TYPE_CHECKING:
    from fw.api.aliases import StatsOptions
    from fw.api.types import ItemStatsOptions
    from fw.util import Absent


@dataclasses.dataclass(kw_only=True)
class BaseStatsItemCmd(BaseCommand):

    item_id: str
    item_options: StatsOptions[ItemStatsOptions] | type[Absent]

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'item_stats'
        body['item_id'] = self.item_id
        conditional_insert(container=body, path=['item_options'], value=self.item_options)
        return body


@dataclasses.dataclass(kw_only=True)
class FitStatsItemCmd(BaseStatsItemCmd):
    ...


@dataclasses.dataclass(kw_only=True)
class SolStatsItemCmd(BaseStatsItemCmd):
    ...
